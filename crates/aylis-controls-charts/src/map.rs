//! MapView: mapa de blocos (o OpenStreetMap por padrão, ou outro servidor de tiles no mesmo
//! formato). Arrastar move, a roda e a pinça dão zoom (no ponto
//! do mouse ou entre os dedos), duplo clique aproxima. Marcadores com rótulo.
//!
//! Os blocos vêm pelo [`HttpService`](aylis_core::platform::HttpService) da plataforma e
//! ficam guardados na memória. Respeite a política do servidor de tiles: o do OpenStreetMap
//! é para uso leve, com a atribuição na tela (o controle desenha) — para muito tráfego, ligue
//! em `provider` um provedor de mapa (o `TileServerProvider`, com o endereço e o crédito de
//! um servidor próprio ou pago). O servidor vem só do provedor: sem ele, o OpenStreetMap.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::f64::consts::PI;

use aylis_core::{
    Brush, Canvas, Color, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, GestureEvent, Image, PaintCtx, PointF,
    PointerButton, PointerEvent, RectF, Stroke, TextAlign, Ui, WheelEvent, properties,
};

use aylis_core::providers::{MapTileProvider, Provided, TileServer};

use crate::leaf_control;

const TILE: f64 = 256.0;
const MIN_ZOOM: f64 = 1.0;
const MAX_ZOOM: f64 = 19.0;
/// Blocos guardados na memória (os mais antigos saem).
const CACHE: usize = 600;

/// Longitude/latitude (graus) → pixels do mundo no zoom `z` (Mercator da web).
pub fn project(lat: f64, lon: f64, z: f64) -> (f64, f64) {
    let size = TILE * 2f64.powf(z);
    let lat = lat.clamp(-85.0511, 85.0511).to_radians();
    let x = (lon + 180.0) / 360.0 * size;
    let y = (1.0 - (lat.tan() + 1.0 / lat.cos()).ln() / PI) / 2.0 * size;
    (x, y)
}

/// Pixels do mundo no zoom `z` → latitude/longitude.
pub fn unproject(x: f64, y: f64, z: f64) -> (f64, f64) {
    let size = TILE * 2f64.powf(z);
    let lon = x / size * 360.0 - 180.0;
    let n = PI - 2.0 * PI * y / size;
    let lat = n.sinh().atan().to_degrees();
    (lat, lon)
}

type Key = (u8, u32, u32);

#[derive(Default)]
struct Tiles {
    images: HashMap<Key, (Image, u64)>,
    pending: HashSet<Key>,
    failed: HashSet<Key>,
    /// Pedidos montados na pintura, feitos logo depois (fora dela).
    wanted: Vec<Key>,
    use_count: u64,
}

/// Mapa interativo de blocos.
pub struct MapView {
    pub base: ControlBase,
    /// A latitude do centro do mapa, em graus (negativa ao sul).
    pub latitude: f64,
    /// A longitude do centro do mapa, em graus (negativa a oeste).
    pub longitude: f64,
    /// 1 (o mundo) a 19 (a rua); pode ser fracionário.
    pub zoom: f64,
    /// Marcadores: `lat,lon,rótulo` separados por `;`.
    pub markers: String,
    /// Quem fornece os blocos (um provedor de mapa, item 25): o endereço e o crédito são os
    /// dele; sem ele, os do OpenStreetMap ([`TileServer`]).
    pub provider: Option<ComponentId>,
    /// O crédito do servidor (lido ao pedir os blocos: a pintura não tem o `Ui`).
    attribution: RefCell<String>,
    tiles: RefCell<Tiles>,
    drag: Option<(PointF, f64, f64)>,
}

impl Default for MapView {
    fn default() -> Self {
        let mut base = ControlBase::new(480.0, 320.0);
        base.set_cursor(CursorShape::Pointer).set_clip_children(true);
        // O dedo arrasta o mapa (não rola a tela).
        base.set_pan_on_touch(false);
        Self {
            base,
            // São Paulo.
            latitude: -23.5505,
            longitude: -46.6333,
            zoom: 12.0,
            markers: String::new(),
            provider: None,
            attribution: RefCell::new(TileServer::default().attribution),
            tiles: RefCell::default(),
            drag: None,
        }
    }
}

impl MapView {
    /// Os marcadores: (latitude, longitude, rótulo).
    pub fn marker_list(&self) -> Vec<(f64, f64, String)> {
        self.markers
            .split(';')
            .filter_map(|m| {
                let mut p = m.splitn(3, ',');
                let lat = p.next()?.trim().parse().ok()?;
                let lon = p.next()?.trim().parse().ok()?;
                Some((lat, lon, p.next().unwrap_or("").trim().to_string()))
            })
            .collect()
    }

    /// Centraliza em `lat`, `lon` (e muda o zoom, se der).
    pub fn go_to(ui: &mut Ui, id: ComponentId, lat: f64, lon: f64, zoom: Option<f64>) {
        if let Some(m) = ui.get_mut::<MapView>(id) {
            m.latitude = lat.clamp(-85.0, 85.0);
            m.longitude = lon;
            if let Some(z) = zoom {
                m.zoom = z.clamp(MIN_ZOOM, MAX_ZOOM);
            }
        }
        ui.request_repaint();
    }

    /// O ponto do mapa sob `at` (coordenadas do controle): (latitude, longitude).
    pub fn coordinate_at(&self, at: PointF) -> (f64, f64) {
        let b = self.base.bounds();
        let (cx, cy) = project(self.latitude, self.longitude, self.zoom);
        unproject(cx + at.x as f64 - b.width as f64 / 2.0, cy + at.y as f64 - b.height as f64 / 2.0, self.zoom)
    }

    /// Zoom por `factor` mantendo parado o ponto `at` (coordenadas do controle).
    fn zoom_at(&mut self, factor: f64, at: PointF) {
        let (lat, lon) = self.coordinate_at(at);
        self.zoom = (self.zoom + factor.log2()).clamp(MIN_ZOOM, MAX_ZOOM);
        // O ponto sob o cursor continua sob ele.
        let b = self.base.bounds();
        let (px, py) = project(lat, lon, self.zoom);
        let (cx, cy) = (px - at.x as f64 + b.width as f64 / 2.0, py - at.y as f64 + b.height as f64 / 2.0);
        let (clat, clon) = unproject(cx, cy, self.zoom);
        self.latitude = clat.clamp(-85.0, 85.0);
        self.longitude = (clon + 180.0).rem_euclid(360.0) - 180.0;
    }

    fn pan(&mut self, dx: f64, dy: f64) {
        let (cx, cy) = project(self.latitude, self.longitude, self.zoom);
        let (lat, lon) = unproject(cx - dx, cy - dy, self.zoom);
        self.latitude = lat.clamp(-85.0, 85.0);
        self.longitude = (lon + 180.0).rem_euclid(360.0) - 180.0;
    }

    /// Pede os blocos que a última pintura não tinha (fora da pintura: precisa do `Ui`).
    fn request_tiles(ui: &mut Ui, id: ComponentId) {
        let Some(m) = ui.get::<MapView>(id) else { return };
        let wanted: Vec<(Key, String)> = {
            let mut t = m.tiles.borrow_mut();
            let keys: Vec<Key> = std::mem::take(&mut t.wanted).into_iter().filter(|k| !t.pending.contains(k) && !t.failed.contains(k)).collect();
            for k in &keys {
                t.pending.insert(*k);
            }
            // O endereço e o crédito: os do provedor ligado ou os do OpenStreetMap.
            let osm = TileServer::default();
            let tiles: &dyn MapTileProvider = match m.provider.and_then(|p| ui.provided(p)) {
                Some(Provided::MapTiles(tiles)) => tiles,
                _ => &osm,
            };
            *m.attribution.borrow_mut() = tiles.attribution();
            keys.into_iter().map(|(z, x, y)| ((z, x, y), tiles.tile_url(u32::from(z), x, y))).collect()
        };
        if wanted.is_empty() {
            return;
        }
        let poster = ui.poster();
        let Some(http) = ui.services_mut().http.as_mut() else { return };
        for (key, url) in wanted {
            let poster = poster.clone();
            http.get(
                &url,
                Box::new(move |result| {
                    poster.post(move |ui| {
                        if let Some(m) = ui.get::<MapView>(id) {
                            let mut t = m.tiles.borrow_mut();
                            t.pending.remove(&key);
                            match result {
                                Ok(bytes) => {
                                    let n = t.use_count;
                                    t.images.insert(key, (Image::encoded(bytes), n));
                                    if t.images.len() > CACHE {
                                        // Sai o bloco usado há mais tempo.
                                        if let Some(old) = t.images.iter().min_by_key(|(_, (_, used))| *used).map(|(k, _)| *k) {
                                            t.images.remove(&old);
                                        }
                                    }
                                }
                                Err(_) => {
                                    t.failed.insert(key);
                                }
                            }
                        }
                        ui.request_repaint();
                    });
                }),
            );
        }
    }
}

impl Component for MapView {
    leaf_control!("MapView");

    properties! {
        latitude: f64 => "Data",
        longitude: f64 => "Data",
        zoom: f64 => "Data",
        markers: String => "Data",
        provider: Option<ComponentId> => "Data";
        events: on_move, on_map_click
    }

    /// Outro provedor: os blocos guardados eram do servidor anterior.
    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if name == "provider" {
            *self.tiles.borrow_mut() = Tiles::default();
            ctx.repaint();
        }
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.provider == Some(freed) {
            self.provider = None;
            *self.tiles.borrow_mut() = Tiles::default();
        }
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button == PointerButton::Primary {
            self.drag = Some((event.position, self.latitude, self.longitude));
            if event.click_count == 2 {
                self.zoom_at(2.0, event.position);
                ctx.fire("on_move", EventArgs::None);
                ctx.repaint();
            }
        }
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let Some((start, lat, lon)) = self.drag else { return };
        self.latitude = lat;
        self.longitude = lon;
        self.pan((event.position.x - start.x) as f64, (event.position.y - start.y) as f64);
        ctx.repaint();
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let Some((start, _, _)) = self.drag.take() else { return };
        let moved = (event.position.x - start.x).abs() + (event.position.y - start.y).abs();
        if moved < 4.0 {
            let (lat, lon) = self.coordinate_at(event.position);
            ctx.fire("on_map_click", EventArgs::Text(format!("{lat:.6},{lon:.6}")));
        } else {
            ctx.fire("on_move", EventArgs::None);
        }
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        if event.delta.y == 0.0 {
            return;
        }
        self.zoom_at(if event.delta.y > 0.0 { 1.25 } else { 0.8 }, event.position);
        ctx.fire("on_move", EventArgs::None);
        ctx.repaint();
        ctx.set_handled();
    }

    fn gesture(&mut self, ctx: &mut EventCtx<'_>, event: &GestureEvent) {
        self.drag = None;
        self.pan(event.pan.x as f64, event.pan.y as f64);
        if event.delta_scale > 0.0 {
            self.zoom_at(event.delta_scale as f64, event.center);
        }
        if event.phase == aylis_core::GesturePhase::End {
            ctx.fire("on_move", EventArgs::None);
        }
        ctx.repaint();
        ctx.set_handled();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        // Os blocos das bordas passam do controle: corta.
        canvas.save();
        canvas.clip_rect(r);
        canvas.fill_rect(r, 0.0, &Brush::Solid(Color::rgb(0xE8, 0xE6, 0xE1)));
        let z = self.zoom.floor().clamp(MIN_ZOOM, MAX_ZOOM) as u8;
        // Zoom fracionário: os blocos do nível inteiro, ampliados.
        let scale = 2f64.powf(self.zoom - z as f64);
        let tile = TILE * scale;
        let (cx, cy) = project(self.latitude, self.longitude, z as f64);
        let (cx, cy) = (cx * scale, cy * scale);
        let left = cx - r.width as f64 / 2.0;
        let top = cy - r.height as f64 / 2.0;
        let count = 1u32 << z;
        let mut t = self.tiles.borrow_mut();
        t.use_count += 1;
        let now = t.use_count;
        let mut missing = Vec::new();
        let (x0, x1) = ((left / tile).floor() as i64, ((left + r.width as f64) / tile).floor() as i64);
        let (y0, y1) = ((top / tile).floor() as i64, ((top + r.height as f64) / tile).floor() as i64);
        for ty in y0.max(0)..=y1.min(count as i64 - 1) {
            for tx in x0..=x1 {
                let wrapped = tx.rem_euclid(count as i64) as u32;
                let key = (z, wrapped, ty as u32);
                let dest = RectF::new(
                    r.x + (tx as f64 * tile - left) as f32,
                    r.y + (ty as f64 * tile - top) as f32,
                    tile.ceil() as f32 + 0.5,
                    tile.ceil() as f32 + 0.5,
                );
                if let Some((image, used)) = t.images.get_mut(&key) {
                    *used = now;
                    let src = RectF::new(0.0, 0.0, TILE as f32, TILE as f32);
                    canvas.draw_image(image, src, dest, 1.0);
                    continue;
                }
                // Ainda não chegou: o bloco do nível de cima, ampliado (fica borrado, mas não
                // vazio).
                if z > 1 {
                    let parent = (z - 1, wrapped / 2, ty as u32 / 2);
                    if let Some((image, _)) = t.images.get(&parent) {
                        let half = (TILE / 2.0) as f32;
                        let src = RectF::new((wrapped % 2) as f32 * half, (ty as u32 % 2) as f32 * half, half, half);
                        canvas.draw_image(image, src, dest, 1.0);
                    }
                }
                missing.push(key);
            }
        }
        if !missing.is_empty() && !ctx.design {
            t.wanted = missing;
            let id = ctx.id;
            ctx.ui.poster().post(move |ui| MapView::request_tiles(ui, id));
        }
        drop(t);

        // Marcadores.
        let mut label = ctx.text_style();
        label.font.size = (label.font.size - 1.0).max(10.0);
        for (lat, lon, text) in self.marker_list() {
            let (px, py) = project(lat, lon, self.zoom);
            let (mcx, mcy) = project(self.latitude, self.longitude, self.zoom);
            let p = PointF::new(r.x + r.width / 2.0 + (px - mcx) as f32, r.y + r.height / 2.0 + (py - mcy) as f32);
            if !r.inflate(24.0).contains(p) {
                continue;
            }
            let pin = RectF::new(p.x - 9.0, p.y - 26.0, 18.0, 18.0);
            canvas.draw_line(PointF::new(p.x, pin.bottom() - 2.0), p, &Stroke::new(Color::rgb(0xC6, 0x28, 0x28), 3.0));
            canvas.fill_ellipse(pin, &Brush::Solid(Color::rgb(0xE5, 0x39, 0x35)));
            canvas.fill_ellipse(pin.inflate(-5.5), &Brush::Solid(Color::WHITE));
            if !text.is_empty() {
                let w = canvas.measure_text(&text, &label).width + 12.0;
                let box_r = RectF::new(p.x - w / 2.0, pin.y - 26.0, w, 22.0);
                canvas.fill_rect(box_r, 6.0, &Brush::Solid(Color::WHITE.with_opacity(0.95)));
                let mut lt = label.clone();
                lt.h_align = TextAlign::Center;
                lt.color = Color::rgb(0x20, 0x20, 0x20);
                canvas.draw_text(&text, box_r, &lt);
            }
        }

        // Atribuição (a licença pede): a do servidor.
        let attribution = self.attribution.borrow().clone();
        if !attribution.is_empty() {
            let mut at = ctx.text_style();
            at.font.size = 10.0;
            at.color = Color::rgb(0x33, 0x33, 0x33);
            let w = canvas.measure_text(&attribution, &at).width + 10.0;
            let box_r = RectF::new(r.right() - w, r.bottom() - 16.0, w, 16.0);
            canvas.fill_rect(box_r, 0.0, &Brush::Solid(Color::WHITE.with_opacity(0.75)));
            at.h_align = TextAlign::Center;
            canvas.draw_text(&attribution, box_r, &at);
        }
        canvas.restore();
    }
}
