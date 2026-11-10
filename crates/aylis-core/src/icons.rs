//! Ícones vetoriais. O aylis traz um conjunto próprio (traço arredondado numa grade de
//! 24×24) e aceita ícones do usuário em SVG (`Ui::register_icon`). Os ícones mudam de
//! cor com o tema: quem desenha passa a cor.

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::canvas::{Brush, Canvas, Stroke};
use crate::color::Color;
use crate::geometry::{PointF, RectF};
use crate::path::Path;

/// Um ícone: a parte contornada e a parte preenchida, na grade `size` × `size`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Icon {
    pub stroke: Path,
    pub fill: Path,
    /// Lado da grade em que o ícone foi desenhado (24 nos ícones do aylis).
    pub size: f32,
}

impl Icon {
    /// Ícone a partir de caminhos SVG (`d`): um contornado e um preenchido (qualquer um
    /// pode ser vazio). `None` se algum estiver mal formado.
    pub fn from_svg(stroke: &str, fill: &str, size: f32) -> Option<Icon> {
        Some(Icon { stroke: Path::from_svg(stroke)?, fill: Path::from_svg(fill)?, size })
    }

    /// Desenha centralizado em `rect`, do tamanho do menor lado, na cor `color`.
    pub fn draw(&self, canvas: &mut dyn Canvas, rect: RectF, color: Color) {
        let side = rect.width.min(rect.height);
        if side <= 0.0 || self.size <= 0.0 {
            return;
        }
        let scale = side / self.size;
        let offset = PointF::new(rect.x + (rect.width - side) / 2.0, rect.y + (rect.height - side) / 2.0);
        if !self.fill.is_empty() {
            canvas.fill_path(&self.fill.transformed(scale, offset), &Brush::Solid(color));
        }
        if !self.stroke.is_empty() {
            // Traço de 2 na grade de 24, nunca mais fino que 1 pixel.
            let width = (self.size / 12.0 * scale).max(1.0);
            canvas.stroke_path(&self.stroke.transformed(scale, offset), &Stroke::new(color, width).round());
        }
    }
}

/// Os ícones do aylis: nome, traço, preenchimento.
const BUILTIN: &[(&str, &str, &str)] = &[
    ("plus", "M12 5v14M5 12h14", ""),
    ("minus", "M5 12h14", ""),
    ("close", "M6 6l12 12M18 6L6 18", ""),
    ("check", "M5 12.5l4.5 4.5L19 7", ""),
    ("chevron-down", "M6 9l6 6 6-6", ""),
    ("chevron-up", "M6 15l6-6 6 6", ""),
    ("chevron-left", "M15 6l-6 6 6 6", ""),
    ("chevron-right", "M9 6l6 6-6 6", ""),
    ("arrow-left", "M19 12H5M11 6l-6 6 6 6", ""),
    ("arrow-right", "M5 12h14M13 6l6 6-6 6", ""),
    ("arrow-up", "M12 19V5M6 11l6-6 6 6", ""),
    ("arrow-down", "M12 5v14M6 13l6 6 6-6", ""),
    ("menu", "M4 6h16M4 12h16M4 18h16", ""),
    ("more", "", "M3.5 12a2 2 0 1 0 4 0a2 2 0 1 0-4 0ZM10 12a2 2 0 1 0 4 0a2 2 0 1 0-4 0ZM16.5 12a2 2 0 1 0 4 0a2 2 0 1 0-4 0Z"),
    ("more-vertical", "", "M12 3.5a2 2 0 1 0 0 4a2 2 0 1 0 0-4ZM12 10a2 2 0 1 0 0 4a2 2 0 1 0 0-4ZM12 16.5a2 2 0 1 0 0 4a2 2 0 1 0 0-4Z"),
    ("search", "M10.5 4a6.5 6.5 0 1 0 0 13a6.5 6.5 0 1 0 0-13ZM15.5 15.5L20 20", ""),
    ("zoom-in", "M10.5 4a6.5 6.5 0 1 0 0 13a6.5 6.5 0 1 0 0-13ZM15.5 15.5L20 20M10.5 8v5M8 10.5h5", ""),
    ("zoom-out", "M10.5 4a6.5 6.5 0 1 0 0 13a6.5 6.5 0 1 0 0-13ZM15.5 15.5L20 20M8 10.5h5", ""),
    ("filter", "M4 5h16l-6 7.5V19l-4 1.5v-8Z", ""),
    ("play", "", "M7.5 4.8a1 1 0 0 1 1.5-.9l10.6 7.2a1 1 0 0 1 0 1.7L9 20.1a1 1 0 0 1-1.5-.9Z"),
    ("pause", "", "M6 5a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v14a1 1 0 0 1-1 1H7a1 1 0 0 1-1-1ZM14 5a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v14a1 1 0 0 1-1 1h-2a1 1 0 0 1-1-1Z"),
    ("stop", "", "M6 7a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v10a1 1 0 0 1-1 1H7a1 1 0 0 1-1-1Z"),
    ("record", "", "M12 5a7 7 0 1 0 0 14a7 7 0 1 0 0-14Z"),
    ("skip-back", "M6 5v14", "M18.5 5.5v13a.8.8 0 0 1-1.2.7L8.5 12.7a.8.8 0 0 1 0-1.4l8.8-6.5a.8.8 0 0 1 1.2.7Z"),
    ("skip-forward", "M18 5v14", "M5.5 5.5v13a.8.8 0 0 0 1.2.7l8.8-6.5a.8.8 0 0 0 0-1.4L6.7 4.8a.8.8 0 0 0-1.2.7Z"),
    ("volume", "M4 9h3l5-4v14l-5-4H4ZM16 9a4 4 0 0 1 0 6M18.5 6.5a7.5 7.5 0 0 1 0 11", ""),
    ("volume-off", "M4 9h3l5-4v14l-5-4H4ZM16 9.5l5 5M21 9.5l-5 5", ""),
    ("mic", "M12 3a3 3 0 0 0-3 3v6a3 3 0 0 0 6 0V6a3 3 0 0 0-3-3ZM5.5 11a6.5 6.5 0 0 0 13 0M12 17.5V21M8.5 21h7", ""),
    ("mic-off", "M12 3a3 3 0 0 0-3 3v6a3 3 0 0 0 6 0V6a3 3 0 0 0-3-3ZM5.5 11a6.5 6.5 0 0 0 13 0M12 17.5V21M8.5 21h7M4 4l16 16", ""),
    ("headphones", "M4 16v-3a8 8 0 0 1 16 0v3M4 15h3v5H4ZM17 15h3v5h-3Z", ""),
    ("video", "M3 7h12v10H3ZM15 10.5l6-3.5v10l-6-3.5", ""),
    ("video-off", "M3 7h12v10H3ZM15 10.5l6-3.5v10l-6-3.5M2 3l20 18", ""),
    ("camera", "M4 8h3l2-3h6l2 3h3v11H4ZM12 10a3.5 3.5 0 1 0 0 7a3.5 3.5 0 1 0 0-7Z", ""),
    ("monitor", "M3 4h18v12H3ZM8 20h8M12 16v4", ""),
    ("window", "M3 5h18v14H3ZM3 9h18", ""),
    ("sliders", "M4 6h9M17 6h3M4 12h3M11 12h9M4 18h11M19 18h1M15 4v4M9 10v4M17 16v4", ""),
    ("trash", "M4 7h16M10 3h4M6 7l1 13h10l1-13M10 11v6M14 11v6", ""),
    ("edit", "M4 20h4L19 9l-4-4L4 16ZM13.5 6.5l4 4", ""),
    ("folder", "M3 6a1 1 0 0 1 1-1h5l2 2h9a1 1 0 0 1 1 1v10a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1Z", ""),
    ("file", "M6 3h8l5 5v13H6ZM14 3v5h5", ""),
    ("copy", "M8 8h12v12H8ZM16 8V4H4v12h4", ""),
    ("refresh", "M20 12a8 8 0 1 1-2.3-5.6M20 4v5h-5", ""),
    ("swap", "M4 9h14l-3-3M20 15H6l3 3", ""),
    ("home", "M4 11l8-7 8 7M6 9.5V20h12V9.5M10 20v-5h4v5", ""),
    ("user", "M12 4a4 4 0 1 0 0 8a4 4 0 1 0 0-8ZM4.5 20a7.5 7.5 0 0 1 15 0", ""),
    ("bell", "M6 16v-5a6 6 0 0 1 12 0v5l2 2H4ZM10 20.5a2 2 0 0 0 4 0", ""),
    ("star", "M12 3.5l2.6 5.3 5.8.8-4.2 4.1 1 5.8-5.2-2.7-5.2 2.7 1-5.8-4.2-4.1 5.8-.8Z", ""),
    ("heart", "M12 20s-7.5-4.6-7.5-10A4.2 4.2 0 0 1 12 7.2A4.2 4.2 0 0 1 19.5 10c0 5.4-7.5 10-7.5 10Z", ""),
    ("info", "M12 3a9 9 0 1 0 0 18a9 9 0 1 0 0-18ZM12 11v6M12 7.5h.01", ""),
    ("help", "M12 3a9 9 0 1 0 0 18a9 9 0 1 0 0-18ZM9.5 9.5a2.5 2.5 0 1 1 3.5 2.3c-.6.3-1 .8-1 1.5V14M12 17.5h.01", ""),
    ("warning", "M12 4L2.5 20h19ZM12 10v4.5M12 17.5h.01", ""),
    ("error", "M12 3a9 9 0 1 0 0 18a9 9 0 1 0 0-18ZM9 9l6 6M15 9l-6 6", ""),
    ("success", "M12 3a9 9 0 1 0 0 18a9 9 0 1 0 0-18ZM8 12.5l3 3 5-6", ""),
    ("lock", "M5 11h14v10H5ZM8 11V8a4 4 0 0 1 8 0v3", ""),
    ("unlock", "M5 11h14v10H5ZM8 11V8a4 4 0 0 1 7.5-2", ""),
    ("eye", "M2 12s3.5-7 10-7 10 7 10 7-3.5 7-10 7S2 12 2 12ZM12 9a3 3 0 1 0 0 6a3 3 0 1 0 0-6Z", ""),
    ("eye-off", "M2 12s3.5-7 10-7 10 7 10 7-3.5 7-10 7S2 12 2 12ZM12 9a3 3 0 1 0 0 6a3 3 0 1 0 0-6ZM3 3l18 18", ""),
    ("download", "M12 4v11M7 10l5 5 5-5M5 20h14", ""),
    ("upload", "M12 16V5M7 10l5-5 5 5M5 20h14", ""),
    ("link", "M10 14a4 4 0 0 0 5.7 0l3-3a4 4 0 0 0-5.7-5.7l-1 1M14 10a4 4 0 0 0-5.7 0l-3 3a4 4 0 0 0 5.7 5.7l1-1", ""),
    ("calendar", "M4 6h16v14H4ZM4 10h16M8 3v4M16 3v4", ""),
    ("clock", "M12 3a9 9 0 1 0 0 18a9 9 0 1 0 0-18ZM12 7v5l3 2", ""),
    ("send", "M4 11.5L20 4l-7.5 16-2.5-6.5ZM10 13.5L20 4", ""),
    ("attach", "M20 11.5l-8 8a5 5 0 0 1-7-7l8.5-8.5a3.3 3.3 0 0 1 4.7 4.7L9.7 17.2a1.7 1.7 0 0 1-2.4-2.4L15 7", ""),
    ("fullscreen", "M4 9V4h5M15 4h5v5M20 15v5h-5M9 20H4v-5", ""),
    ("fullscreen-exit", "M9 4v5H4M20 9h-5V4M15 20v-5h5M4 15h5v5", ""),
    ("grid", "M4 4h7v7H4ZM13 4h7v7h-7ZM4 13h7v7H4ZM13 13h7v7h-7Z", ""),
    ("list", "M9 6h11M9 12h11M9 18h11M4.5 6h.01M4.5 12h.01M4.5 18h.01", ""),
    ("pin", "M9 4h6M10 4v5l-3 4h10l-3-4V4M12 13v7", ""),
    ("plug", "M9 3v5M15 3v5M7 8h10v3a5 5 0 0 1-10 0ZM12 16v5", ""),
    ("wifi", "M2.5 9a14 14 0 0 1 19 0M5.5 12.5a9.5 9.5 0 0 1 13 0M8.5 16a5 5 0 0 1 7 0M12 19.5h.01", ""),
    ("logout", "M15 4h4v16h-4M10 8l-4 4 4 4M6 12h10", ""),
    ("phone", "M5 4h4l2 5-2.5 1.5a11 11 0 0 0 5 5L15 13l5 2v4a1 1 0 0 1-1 1A16 16 0 0 1 4 5a1 1 0 0 1 1-1Z", ""),
    ("chat", "M4 5h16v11H9l-5 4Z", ""),
    ("emoji", "M12 3a9 9 0 1 0 0 18a9 9 0 1 0 0-18ZM8.5 14a4 4 0 0 0 7 0M9 9.5h.01M15 9.5h.01", ""),
    ("image", "M4 5h16v14H4ZM4 16l5-5 4 4 3-3 4 4M15 8.5h.01", ""),
    ("map-pin", "M12 21s-6.5-6-6.5-11a6.5 6.5 0 0 1 13 0c0 5-6.5 11-6.5 11ZM12 7.5a2.5 2.5 0 1 0 0 5a2.5 2.5 0 1 0 0-5Z", ""),
    ("scissors", "M6 4a2.5 2.5 0 1 0 0 5a2.5 2.5 0 1 0 0-5ZM6 15a2.5 2.5 0 1 0 0 5a2.5 2.5 0 1 0 0-5ZM8 8l12 10M8 16l12-10", ""),
    ("crop", "M6 3v15h15M3 6h15v15", ""),
    ("sun", "M12 8a4 4 0 1 0 0 8a4 4 0 1 0 0-8ZM12 2v2M12 20v2M2 12h2M20 12h2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4", ""),
    ("moon", "M20 14.5A8 8 0 1 1 9.5 4a6.5 6.5 0 0 0 10.5 10.5Z", ""),
    ("grip", "M9 6h.01M15 6h.01M9 12h.01M15 12h.01M9 18h.01M15 18h.01", ""),
    ("circle", "M12 4a8 8 0 1 0 0 16a8 8 0 1 0 0-16Z", ""),
    ("dot", "", "M12 8a4 4 0 1 0 0 8a4 4 0 1 0 0-8Z"),
    // Desenho (as ferramentas de um editor de imagem, como o Paint).
    ("pencil", "M4 20l1-4L16 5l3 3L8 19ZM14 7l3 3", ""),
    ("brush", "M20 4l-8 8M12 12c-3-1-6 1-6 4c0 2-1 3-2 4c4 0 8-1 8-4c0-1.5 .5-3 0-4Z", ""),
    ("eraser", "M8 20h12M4 15l9-9a2 2 0 0 1 3 0l3 3a2 2 0 0 1 0 3l-8 8H8ZM9 10l5 5", ""),
    ("bucket", "M4 11l7-7l7 7l-7 7ZM4 11h14M20 15c0 1.5 1 2.5 1 3.5a1 1 0 0 1-2 0c0-1 1-2 1-3.5Z", ""),
    ("eyedropper", "M14 4l6 6M16 6l2-2a2 2 0 0 1 3 3l-2 2M17 8L7 18l-3 1l1-3L15 6", ""),
    ("line", "M5 19L19 5", ""),
    ("square", "M5 5h14v14H5Z", ""),
    ("ellipse", "M12 6c5 0 9 2.7 9 6s-4 6-9 6s-9-2.7-9-6s4-6 9-6Z", ""),
    ("select", "M4 4h3M10 4h4M17 4h3v3M20 10v4M20 17v3h-3M14 20h-4M7 20H4v-3M4 14v-4M4 7V4", ""),
    ("undo", "M9 14L4 9l5-5M4 9h11a5 5 0 0 1 0 10h-3", ""),
    ("redo", "M15 14l5-5l-5-5M20 9H9a5 5 0 0 0 0 10h3", ""),
    // Reprodução (um player de mídia).
    ("previous", "M6 5v14", "M19 6v12l-10-6Z"),
    ("next", "M18 5v14", "M5 6v12l10-6Z"),
    ("shuffle", "M4 7h3c5 0 5 10 10 10h3M17 14l3 3l-3 3M4 17h3c1.5 0 2.5-1 3.5-2.2M13.5 9.2C14.5 8 15.5 7 17 7h3M17 4l3 3l-3 3", ""),
    ("repeat", "M4 11V9a3 3 0 0 1 3-3h13M17 3l3 3l-3 3M20 13v2a3 3 0 0 1-3 3H4M7 21l-3-3l3-3", ""),
    ("repeat_one", "M4 11V9a3 3 0 0 1 3-3h13M17 3l3 3l-3 3M20 13v2a3 3 0 0 1-3 3H4M7 21l-3-3l3-3M11 10l1.5-1v6", ""),
    ("music", "M9 18V5l11-2v13M9 18a3 3 0 1 1-6 0a3 3 0 0 1 6 0ZM20 16a3 3 0 1 1-6 0a3 3 0 0 1 6 0Z", ""),
    ("mute", "M4 9h4l5-4v14l-5-4H4ZM17 9l4 6M21 9l-4 6", ""),
];

/// Engrenagem: oito dentes em volta de um furo (gerada, não desenhada à mão).
fn gear() -> Icon {
    let mut outline = Path::new();
    let c = PointF::new(12.0, 12.0);
    let (outer, inner) = (9.5f32, 7.0f32);
    let teeth = 8;
    for i in 0..teeth * 4 {
        // Cada dente: subida, topo, descida, vale.
        let a = (i as f32 / (teeth * 4) as f32) * std::f32::consts::TAU - std::f32::consts::FRAC_PI_2;
        let r = if i % 4 == 1 || i % 4 == 2 { outer } else { inner };
        let p = PointF::new(c.x + r * a.cos(), c.y + r * a.sin());
        if i == 0 { outline.move_to(p.x, p.y) } else { outline.line_to(p.x, p.y) };
    }
    outline.close();
    outline.ellipse(RectF::new(9.0, 9.0, 6.0, 6.0));
    Icon { stroke: outline, fill: Path::new(), size: 24.0 }
}

fn builtins() -> &'static HashMap<&'static str, Icon> {
    static ICONS: OnceLock<HashMap<&'static str, Icon>> = OnceLock::new();
    ICONS.get_or_init(|| {
        let mut map: HashMap<&'static str, Icon> = BUILTIN
            .iter()
            .map(|(name, stroke, fill)| (*name, Icon::from_svg(stroke, fill, 24.0).expect("ícone do Aylis mal formado")))
            .collect();
        map.insert("settings", gear());
        map
    })
}

// ---------------------------------------------------------------------------
// Pacotes de ícones (Material Symbols, Fluent, os do app)

/// Um pacote de ícones lido do formato `aylis-icons` (veja [`parse_pack`]).
#[derive(Clone, Debug, Default)]
pub struct IconPack {
    pub title: String,
    pub license: String,
    /// Texto da licença (vai junto com o app).
    pub license_text: String,
    /// Nome → caminho SVG, ainda sem ler (ler todos de uma vez seria lento).
    pub icons: Vec<(String, String)>,
    /// `viewBox` dos caminhos (x, y, largura, altura).
    pub view: (f32, f32, f32, f32),
}

impl IconPack {
    /// Monta o ícone de um nome do pacote.
    pub fn icon(&self, name: &str) -> Option<Icon> {
        let d = &self.icons.iter().find(|(n, _)| n == name)?.1;
        self.build(d)
    }

    /// Monta um ícone de um caminho no `viewBox` do pacote.
    pub fn build(&self, d: &str) -> Option<Icon> {
        let (x, y, w, h) = self.view;
        let size = w.max(h);
        let fill = Path::from_svg(d)?.transformed(1.0, PointF::new(-x, -y));
        Some(Icon { stroke: Path::new(), fill, size })
    }

    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.icons.iter().map(|(n, _)| n.as_str())
    }

    /// O mesmo pacote só com alguns ícones (o que o app leva).
    pub fn subset(&self, names: &[&str]) -> String {
        let mut out = format!(
            "# aylis-icons 1\n# title: {}\n# license: {}\n# view: {} {} {} {}\n",
            self.title, self.license, self.view.0, self.view.1, self.view.2, self.view.3
        );
        for line in self.license_text.lines() {
            out.push_str(&format!("#| {line}\n"));
        }
        for (n, d) in &self.icons {
            if names.contains(&n.as_str()) {
                out.push_str(&format!("{n}\t{d}\n"));
            }
        }
        out
    }
}

/// Lê um pacote de ícones:
///
/// ```text
/// # aylis-icons 1
/// # title: Material Symbols (Outlined)
/// # license: Apache-2.0 (Google)
/// # view: 0 -960 960 960
/// #| (texto da licença, linha a linha)
/// home<TAB>M220-180h150v-250h220...
/// home-fill<TAB>M160-120v-480l320-240...
/// ```
///
/// (`<TAB>` é uma tabulação.) Os caminhos são preenchidos (não contornados).
pub fn parse_pack(text: &str) -> Result<IconPack, String> {
    let mut lines = text.lines();
    if lines.next().map(str::trim) != Some("# aylis-icons 1") {
        return Err("não é um pacote de ícones do Aylis (falta \"# aylis-icons 1\")".into());
    }
    let mut pack = IconPack { view: (0.0, 0.0, 24.0, 24.0), ..Default::default() };
    let mut license = Vec::new();
    for line in lines {
        if let Some(rest) = line.strip_prefix("#|") {
            license.push(rest.strip_prefix(' ').unwrap_or(rest).to_string());
        } else if let Some(rest) = line.strip_prefix("# title:") {
            pack.title = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("# license:") {
            pack.license = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("# view:") {
            let v: Vec<f32> = rest.split_whitespace().filter_map(|n| n.parse().ok()).collect();
            if let [x, y, w, h] = v[..] {
                pack.view = (x, y, w, h);
            } else {
                return Err(format!("view inválido: {rest}"));
            }
        } else if line.starts_with('#') || line.trim().is_empty() {
            continue;
        } else if let Some((name, d)) = line.split_once('\t') {
            pack.icons.push((name.to_string(), d.to_string()));
        }
    }
    pack.license_text = license.join("\n");
    Ok(pack)
}

/// Um ícone do aylis pelo nome.
pub fn builtin(name: &str) -> Option<&'static Icon> {
    builtins().get(name)
}

/// Os nomes dos ícones do aylis, em ordem alfabética (para a IDE e a documentação).
pub fn builtin_names() -> Vec<&'static str> {
    let mut names: Vec<&'static str> = builtins().keys().copied().collect();
    names.sort_unstable();
    names
}
