//! Os temas no estilo dos frameworks visuais da web: Vuetify 3, Bootstrap 5, shadcn/ui
//! (Tailwind), Ant Design 5, Chakra UI e Quasar 2, em claro e escuro. As cores, os raios e as fontes
//! são os padrões de cada um (os tokens da documentação); os controles nativos do aylis
//! ficam com a cara do framework, como os temas de plataforma.

use super::{DISABLED, base, colors, fields, fonts, p, radius, toggles};
use crate::color::Color;
use crate::theme::{Look, Style, Theme};

/// As famílias da web, na ordem da IDE.
pub const FAMILIES: [&str; 6] = ["Vuetify 3", "Bootstrap 5", "shadcn/ui", "Ant Design 5", "Chakra UI", "Quasar 2"];

pub fn all() -> Vec<Theme> {
    vec![
        vuetify(false),
        vuetify(true),
        bootstrap(false),
        bootstrap(true),
        shadcn(false),
        shadcn(true),
        ant(false),
        ant(true),
        chakra(false),
        chakra(true),
        quasar(false),
        quasar(true),
    ]
}

fn name(family: &str, dark: bool) -> String {
    format!("{family} {}", if dark { "Dark" } else { "Light" })
}

/// O que os frameworks da web têm em comum: o botão cheio (primário), o secundário, o de
/// contorno, o de texto e o de perigo; os campos com borda e o anel de foco; as caixas;
/// os painéis e os cartões. `r` é o raio dos botões e dos campos; `card` o dos cartões.
fn common(t: &mut Theme, r: f32, card: f32, button_padding: (f32, f32)) {
    t.set_style(
        "Button",
        Style::new(p().fill("primary").text("on_primary").radius(r).padding(button_padding).font("button").border(Color::TRANSPARENT, 0.0).transition("fast"))
            .hover(p().fill("primary_hover"))
            .pressed(p().fill("primary_pressed"))
            .focused(p().border("focus_ring", 2.0))
            .disabled(p().opacity(0.65)),
    );
    t.set_style(
        "Button.secondary",
        Style::new(p().fill("secondary").text("on_secondary").border(Color::TRANSPARENT, 0.0)).hover(p().fill("secondary_hover")).pressed(p().fill("secondary_hover")),
    );
    t.set_style("Button.outline", Style::new(p().fill(Color::TRANSPARENT).text("text").border("border", 1.0)).hover(p().fill("hover_overlay")).pressed(p().fill("hover_overlay")));
    t.set_style("Button.text", Style::new(p().fill(Color::TRANSPARENT).text("primary").border(Color::TRANSPARENT, 0.0)).hover(p().fill("primary_soft")));
    t.set_style("Button.danger", Style::new(p().fill("danger").text("on_danger").border(Color::TRANSPARENT, 0.0)).hover(p().fill("danger_hover")).pressed(p().fill("danger_pressed")));
    t.set_style("Button.success", Style::new(p().fill("success").text("on_success").border(Color::TRANSPARENT, 0.0)));
    fields(
        t,
        Style::new(p().fill("surface").text("text").border("input", 1.0).radius(r).padding((12.0, 4.0)).transition("fast"))
            .hover(p().border("input_hover", 1.0))
            .focused(p().border("focus_ring", 2.0))
            .disabled(p().opacity(DISABLED)),
    );
    toggles(
        t,
        Style::new(p().fill("surface").text("text").border("input", 1.0).radius(r.min(4.0)).transition("fast"))
            .hover(p().border("input_hover", 1.0))
            .checked(p().fill("primary").border("primary", 1.0))
            .focused(p().border("focus_ring", 2.0))
            .disabled(p().opacity(DISABLED)),
    );
    t.set_style(
        "Switch",
        Style::new(p().fill("track").text("text").border(Color::TRANSPARENT, 0.0).transition("normal")).checked(p().fill("primary")).disabled(p().opacity(DISABLED)),
    );
    t.set_style("Panel", Style::new(p().fill("surface").border("border", 1.0).radius(card)));
    t.set_style("Panel.card", Style::new(p().fill("surface").border("border", 1.0).radius(card).shadow("sm")));
    t.set_style("Panel.outlined", Style::new(p().fill("surface").border("border", 1.0).radius(card).shadow("none")));
    t.set_style("ProgressBar", Style::new(p().fill("track").radius(r.min(4.0))));
}

// ----- Vuetify 3 ------------------------------------------------------------

/// Vuetify 3: o Material Design da biblioteca (o tema padrão "light"/"dark"), botões
/// elevados de cantos 4, campos "filled" com a linha embaixo.
fn vuetify(dark: bool) -> Theme {
    let mut t = base(&name("Vuetify 3", dark), "Vuetify 3", dark, Look::Material2);
    if dark {
        colors(&mut t, &[
            ("background", "#121212"), ("surface", "#212121"), ("surface_2", "#2C2C2C"), ("text", "#FFFFFF"), ("text_muted", "#B3B3B3"),
            ("border", "#3C3C3C"), ("input", "#5C5C5C"), ("input_hover", "#9E9E9E"), ("track", "#5C5C5C"),
            ("primary", "#2196F3"), ("primary_hover", "#42A5F5"), ("primary_pressed", "#1E88E5"), ("on_primary", "#FFFFFF"),
            ("secondary", "#54B6B2"), ("secondary_hover", "#62C2BE"), ("on_secondary", "#000000"),
            ("danger", "#CF6679"), ("danger_hover", "#D77A8B"), ("danger_pressed", "#C25368"), ("success", "#4CAF50"), ("warning", "#FB8C00"), ("info", "#2196F3"),
            ("focus_ring", "#2196F3"), ("selection", "#2196F355"), ("primary_soft", "#2196F329"), ("hover_overlay", "#FFFFFF14"),
        ]);
    } else {
        colors(&mut t, &[
            ("background", "#FFFFFF"), ("surface", "#FFFFFF"), ("surface_2", "#F5F5F5"), ("text", "#212121"), ("text_muted", "#666666"),
            ("border", "#E0E0E0"), ("input", "#9E9E9E"), ("input_hover", "#212121"), ("track", "#BDBDBD"),
            ("primary", "#1867C0"), ("primary_hover", "#2B76CA"), ("primary_pressed", "#155AA8"), ("on_primary", "#FFFFFF"),
            ("secondary", "#48A9A6"), ("secondary_hover", "#56B5B2"), ("on_secondary", "#FFFFFF"),
            ("danger", "#B00020"), ("danger_hover", "#C2183A"), ("danger_pressed", "#97001B"), ("success", "#4CAF50"), ("warning", "#FB8C00"), ("info", "#2196F3"),
            ("focus_ring", "#1867C0"), ("selection", "#1867C040"), ("primary_soft", "#1867C01A"), ("hover_overlay", "#0000000A"),
        ]);
    }
    radius(&mut t, 4.0, 4.0, 4.0);
    fonts(&mut t, "Roboto", 14.0, 22.0);
    common(&mut t, 4.0, 4.0, (16.0, 8.0));
    // O v-btn padrão é elevado; o v-text-field padrão é "filled" (fundo e a linha de baixo).
    t.set_style(
        "Button",
        Style::new(p().fill("primary").text("on_primary").radius(4.0).padding((16.0, 8.0)).font("button").border(Color::TRANSPARENT, 0.0).shadow("sm").transition("fast"))
            .hover(p().fill("primary_hover").shadow("md"))
            .pressed(p().fill("primary_pressed"))
            .focused(p().border("focus_ring", 2.0))
            .disabled(p().opacity(DISABLED)),
    );
    fields(
        &mut t,
        Style::new(p().fill("surface_2").text("text").border(Color::TRANSPARENT, 0.0).radius(4.0).padding((16.0, 4.0)))
            .hover(p().fill("surface_2"))
            .focused(p().border("primary", 2.0))
            .disabled(p().opacity(DISABLED)),
    );
    t.set_style("Edit.outlined", Style::new(p().fill(Color::TRANSPARENT).border("input", 1.0)).focused(p().border("primary", 2.0)));
    t.set_style("Panel.card", Style::new(p().fill("surface").border(Color::TRANSPARENT, 0.0).radius(4.0).shadow("md")));
    t
}

// ----- Bootstrap 5 ----------------------------------------------------------

/// Bootstrap 5.3: o azul `#0d6efd`, cantos de 6 (0,375rem), o anel de foco claro.
fn bootstrap(dark: bool) -> Theme {
    let mut t = base(&name("Bootstrap 5", dark), "Bootstrap 5", dark, Look::Aylis);
    if dark {
        colors(&mut t, &[
            ("background", "#212529"), ("surface", "#212529"), ("surface_2", "#2B3035"), ("text", "#DEE2E6"), ("text_muted", "#ADB5BD"),
            ("border", "#495057"), ("input", "#495057"), ("input_hover", "#6C757D"), ("track", "#343A40"),
            ("primary", "#0D6EFD"), ("primary_hover", "#0B5ED7"), ("primary_pressed", "#0A58CA"), ("on_primary", "#FFFFFF"),
            ("secondary", "#6C757D"), ("secondary_hover", "#5C636A"), ("on_secondary", "#FFFFFF"),
            ("danger", "#DC3545"), ("danger_hover", "#BB2D3B"), ("danger_pressed", "#B02A37"), ("success", "#198754"), ("warning", "#FFC107"), ("info", "#0DCAF0"),
            ("focus_ring", "#3D8BFD"), ("selection", "#0D6EFD55"), ("primary_soft", "#0D6EFD29"), ("hover_overlay", "#FFFFFF14"),
        ]);
    } else {
        colors(&mut t, &[
            ("background", "#FFFFFF"), ("surface", "#FFFFFF"), ("surface_2", "#F8F9FA"), ("text", "#212529"), ("text_muted", "#6C757D"),
            ("border", "#DEE2E6"), ("input", "#DEE2E6"), ("input_hover", "#ADB5BD"), ("track", "#E9ECEF"),
            ("primary", "#0D6EFD"), ("primary_hover", "#0B5ED7"), ("primary_pressed", "#0A58CA"), ("on_primary", "#FFFFFF"),
            ("secondary", "#6C757D"), ("secondary_hover", "#5C636A"), ("on_secondary", "#FFFFFF"),
            ("danger", "#DC3545"), ("danger_hover", "#BB2D3B"), ("danger_pressed", "#B02A37"), ("success", "#198754"), ("warning", "#FFC107"), ("info", "#0DCAF0"),
            ("focus_ring", "#86B7FE"), ("selection", "#0D6EFD40"), ("primary_soft", "#0D6EFD1A"), ("hover_overlay", "#0000000D"),
        ]);
    }
    radius(&mut t, 4.0, 6.0, 8.0);
    fonts(&mut t, "Segoe UI", 15.0, 24.0);
    common(&mut t, 6.0, 6.0, (12.0, 6.0));
    // O .btn-outline-primary e o .form-switch (a bolinha num trilho com borda).
    t.set_style("Button.outline", Style::new(p().fill(Color::TRANSPARENT).text("primary").border("primary", 1.0)).hover(p().fill("primary").text("on_primary")));
    t.set_style(
        "Switch",
        Style::new(p().fill("surface").text("text").border("input", 1.0).transition("normal")).checked(p().fill("primary").border("primary", 1.0)).disabled(p().opacity(0.5)),
    );
    t
}

// ----- shadcn/ui ------------------------------------------------------------

/// shadcn/ui (Tailwind, a paleta zinc): botão quase preto, bordas finas cinza, cantos de 6,
/// a fonte Inter (ou a do sistema).
fn shadcn(dark: bool) -> Theme {
    let mut t = base(&name("shadcn/ui", dark), "shadcn/ui", dark, Look::Aylis);
    if dark {
        colors(&mut t, &[
            ("background", "#09090B"), ("surface", "#09090B"), ("surface_2", "#27272A"), ("text", "#FAFAFA"), ("text_muted", "#A1A1AA"),
            ("border", "#27272A"), ("input", "#27272A"), ("input_hover", "#3F3F46"), ("track", "#27272A"),
            ("primary", "#FAFAFA"), ("primary_hover", "#E4E4E7"), ("primary_pressed", "#D4D4D8"), ("on_primary", "#18181B"),
            ("secondary", "#27272A"), ("secondary_hover", "#3F3F46"), ("on_secondary", "#FAFAFA"),
            ("danger", "#7F1D1D"), ("danger_hover", "#991B1B"), ("danger_pressed", "#681717"), ("on_danger", "#FAFAFA"), ("success", "#22C55E"), ("warning", "#EAB308"), ("info", "#3B82F6"),
            ("focus_ring", "#D4D4D8"), ("selection", "#FAFAFA33"), ("primary_soft", "#FAFAFA14"), ("hover_overlay", "#FFFFFF0F"),
        ]);
    } else {
        colors(&mut t, &[
            ("background", "#FFFFFF"), ("surface", "#FFFFFF"), ("surface_2", "#F4F4F5"), ("text", "#09090B"), ("text_muted", "#71717A"),
            ("border", "#E4E4E7"), ("input", "#E4E4E7"), ("input_hover", "#D4D4D8"), ("track", "#E4E4E7"),
            ("primary", "#18181B"), ("primary_hover", "#27272A"), ("primary_pressed", "#3F3F46"), ("on_primary", "#FAFAFA"),
            ("secondary", "#F4F4F5"), ("secondary_hover", "#E4E4E7"), ("on_secondary", "#18181B"),
            ("danger", "#EF4444"), ("danger_hover", "#DC2626"), ("danger_pressed", "#B91C1C"), ("on_danger", "#FAFAFA"), ("success", "#16A34A"), ("warning", "#CA8A04"), ("info", "#2563EB"),
            ("focus_ring", "#A1A1AA"), ("selection", "#18181B26"), ("primary_soft", "#18181B0F"), ("hover_overlay", "#0000000A"),
        ]);
    }
    radius(&mut t, 4.0, 6.0, 8.0);
    fonts(&mut t, "Inter", 14.0, 24.0);
    common(&mut t, 6.0, 8.0, (16.0, 8.0));
    t.set_style("Button.text", Style::new(p().fill(Color::TRANSPARENT).text("text").border(Color::TRANSPARENT, 0.0)).hover(p().fill("secondary")));
    t.set_style("Button.ghost", Style::new(p().fill(Color::TRANSPARENT).text("text").border(Color::TRANSPARENT, 0.0)).hover(p().fill("secondary")));
    t.set_style("Panel.card", Style::new(p().fill("surface").border("border", 1.0).radius(8.0).shadow("sm")));
    t
}

// ----- Ant Design 5 ---------------------------------------------------------

/// Ant Design 5: o azul `#1677ff`, cantos de 6, texto de 14, botão padrão branco com borda.
fn ant(dark: bool) -> Theme {
    let mut t = base(&name("Ant Design 5", dark), "Ant Design 5", dark, Look::Aylis);
    if dark {
        colors(&mut t, &[
            ("background", "#000000"), ("surface", "#141414"), ("surface_2", "#1F1F1F"), ("text", "#DBDBDB"), ("text_muted", "#737373"),
            ("border", "#303030"), ("input", "#424242"), ("input_hover", "#3C89E8"), ("track", "#424242"),
            ("primary", "#1668DC"), ("primary_hover", "#3C89E8"), ("primary_pressed", "#1554AD"), ("on_primary", "#FFFFFF"),
            ("secondary", "#141414"), ("secondary_hover", "#1F1F1F"), ("on_secondary", "#DBDBDB"),
            ("danger", "#DC4446"), ("danger_hover", "#E86E6B"), ("danger_pressed", "#AD393A"), ("success", "#49AA19"), ("warning", "#D89614"), ("info", "#1668DC"),
            ("focus_ring", "#1668DC"), ("selection", "#1668DC55"), ("primary_soft", "#1668DC29"), ("hover_overlay", "#FFFFFF14"),
        ]);
    } else {
        colors(&mut t, &[
            ("background", "#F5F5F5"), ("surface", "#FFFFFF"), ("surface_2", "#FAFAFA"), ("text", "#1F1F1F"), ("text_muted", "#8C8C8C"),
            ("border", "#F0F0F0"), ("input", "#D9D9D9"), ("input_hover", "#4096FF"), ("track", "#F0F0F0"),
            ("primary", "#1677FF"), ("primary_hover", "#4096FF"), ("primary_pressed", "#0958D9"), ("on_primary", "#FFFFFF"),
            ("secondary", "#FFFFFF"), ("secondary_hover", "#FFFFFF"), ("on_secondary", "#1F1F1F"),
            ("danger", "#FF4D4F"), ("danger_hover", "#FF7875"), ("danger_pressed", "#D9363E"), ("success", "#52C41A"), ("warning", "#FAAD14"), ("info", "#1677FF"),
            ("focus_ring", "#4096FF"), ("selection", "#1677FF33"), ("primary_soft", "#1677FF1A"), ("hover_overlay", "#0000000A"),
        ]);
    }
    radius(&mut t, 4.0, 6.0, 8.0);
    fonts(&mut t, "Segoe UI", 14.0, 24.0);
    common(&mut t, 6.0, 8.0, (15.0, 4.0));
    // O botão "default" do Ant (o secundário): branco com borda, que fica azul no hover.
    t.set_style(
        "Button.secondary",
        Style::new(p().fill("secondary").text("on_secondary").border("input", 1.0)).hover(p().text("primary_hover").border("primary_hover", 1.0)).pressed(p().text("primary_pressed").border("primary_pressed", 1.0)),
    );
    t.set_style("Button.outline", Style::new(p().fill(Color::TRANSPARENT).text("primary").border("primary", 1.0)).hover(p().text("primary_hover").border("primary_hover", 1.0)));
    t.set_style("Panel.card", Style::new(p().fill("surface").border("border", 1.0).radius(8.0).shadow("none")));
    t
}

// ----- Chakra UI ------------------------------------------------------------

/// Chakra UI (v2): o azul `blue.500`, o botão cinza de fundo `gray.100`, cantos de 6 e
/// texto `gray.800`.
fn chakra(dark: bool) -> Theme {
    let mut t = base(&name("Chakra UI", dark), "Chakra UI", dark, Look::Aylis);
    if dark {
        colors(&mut t, &[
            ("background", "#1A202C"), ("surface", "#1A202C"), ("surface_2", "#2D3748"), ("text", "#EBEBEB"), ("text_muted", "#A0AEC0"),
            ("border", "#4A5568"), ("input", "#4A5568"), ("input_hover", "#718096"), ("track", "#4A5568"),
            ("primary", "#90CDF4"), ("primary_hover", "#63B3ED"), ("primary_pressed", "#4299E1"), ("on_primary", "#1A202C"),
            ("secondary", "#FFFFFF29"), ("secondary_hover", "#FFFFFF3D"), ("on_secondary", "#EBEBEB"),
            ("danger", "#FEB2B2"), ("danger_hover", "#FC8181"), ("danger_pressed", "#F56565"), ("on_danger", "#1A202C"), ("success", "#9AE6B4"), ("on_success", "#1A202C"), ("warning", "#FBD38D"), ("info", "#90CDF4"),
            ("focus_ring", "#63B3ED"), ("selection", "#90CDF455"), ("primary_soft", "#90CDF429"), ("hover_overlay", "#FFFFFF14"),
        ]);
    } else {
        colors(&mut t, &[
            ("background", "#FFFFFF"), ("surface", "#FFFFFF"), ("surface_2", "#F7FAFC"), ("text", "#1A202C"), ("text_muted", "#718096"),
            ("border", "#E2E8F0"), ("input", "#E2E8F0"), ("input_hover", "#CBD5E0"), ("track", "#CBD5E0"),
            ("primary", "#3182CE"), ("primary_hover", "#2B6CB0"), ("primary_pressed", "#2C5282"), ("on_primary", "#FFFFFF"),
            ("secondary", "#EDF2F7"), ("secondary_hover", "#E2E8F0"), ("on_secondary", "#1A202C"),
            ("danger", "#E53E3E"), ("danger_hover", "#C53030"), ("danger_pressed", "#9B2C2C"), ("success", "#38A169"), ("warning", "#DD6B20"), ("info", "#3182CE"),
            ("focus_ring", "#63B3ED"), ("selection", "#3182CE40"), ("primary_soft", "#3182CE1A"), ("hover_overlay", "#0000000A"),
        ]);
    }
    radius(&mut t, 4.0, 6.0, 8.0);
    fonts(&mut t, "Segoe UI", 15.0, 24.0);
    common(&mut t, 6.0, 8.0, (16.0, 8.0));
    t.set_style("Panel.card", Style::new(p().fill("surface").border(Color::TRANSPARENT, 0.0).radius(8.0).shadow("md")));
    t
}

// ----- Quasar 2 -------------------------------------------------------------

/// Quasar 2: as cores da marca (`primary` #1976D2, `secondary` #26A69A, `accent` #9C27B0,
/// `positive`, `negative`, `info`, `warning`), o Roboto, o QBtn elevado de cantos 4 em
/// maiúsculas, e os estilos do QBtn: `flat`, `outline`, `push` (com a borda de baixo
/// funda), `unelevated`, `rounded` e `glossy`. O QInput padrão é o "standard" (só a linha
/// de baixo); `Edit.outlined` e `Edit.filled` são os outros dois.
fn quasar(dark: bool) -> Theme {
    let mut t = base(&name("Quasar 2", dark), "Quasar 2", dark, Look::Material2);
    let brand = [
        ("primary", "#1976D2"), ("primary_hover", "#1E88E5"), ("primary_pressed", "#1565C0"), ("on_primary", "#FFFFFF"),
        ("secondary", "#26A69A"), ("secondary_hover", "#2BBBAD"), ("on_secondary", "#FFFFFF"),
        ("accent", "#9C27B0"), ("accent_hover", "#AB47BC"), ("on_accent", "#FFFFFF"),
        ("positive", "#21BA45"), ("negative", "#C10015"), ("info", "#31CCEC"), ("warning", "#F2C037"),
        ("success", "#21BA45"), ("on_success", "#FFFFFF"), ("danger", "#C10015"), ("danger_hover", "#D32F2F"), ("danger_pressed", "#9A0011"), ("on_danger", "#FFFFFF"),
        ("on_info", "#000000"), ("on_warning", "#000000"), ("focus_ring", "#1976D2"), ("rating", "#F2C037"),
    ];
    colors(&mut t, &brand);
    if dark {
        colors(&mut t, &[
            ("background", "#121212"), ("surface", "#1D1D1D"), ("surface_2", "#2A2A2A"), ("text", "#FFFFFF"), ("text_muted", "#FFFFFFB3"),
            ("border", "#FFFFFF33"), ("input", "#FFFFFF8A"), ("input_hover", "#FFFFFF"), ("track", "#FFFFFF4D"),
            ("selection", "#1976D255"), ("primary_soft", "#1976D229"), ("hover_overlay", "#FFFFFF1A"), ("push_edge", "#00000066"),
        ]);
    } else {
        colors(&mut t, &[
            ("background", "#FFFFFF"), ("surface", "#FFFFFF"), ("surface_2", "#F5F5F5"), ("text", "#000000DE"), ("text_muted", "#0000008A"),
            ("border", "#0000001F"), ("input", "#0000006B"), ("input_hover", "#000000"), ("track", "#0000004D"),
            ("selection", "#1976D240"), ("primary_soft", "#1976D21A"), ("hover_overlay", "#00000014"), ("push_edge", "#00000033"),
        ]);
    }
    radius(&mut t, 4.0, 4.0, 4.0);
    fonts(&mut t, "Roboto", 14.0, 20.0);
    common(&mut t, 4.0, 4.0, (16.0, 6.0));
    // O QBtn padrão: cheio e elevado (shadow-2), mais fundo ao apertar.
    t.set_style(
        "Button",
        Style::new(p().fill("primary").text("on_primary").radius(4.0).padding((16.0, 6.0)).font("button").border(Color::TRANSPARENT, 0.0).shadow("sm").transition("fast"))
            .hover(p().fill("primary_hover").shadow("md"))
            .pressed(p().fill("primary_pressed").shadow("lg"))
            .focused(p().border("focus_ring", 2.0))
            .disabled(p().opacity(DISABLED)),
    );
    let plain = |fill: &str, text: &str| p().fill(fill).text(text).border(Color::TRANSPARENT, 0.0).shadow("none");
    t.set_style("Button.flat", Style::new(p().fill(Color::TRANSPARENT).text("primary").border(Color::TRANSPARENT, 0.0).shadow("none")).hover(p().fill("primary_soft")).pressed(p().fill("primary_soft")));
    t.set_style("Button.outline", Style::new(p().fill(Color::TRANSPARENT).text("primary").border("primary", 1.0).shadow("none")).hover(p().fill("primary_soft")));
    t.set_style("Button.unelevated", Style::new(plain("primary", "on_primary")).hover(p().fill("primary_hover").shadow("none")).pressed(p().fill("primary_pressed").shadow("none")));
    t.set_style("Button.rounded", Style::new(p().radius(28.0)));
    t.set_style("Button.round", Style::new(p().radius(999.0).padding((8.0, 8.0))));
    // O push: a borda de baixo funda, que some ao apertar (o botão "afunda").
    t.set_style(
        "Button.push",
        Style::new(p().fill("primary").text("on_primary").border("push_edge", 3.0).shadow("none")).pressed(p().fill("primary_pressed").border("push_edge", 1.0)),
    );
    // O glossy: o brilho por cima (aqui, um tom mais claro com sombra suave).
    t.set_style("Button.glossy", Style::new(p().fill("primary_hover").text("on_primary").shadow("md")).hover(p().fill("primary")).pressed(p().fill("primary_pressed")));
    t.set_style("Button.accent", Style::new(plain("accent", "on_accent").shadow("sm")).hover(p().fill("accent_hover")));
    t.set_style("Button.positive", Style::new(plain("positive", "on_success").shadow("sm")));
    t.set_style("Button.negative", Style::new(plain("negative", "on_danger").shadow("sm")).hover(p().fill("danger_hover")));
    t.set_style("Button.info", Style::new(plain("info", "on_info").shadow("sm")));
    t.set_style("Button.warning", Style::new(plain("warning", "on_warning").shadow("sm")));
    // O QInput "standard": sem caixa, só a linha de baixo (que fica da cor primária no foco).
    fields(
        &mut t,
        Style::new(p().fill(Color::TRANSPARENT).text("text").border(Color::TRANSPARENT, 0.0).radius(0.0).padding((0.0, 4.0)).transition("fast"))
            .hover(p().fill(Color::TRANSPARENT))
            .focused(p().border(Color::TRANSPARENT, 0.0))
            .disabled(p().opacity(DISABLED)),
    );
    t.set_style("Edit.outlined", Style::new(p().fill(Color::TRANSPARENT).border("input", 1.0).radius(4.0).padding((12.0, 4.0))).focused(p().border("primary", 2.0)));
    t.set_style("Edit.filled", Style::new(p().fill("surface_2").border(Color::TRANSPARENT, 0.0).radius(4.0).padding((12.0, 4.0))).focused(p().border("primary", 2.0)));
    // O QCard: branco com a sombra padrão (shadow-1) e cantos 4.
    t.set_style("Panel.card", Style::new(p().fill("surface").border(Color::TRANSPARENT, 0.0).radius(4.0).shadow("sm")));
    t.set_style("Card", Style::new(p().fill("surface").text("text").border(Color::TRANSPARENT, 0.0).radius(4.0).shadow("sm")));
    t.set_style("Card.flat", Style::new(p().shadow("none")));
    t.set_style("Card.bordered", Style::new(p().border("border", 1.0).shadow("none")));
    t
}
