//! Temas de plataforma que o aylis traz: o visual do Android (7, Material 2 e 3), do iOS
//! (7–12, 13–18 e 26), do Windows (7, 10, 11 e WinUI 3) e do Windows Phone, em claro e
//! escuro (quando a plataforma tem), e os da web ([`web`]: Vuetify 3, Bootstrap 5,
//! shadcn/ui, Ant Design 5 e Chakra UI).
//!
//! Cada tema parte do tema do aylis (todos os estilos e subtipos continuam existindo) e
//! troca as cores, raios, fontes e os estilos que mudam na plataforma; o [`Look`] faz os
//! controles mudarem de forma (o Switch do iOS, o CheckBox redondo...). São montados uma
//! vez, no primeiro uso, e ficam disponíveis pelo nome em qualquer `Ui`
//! ([`Ui::theme_named`](crate::Ui::theme_named)): no projeto inteiro, numa classe
//! ([`Theme::mix`]) ou num controle só (propriedade `theme`).
//!
//! As fontes de cada plataforma que não podem ser embutidas (SF Pro, Segoe UI) não vão no
//! app: no Windows a Segoe UI já está instalada; nos outros sistemas vale a fonte padrão.

use std::sync::OnceLock;

use crate::canvas::{FontSpec, FontWeight};
use crate::color::Color;
use crate::theme::{Look, Style, StyleProps, Theme};

pub mod web;

/// As famílias, na ordem da IDE.
pub const FAMILIES: [&str; 19] = [
    "Aylis",
    "Material 3",
    "Material 2",
    "Android 7",
    "iOS 26",
    "iOS 13",
    "iOS 7",
    "WinUI 3",
    "Windows 11",
    "Windows 10",
    "Win32",
    "Windows 7",
    "Windows Phone",
    "Vuetify 3",
    "Bootstrap 5",
    "shadcn/ui",
    "Ant Design 5",
    "Chakra UI",
    "Quasar 2",
];

/// Todos os temas de plataforma (sem os do aylis, que o `Ui` já tem).
pub fn all() -> &'static [Theme] {
    static THEMES: OnceLock<Vec<Theme>> = OnceLock::new();
    THEMES.get_or_init(|| {
        let mut list = vec![
            material3(false),
            material3(true),
            material2(false),
            material2(true),
            android7(false),
            android7(true),
            ios26(false),
            ios26(true),
            ios13(false),
            ios13(true),
            ios7(),
            winui3(false),
            winui3(true),
            windows11(false),
            windows11(true),
            windows10(false),
            windows10(true),
            win32(),
            windows7(),
            windows_phone(true),
            windows_phone(false),
        ];
        list.extend(web::all());
        list
    })
}

/// Um tema de plataforma pelo nome ("Material 3 Light", "Windows 11 Dark"...). Os do aylis
/// ("Aylis Light", "Aylis Dark") também respondem aqui.
pub fn builtin(name: &str) -> Option<&'static Theme> {
    static AYLIS: OnceLock<[Theme; 2]> = OnceLock::new();
    let aylis = AYLIS.get_or_init(|| [Theme::light(), Theme::dark()]);
    aylis.iter().chain(all().iter()).find(|t| t.name.eq_ignore_ascii_case(name))
}

/// O tema de uma família na versão clara ou escura (a que existir).
pub fn of_family(family: &str, dark: bool) -> Option<&'static Theme> {
    let mut found = None;
    let aylis = ["Aylis Light", "Aylis Dark"];
    for t in aylis.iter().filter_map(|n| builtin(n)).chain(all().iter()) {
        if t.family_name().eq_ignore_ascii_case(family) {
            if t.dark == dark {
                return Some(t);
            }
            found = Some(t);
        }
    }
    found
}

// ---------------------------------------------------------------------------

fn base(name: &str, family: &str, dark: bool, look: Look) -> Theme {
    let mut t = if dark { Theme::dark() } else { Theme::light() };
    t.name = name.to_string();
    t.family = family.to_string();
    t.dark = dark;
    t.look = look;
    t
}

fn colors(t: &mut Theme, list: &[(&str, &str)]) {
    for (token, hex) in list {
        t.colors.insert(token.to_string(), Color::from_hex(hex).expect("cor inválida num tema de plataforma"));
    }
}

fn radius(t: &mut Theme, sm: f32, md: f32, lg: f32) {
    for (k, v) in [("sm", sm), ("md", md), ("lg", lg)] {
        t.radius.insert(k.into(), v);
    }
}

fn fonts(t: &mut Theme, family: &str, size: f32, title: f32) {
    t.fonts.insert("default".into(), FontSpec::new(family, size));
    t.fonts.insert("title".into(), FontSpec::new(family, title).weight(FontWeight::Bold));
    t.fonts.insert("heading".into(), FontSpec::new(family, size + 2.0).weight(FontWeight::SemiBold));
    t.fonts.insert("small".into(), FontSpec::new(family, (size - 2.0).max(9.0)));
    t.fonts.insert("strong".into(), FontSpec::new(family, size).weight(FontWeight::SemiBold));
    t.fonts.insert("button".into(), FontSpec::new(family, size).weight(FontWeight::Medium));
}

fn p() -> StyleProps {
    StyleProps::new()
}

/// Estilo de campo (Edit, ComboBox, Memo, HotkeyEdit...): o mesmo para todos.
fn fields(t: &mut Theme, style: Style) {
    for class in ["Edit", "ComboBox", "Memo", "HotkeyEdit", "ListBox", "ListView", "TreeView", "Calendar"] {
        t.set_style(class, style.clone());
    }
}

fn toggles(t: &mut Theme, style: Style) {
    for class in ["CheckBox", "RadioButton"] {
        t.set_style(class, style.clone());
    }
}

const DISABLED: f32 = 0.38;

// ----- Material 3 -----------------------------------------------------------

fn material3(dark: bool) -> Theme {
    let mut t = base(if dark { "Material 3 Dark" } else { "Material 3 Light" }, "Material 3", dark, Look::Material3);
    if dark {
        colors(&mut t, &[
            ("background", "#141218"), ("surface", "#1D1B20"), ("surface_2", "#2B2930"), ("text", "#E6E0E9"),
            ("text_muted", "#CAC4D0"), ("border", "#938F99"), ("track", "#36343B"), ("primary", "#D0BCFF"),
            ("primary_hover", "#DCCBFF"), ("primary_pressed", "#C4AEFF"), ("on_primary", "#381E72"), ("on_danger", "#601410"),
            ("secondary_container", "#4A4458"), ("on_secondary_container", "#E8DEF8"), ("focus_ring", "#D0BCFF"),
            ("selection", "#D0BCFF55"), ("primary_soft", "#D0BCFF29"), ("hover_overlay", "#E6E0E914"),
            ("danger", "#F2B8B5"), ("danger_hover", "#F5C6C4"), ("danger_pressed", "#EFA9A5"), ("accent", "#EFB8C8"),
            ("primary_container", "#4F378B"), ("on_primary_container", "#EADDFF"), ("surface_container", "#211F26"),
            ("surface_container_high", "#2B2930"), ("surface_container_highest", "#36343B"), ("outline_variant", "#49454F"),
        ]);
    } else {
        colors(&mut t, &[
            ("background", "#FEF7FF"), ("surface", "#F7F2FA"), ("surface_2", "#ECE6F0"), ("text", "#1D1B20"),
            ("text_muted", "#49454F"), ("border", "#79747E"), ("track", "#E6E0E9"), ("primary", "#6750A4"),
            ("primary_hover", "#7461B0"), ("primary_pressed", "#5A4596"), ("on_primary", "#FFFFFF"),
            ("secondary_container", "#E8DEF8"), ("on_secondary_container", "#1D192B"), ("focus_ring", "#6750A4"),
            ("selection", "#6750A433"), ("primary_soft", "#6750A41F"), ("hover_overlay", "#1D1B2014"),
            ("danger", "#B3261E"), ("danger_hover", "#C0392F"), ("danger_pressed", "#9C1F18"), ("accent", "#7D5260"),
            ("primary_container", "#EADDFF"), ("on_primary_container", "#21005D"), ("surface_container", "#F3EDF7"),
            ("surface_container_high", "#ECE6F0"), ("surface_container_highest", "#E6E0E9"), ("outline_variant", "#CAC4D0"),
        ]);
    }
    radius(&mut t, 4.0, 20.0, 12.0);
    fonts(&mut t, "Roboto", 14.0, 22.0);
    t.set_style(
        "Button",
        Style::new(p().fill("primary").text("on_primary").radius(20.0).padding((24.0, 8.0)).font("button").border(Color::TRANSPARENT, 0.0).transition("fast"))
            .hover(p().fill("primary_hover").shadow("sm"))
            .pressed(p().fill("primary_pressed"))
            .focused(p().border("focus_ring", 2.0))
            .disabled(p().opacity(DISABLED)),
    );
    t.set_style(
        "Button.secondary",
        Style::new(p().fill("secondary_container").text("on_secondary_container").border(Color::TRANSPARENT, 0.0))
            .hover(p().fill("secondary_container").shadow("sm"))
            .pressed(p().fill("secondary_container")),
    );
    t.set_style("Button.accent", Style::new(p()));
    t.set_style(
        "Button.outline",
        Style::new(p().fill(Color::TRANSPARENT).text("primary").border("border", 1.0)).hover(p().fill("primary_soft")).pressed(p().fill("primary_soft")),
    );
    fields(
        &mut t,
        Style::new(p().fill(Color::TRANSPARENT).text("text").border("border", 1.0).radius(4.0).padding((14.0, 4.0)))
            .hover(p().border("text", 1.0))
            .focused(p().border("primary", 2.0))
            .disabled(p().opacity(DISABLED)),
    );
    toggles(
        &mut t,
        Style::new(p().fill(Color::TRANSPARENT).text("text").border("text_muted", 2.0).radius(2.0).transition("fast"))
            .hover(p().border("text", 2.0))
            .checked(p().fill("primary").border("primary", 2.0))
            .focused(p().border("primary", 2.0))
            .disabled(p().opacity(DISABLED)),
    );
    t.set_style(
        "Switch",
        Style::new(p().fill("track").text("text").border("border", 2.0).transition("normal"))
            .checked(p().fill("primary").border("primary", 2.0))
            .disabled(p().opacity(DISABLED)),
    );
    t.set_style("Panel", Style::new(p().fill("surface").border(Color::TRANSPARENT, 0.0).radius(12.0)));
    // Cartões: elevado (padrão), preenchido e com contorno.
    t.set_style("Panel.card", Style::new(p().fill("surface_container").border(Color::TRANSPARENT, 0.0).radius(12.0).shadow("sm")));
    t.set_style("Panel.filled", Style::new(p().fill("surface_container_highest").border(Color::TRANSPARENT, 0.0).radius(12.0).shadow("none")));
    t.set_style("Panel.outlined", Style::new(p().fill("background").border("outline_variant", 1.0).radius(12.0).shadow("none")));
    t.set_style("ProgressBar", Style::new(p().fill("track").radius(2.0)));
    // Botões: elevado, tonal, texto; FAB de cantos 16 (não redondo) e o estendido.
    t.set_style(
        "Button.elevated",
        Style::new(p().fill("surface_container").text("primary").border(Color::TRANSPARENT, 0.0).radius(20.0).shadow("sm")).hover(p().shadow("md")),
    );
    t.set_style("Button.tonal", Style::new(p().fill("secondary_container").text("on_secondary_container").border(Color::TRANSPARENT, 0.0)).hover(p().shadow("sm")));
    t.set_style("Button.text", Style::new(p().fill(Color::TRANSPARENT).text("primary").border(Color::TRANSPARENT, 0.0).padding((12.0, 8.0))).hover(p().fill("primary_soft")));
    t.set_style(
        "Button.fab",
        Style::new(p().fill("primary_container").text("on_primary_container").border(Color::TRANSPARENT, 0.0).radius(16.0).shadow("md").padding(16.0)).hover(p().shadow("lg")),
    );
    t.set_style(
        "Button.fab_extended",
        Style::new(p().fill("primary_container").text("on_primary_container").border(Color::TRANSPARENT, 0.0).radius(16.0).shadow("md").padding((16.0, 0.0))),
    );
    t.set_style("Button.icon_tonal", Style::new(p().fill("secondary_container").text("on_secondary_container").border(Color::TRANSPARENT, 0.0).radius(999.0)));
    t.set_style("Button.icon_outlined", Style::new(p().fill(Color::TRANSPARENT).text("text_muted").border("border", 1.0).radius(999.0)));
    // Campos: com contorno (padrão) e preenchido (com a linha embaixo).
    t.set_style(
        "Edit.filled",
        Style::new(p().fill("surface_container_highest").text("text").border(Color::TRANSPARENT, 0.0).radius(4.0).padding((16.0, 4.0))),
    );
    t.set_style(
        "Edit.search",
        Style::new(p().fill("surface_container_high").text("text").border(Color::TRANSPARENT, 0.0).radius(999.0).padding((16.0, 4.0)))
            .focused(p().fill("surface_container_high").border(Color::TRANSPARENT, 0.0)),
    );
    t.set_style("ToolBar.floating", Style::new(p().fill("surface_container").border(Color::TRANSPARENT, 0.0).radius(999.0).shadow("md").padding(8.0)));
    t
}

// ----- Material 2 -----------------------------------------------------------

fn material2(dark: bool) -> Theme {
    let mut t = base(if dark { "Material 2 Dark" } else { "Material 2 Light" }, "Material 2", dark, Look::Material2);
    if dark {
        colors(&mut t, &[
            ("background", "#121212"), ("surface", "#1E1E1E"), ("surface_2", "#2A2A2A"), ("text", "#E0E0E0"),
            ("text_muted", "#9E9E9E"), ("border", "#3D3D3D"), ("track", "#5F5F5F"), ("primary", "#BB86FC"),
            ("primary_hover", "#C79BFD"), ("primary_pressed", "#A66EF7"), ("on_primary", "#000000"), ("accent", "#03DAC6"),
            ("focus_ring", "#BB86FC"), ("selection", "#BB86FC55"), ("primary_soft", "#BB86FC29"), ("danger", "#CF6679"),
        ]);
    } else {
        colors(&mut t, &[
            ("background", "#FAFAFA"), ("surface", "#FFFFFF"), ("surface_2", "#F5F5F5"), ("text", "#212121"),
            ("text_muted", "#757575"), ("border", "#E0E0E0"), ("track", "#BDBDBD"), ("primary", "#6200EE"),
            ("primary_hover", "#7722FF"), ("primary_pressed", "#3700B3"), ("on_primary", "#FFFFFF"), ("accent", "#03DAC6"),
            ("focus_ring", "#6200EE"), ("selection", "#6200EE33"), ("primary_soft", "#6200EE1A"), ("danger", "#B00020"),
        ]);
    }
    radius(&mut t, 2.0, 4.0, 4.0);
    fonts(&mut t, "Roboto", 14.0, 20.0);
    t.set_style(
        "Button",
        Style::new(p().fill("primary").text("on_primary").radius(4.0).padding((16.0, 6.0)).font("button").border(Color::TRANSPARENT, 0.0).shadow("sm").transition("fast"))
            .hover(p().fill("primary_hover").shadow("md"))
            .pressed(p().fill("primary_pressed").shadow("lg"))
            .focused(p().border("focus_ring", 2.0))
            .disabled(p().opacity(DISABLED)),
    );
    t.set_style("Button.accent", Style::new(p()));
    t.set_style(
        "Button.secondary",
        Style::new(p().fill(Color::TRANSPARENT).text("primary").border("border", 1.0).shadow("none")).hover(p().fill("primary_soft").shadow("none")),
    );
    t.set_style("Button.flat", Style::new(p().fill(Color::TRANSPARENT).text("primary").shadow("none")).hover(p().fill("primary_soft").shadow("none")));
    // Campo "preenchido": fundo e a linha de baixo (desenhada pelo look).
    fields(
        &mut t,
        Style::new(p().fill("surface_2").text("text").border(Color::TRANSPARENT, 0.0).radius(4.0).padding((12.0, 4.0)))
            .hover(p().fill("track"))
            .focused(p().fill("surface_2"))
            .disabled(p().opacity(DISABLED)),
    );
    toggles(
        &mut t,
        Style::new(p().fill(Color::TRANSPARENT).text("text").border("text_muted", 2.0).radius(2.0).transition("fast"))
            .checked(p().fill("primary").border("primary", 2.0))
            .disabled(p().opacity(DISABLED)),
    );
    t.set_style("Switch", Style::new(p().fill("track").text("text").transition("normal")).checked(p().fill("primary")).disabled(p().opacity(DISABLED)));
    t.set_style("Panel", Style::new(p().fill("surface").border(Color::TRANSPARENT, 0.0).radius(4.0).shadow("sm")));
    t.set_style("ProgressBar", Style::new(p().fill("track").radius(0.0)));
    t
}

// ----- Android 7 (Nougat) -----------------------------------------------------

/// O AppCompat do Android 5 a 7: barra índigo, destaque rosa nos controles, botão cinza em
/// relevo com texto em maiúsculas e campo só com a linha de baixo.
fn android7(dark: bool) -> Theme {
    let mut t = base(if dark { "Android 7 Dark" } else { "Android 7 Light" }, "Android 7", dark, Look::Material2);
    // `primary` é o colorAccent (o que pinta CheckBox, Switch, foco); `app_bar` é o
    // colorPrimary.
    if dark {
        colors(&mut t, &[
            ("background", "#303030"), ("surface", "#424242"), ("surface_2", "#4A4A4A"), ("text", "#FFFFFF"),
            ("text_muted", "#B3B3B3"), ("border", "#6E6E6E"), ("track", "#B9B9B9"), ("primary", "#80CBC4"),
            ("primary_hover", "#93D3CD"), ("primary_pressed", "#4DB6AC"), ("on_primary", "#000000"), ("accent", "#80CBC4"),
            ("focus_ring", "#80CBC4"), ("selection", "#80CBC455"), ("primary_soft", "#80CBC433"), ("danger", "#EF5350"),
            ("control", "#5A595B"), ("control_pressed", "#6E6E6E"), ("app_bar", "#212121"), ("app_bar_dark", "#000000"),
        ]);
    } else {
        colors(&mut t, &[
            ("background", "#FAFAFA"), ("surface", "#FFFFFF"), ("surface_2", "#F5F5F5"), ("text", "#212121"),
            ("text_muted", "#757575"), ("border", "#BDBDBD"), ("track", "#B2B2B2"), ("primary", "#FF4081"),
            ("primary_hover", "#FF5C93"), ("primary_pressed", "#F50057"), ("on_primary", "#FFFFFF"), ("accent", "#FF4081"),
            ("focus_ring", "#FF4081"), ("selection", "#FF408140"), ("primary_soft", "#FF40811F"), ("danger", "#F44336"),
            ("control", "#D6D7D7"), ("control_pressed", "#BDBDBD"), ("app_bar", "#3F51B5"), ("app_bar_dark", "#303F9F"),
        ]);
    }
    radius(&mut t, 2.0, 2.0, 2.0);
    fonts(&mut t, "Roboto", 14.0, 20.0);
    t.set_style(
        "Button",
        Style::new(p().fill("control").text("text").radius(2.0).padding((16.0, 6.0)).font("button").border(Color::TRANSPARENT, 0.0).shadow("sm").transition("fast"))
            .hover(p().shadow("md"))
            .pressed(p().fill("control_pressed").shadow("lg"))
            .focused(p().shadow("md"))
            .disabled(p().opacity(0.26)),
    );
    t.set_style("Button.accent", Style::new(p().fill("primary").text("on_primary")).hover(p().fill("primary_hover")).pressed(p().fill("primary_pressed")));
    t.set_style(
        "Button.secondary",
        Style::new(p().fill(Color::TRANSPARENT).text("primary").shadow("none")).hover(p().fill("primary_soft").shadow("none")).pressed(p().fill("primary_soft").shadow("none")),
    );
    t.set_style("Button.flat", Style::new(p().fill(Color::TRANSPARENT).text("primary").shadow("none")).hover(p().fill("primary_soft").shadow("none")));
    // O EditText: só a linha de baixo (desenhada pelo look), na cor de destaque com foco.
    fields(
        &mut t,
        Style::new(p().fill(Color::TRANSPARENT).text("text").border(Color::TRANSPARENT, 0.0).radius(0.0).padding((4.0, 4.0)))
            .disabled(p().opacity(0.38)),
    );
    for class in ["ListBox", "ListView", "TreeView", "Calendar"] {
        t.set_style(class, Style::new(p().fill("surface").text("text").border(Color::TRANSPARENT, 0.0).radius(2.0)));
    }
    toggles(
        &mut t,
        Style::new(p().fill(Color::TRANSPARENT).text("text").border("text_muted", 2.0).radius(2.0).transition("fast"))
            .checked(p().fill("primary").border("primary", 2.0))
            .disabled(p().opacity(0.38)),
    );
    t.set_style("Switch", Style::new(p().fill("track").text("text").transition("normal")).checked(p().fill("primary")).disabled(p().opacity(0.38)));
    t.set_style("Panel", Style::new(p().fill("surface").border(Color::TRANSPARENT, 0.0).radius(2.0).shadow("sm")));
    t.set_style("Panel.appbar", Style::new(p().fill("app_bar").text("#FFFFFF").border(Color::TRANSPARENT, 0.0).radius(0.0).shadow("md")));
    t.set_style("ProgressBar", Style::new(p().fill("primary_soft").radius(0.0)));
    t
}

// ----- iOS -----------------------------------------------------------------

fn ios_common(t: &mut Theme) {
    t.set_style(
        "Button.flat",
        Style::new(p().fill(Color::TRANSPARENT).text("primary").border(Color::TRANSPARENT, 0.0)).pressed(p().opacity(0.4)),
    );
    t.set_style("Button.link", Style::new(p().fill(Color::TRANSPARENT).text("primary")).pressed(p().opacity(0.4)));
    // O "CheckBox" do iOS é a marca redonda das listas; o Switch fica verde.
    toggles(
        t,
        Style::new(p().fill(Color::TRANSPARENT).text("text").border("border", 1.5).radius(999.0).transition("fast"))
            .checked(p().fill("primary").border("primary", 1.5))
            .disabled(p().opacity(0.4)),
    );
    t.set_style("Switch", Style::new(p().fill("track").text("text").transition("normal")).checked(p().fill("success")).disabled(p().opacity(0.4)));
    t.set_style("ProgressBar", Style::new(p().fill("track").radius(2.0)));
}

fn ios7() -> Theme {
    let mut t = base("iOS 7 Light", "iOS 7", false, Look::Ios7);
    colors(&mut t, &[
        ("background", "#EFEFF4"), ("surface", "#FFFFFF"), ("surface_2", "#F7F7F7"), ("text", "#000000"),
        ("text_muted", "#8E8E93"), ("border", "#C8C7CC"), ("track", "#E5E5EA"), ("primary", "#007AFF"),
        ("primary_hover", "#3395FF"), ("primary_pressed", "#0062CC"), ("on_primary", "#FFFFFF"), ("success", "#4CD964"),
        ("danger", "#FF3B30"), ("warning", "#FF9500"), ("focus_ring", "#007AFF"), ("selection", "#007AFF33"),
        ("primary_soft", "#007AFF1A"),
    ]);
    radius(&mut t, 5.0, 5.0, 10.0);
    fonts(&mut t, "", 15.0, 20.0);
    // No iOS 7 o botão padrão é só o texto na cor de destaque.
    t.set_style(
        "Button",
        Style::new(p().fill(Color::TRANSPARENT).text("primary").border(Color::TRANSPARENT, 0.0).radius(5.0).padding((10.0, 6.0)).transition("fast"))
            .hover(p().text("primary_hover"))
            .pressed(p().text("primary_pressed").opacity(0.5))
            .disabled(p().opacity(0.35)),
    );
    t.set_style("Button.accent", Style::new(p().fill("primary").text("on_primary")).hover(p().fill("primary_hover").text("on_primary")));
    t.set_style("Button.secondary", Style::new(p().fill(Color::TRANSPARENT).text("primary").border("primary", 1.0)).hover(p().fill("primary_soft")));
    t.set_style("Button.outline", Style::new(p().fill(Color::TRANSPARENT).text("primary").border("primary", 1.0)).hover(p().fill("primary_soft")));
    fields(
        &mut t,
        Style::new(p().fill("surface").text("text").border("border", 1.0).radius(5.0).padding((8.0, 4.0)))
            .focused(p().border("primary", 1.0))
            .disabled(p().opacity(0.4)),
    );
    ios_common(&mut t);
    t.set_style("Panel", Style::new(p().fill("surface").border("border", 0.5).radius(0.0)));
    t.set_style("SegmentedControl", Style::new(p().fill(Color::TRANSPARENT).text("primary").border("primary", 1.0).radius(5.0)));
    t
}

fn ios13(dark: bool) -> Theme {
    let mut t = base(if dark { "iOS 13 Dark" } else { "iOS 13 Light" }, "iOS 13", dark, Look::Ios13);
    if dark {
        colors(&mut t, &[
            ("background", "#000000"), ("surface", "#1C1C1E"), ("surface_2", "#2C2C2E"), ("text", "#FFFFFF"),
            ("text_muted", "#8D8D93"), ("border", "#38383A"), ("track", "#39393D"), ("primary", "#0A84FF"),
            ("primary_hover", "#3D9BFF"), ("primary_pressed", "#0071E0"), ("on_primary", "#FFFFFF"), ("success", "#30D158"),
            ("danger", "#FF453A"), ("warning", "#FF9F0A"), ("focus_ring", "#0A84FF"), ("selection", "#0A84FF55"),
            ("primary_soft", "#0A84FF29"),
        ]);
    } else {
        colors(&mut t, &[
            ("background", "#F2F2F7"), ("surface", "#FFFFFF"), ("surface_2", "#E9E9EB"), ("text", "#000000"),
            ("text_muted", "#8A8A8E"), ("border", "#C6C6C8"), ("track", "#E9E9EA"), ("primary", "#007AFF"),
            ("primary_hover", "#2F8FFF"), ("primary_pressed", "#0062CC"), ("on_primary", "#FFFFFF"), ("success", "#34C759"),
            ("danger", "#FF3B30"), ("warning", "#FF9500"), ("focus_ring", "#007AFF"), ("selection", "#007AFF33"),
            ("primary_soft", "#007AFF1F"),
        ]);
    }
    radius(&mut t, 8.0, 10.0, 12.0);
    fonts(&mut t, "", 15.0, 22.0);
    t.set_style(
        "Button",
        Style::new(p().fill("primary").text("on_primary").border(Color::TRANSPARENT, 0.0).radius(10.0).padding((16.0, 7.0)).font("strong").transition("fast"))
            .hover(p().fill("primary_hover"))
            .pressed(p().opacity(0.7))
            .disabled(p().opacity(0.35)),
    );
    t.set_style("Button.accent", Style::new(p()));
    t.set_style("Button.secondary", Style::new(p().fill("primary_soft").text("primary")).hover(p().fill("primary_soft")));
    t.set_style("Button.outline", Style::new(p().fill(Color::TRANSPARENT).text("primary").border("primary", 1.0)).hover(p().fill("primary_soft")));
    fields(
        &mut t,
        Style::new(p().fill("surface_2").text("text").border(Color::TRANSPARENT, 0.0).radius(10.0).padding((10.0, 4.0)))
            .focused(p().border("primary", 1.5))
            .disabled(p().opacity(0.4)),
    );
    ios_common(&mut t);
    t.set_style("Panel", Style::new(p().fill("surface").border(Color::TRANSPARENT, 0.0).radius(12.0)));
    t
}

fn ios26(dark: bool) -> Theme {
    let mut t = ios13(dark);
    t.name = if dark { "iOS 26 Dark" } else { "iOS 26 Light" }.into();
    t.family = "iOS 26".into();
    t.look = Look::Ios26;
    // Vidro: superfícies translúcidas, borda clara e cápsulas.
    if dark {
        colors(&mut t, &[
            ("background", "#0E0F14"), ("surface", "#FFFFFF1A"), ("surface_2", "#FFFFFF14"), ("border", "#FFFFFF2E"),
            ("glass_edge", "#FFFFFF40"), ("glass_fill", "#2A2A3070"),
        ]);
    } else {
        colors(&mut t, &[
            ("background", "#E6ECF4"), ("surface", "#FFFFFFB8"), ("surface_2", "#FFFFFF8C"), ("border", "#FFFFFFE6"),
            ("glass_edge", "#FFFFFFF2"), ("glass_fill", "#FFFFFF66"),
        ]);
    }
    radius(&mut t, 12.0, 999.0, 26.0);
    // Liquid Glass: o fundo dos controles desfoca o que está atrás (material "glass").
    t.set_style(
        "Button",
        Style::new(
            p().fill("glass_fill").text("primary").border(Color::TRANSPARENT, 0.0).radius(999.0).padding((18.0, 7.0)).font("strong").shadow("sm").material("glass").transition("fast"),
        )
        .hover(p().shadow("md"))
        .pressed(p().opacity(0.75))
        .disabled(p().opacity(0.35)),
    );
    t.set_style("Button.accent", Style::new(p().fill("primary").text("on_primary").material("glass")));
    t.set_style("Button.glass_prominent", Style::new(p().fill("primary").text("on_primary").material("glass")));
    t.set_style("Button.secondary", Style::new(p().fill("glass_fill").text("text").material("glass")));
    t.set_style("Button.flat", Style::new(p().fill(Color::TRANSPARENT).text("primary").shadow("none").material("none")));
    t.set_style("Button.link", Style::new(p().fill(Color::TRANSPARENT).text("primary").shadow("none").material("none")));
    fields(
        &mut t,
        Style::new(p().fill("surface_2").text("text").border("glass_edge", 1.0).radius(999.0).padding((14.0, 4.0)))
            .focused(p().border("primary", 1.5))
            .disabled(p().opacity(0.4)),
    );
    // O campo de busca é uma cápsula de vidro (como na barra do iOS 26).
    t.set_style(
        "Edit.search",
        Style::new(p().fill("glass_fill").text("text").border(Color::TRANSPARENT, 0.0).radius(999.0).padding((14.0, 4.0)).shadow("sm").material("glass")),
    );
    t.set_style("ToolBar.floating", Style::new(p().fill("glass_fill").border(Color::TRANSPARENT, 0.0).radius(999.0).shadow("md").padding(6.0).material("glass")));
    t.set_style("Panel.glass", Style::new(p().fill("glass_fill").border(Color::TRANSPARENT, 0.0).radius(26.0).shadow("md").material("glass")));
    t.set_style("Memo", Style::new(p().fill("surface_2").text("text").border("glass_edge", 1.0).radius(20.0).padding((12.0, 8.0))).focused(p().border("primary", 1.5)));
    t.set_style("Panel", Style::new(p().fill("surface").border("glass_edge", 1.0).radius(26.0).shadow("md")));
    t.set_style("Panel.card", Style::new(p().fill("glass_fill").border(Color::TRANSPARENT, 0.0).radius(26.0).shadow("md").material("glass")));
    t.set_style("SegmentedControl", Style::new(p().fill("glass_fill").text("text").radius(999.0).transition("fast").material("glass")));
    t
}

// ----- Windows -------------------------------------------------------------

fn windows7() -> Theme {
    let mut t = base("Windows 7", "Windows 7", false, Look::Win7);
    colors(&mut t, &[
        ("background", "#F0F0F0"), ("surface", "#FFFFFF"), ("surface_2", "#F0F0F0"), ("text", "#000000"),
        ("text_muted", "#6D6D6D"), ("border", "#ABADB3"), ("track", "#E6E6E6"), ("primary", "#3399FF"),
        ("primary_hover", "#4DA6FF"), ("primary_pressed", "#1F7FDF"), ("on_primary", "#FFFFFF"), ("success", "#06B025"),
        ("focus_ring", "#569DE5"), ("selection", "#3399FF"), ("primary_soft", "#3399FF26"), ("control", "#E5E5E5"),
        ("control_border", "#707070"), ("control_hover_border", "#3C7FB1"), ("control_pressed_border", "#2C628B"),
    ]);
    radius(&mut t, 2.0, 3.0, 3.0);
    fonts(&mut t, "Segoe UI", 12.0, 16.0);
    // A cor sólida é a reserva: o look desenha o gradiente do Aero.
    t.set_style(
        "Button",
        Style::new(p().fill("control").text("text").border("control_border", 1.0).radius(3.0).padding((12.0, 3.0)))
            .hover(p().border("control_hover_border", 1.0))
            .pressed(p().border("control_pressed_border", 1.0))
            .focused(p().border("focus_ring", 1.0))
            .disabled(p().opacity(0.5)),
    );
    t.set_style("Button.accent", Style::new(p()));
    fields(
        &mut t,
        Style::new(p().fill("surface").text("text").border("border", 1.0).radius(0.0).padding((5.0, 2.0)))
            .hover(p().border("#5794BF", 1.0))
            .focused(p().border("focus_ring", 1.0))
            .disabled(p().opacity(0.5)),
    );
    toggles(
        &mut t,
        Style::new(p().fill("surface").text("text").border("#8E8F8F", 1.0).radius(0.0))
            .hover(p().border("#3C7FB1", 1.0))
            .checked(p().fill("surface"))
            .disabled(p().opacity(0.5)),
    );
    t.set_style("Switch", Style::new(p().fill(Color::TRANSPARENT).text("text").border("text", 1.0)).checked(p().fill("primary").border("primary", 1.0)));
    t.set_style("Panel", Style::new(p().fill("surface_2").border("#D5DFE5", 1.0).radius(3.0)));
    t.set_style("ProgressBar", Style::new(p().fill("track").radius(2.0).text("success")));
    t
}

fn windows10(dark: bool) -> Theme {
    let mut t = base(if dark { "Windows 10 Dark" } else { "Windows 10 Light" }, "Windows 10", dark, Look::Win10);
    if dark {
        colors(&mut t, &[
            ("background", "#1F1F1F"), ("surface", "#2B2B2B"), ("surface_2", "#333333"), ("text", "#FFFFFF"),
            ("text_muted", "#999999"), ("border", "#666666"), ("track", "#5C5C5C"), ("primary", "#0078D7"),
            ("primary_hover", "#1A86DB"), ("primary_pressed", "#005A9E"), ("on_primary", "#FFFFFF"), ("focus_ring", "#FFFFFF"),
            ("selection", "#0078D7"), ("control", "#333333"), ("control_hover_border", "#858585"), ("control_pressed", "#666666"),
        ]);
    } else {
        colors(&mut t, &[
            ("background", "#FFFFFF"), ("surface", "#FFFFFF"), ("surface_2", "#F2F2F2"), ("text", "#000000"),
            ("text_muted", "#666666"), ("border", "#999999"), ("track", "#CCCCCC"), ("primary", "#0078D7"),
            ("primary_hover", "#1A86DB"), ("primary_pressed", "#005A9E"), ("on_primary", "#FFFFFF"), ("focus_ring", "#000000"),
            ("selection", "#0078D7"), ("control", "#CCCCCC"), ("control_hover_border", "#7A7A7A"), ("control_pressed", "#999999"),
        ]);
    }
    radius(&mut t, 0.0, 0.0, 0.0);
    fonts(&mut t, "Segoe UI", 14.0, 20.0);
    t.set_style(
        "Button",
        Style::new(p().fill("control").text("text").border(Color::TRANSPARENT, 2.0).radius(0.0).padding((12.0, 4.0)))
            .hover(p().border("control_hover_border", 2.0))
            .pressed(p().fill("control_pressed").border("control_pressed", 2.0))
            .focused(p().border("focus_ring", 2.0))
            .disabled(p().opacity(0.4)),
    );
    t.set_style(
        "Button.accent",
        Style::new(p().fill("primary").text("on_primary")).hover(p().fill("primary").border("control_hover_border", 2.0)).pressed(p().fill("primary_pressed")),
    );
    fields(
        &mut t,
        Style::new(p().fill("surface").text("text").border("border", 2.0).radius(0.0).padding((8.0, 3.0)))
            .hover(p().border("text_muted", 2.0))
            .focused(p().border("primary", 2.0))
            .disabled(p().opacity(0.4)),
    );
    toggles(
        &mut t,
        Style::new(p().fill(Color::TRANSPARENT).text("text").border("text", 2.0).radius(0.0))
            .checked(p().fill("primary").border("primary", 2.0))
            .disabled(p().opacity(0.4)),
    );
    t.set_style("Switch", Style::new(p().fill(Color::TRANSPARENT).text("text").border("text", 2.0)).checked(p().fill("primary").border("primary", 2.0)));
    t.set_style("Panel", Style::new(p().fill("surface_2").border(Color::TRANSPARENT, 0.0).radius(0.0)));
    t.set_style("ProgressBar", Style::new(p().fill("track").radius(0.0)));
    t
}

/// Win32: os controles comuns do Windows num programa desktop clássico, como eles aparecem
/// no Windows 10 e 11 (o tema do sistema nos controles nativos): fundo `ButtonFace` cinza,
/// botão cinza claro de borda fina que fica azul no hover, campo branco de borda 1 px, caixa
/// marcada branca com o visto escuro, Segoe UI 9 pt e cantos retos.
fn win32() -> Theme {
    let mut t = base("Win32", "Win32", false, Look::Win10);
    colors(&mut t, &[
        ("background", "#F0F0F0"), ("surface", "#FFFFFF"), ("surface_2", "#F0F0F0"), ("text", "#000000"),
        ("text_muted", "#6D6D6D"), ("border", "#ADADAD"), ("track", "#E6E6E6"), ("primary", "#0078D7"),
        ("primary_hover", "#1A86DB"), ("primary_pressed", "#005A9E"), ("on_primary", "#FFFFFF"), ("focus_ring", "#0078D7"),
        ("selection", "#CCE8FF"), ("control", "#E1E1E1"), ("control_hover", "#E5F1FB"), ("control_hover_border", "#0078D7"),
        ("control_pressed", "#CCE4F7"), ("control_pressed_border", "#005499"), ("field_border", "#7A7A7A"), ("check_mark", "#000000"), ("progress", "#06B025"),
        ("success", "#06B025"), ("danger", "#E81123"), ("danger_hover", "#F1707A"), ("danger_pressed", "#BF0E1C"),
    ]);
    radius(&mut t, 0.0, 0.0, 0.0);
    fonts(&mut t, "Segoe UI", 12.0, 18.0);
    t.set_style(
        "Button",
        Style::new(p().fill("control").text("text").border("border", 1.0).radius(0.0).padding((10.0, 3.0)))
            .hover(p().fill("control_hover").border("control_hover_border", 1.0))
            .pressed(p().fill("control_pressed").border("control_pressed_border", 1.0))
            .focused(p().border("primary", 1.0))
            .disabled(p().fill("surface_2").border("track", 1.0).opacity(0.6)),
    );
    // O botão padrão (Default = True) tem a borda azul.
    t.set_style("Button.accent", Style::new(p().border("primary", 2.0)).hover(p().fill("control_hover")));
    fields(
        &mut t,
        Style::new(p().fill("surface").text("text").border("field_border", 1.0).radius(0.0).padding((6.0, 2.0)))
            .hover(p().border("text", 1.0))
            .focused(p().border("primary", 1.0))
            .disabled(p().fill("surface_2").border("track", 1.0).opacity(0.6)),
    );
    toggles(
        &mut t,
        Style::new(p().fill("surface").text("text").border("text", 1.0).radius(0.0))
            .hover(p().border("primary", 1.0))
            .checked(p().fill("surface").border("text", 1.0))
            .disabled(p().opacity(0.5)),
    );
    t.set_style("Switch", Style::new(p().fill("surface").text("text").border("text", 1.0)).checked(p().fill("primary").border("primary", 1.0)));
    t.set_style("Panel", Style::new(p().fill("background").border(Color::TRANSPARENT, 0.0).radius(0.0)));
    t.set_style("GroupBox", Style::new(p().fill(Color::TRANSPARENT).text("text").border("track", 1.0).radius(0.0)));
    // A barra de progresso verde, num trilho com borda.
    t.set_style("ProgressBar", Style::new(p().fill("track").border("border", 1.0).radius(0.0)));
    t.colors.insert("primary".into(), Color::from_hex("#0078D7").unwrap_or_default());
    t
}

/// WinUI 3 (Windows App SDK): os recursos de tema do Fluent 2 como o WinUI define,
/// com os preenchimentos translúcidos sobre a Mica e a fonte Segoe UI Variable.
fn winui3(dark: bool) -> Theme {
    let mut t = windows11(dark);
    t.name = if dark { "WinUI 3 Dark" } else { "WinUI 3 Light" }.into();
    t.family = "WinUI 3".into();
    // ControlFillColorDefault/Secondary/Tertiary, ControlStrokeColorDefault/Secondary,
    // ControlStrongStrokeColorDefault, CardBackgroundFillColorDefault, TextFillColor...
    if dark {
        colors(&mut t, &[
            ("background", "#202020"), ("surface", "#FFFFFF0D"), ("surface_2", "#FFFFFF0F"), ("text", "#FFFFFF"),
            ("text_muted", "#FFFFFFC5"), ("border", "#00000019"), ("track", "#FFFFFF8B"), ("primary", "#60CDFF"),
            ("primary_hover", "#60CDFFE6"), ("primary_pressed", "#60CDFFCC"), ("on_primary", "#000000"), ("focus_ring", "#FFFFFF"),
            ("selection", "#60CDFF55"), ("control", "#FFFFFF0F"), ("control_hover", "#FFFFFF15"), ("control_pressed", "#FFFFFF08"),
            ("control_stroke", "#FFFFFF12"), ("control_bottom", "#FFFFFF18"), ("field_bottom", "#FFFFFF8B"), ("mica", "#202020"),
        ]);
    } else {
        colors(&mut t, &[
            ("background", "#F3F3F3"), ("surface", "#FFFFFFB3"), ("surface_2", "#F9F9F980"), ("text", "#000000E4"),
            ("text_muted", "#0000009E"), ("border", "#0000000F"), ("track", "#00000072"), ("primary", "#005FB8"),
            ("primary_hover", "#005FB8E6"), ("primary_pressed", "#005FB8CC"), ("on_primary", "#FFFFFF"), ("focus_ring", "#000000E4"),
            ("selection", "#005FB840"), ("control", "#FFFFFFB3"), ("control_hover", "#F9F9F980"), ("control_pressed", "#F9F9F94D"),
            ("control_stroke", "#0000000F"), ("control_bottom", "#00000029"), ("field_bottom", "#00000072"), ("mica", "#F3F3F3"),
        ]);
    }
    fonts(&mut t, "Segoe UI Variable Text", 14.0, 28.0);
    t.fonts.insert("title".into(), FontSpec::new("Segoe UI Variable Display", 28.0).weight(FontWeight::SemiBold));
    t
}

fn windows11(dark: bool) -> Theme {
    let mut t = base(if dark { "Windows 11 Dark" } else { "Windows 11 Light" }, "Windows 11", dark, Look::Win11);
    if dark {
        colors(&mut t, &[
            ("background", "#202020"), ("surface", "#2B2B2B"), ("surface_2", "#323232"), ("text", "#FFFFFF"),
            ("text_muted", "#C5C5C5"), ("border", "#3B3B3B"), ("track", "#9A9A9A"), ("primary", "#60CDFF"),
            ("primary_hover", "#5BBDEB"), ("primary_pressed", "#55AFD9"), ("on_primary", "#000000"), ("focus_ring", "#FFFFFF"),
            ("selection", "#60CDFF55"), ("control", "#2D2D2D"), ("control_hover", "#323232"), ("control_pressed", "#272727"),
            ("control_stroke", "#3A3A3A"), ("control_bottom", "#454545"), ("field_bottom", "#9A9A9A"),
        ]);
    } else {
        colors(&mut t, &[
            ("background", "#F3F3F3"), ("surface", "#FBFBFB"), ("surface_2", "#F9F9F9"), ("text", "#1A1A1A"),
            ("text_muted", "#5D5D5D"), ("border", "#E5E5E5"), ("track", "#8A8A8A"), ("primary", "#005FB8"),
            ("primary_hover", "#1A6FC0"), ("primary_pressed", "#3378C2"), ("on_primary", "#FFFFFF"), ("focus_ring", "#1A1A1A"),
            ("selection", "#005FB840"), ("control", "#FBFBFB"), ("control_hover", "#F6F6F6"), ("control_pressed", "#F5F5F5"),
            ("control_stroke", "#E5E5E5"), ("control_bottom", "#CCCCCC"), ("field_bottom", "#868686"),
        ]);
    }
    radius(&mut t, 4.0, 4.0, 8.0);
    fonts(&mut t, "Segoe UI", 14.0, 20.0);
    t.set_style(
        "Button",
        Style::new(p().fill("control").text("text").border("control_stroke", 1.0).radius(4.0).padding((12.0, 4.0)).transition("fast"))
            .hover(p().fill("control_hover"))
            .pressed(p().fill("control_pressed").text("text_muted"))
            .focused(p().border("focus_ring", 2.0))
            .disabled(p().opacity(0.4)),
    );
    t.set_style(
        "Button.accent",
        Style::new(p().fill("primary").text("on_primary").border("primary", 1.0)).hover(p().fill("primary_hover")).pressed(p().fill("primary_pressed").text("on_primary")),
    );
    fields(
        &mut t,
        Style::new(p().fill("surface").text("text").border("control_stroke", 1.0).radius(4.0).padding((10.0, 4.0)))
            .hover(p().fill("control_hover"))
            .focused(p().fill("surface"))
            .disabled(p().opacity(0.4)),
    );
    toggles(
        &mut t,
        Style::new(p().fill("surface_2").text("text").border("text_muted", 1.0).radius(4.0).transition("fast"))
            .hover(p().fill("control_hover"))
            .checked(p().fill("primary").border("primary", 1.0))
            .disabled(p().opacity(0.4)),
    );
    t.set_style(
        "Switch",
        Style::new(p().fill(Color::TRANSPARENT).text("text").border("text_muted", 1.0).transition("fast")).checked(p().fill("primary").border("primary", 1.0)),
    );
    t.set_style("Panel", Style::new(p().fill("surface").border("control_stroke", 1.0).radius(8.0)));
    t.set_style("ProgressBar", Style::new(p().fill("track").radius(2.0)));
    t
}

// ----- Windows Phone -----------------------------------------------------------

/// Windows Phone 7 e 8 (Metro): fundo preto ou branco, cor de destaque (azul), bordas de
/// 3 px, nada arredondado, títulos finos e grandes. A Segoe WP não pode ir no app; vale a
/// Segoe UI.
fn windows_phone(dark: bool) -> Theme {
    let mut t = base(if dark { "Windows Phone Dark" } else { "Windows Phone Light" }, "Windows Phone", dark, Look::WinPhone);
    if dark {
        colors(&mut t, &[
            ("background", "#000000"), ("surface", "#000000"), ("surface_2", "#1F1F1F"), ("text", "#FFFFFF"),
            ("text_muted", "#999999"), ("border", "#FFFFFF"), ("track", "#333333"), ("primary", "#1BA1E2"),
            ("primary_hover", "#1BA1E2"), ("primary_pressed", "#1581B5"), ("on_primary", "#FFFFFF"), ("focus_ring", "#1BA1E2"),
            ("selection", "#1BA1E2"), ("primary_soft", "#1BA1E240"), ("field", "#BFBFBF"), ("field_text", "#000000"),
            ("danger", "#E51400"), ("success", "#339933"),
        ]);
    } else {
        colors(&mut t, &[
            ("background", "#FFFFFF"), ("surface", "#FFFFFF"), ("surface_2", "#F2F2F2"), ("text", "#000000"),
            ("text_muted", "#666666"), ("border", "#000000"), ("track", "#CCCCCC"), ("primary", "#1BA1E2"),
            ("primary_hover", "#1BA1E2"), ("primary_pressed", "#1581B5"), ("on_primary", "#FFFFFF"), ("focus_ring", "#1BA1E2"),
            ("selection", "#1BA1E2"), ("primary_soft", "#1BA1E240"), ("field", "#DEDEDE"), ("field_text", "#000000"),
            ("danger", "#E51400"), ("success", "#339933"),
        ]);
    }
    radius(&mut t, 0.0, 0.0, 0.0);
    fonts(&mut t, "Segoe UI", 15.0, 32.0);
    t.fonts.insert("title".into(), FontSpec::new("Segoe UI", 32.0).weight(FontWeight::Light));
    t.fonts.insert("heading".into(), FontSpec::new("Segoe UI", 22.0).weight(FontWeight::Light));
    t.set_style(
        "Button",
        Style::new(p().fill(Color::TRANSPARENT).text("text").border("text", 3.0).radius(0.0).padding((12.0, 4.0)).font("strong"))
            .pressed(p().fill("primary").text("on_primary").border("primary", 3.0))
            .focused(p().border("text", 3.0))
            .disabled(p().opacity(0.4)),
    );
    t.set_style("Button.accent", Style::new(p().fill("primary").text("on_primary").border("primary", 3.0)).pressed(p().fill("primary_pressed").border("primary_pressed", 3.0)));
    t.set_style("Button.secondary", Style::new(p().fill(Color::TRANSPARENT).text("text_muted").border("text_muted", 3.0)));
    t.set_style("Button.flat", Style::new(p().fill(Color::TRANSPARENT).text("text").border(Color::TRANSPARENT, 0.0)).pressed(p().text("primary")));
    // TextBox: cinza claro sem foco; branco com a borda na cor de destaque com foco.
    fields(
        &mut t,
        Style::new(p().fill("field").text("field_text").border("field", 3.0).radius(0.0).padding((10.0, 4.0)))
            .focused(p().fill("#FFFFFF").border("primary", 3.0))
            .disabled(p().opacity(0.4)),
    );
    for class in ["ListBox", "ListView", "TreeView", "Calendar"] {
        t.set_style(class, Style::new(p().fill("background").text("text").border(Color::TRANSPARENT, 0.0).radius(0.0)));
    }
    toggles(
        &mut t,
        Style::new(p().fill("field").text("text").border("field", 3.0).radius(0.0))
            .checked(p().fill("field").border("field", 3.0))
            .disabled(p().opacity(0.4)),
    );
    t.set_style("Switch", Style::new(p().fill(Color::TRANSPARENT).text("text").border("text", 3.0)).checked(p().fill("primary")).disabled(p().opacity(0.4)));
    t.set_style("Panel", Style::new(p().fill("background").border(Color::TRANSPARENT, 0.0).radius(0.0)));
    t.set_style("Panel.card", Style::new(p().fill("primary").text("on_primary").border(Color::TRANSPARENT, 0.0).radius(0.0)));
    t.set_style("ProgressBar", Style::new(p().fill("track").radius(0.0)));
    t
}
