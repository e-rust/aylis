//! Temas: tokens (cores, fontes, raios, espaçamentos, durações), estilos por classe e
//! por estado, e variantes (`"Button.danger"`).
//!
//! Ordem de prioridade na resolução (do mais fraco para o mais forte):
//! 1. estilo da classe base (ex.: `Button` para um `GlowButton`);
//! 2. estilo da própria classe;
//! 3. variante (`Classe.variante`);
//! 4. estados ativos, na ordem hover → pressed → focused → checked → disabled.
//!
//! Valores definidos na instância (por exemplo um `fill` próprio de um retângulo)
//! ganham de tudo isso; quem decide é o próprio controle ao pintar.
//!
//! **Misturar temas:** um tema pode pegar o visual de algumas classes de outro tema
//! ([`Theme::mix`]: os botões do "Aylis Dark" num app claro), e cada controle pode
//! escolher um tema pela propriedade `theme`, que vale também para os filhos (uma barra
//! lateral escura num form claro). Os temas são procurados pelo nome na biblioteca do
//! [`Ui`](crate::Ui) (`register_theme`).

use std::collections::HashMap;

use crate::canvas::{FontSpec, FontWeight, Shadow};
use crate::color::Color;
use crate::component::ControlState;
use crate::geometry::Bounds;

/// Cor no tema: um token (`"primary"`) ou um valor fixo.
#[derive(Clone, Debug, PartialEq)]
pub enum ColorRef {
    Token(String),
    Value(Color),
}

impl From<Color> for ColorRef {
    fn from(c: Color) -> Self {
        ColorRef::Value(c)
    }
}

/// Texto começando com `#` é uma cor; qualquer outro é o nome de um token.
impl From<&str> for ColorRef {
    fn from(s: &str) -> Self {
        match Color::from_hex(s) {
            Some(c) if s.trim_start().starts_with('#') => ColorRef::Value(c),
            _ => ColorRef::Token(s.to_string()),
        }
    }
}

/// Medida no tema: um token (`"md"`) ou um valor fixo.
#[derive(Clone, Debug, PartialEq)]
pub enum Metric {
    Token(String),
    Value(f32),
}

impl From<f32> for Metric {
    fn from(v: f32) -> Self {
        Metric::Value(v)
    }
}

impl From<&str> for Metric {
    fn from(s: &str) -> Self {
        Metric::Token(s.to_string())
    }
}

/// Um conjunto parcial de propriedades visuais. `None` = não altera.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StyleProps {
    pub fill: Option<ColorRef>,
    pub text: Option<ColorRef>,
    pub border_color: Option<ColorRef>,
    pub border_width: Option<f32>,
    /// Token procurado em [`Theme::radius`].
    pub radius: Option<Metric>,
    pub padding: Option<Bounds>,
    /// Nome em [`Theme::fonts`].
    pub font: Option<String>,
    pub opacity: Option<f32>,
    /// Duração da transição entre estados; token procurado em [`Theme::durations`].
    pub transition: Option<Metric>,
    /// Sombra; token procurado em [`Theme::shadows`] (`"none"` tira a sombra).
    pub shadow: Option<String>,
    /// Material do fundo: `glass` (Liquid Glass: desfoca o que está atrás, com brilho e
    /// borda de luz), `frosted` (vidro fosco, como o Acrylic) ou `none`.
    pub material: Option<String>,
    /// Maiúsculas ou minúsculas no texto (`upper`, `lower`, `none`; o `text-transform` do CSS).
    pub text_case: Option<String>,
    /// Espaço a mais entre as letras, em pixels (o `letter-spacing` do CSS).
    pub letter_spacing: Option<f32>,
    /// A largura da borda de cada lado (esquerda, cima, direita, baixo), no lugar da
    /// `border_width` (o `border-bottom` do CSS).
    pub border_sides: Option<Bounds>,
    /// O traço da borda: `solid` (o padrão), `dashed` ou `dotted`.
    pub border_style: Option<String>,
    /// A altura da linha do texto quebrado, em vezes o tamanho da fonte (o `line-height` do
    /// CSS: 1.5). O relatório do Tabernáculo, T04.
    pub line_height: Option<f32>,
    /// Troca só o peso, só o tamanho ou só o itálico da fonte herdada (do `font` do estilo,
    /// ou da `default`), sem nomear uma fonte nova (T05).
    pub font_weight: Option<crate::canvas::FontWeight>,
    pub font_size: Option<f32>,
    pub italic: Option<bool>,
    /// O espaço entre as partes de um controle feito de várias (os botões do `ButtonGroup`, a
    /// marcação do `SegmentedControl` até a borda), em pixels: 0 cola (o relatório do
    /// Tabernáculo, C38). Sem ele, o de cada controle.
    pub gap: Option<f32>,
}

/// O material do fundo de um controle ([`StyleProps::material`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Material {
    #[default]
    None,
    /// Liquid Glass (iOS 26): desfoque leve, brilho em cima e borda de luz.
    Glass,
    /// Vidro fosco (Acrylic, materiais "regular" da Apple): desfoque forte, sem brilho.
    Frosted,
}

impl Material {
    pub fn parse(name: &str) -> Material {
        match name {
            "glass" => Material::Glass,
            "frosted" | "acrylic" => Material::Frosted,
            _ => Material::None,
        }
    }

    /// Desvio do desfoque (unidades lógicas).
    pub fn blur(self) -> f32 {
        match self {
            Material::None => 0.0,
            Material::Glass => 8.0,
            Material::Frosted => 22.0,
        }
    }
}

impl StyleProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fill(mut self, c: impl Into<ColorRef>) -> Self {
        self.fill = Some(c.into());
        self
    }

    pub fn text(mut self, c: impl Into<ColorRef>) -> Self {
        self.text = Some(c.into());
        self
    }

    pub fn border(mut self, c: impl Into<ColorRef>, width: f32) -> Self {
        self.border_color = Some(c.into());
        self.border_width = Some(width);
        self
    }

    pub fn radius(mut self, m: impl Into<Metric>) -> Self {
        self.radius = Some(m.into());
        self
    }

    pub fn padding(mut self, b: impl Into<Bounds>) -> Self {
        self.padding = Some(b.into());
        self
    }

    pub fn font(mut self, name: &str) -> Self {
        self.font = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, v: f32) -> Self {
        self.opacity = Some(v);
        self
    }

    pub fn transition(mut self, m: impl Into<Metric>) -> Self {
        self.transition = Some(m.into());
        self
    }

    pub fn shadow(mut self, token: &str) -> Self {
        self.shadow = Some(token.to_string());
        self
    }

    /// `glass`, `frosted` ou `none` (veja [`Material`]).
    pub fn material(mut self, name: &str) -> Self {
        self.material = Some(name.to_string());
        self
    }

    /// Copia para `self` tudo que está definido em `other`.
    pub fn merge(&mut self, other: &StyleProps) {
        if other.fill.is_some() {
            self.fill.clone_from(&other.fill);
        }
        if other.text.is_some() {
            self.text.clone_from(&other.text);
        }
        if other.border_color.is_some() {
            self.border_color.clone_from(&other.border_color);
        }
        if other.border_width.is_some() {
            self.border_width = other.border_width;
        }
        if other.radius.is_some() {
            self.radius.clone_from(&other.radius);
        }
        if other.padding.is_some() {
            self.padding = other.padding;
        }
        if other.font.is_some() {
            self.font.clone_from(&other.font);
        }
        if other.opacity.is_some() {
            self.opacity = other.opacity;
        }
        if other.transition.is_some() {
            self.transition.clone_from(&other.transition);
        }
        if other.shadow.is_some() {
            self.shadow.clone_from(&other.shadow);
        }
        if other.material.is_some() {
            self.material.clone_from(&other.material);
        }
        if other.text_case.is_some() {
            self.text_case.clone_from(&other.text_case);
        }
        if other.letter_spacing.is_some() {
            self.letter_spacing = other.letter_spacing;
        }
        if other.gap.is_some() {
            self.gap = other.gap;
        }
        if other.border_sides.is_some() {
            self.border_sides = other.border_sides;
        }
        if other.border_style.is_some() {
            self.border_style.clone_from(&other.border_style);
        }
        if other.line_height.is_some() {
            self.line_height = other.line_height;
        }
        if other.font_weight.is_some() {
            self.font_weight = other.font_weight;
        }
        if other.font_size.is_some() {
            self.font_size = other.font_size;
        }
        if other.italic.is_some() {
            self.italic = other.italic;
        }
    }
}

/// Estilo de uma classe (ou variante): valores base e ajustes por estado.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Style {
    pub base: StyleProps,
    pub hover: StyleProps,
    pub pressed: StyleProps,
    pub focused: StyleProps,
    pub checked: StyleProps,
    pub disabled: StyleProps,
}

impl Style {
    pub fn new(base: StyleProps) -> Self {
        Self { base, ..Default::default() }
    }

    pub fn hover(mut self, p: StyleProps) -> Self {
        self.hover = p;
        self
    }

    pub fn pressed(mut self, p: StyleProps) -> Self {
        self.pressed = p;
        self
    }

    pub fn focused(mut self, p: StyleProps) -> Self {
        self.focused = p;
        self
    }

    pub fn checked(mut self, p: StyleProps) -> Self {
        self.checked = p;
        self
    }

    pub fn disabled(mut self, p: StyleProps) -> Self {
        self.disabled = p;
        self
    }

    /// Estados na ordem de aplicação: hover, pressed, focused, checked, disabled.
    fn state(&self, index: usize) -> &StyleProps {
        match index {
            0 => &self.hover,
            1 => &self.pressed,
            2 => &self.focused,
            3 => &self.checked,
            _ => &self.disabled,
        }
    }
}

/// Estilo final, já com tokens resolvidos, entregue ao controle na pintura.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedStyle {
    pub fill: Color,
    pub text: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub radius: f32,
    pub padding: Bounds,
    pub font: FontSpec,
    pub opacity: f32,
    pub transition_ms: f32,
    /// Sombra desenhada pelo `Ui` antes do controle (no retângulo dele, com o `radius`).
    pub shadow: Option<Shadow>,
    /// Fundo desfocado (vidro), feito pelo `Ui` antes do controle.
    pub material: Material,
    /// Maiúsculas ou minúsculas no texto (os controles de texto aplicam ao desenhar).
    pub text_case: crate::text::TextCase,
    /// Espaço a mais entre as letras.
    pub letter_spacing: f32,
    /// A borda de cada lado (`None`: a `border_width` em volta toda).
    pub border_sides: Option<Bounds>,
    /// O traço da borda (vazio: contínuo).
    pub border_dash: Vec<f32>,
    /// A altura da linha em vezes o tamanho da fonte (0: a da fonte).
    pub line_height: f32,
    /// O espaço entre as partes do controle (`None`: o dele; [`StyleProps::gap`]).
    pub gap: Option<f32>,
}

impl ResolvedStyle {
    /// O traço da borda do estilo, na cor `color` e na largura `width`.
    pub fn border_stroke(&self, color: Color, width: f32) -> crate::canvas::Stroke {
        let stroke = crate::canvas::Stroke::new(color, width);
        if self.border_dash.is_empty() { stroke } else { stroke.dashed(self.border_dash.clone()) }
    }
}

impl Default for ResolvedStyle {
    fn default() -> Self {
        Self {
            fill: Color::TRANSPARENT,
            text: Color::BLACK,
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
            radius: 0.0,
            padding: Bounds::ZERO,
            font: FontSpec::default(),
            opacity: 1.0,
            transition_ms: 0.0,
            shadow: None,
            material: Material::None,
            text_case: crate::text::TextCase::None,
            letter_spacing: 0.0,
            border_sides: None,
            border_dash: Vec::new(),
            line_height: 0.0,
            gap: None,
        }
    }
}

impl ResolvedStyle {
    /// Estilo intermediário numa transição (`t` de 0 a 1). Cores, espessuras, raio e
    /// opacidade são interpolados; fonte e padding mudam direto para os de `other`.
    pub fn lerp(&self, other: &ResolvedStyle, t: f32) -> ResolvedStyle {
        let mix = |a: f32, b: f32| a + (b - a) * t;
        ResolvedStyle {
            fill: self.fill.lerp(other.fill, t),
            text: self.text.lerp(other.text, t),
            border_color: self.border_color.lerp(other.border_color, t),
            border_width: mix(self.border_width, other.border_width),
            radius: mix(self.radius, other.radius),
            padding: other.padding,
            font: other.font.clone(),
            opacity: mix(self.opacity, other.opacity),
            transition_ms: other.transition_ms,
            shadow: match (self.shadow, other.shadow) {
                (Some(a), Some(b)) => Some(Shadow {
                    color: a.color.lerp(b.color, t),
                    blur: mix(a.blur, b.blur),
                    offset: crate::PointF::new(mix(a.offset.x, b.offset.x), mix(a.offset.y, b.offset.y)),
                    spread: mix(a.spread, b.spread),
                }),
                _ => other.shadow,
            },
            material: other.material,
            text_case: other.text_case,
            letter_spacing: mix(self.letter_spacing, other.letter_spacing),
            border_sides: other.border_sides,
            border_dash: other.border_dash.clone(),
            line_height: mix(self.line_height, other.line_height),
            gap: other.gap,
        }
    }
}

/// O desenho dos controles que mudam de forma conforme a plataforma imitada (o Switch do
/// iOS é largo e verde, o do Windows 11 é estreito com bolinha pequena; o CheckBox do
/// iOS é redondo...). As cores, raios e fontes vêm dos tokens; o look decide a forma.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Look {
    /// O visual próprio do aylis.
    #[default]
    Aylis,
    /// Android 5 a 11 (Material Design 2).
    Material2,
    /// Android 12 em diante (Material 3 / Material You).
    Material3,
    /// iOS 7 a 12 (plano, botões só com texto).
    Ios7,
    /// iOS 13 a 18.
    Ios13,
    /// iOS 26 (vidro: superfícies translúcidas e cápsulas).
    Ios26,
    /// Windows 7 (Aero: gradientes).
    Win7,
    /// Windows 10 (Fluent/Metro: cantos retos).
    Win10,
    /// Windows 11 e WinUI 3 (Fluent 2: cantos arredondados, traço de baixo nos campos).
    Win11,
    /// Windows Phone 7 e 8 (Metro: bordas grossas, cantos retos, Switch retangular).
    WinPhone,
}

impl crate::prop::PropEnum for Look {
    const NAMES: &'static [&'static str] = &["aylis", "material2", "material3", "ios7", "ios13", "ios26", "win7", "win10", "win11", "winphone"];

    fn from_index(index: usize) -> Option<Self> {
        [
            Look::Aylis,
            Look::Material2,
            Look::Material3,
            Look::Ios7,
            Look::Ios13,
            Look::Ios26,
            Look::Win7,
            Look::Win10,
            Look::Win11,
            Look::WinPhone,
        ]
        .get(index)
        .copied()
    }

    fn index(self) -> usize {
        self as usize
    }
}

impl Look {
    pub fn is_material(self) -> bool {
        matches!(self, Look::Material2 | Look::Material3)
    }

    pub fn is_ios(self) -> bool {
        matches!(self, Look::Ios7 | Look::Ios13 | Look::Ios26)
    }

    pub fn is_windows(self) -> bool {
        matches!(self, Look::Win7 | Look::Win10 | Look::Win11 | Look::WinPhone)
    }
}

/// Um tema completo.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Theme {
    pub name: String,
    /// Família do tema ("Material 3", "Windows 11"...): agrupa o claro e o escuro na IDE.
    /// Vazio = o próprio nome.
    pub family: String,
    /// É a versão escura da família.
    pub dark: bool,
    /// Forma dos controles (veja [`Look`]).
    pub look: Look,
    pub colors: HashMap<String, Color>,
    pub fonts: HashMap<String, FontSpec>,
    pub radius: HashMap<String, f32>,
    pub spacing: HashMap<String, f32>,
    /// Em milissegundos.
    pub durations: HashMap<String, f32>,
    pub shadows: HashMap<String, Shadow>,
    /// Chave: nome da classe (`"Button"`) ou classe + variante (`"Button.danger"`).
    pub styles: HashMap<String, Style>,
    /// Classes que usam o visual de outro tema: classe → nome do tema ([`Theme::mix`]).
    pub components: HashMap<String, String>,
}

impl Theme {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), ..Default::default() }
    }

    /// Tema claro padrão.
    pub fn light() -> Self {
        let mut t = Theme::new("Aylis Light");
        t.family = "Aylis".into();
        t.set_colors(&[
            ("background", "#F5F5F7"),
            ("surface", "#FFFFFF"),
            ("text", "#1F2328"),
            ("text_muted", "#6B7280"),
            ("border", "#D0D7DE"),
            ("primary", "#2563EB"),
            ("primary_hover", "#3B82F6"),
            ("primary_pressed", "#1D4ED8"),
            ("on_primary", "#FFFFFF"),
            // O texto sobre o vermelho e o verde (botões Perigo e Sucesso), como o `on_primary`.
            ("on_danger", "#FFFFFF"),
            ("on_success", "#FFFFFF"),
            ("danger", "#DC2626"),
            ("danger_hover", "#EF4444"),
            ("danger_pressed", "#B91C1C"),
            ("success", "#16A34A"),
            ("warning", "#D97706"),
            ("focus_ring", "#3B82F6"),
            ("accent", "#9333EA"),
            ("hover_overlay", "#0000000F"),
            ("glass_fill", "#FFFFFF73"),
            ("primary_container", "#DBEAFE"),
            ("on_primary_container", "#1E3A8A"),
            ("selection", "#2563EB40"),
            ("track", "#D0D7DE"),
            ("surface_2", "#F0F1F4"),
            ("info", "#0284C7"),
            ("primary_soft", "#2563EB1A"),
            ("success_soft", "#16A34A1A"),
            ("warning_soft", "#D977061F"),
            ("danger_soft", "#DC26261A"),
            ("info_soft", "#0284C71A"),
            ("meter_low", "#16A34A"),
            ("meter_mid", "#EAB308"),
            ("meter_high", "#DC2626"),
            ("presence_online", "#16A34A"),
            ("presence_away", "#EAB308"),
            ("presence_busy", "#DC2626"),
            ("presence_offline", "#9CA3AF"),
        ]);
        t.shadows.insert("sm".into(), Shadow::new(Color::rgba(15, 23, 42, 20), 4.0, 0.0, 1.0));
        t.shadows.insert("md".into(), Shadow::new(Color::rgba(15, 23, 42, 28), 12.0, 0.0, 4.0));
        t.shadows.insert("lg".into(), Shadow::new(Color::rgba(15, 23, 42, 36), 28.0, 0.0, 10.0));
        // A da gaveta e do diálogo grande (o `shadow-2xl` do Tailwind; C32 do Tabernáculo).
        t.shadows.insert("2xl".into(), Shadow::new(Color::rgba(15, 23, 42, 64), 50.0, 0.0, 25.0));
        t.add_standard_tokens();
        t.add_standard_styles();
        t
    }

    /// Tema escuro padrão.
    pub fn dark() -> Self {
        let mut t = Theme::new("Aylis Dark");
        t.family = "Aylis".into();
        t.dark = true;
        t.set_colors(&[
            ("background", "#1E1E1E"),
            ("surface", "#252526"),
            ("text", "#E6E6E6"),
            ("text_muted", "#9A9A9A"),
            ("border", "#3C3C3C"),
            ("primary", "#3B82F6"),
            ("primary_hover", "#60A5FA"),
            ("primary_pressed", "#2563EB"),
            ("on_primary", "#FFFFFF"),
            // O texto sobre o vermelho e o verde (botões Perigo e Sucesso), como o `on_primary`.
            ("on_danger", "#FFFFFF"),
            ("on_success", "#FFFFFF"),
            ("danger", "#EF4444"),
            ("danger_hover", "#F87171"),
            ("danger_pressed", "#DC2626"),
            ("success", "#22C55E"),
            ("warning", "#F59E0B"),
            ("focus_ring", "#60A5FA"),
            ("accent", "#A855F7"),
            ("hover_overlay", "#FFFFFF14"),
            ("glass_fill", "#1E1E2273"),
            ("primary_container", "#1E3A8A"),
            ("on_primary_container", "#DBEAFE"),
            ("selection", "#3B82F660"),
            ("track", "#4A4A4F"),
            ("surface_2", "#2D2D30"),
            ("info", "#38BDF8"),
            ("primary_soft", "#3B82F62E"),
            ("success_soft", "#22C55E29"),
            ("warning_soft", "#F59E0B29"),
            ("danger_soft", "#EF444429"),
            ("info_soft", "#38BDF829"),
            ("meter_low", "#22C55E"),
            ("meter_mid", "#FACC15"),
            ("meter_high", "#EF4444"),
            ("presence_online", "#22C55E"),
            ("presence_away", "#FACC15"),
            ("presence_busy", "#EF4444"),
            ("presence_offline", "#6B7280"),
        ]);
        t.shadows.insert("sm".into(), Shadow::new(Color::rgba(0, 0, 0, 70), 4.0, 0.0, 1.0));
        t.shadows.insert("md".into(), Shadow::new(Color::rgba(0, 0, 0, 90), 14.0, 0.0, 4.0));
        t.shadows.insert("lg".into(), Shadow::new(Color::rgba(0, 0, 0, 110), 30.0, 0.0, 10.0));
        t.shadows.insert("2xl".into(), Shadow::new(Color::rgba(0, 0, 0, 150), 50.0, 0.0, 25.0));
        t.add_standard_tokens();
        t.add_standard_styles();
        t
    }

    fn set_colors(&mut self, colors: &[(&str, &str)]) {
        for (name, hex) in colors {
            let color = Color::from_hex(hex).expect("cor inválida no tema padrão");
            self.colors.insert(name.to_string(), color);
        }
    }

    fn add_standard_tokens(&mut self) {
        self.fonts.insert("default".into(), FontSpec::new("", 14.0));
        self.fonts.insert("title".into(), FontSpec::new("", 20.0).weight(FontWeight::Bold));
        self.fonts.insert("heading".into(), FontSpec::new("", 16.0).weight(FontWeight::SemiBold));
        self.fonts.insert("small".into(), FontSpec::new("", 12.0));
        self.fonts.insert("strong".into(), FontSpec::new("", 14.0).weight(FontWeight::SemiBold));
        self.fonts.insert("mono".into(), FontSpec::new("monospace", 13.0));
        for (k, v) in [("sm", 4.0), ("md", 6.0), ("lg", 10.0)] {
            self.radius.insert(k.into(), v);
        }
        for (k, v) in [("xs", 4.0), ("sm", 8.0), ("md", 12.0), ("lg", 16.0)] {
            self.spacing.insert(k.into(), v);
        }
        for (k, v) in [("fast", 120.0), ("normal", 200.0), ("slow", 350.0)] {
            self.durations.insert(k.into(), v);
        }
    }

    fn add_standard_styles(&mut self) {
        self.set_style("Form", Style::new(StyleProps::new().fill("background").text("text")));
        self.set_style(
            "Panel",
            Style::new(StyleProps::new().fill("surface").border("border", 1.0).radius("sm")),
        );
        self.set_style("Label", Style::new(StyleProps::new().text("text")));
        self.set_style(
            "Button",
            Style::new(
                StyleProps::new()
                    .fill("primary")
                    .text("on_primary")
                    .radius("md")
                    .padding((12.0, 6.0))
                    .transition("fast"),
            )
            .hover(StyleProps::new().fill("primary_hover"))
            .pressed(StyleProps::new().fill("primary_pressed"))
            .focused(StyleProps::new().border("focus_ring", 2.0))
            .disabled(StyleProps::new().opacity(0.5)),
        );
        self.set_style(
            "Button.danger",
            Style::new(StyleProps::new().fill("danger").text("on_danger"))
                .hover(StyleProps::new().fill("danger_hover"))
                .pressed(StyleProps::new().fill("danger_pressed")),
        );
        // Redondo, na cor de destaque: o botão de tocar de um player, uma ação principal.
        self.set_style(
            "Button.round",
            Style::new(StyleProps::new().fill("primary").text("on_primary").radius(999.0).border(Color::TRANSPARENT, 0.0))
                .hover(StyleProps::new().fill("primary_hover"))
                .pressed(StyleProps::new().fill("primary_pressed")),
        );
        self.set_style(
            "Button.flat",
            Style::new(StyleProps::new().fill(Color::TRANSPARENT).text("primary"))
                .hover(StyleProps::new().fill("hover_overlay"))
                .pressed(StyleProps::new().fill("hover_overlay")),
        );
        self.set_style(
            "Edit",
            Style::new(
                StyleProps::new()
                    .fill("surface")
                    .text("text")
                    .border("border", 1.0)
                    .radius("sm")
                    .padding((8.0, 4.0)),
            )
            .hover(StyleProps::new().border("text_muted", 1.0))
            .focused(StyleProps::new().border("focus_ring", 2.0))
            .disabled(StyleProps::new().opacity(0.5)),
        );
        self.set_style(
            "ComboBox",
            Style::new(
                StyleProps::new().fill("surface").text("text").border("border", 1.0).radius("sm").padding((8.0, 4.0)),
            )
            .hover(StyleProps::new().border("text_muted", 1.0))
            .focused(StyleProps::new().border("focus_ring", 2.0))
            .disabled(StyleProps::new().opacity(0.5)),
        );
        // CheckBox e RadioButton: `fill`/`border` são da caixinha, `text` é do rótulo.
        for class in ["CheckBox", "RadioButton"] {
            self.set_style(
                class,
                Style::new(
                    StyleProps::new().fill("surface").text("text").border("text_muted", 1.5).radius("sm").transition("fast"),
                )
                .hover(StyleProps::new().border("primary", 1.5))
                .checked(StyleProps::new().fill("primary").border("primary", 1.5))
                .focused(StyleProps::new().border("focus_ring", 2.0))
                .disabled(StyleProps::new().opacity(0.5)),
            );
        }
        // Switch: `fill` é o trilho.
        self.set_style(
            "Switch",
            Style::new(StyleProps::new().fill("track").text("text").transition("normal"))
                .checked(StyleProps::new().fill("primary"))
                .focused(StyleProps::new().border("focus_ring", 2.0))
                .disabled(StyleProps::new().opacity(0.5)),
        );
        // ProgressBar: `fill` é o trilho; a barra usa o token `primary`.
        self.set_style("ProgressBar", Style::new(StyleProps::new().fill("track").radius(4.0)));
        // Campos e listas com borda (como o Edit).
        for class in ["Memo", "ListBox", "ListView", "TreeView", "Calendar"] {
            self.set_style(
                class,
                Style::new(StyleProps::new().fill("surface").text("text").border("border", 1.0).radius("sm").padding((6.0, 4.0)))
                    .focused(StyleProps::new().border("focus_ring", 2.0))
                    .disabled(StyleProps::new().opacity(0.5)),
            );
            // `flat`: a lista solta na tela, sem borda (nem a do foco), como os recentes da
            // tela inicial da IDE.
            self.set_style(
                &format!("{class}.flat"),
                Style::new(StyleProps::new().border(Color::TRANSPARENT, 0.0))
                    .focused(StyleProps::new().border(Color::TRANSPARENT, 0.0)),
            );
        }
        // O editor de código ocupa a janela: sem borda (nem a do foco) e sem cantos, como no
        // VS Code.
        self.set_style(
            "CodeEditor",
            Style::new(StyleProps::new().fill("surface").text("text").border(Color::TRANSPARENT, 0.0).radius(0.0).padding((4.0, 2.0)))
                .focused(StyleProps::new().border(Color::TRANSPARENT, 0.0))
                .disabled(StyleProps::new().opacity(0.5)),
        );
        // A faixa de abas do navegador: a aba ativa tem a cor do conteúdo (`surface`) e se
        // destaca da faixa.
        self.set_style("TabStrip", Style::new(StyleProps::new().fill("surface_2").text("text").border(Color::TRANSPARENT, 0.0)));
        // TrackBar: `fill` é o trilho; o polegar usa `primary`.
        self.set_style(
            "TrackBar",
            Style::new(StyleProps::new().fill("track").text("text").radius(3.0))
                .focused(StyleProps::new().border("focus_ring", 2.0))
                .disabled(StyleProps::new().opacity(0.5)),
        );
        self.set_style("GroupBox", Style::new(StyleProps::new().text("text").border("border", 1.0).radius("md")));
        self.set_style("Expander", Style::new(StyleProps::new().fill("surface").text("text").border("border", 1.0).radius("md")));
        self.set_style("TabControl", Style::new(StyleProps::new().fill("surface").text("text").border("border", 1.0).radius("md")));
        self.set_style("TabItem", Style::new(StyleProps::new().text("text")));
        self.set_style("StatusBar", Style::new(StyleProps::new().fill("surface").text("text_muted").border("border", 1.0)));
        self.set_style("ToolBar", Style::new(StyleProps::new().fill("surface").text("text").border("border", 1.0)));
        self.set_style("RadioGroup", Style::new(StyleProps::new().text("text").border("border", 1.0).radius("md")));
        self.set_style("Chart", Style::new(StyleProps::new().fill("surface").text("text").border("border", 1.0).radius("md")));
        self.add_variant_styles();
    }

    /// Os subtipos que o aylis traz (aparecem na paleta debaixo de cada classe).
    fn add_variant_styles(&mut self) {
        // ----- Button -----
        self.set_style(
            "Button.secondary",
            Style::new(StyleProps::new().fill("surface").text("text").border("border", 1.0))
                .hover(StyleProps::new().fill("surface_2").border("text_muted", 1.0))
                .pressed(StyleProps::new().fill("track")),
        );
        self.set_style(
            "Button.outline",
            Style::new(StyleProps::new().fill(Color::TRANSPARENT).text("primary").border("primary", 1.0))
                .hover(StyleProps::new().fill("primary_soft"))
                .pressed(StyleProps::new().fill("primary_soft")),
        );
        self.set_style(
            "Button.success",
            Style::new(StyleProps::new().fill("success").text("on_success"))
                .hover(StyleProps::new().opacity(0.9))
                .pressed(StyleProps::new().opacity(0.8)),
        );
        // Os estilos do QBtn do Quasar (no tema "Quasar 2" eles têm a cara exata do Quasar).
        self.set_style(
            "Button.positive",
            Style::new(StyleProps::new().fill("success").text("on_success")).hover(StyleProps::new().opacity(0.9)).pressed(StyleProps::new().opacity(0.8)),
        );
        self.set_style(
            "Button.negative",
            Style::new(StyleProps::new().fill("danger").text("on_danger")).hover(StyleProps::new().fill("danger_hover")).pressed(StyleProps::new().fill("danger_pressed")),
        );
        self.set_style(
            "Button.accent",
            Style::new(StyleProps::new().fill("accent").text("on_primary")).hover(StyleProps::new().opacity(0.9)).pressed(StyleProps::new().opacity(0.8)),
        );
        self.set_style("Button.rounded", Style::new(StyleProps::new().radius(28.0)));
        self.set_style("Button.unelevated", Style::new(StyleProps::new().shadow("none")));
        self.set_style(
            "Button.push",
            Style::new(StyleProps::new().border(Color::rgba(0, 0, 0, 0x40), 3.0)).pressed(StyleProps::new().fill("primary_pressed").border(Color::rgba(0, 0, 0, 0x40), 1.0)),
        );
        self.set_style("Button.glossy", Style::new(StyleProps::new().fill("primary_hover").shadow("md")).hover(StyleProps::new().fill("primary")));
        self.set_style(
            "Button.icon",
            Style::new(StyleProps::new().fill(Color::TRANSPARENT).text("text").padding(6.0).radius("md"))
                .hover(StyleProps::new().fill("hover_overlay"))
                .pressed(StyleProps::new().fill("track"))
                .checked(StyleProps::new().fill("primary_soft").text("primary")),
        );
        self.set_style(
            "Button.fab",
            Style::new(StyleProps::new().fill("primary").text("on_primary").radius(999.0).shadow("md").padding(10.0))
                .hover(StyleProps::new().fill("primary_hover").shadow("lg"))
                .pressed(StyleProps::new().fill("primary_pressed").shadow("sm")),
        );
        self.set_style(
            "Button.link",
            Style::new(StyleProps::new().fill(Color::TRANSPARENT).text("primary").padding((2.0, 2.0)))
                .hover(StyleProps::new().fill(Color::TRANSPARENT).text("primary_hover"))
                .pressed(StyleProps::new().fill(Color::TRANSPARENT).text("primary_pressed")),
        );
        self.set_style(
            "Button.pill",
            Style::new(StyleProps::new().fill("surface_2").text("text").radius(999.0).padding((14.0, 6.0)))
                .hover(StyleProps::new().fill("track"))
                .checked(StyleProps::new().fill("primary").text("on_primary")),
        );
        // Os botões do Material 3 e o vidro do iOS 26 (valem em qualquer tema; os temas de
        // plataforma refinam).
        self.set_style(
            "Button.elevated",
            Style::new(StyleProps::new().fill("surface").text("primary").radius(999.0).shadow("sm"))
                .hover(StyleProps::new().shadow("md"))
                .pressed(StyleProps::new().fill("surface_2")),
        );
        self.set_style(
            "Button.tonal",
            Style::new(StyleProps::new().fill("primary_soft").text("primary").border(Color::TRANSPARENT, 0.0).radius(999.0))
                .hover(StyleProps::new().shadow("sm")),
        );
        self.set_style(
            "Button.text",
            Style::new(StyleProps::new().fill(Color::TRANSPARENT).text("primary").border(Color::TRANSPARENT, 0.0).shadow("none"))
                .hover(StyleProps::new().fill("primary_soft")),
        );
        self.set_style(
            "Button.fab_extended",
            Style::new(StyleProps::new().fill("primary_container").text("on_primary_container").radius(16.0).shadow("md").padding((16.0, 0.0)))
                .hover(StyleProps::new().shadow("lg")),
        );
        self.set_style(
            "Button.icon_filled",
            Style::new(StyleProps::new().fill("primary").text("on_primary").radius(999.0).padding(6.0)).hover(StyleProps::new().fill("primary_hover")),
        );
        self.set_style(
            "Button.icon_tonal",
            Style::new(StyleProps::new().fill("primary_soft").text("primary").radius(999.0).padding(6.0)).hover(StyleProps::new().shadow("sm")),
        );
        self.set_style(
            "Button.icon_outlined",
            Style::new(StyleProps::new().fill(Color::TRANSPARENT).text("text").border("border", 1.0).radius(999.0).padding(6.0))
                .hover(StyleProps::new().fill("hover_overlay")),
        );
        self.set_style(
            "Button.glass",
            Style::new(StyleProps::new().fill("glass_fill").text("text").border(Color::TRANSPARENT, 0.0).radius(999.0).shadow("sm").material("glass"))
                .pressed(StyleProps::new().opacity(0.8)),
        );
        self.set_style(
            "Button.glass_prominent",
            Style::new(StyleProps::new().fill("primary").text("on_primary").border(Color::TRANSPARENT, 0.0).radius(999.0).shadow("sm").material("glass"))
                .pressed(StyleProps::new().opacity(0.85)),
        );
        // ----- Edit -----
        self.set_style(
            "Edit.filled",
            Style::new(StyleProps::new().fill("surface_2").border(Color::TRANSPARENT, 0.0).radius(4.0).padding((14.0, 4.0)))
                .hover(StyleProps::new().fill("track")),
        );
        self.set_style(
            "Edit.outlined",
            Style::new(StyleProps::new().fill(Color::TRANSPARENT).border("border", 1.0).radius(4.0).padding((14.0, 4.0)))
                .focused(StyleProps::new().border("primary", 2.0)),
        );
        self.set_style(
            "Edit.search",
            Style::new(StyleProps::new().fill("surface_2").border(Color::TRANSPARENT, 1.0).radius(999.0).padding((12.0, 4.0)))
                .hover(StyleProps::new().border("border", 1.0))
                .focused(StyleProps::new().fill("surface").border("focus_ring", 2.0)),
        );
        self.set_style(
            "Edit.flat",
            Style::new(StyleProps::new().fill(Color::TRANSPARENT).border(Color::TRANSPARENT, 0.0).radius(0.0))
                .hover(StyleProps::new().fill("hover_overlay").border(Color::TRANSPARENT, 0.0))
                .focused(StyleProps::new().border(Color::TRANSPARENT, 0.0)),
        );
        // ----- Panel -----
        self.set_style("Panel.card", Style::new(StyleProps::new().fill("surface").border("border", 1.0).radius("lg").shadow("sm")));
        // O Card (do Quasar): o cartão, sem borda e com sombra; `flat` e `bordered` como no QCard.
        self.set_style("Card", Style::new(StyleProps::new().fill("surface").text("text").border("border", 1.0).radius("lg").shadow("sm")));
        self.set_style("Popover", Style::new(StyleProps::new().fill("surface").text("text").border("border", 1.0).radius("md").shadow("lg").padding(12.0)));
        self.set_style("Card.flat", Style::new(StyleProps::new().shadow("none")));
        self.set_style("Card.bordered", Style::new(StyleProps::new().border("border", 1.0).shadow("none")));
        self.set_style("Panel.flat", Style::new(StyleProps::new().fill("surface").border(Color::TRANSPARENT, 0.0).radius(0.0)));
        self.set_style("Panel.sidebar", Style::new(StyleProps::new().fill("surface_2").border("border", 1.0).radius(0.0)));
        self.set_style("Panel.overlay", Style::new(StyleProps::new().fill("surface").border("border", 1.0).radius("lg").shadow("lg")));
        self.set_style("Panel.filled", Style::new(StyleProps::new().fill("surface_2").border(Color::TRANSPARENT, 0.0).radius("lg").shadow("none")));
        self.set_style("Panel.outlined", Style::new(StyleProps::new().fill("surface").border("border", 1.0).radius("lg").shadow("none")));
        self.set_style("Panel.glass", Style::new(StyleProps::new().fill("glass_fill").border(Color::TRANSPARENT, 0.0).radius("lg").shadow("md").material("glass")));
        self.set_style("ToolBar.floating", Style::new(StyleProps::new().fill("surface_2").border(Color::TRANSPARENT, 0.0).radius(999.0).shadow("md").padding(6.0)));
        // ----- Label -----
        self.set_style("Label.title", Style::new(StyleProps::new().font("title")));
        self.set_style("Label.heading", Style::new(StyleProps::new().font("heading")));
        self.set_style("Label.muted", Style::new(StyleProps::new().text("text_muted")));
        self.set_style("Label.caption", Style::new(StyleProps::new().text("text_muted").font("small")));
        self.set_style("Label.mono", Style::new(StyleProps::new().font("mono")));
        // A tecla (`<kbd>`): monoespaçada, com borda e fundo.
        self.set_style(
            "Label.kbd",
            Style::new(StyleProps::new().font("mono").fill("surface_2").border("border", 1.0).radius(4.0).padding((6.0, 1.0))),
        );
        self.set_style("Label.link", Style::new(StyleProps::new().text("primary")).hover(StyleProps::new().text("primary_hover")));
        self.set_style("Label.error", Style::new(StyleProps::new().text("danger").font("small")));
        // ----- ProgressBar (`text` é a cor da barra) -----
        self.set_style("ProgressBar.thin", Style::new(StyleProps::new().radius(2.0)));
        self.set_style("ProgressBar.success", Style::new(StyleProps::new().text("success")));
        self.set_style("ProgressBar.danger", Style::new(StyleProps::new().text("danger")));
        // ----- Switch / CheckBox -----
        self.set_style("Switch.success", Style::new(StyleProps::new()).checked(StyleProps::new().fill("success")));
        self.set_style("CheckBox.round", Style::new(StyleProps::new().radius(999.0)));
        // ----- componentes modernos -----
        self.set_style("Icon", Style::new(StyleProps::new().text("text")));
        // Balão da dica (propriedade `hint`): cores invertidas.
        // A dica: no tema claro, escura (invertida); no escuro, escura também (a superfície
        // elevada com borda), e não uma caixa clara acesa no meio da tela.
        let hint = if self.dark {
            StyleProps::new().fill("surface_2").text("text").border("border", 1.0)
        } else {
            StyleProps::new().fill("text").text("surface")
        };
        self.set_style("Hint", Style::new(hint.radius("sm").padding((8.0, 5.0)).font("small").shadow("md")));
        self.set_style("Icon.muted", Style::new(StyleProps::new().text("text_muted")));
        self.set_style("Badge", Style::new(StyleProps::new().fill("surface_2").text("text_muted").radius(999.0).padding((8.0, 2.0)).font("small")));
        for tone in ["primary", "success", "warning", "danger", "info"] {
            self.set_style(&format!("Badge.{tone}"), Style::new(StyleProps::new().fill(format!("{tone}_soft").as_str()).text(tone)));
        }
        self.set_style("Badge.live", Style::new(StyleProps::new().fill("danger").text("on_primary").font("strong")));
        self.set_style("Badge.outline", Style::new(StyleProps::new().fill(Color::TRANSPARENT).text("text").border("border", 1.0)));
        self.set_style("Banner", Style::new(StyleProps::new().text("text").radius("md").padding((12.0, 10.0))));
        self.set_style(
            "SegmentedControl",
            Style::new(StyleProps::new().fill("surface_2").text("text").radius("md").transition("fast")).disabled(StyleProps::new().opacity(0.5)),
        );
        self.set_style("LevelMeter", Style::new(StyleProps::new().fill("track").radius(2.0)));
        self.set_style(
            "HotkeyEdit",
            Style::new(StyleProps::new().fill("surface").text("text").border("border", 1.0).radius("sm").padding((8.0, 4.0)))
                .hover(StyleProps::new().border("text_muted", 1.0))
                .focused(StyleProps::new().border("focus_ring", 2.0))
                .disabled(StyleProps::new().opacity(0.5)),
        );
        self.set_style("SeekBar", Style::new(StyleProps::new().text("text")).disabled(StyleProps::new().opacity(0.5)));
        self.set_style("NodeGraph", Style::new(StyleProps::new().fill("background").text("text").border("border", 1.0).radius("lg")));
        self.set_style("ChatView", Style::new(StyleProps::new().fill("background").text("text")));
        self.set_style("DirectionPad", Style::new(StyleProps::new().text("text")).disabled(StyleProps::new().opacity(0.5)));
    }

    pub fn set_style(&mut self, key: &str, style: Style) {
        self.styles.insert(key.to_string(), style);
    }

    /// A família (ou o nome, se não tiver).
    pub fn family_name(&self) -> &str {
        if self.family.is_empty() { &self.name } else { &self.family }
    }

    /// Faz a classe `class` (e as derivadas dela, e todas as variantes) usar o visual do
    /// tema `theme` (pelo nome, na biblioteca do `Ui`). Vazio volta ao deste tema.
    pub fn mix(&mut self, class: &str, theme: &str) {
        if theme.is_empty() {
            self.components.remove(class);
        } else {
            self.components.insert(class.to_string(), theme.to_string());
        }
    }

    /// O tema que cuida da classe: o nome do tema misturado para a classe mais derivada
    /// que tiver um (`classes` da mais derivada para a base).
    pub fn mixed_for(&self, classes: &[&str]) -> Option<&str> {
        if self.components.is_empty() {
            return None;
        }
        classes.iter().find_map(|c| self.components.get(*c).map(String::as_str))
    }

    /// As variantes definidas para a classe (`"Button.danger"` → `"danger"`), em ordem.
    pub fn variants_of(&self, class: &str) -> Vec<String> {
        let prefix = format!("{class}.");
        let mut v: Vec<String> = self.styles.keys().filter_map(|k| k.strip_prefix(&prefix).map(str::to_string)).collect();
        v.sort();
        v
    }

    pub fn style(&self, key: &str) -> Option<&Style> {
        self.styles.get(key)
    }

    /// Cor de um token; transparente se não existir.
    /// A cor de um nome do tema; aceita também `nome/NN` (opacidade) e `#hex`.
    pub fn color(&self, token: &str) -> Color {
        self.colors.get(token).copied().or_else(|| self.resolve_color(token)).unwrap_or(Color::TRANSPARENT)
    }

    pub fn color_of(&self, c: &ColorRef) -> Color {
        match c {
            ColorRef::Value(v) => *v,
            ColorRef::Token(t) => self.color(t),
        }
    }

    fn metric(map: &HashMap<String, f32>, m: &Metric) -> f32 {
        match m {
            Metric::Value(v) => *v,
            Metric::Token(t) => map.get(t).copied().unwrap_or(0.0),
        }
    }

    /// Resolve o estilo de um controle.
    ///
    /// `classes` é a cadeia de classes da mais derivada para a base (como devolvida por
    /// [`Component::class_chain`](crate::Component::class_chain)).
    /// Até onde a sombra do estilo passa do controle, no estado que mais passa (o mouse em
    /// cima troca a sombra pequena pela grande): a repintura por parte cobre isso.
    pub fn reach(&self, classes: &[&str], variant: &str) -> f32 {
        let mut reach: f32 = 0.0;
        let mut add = |s: &Style| {
            for props in [&s.base, &s.hover, &s.pressed, &s.focused, &s.checked, &s.disabled] {
                if let Some(shadow) = props.shadow.as_deref().and_then(|t| self.shadows.get(t)) {
                    reach = reach.max(shadow.reach());
                }
            }
        };
        for class in classes {
            if let Some(s) = self.styles.get(*class) {
                add(s);
            }
            if !variant.is_empty() {
                if let Some(s) = self.styles.get(&format!("{class}.{variant}")) {
                    add(s);
                }
            }
        }
        reach
    }

    pub fn resolve(&self, classes: &[&str], variant: &str, state: ControlState) -> ResolvedStyle {
        let mut layers: Vec<&Style> = Vec::new();
        for class in classes.iter().rev() {
            if let Some(s) = self.styles.get(*class) {
                layers.push(s);
            }
            if !variant.is_empty() {
                if let Some(s) = self.styles.get(&format!("{class}.{variant}")) {
                    layers.push(s);
                }
            }
        }

        let mut acc = StyleProps::default();
        for s in &layers {
            acc.merge(&s.base);
        }
        let active = [state.hover, state.pressed, state.focused, state.checked, state.disabled];
        // Marcado sem estilo próprio (um botão `checkable` num tema que não pinta o
        // `checked`): fica com cara de apertado, como o `StaysPressed` do FMX.
        let checked_as_pressed = state.checked && layers.iter().all(|s| s.checked == StyleProps::default());
        for (i, on) in active.into_iter().enumerate() {
            if on {
                for s in &layers {
                    acc.merge(s.state(if i == 3 && checked_as_pressed { 1 } else { i }));
                }
            }
        }

        let defaults = ResolvedStyle::default();
        ResolvedStyle {
            fill: acc.fill.as_ref().map_or(defaults.fill, |c| self.color_of(c)),
            text: acc
                .text
                .as_ref()
                .map(|c| self.color_of(c))
                .or_else(|| self.colors.get("text").copied())
                .unwrap_or(defaults.text),
            border_color: acc.border_color.as_ref().map_or(defaults.border_color, |c| self.color_of(c)),
            border_width: acc.border_width.unwrap_or(defaults.border_width),
            radius: acc.radius.as_ref().map_or(0.0, |m| Self::metric(&self.radius, m)),
            padding: acc.padding.unwrap_or(defaults.padding),
            font: {
                let mut font = self.fonts.get(acc.font.as_deref().unwrap_or("default")).cloned().unwrap_or(defaults.font);
                if let Some(w) = acc.font_weight {
                    font.weight = w;
                }
                if let Some(size) = acc.font_size {
                    font.size = size;
                }
                if let Some(italic) = acc.italic {
                    font.italic = italic;
                }
                font
            },
            line_height: acc.line_height.unwrap_or(0.0),
            opacity: acc.opacity.unwrap_or(1.0),
            transition_ms: acc.transition.as_ref().map_or(0.0, |m| Self::metric(&self.durations, m)),
            shadow: acc.shadow.as_deref().and_then(|t| self.shadows.get(t)).copied(),
            material: acc.material.as_deref().map_or(Material::None, Material::parse),
            text_case: match acc.text_case.as_deref() {
                Some("upper") => crate::text::TextCase::Upper,
                Some("lower") => crate::text::TextCase::Lower,
                _ => crate::text::TextCase::None,
            },
            letter_spacing: acc.letter_spacing.unwrap_or(0.0),
            gap: acc.gap,
            border_sides: acc.border_sides,
            border_dash: match acc.border_style.as_deref() {
                Some("dashed") => vec![6.0, 4.0],
                Some("dotted") => vec![1.5, 3.0],
                _ => Vec::new(),
            },
        }
    }
}
