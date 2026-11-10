//! Sombra, mapas do material (normal, oclusão, metal/aspereza, emissão), luzes e câmeras do
//! arquivo, desenhados de verdade na GPU (pula se a máquina não tiver uma).
//!
//! `AYLIS_3D_PNG=pasta` grava as imagens para conferir a olho.

use glam::{Mat4, Quat, Vec3};
use aylis_3d::model::{Mesh, Node, Primitive, Texture, Vertex};
use aylis_3d::{Camera, Engine, Frame, Light, LightKind, Model, Output, Projection, sample};

fn frame(model: &Model, w: u32, h: u32, eye_dir: Vec3, shadows: bool) -> Frame {
    let (lo, hi) = model.bounds;
    let center = (lo + hi) / 2.0;
    let radius = (hi - lo).length() / 2.0;
    let eye = center + eye_dir.normalize() * radius * 2.6;
    Frame {
        width: w,
        height: h,
        view: glam::camera::rh::view::look_at_mat4(eye, center, if eye_dir.y.abs() > 0.99 { Vec3::NEG_Z } else { Vec3::Y }),
        proj: glam::camera::rh::proj::directx::perspective(40f32.to_radians(), w as f32 / h as f32, 0.05, 100.0),
        eye,
        background: [1.0, 1.0, 1.0, 1.0],
        light_dir: Vec3::new(-0.4, -1.0, -0.6),
        file_lights: true,
        shadows,
        ray_traced: false,
        sample: 0,
        glow: 0.0,
    }
}

struct Image {
    w: u32,
    h: u32,
    rgba: Vec<u8>,
}

impl Image {
    fn at(&self, x: u32, y: u32) -> [u8; 4] {
        let i = ((y * self.w + x) * 4) as usize;
        [self.rgba[i], self.rgba[i + 1], self.rgba[i + 2], self.rgba[i + 3]]
    }

    fn luma(&self, x: u32, y: u32) -> f32 {
        let p = self.at(x, y);
        0.3 * p[0] as f32 + 0.59 * p[1] as f32 + 0.11 * p[2] as f32
    }

    /// Média do brilho num retângulo.
    fn mean(&self, x0: u32, y0: u32, x1: u32, y1: u32) -> f32 {
        let mut sum = 0.0;
        for y in y0..y1 {
            for x in x0..x1 {
                sum += self.luma(x, y);
            }
        }
        sum / ((x1 - x0) * (y1 - y0)) as f32
    }

    fn save(&self, name: &str) {
        let Some(dir) = std::env::var_os("AYLIS_3D_PNG") else { return };
        let dir = std::path::PathBuf::from(dir);
        let _ = std::fs::create_dir_all(&dir);
        let _ = image::save_buffer(dir.join(format!("{name}.png")), &self.rgba, self.w, self.h, image::ColorType::Rgba8);
    }
}

fn render(engine: &mut Engine, model: &Model, frame: &Frame) -> Image {
    let mut gpu = engine.upload(model);
    let mut view = engine.view();
    let pose = model.rest_pose();
    let world = model.world(&pose, Mat4::IDENTITY);
    match engine.render(model, &mut gpu, &mut view, &world, &pose.weights, frame, false) {
        Some(Output::Pixels { width, height, rgba }) => Image { w: width, h: height, rgba },
        _ => panic!("sem pixels"),
    }
}

fn engine() -> Option<Engine> {
    match Engine::new(None) {
        Ok(e) => Some(e),
        Err(_) => {
            eprintln!("sem GPU: teste pulado");
            None
        }
    }
}

/// Um quadrado de lado 2 no plano XY (olhando para +Z), com UV de cima para baixo como no
/// glTF, e o material 0.
fn quad() -> Primitive {
    let v = |x: f32, y: f32, u: f32, w: f32| Vertex { position: [x, y, 0.0], normal: [0.0, 0.0, 1.0], uv: [u, w], ..Default::default() };
    Primitive { vertices: vec![v(-1.0, -1.0, 0.0, 1.0), v(1.0, -1.0, 1.0, 1.0), v(1.0, 1.0, 1.0, 0.0), v(-1.0, 1.0, 0.0, 0.0)], indices: vec![0, 1, 2, 0, 2, 3], material: 0, targets: Vec::new() }
}

fn node(name: &str, mesh: Option<usize>) -> Node {
    Node { name: name.into(), parent: None, children: Vec::new(), translation: Vec3::ZERO, rotation: Quat::IDENTITY, scale: Vec3::ONE, mesh, skin: None, weights: Vec::new() }
}

/// Um modelo com um quadrado e um material, mais os nós extras dados.
fn quad_model(material: aylis_3d::model::Material, textures: Vec<Texture>, extra: Vec<Node>) -> Model {
    let mut nodes = vec![node("Quadrado", Some(0))];
    nodes.extend(extra);
    let n = nodes.len();
    let mut model = Model {
        nodes,
        roots: (0..n).collect(),
        order: (0..n).collect(),
        meshes: vec![Mesh { name: "q".into(), primitives: vec![quad()], weights: Vec::new() }],
        materials: vec![material, Default::default()],
        textures,
        ..Default::default()
    };
    model.bounds = (Vec3::new(-1.0, -1.0, -0.1), Vec3::new(1.0, 1.0, 0.1));
    model
}

fn solid(rgba: [u8; 4]) -> Texture {
    Texture { width: 4, height: 4, rgba: rgba.repeat(16) }
}

#[test]
fn the_tube_casts_a_shadow_on_the_floor() {
    let Some(mut engine) = engine() else { return };
    let model = Model::from_glb(&sample::tube_glb()).unwrap();
    // De cima: a sombra cai no chão para -X e -Z (a luz vem de +X, +Y, +Z).
    let lit = render(&mut engine, &model, &frame(&model, 200, 200, Vec3::Y, false));
    let shaded = render(&mut engine, &model, &frame(&model, 200, 200, Vec3::Y, true));
    lit.save("sombra_sem");
    shaded.save("sombra_com");
    let darker = (0..200).flat_map(|x| (0..200).map(move |y| (x, y))).filter(|&(x, y)| shaded.luma(x, y) + 20.0 < lit.luma(x, y)).count();
    assert!(darker > 150, "a sombra escurece o chão ({darker} pixels)");
    // Nada fica mais claro com a sombra.
    let brighter = (0..200).flat_map(|x| (0..200).map(move |y| (x, y))).filter(|&(x, y)| shaded.luma(x, y) > lit.luma(x, y) + 3.0).count();
    assert!(brighter < 10, "{brighter} pixels mais claros");
}

#[test]
fn the_normal_map_tilts_the_light() {
    let Some(mut engine) = engine() else { return };
    // Mapas de normal inclinando para cima (+Y, verde alto) e para baixo; a luz vem de cima.
    let render_with = |engine: &mut Engine, green: u8| {
        let material = aylis_3d::model::Material { base_color: [0.8, 0.8, 0.8, 1.0], normal_texture: Some(0), ..Default::default() };
        let model = quad_model(material, vec![solid([128, green, 200, 255])], Vec::new());
        let mut f = frame(&model, 64, 64, Vec3::Z, false);
        f.light_dir = Vec3::new(0.0, -1.0, -0.6);
        render(engine, &model, &f)
    };
    let up = render_with(&mut engine, 230);
    let flat = render_with(&mut engine, 128);
    let down = render_with(&mut engine, 26);
    up.save("normal_cima");
    down.save("normal_baixo");
    let (u, f, d) = (up.mean(24, 24, 40, 40), flat.mean(24, 24, 40, 40), down.mean(24, 24, 40, 40));
    assert!(u > f + 8.0 && f > d + 8.0, "cima {u}, plano {f}, baixo {d}");
}

#[test]
fn occlusion_metal_roughness_and_emission_maps_change_the_color() {
    let Some(mut engine) = engine() else { return };
    let base = aylis_3d::model::Material { base_color: [0.6, 0.6, 0.6, 1.0], roughness: 1.0, ..Default::default() };
    let shot = |engine: &mut Engine, material: aylis_3d::model::Material, textures: Vec<Texture>| {
        let model = quad_model(material, textures, Vec::new());
        render(engine, &model, &frame(&model, 64, 64, Vec3::Z, false)).mean(24, 24, 40, 40)
    };
    let plain = shot(&mut engine, base.clone(), Vec::new());
    // Oclusão preta: some a luz ambiente.
    let occluded = shot(&mut engine, aylis_3d::model::Material { occlusion_texture: Some(0), ..base.clone() }, vec![solid([0, 0, 0, 255])]);
    assert!(occluded + 15.0 < plain, "oclusão {occluded} x {plain}");
    // Metal total pela textura (azul): o difuso some.
    let metal = shot(&mut engine, aylis_3d::model::Material { metallic: 1.0, metallic_roughness_texture: Some(0), ..base.clone() }, vec![solid([0, 255, 255, 255])]);
    let no_metal = shot(&mut engine, aylis_3d::model::Material { metallic: 1.0, metallic_roughness_texture: Some(0), ..base.clone() }, vec![solid([0, 255, 0, 255])]);
    assert!((metal - no_metal).abs() > 15.0, "metal {metal} x {no_metal}");
    // Emissão pela textura: mais claro só com a textura branca.
    let glow = aylis_3d::model::Material { emissive: [1.0, 1.0, 1.0], emissive_texture: Some(0), base_color: [0.1, 0.1, 0.1, 1.0], ..base.clone() };
    let lit = shot(&mut engine, glow.clone(), vec![solid([255, 255, 255, 255])]);
    let dark = shot(&mut engine, glow, vec![solid([0, 0, 0, 255])]);
    assert!(lit > dark + 60.0, "emissão {lit} x {dark}");
}

#[test]
fn file_lights_replace_the_default_one() {
    let Some(mut engine) = engine() else { return };
    let material = aylis_3d::model::Material { base_color: [0.9, 0.9, 0.9, 1.0], roughness: 0.9, ..Default::default() };
    // Uma luz pontual vermelha na frente do quadrado.
    let mut lamp = node("Lâmpada", None);
    lamp.translation = Vec3::new(0.0, 0.0, 0.6);
    let mut model = quad_model(material, Vec::new(), vec![lamp]);
    model.lights.push(Light { name: "Vermelha".into(), node: 1, kind: LightKind::Point, color: [1.0, 0.0, 0.0], intensity: 3.0, range: None });
    let red = render(&mut engine, &model, &frame(&model, 64, 64, Vec3::Z, false));
    let mut f = frame(&model, 64, 64, Vec3::Z, false);
    f.file_lights = false;
    let default = render(&mut engine, &model, &f);
    red.save("luz_vermelha");
    let p = red.at(32, 32);
    let q = default.at(32, 32);
    // A luz ambiente (céu e chão) continua: o vermelho vem por cima dela.
    assert!(p[0] as i32 - p[2] as i32 > 35, "o centro fica vermelho: {p:?}");
    assert!((q[0] as i32 - q[2] as i32).abs() < 15, "sem as do arquivo, a luz padrão (branca): {q:?}");
    // No canto, longe da lâmpada, menos luz (cai com a distância).
    assert!(red.at(32, 32)[0] > red.at(10, 10)[0], "{:?} x {:?}", red.at(32, 32), red.at(10, 10));
}

#[test]
fn lights_and_cameras_round_trip_through_glb() {
    let mut lamp = node("Lâmpada", None);
    lamp.translation = Vec3::new(1.0, 2.0, 3.0);
    let mut eye = node("Olho", None);
    eye.translation = Vec3::new(0.0, 1.0, 5.0);
    let mut sun = node("Sol", None);
    sun.rotation = Quat::from_rotation_x(-1.0);
    let mut model = quad_model(Default::default(), vec![solid([128, 128, 255, 255])], vec![lamp, eye, sun]);
    model.materials[0].normal_texture = Some(0);
    model.materials[0].normal_scale = 0.5;
    model.materials[0].occlusion_texture = Some(0);
    model.materials[0].occlusion_strength = 0.7;
    model.materials[0].metallic_roughness_texture = Some(0);
    model.materials[0].emissive_texture = Some(0);
    model.lights.push(Light { name: "Cone".into(), node: 1, kind: LightKind::Spot { inner: 0.2, outer: 0.5 }, color: [1.0, 0.5, 0.25], intensity: 40.0, range: Some(10.0) });
    model.lights.push(Light { name: "Sol".into(), node: 3, kind: LightKind::Directional, color: [1.0, 1.0, 1.0], intensity: 2.0, range: None });
    model.cameras.push(Camera { name: "Frente".into(), node: 2, projection: Projection::Perspective { yfov: 0.8, aspect: None, znear: 0.1, zfar: Some(50.0) } });
    model.cameras.push(Camera { name: "Planta".into(), node: 2, projection: Projection::Orthographic { xmag: 3.0, ymag: 2.0, znear: 0.1, zfar: 20.0 } });

    let back = Model::from_glb(&aylis_3d::glb::to_glb(&model).unwrap()).unwrap();
    assert_eq!(back.lights, model.lights);
    assert_eq!(back.cameras[0], model.cameras[0]);
    assert_eq!(back.cameras[1].projection, model.cameras[1].projection);
    let (a, b) = (&model.materials[0], &back.materials[0]);
    assert_eq!((b.normal_texture, b.occlusion_texture, b.metallic_roughness_texture, b.emissive_texture), (Some(0), Some(0), Some(0), Some(0)));
    assert_eq!((b.normal_scale, b.occlusion_strength), (a.normal_scale, a.occlusion_strength));
    // Duas câmeras no mesmo nó: a segunda vai num nó filho (o glTF aceita uma por nó).
    let plant = back.camera("Planta").unwrap().node;
    assert_eq!(back.nodes[plant].parent, Some(2));
    assert_eq!(back.camera("Frente").map(|c| c.node), Some(2));
}

#[test]
fn the_clearcoat_adds_a_sharp_highlight_over_a_rough_material() {
    let Some(mut engine) = engine() else { return };
    let paint = aylis_3d::model::Material { base_color: [0.5, 0.05, 0.05, 1.0], roughness: 1.0, ..Default::default() };
    let shot = |engine: &mut Engine, material: aylis_3d::model::Material| {
        let model = quad_model(material, Vec::new(), Vec::new());
        let mut f = frame(&model, 64, 64, Vec3::Z, false);
        // A luz vem de trás da câmera: o reflexo cai no meio do quadrado.
        f.light_dir = Vec3::new(0.0, 0.0, -1.0);
        render(engine, &model, &f)
    };
    let plain = shot(&mut engine, paint.clone());
    let coated = shot(&mut engine, aylis_3d::model::Material { clearcoat: 1.0, clearcoat_roughness: 0.05, ..paint });
    coated.save("verniz");
    let (p, c) = (plain.mean(28, 28, 36, 36), coated.mean(28, 28, 36, 36));
    assert!(c > p + 25.0, "o verniz brilha no meio: {c} x {p}");
}
