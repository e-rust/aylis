//! Os editores visuais do Object Inspector, como as ferramentas do DevTools: relógio de
//! ângulo, editor de sombra, editor da curva de suavização, a linha do tempo da animação e
//! o editor de fonte. O inspector abre um deles conforme o tipo da propriedade
//! (`PropKind::Angle`, `Shadow`, `Easing` e `Duration`) ou o nome (`font_family`); a cor
//! tem o seu em [`dialogs::color_picker`].

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::{AngleDial, Button, CheckBox, ComboBox, EasingEditor, Label, Layout, ListBox, Rectangle, ScrollBox, ShadowEditor, TimingEditor};
use aylis_core::{Align, Color, ComponentId, CursorShape, Easing, EventArgs, PropValue, Shadow, Ui};

use crate::dialogs::{self, edit, field, new_dialog, run, text_of};
use crate::widgets::{caption, label, muted};

fn number(v: f64) -> String {
    let v = (v * 100.0).round() / 100.0;
    if v.fract() == 0.0 { format!("{}", v as i64) } else { format!("{v}") }
}

/// Número digitado (aceita vírgula e as unidades `°` e `ms`).
pub fn parse_number(text: &str) -> Option<f64> {
    let t = text.trim().trim_end_matches("ms").trim_end_matches('°').trim();
    let t = if t.contains('.') { t.to_string() } else { t.replace(',', ".") };
    t.parse::<f64>().ok().filter(|v| v.is_finite())
}

fn hint(ui: &mut Ui, parent: ComponentId, text: &str) {
    let l = label(ui, parent, text, Align::Top, 22.0);
    muted(ui, l);
    let _ = ui.set_prop(l, "font_size", 11.5);
}

/// Uma faixa de botões pequenos (atalhos de valores).
fn chips(ui: &mut Ui, parent: ComponentId, names: &[&str], width: f32) -> Vec<ComponentId> {
    let bar = ui.add(Layout::default(), parent);
    ui.control_mut(bar).unwrap().set_align(Align::Top).set_height(30.0).set_margins([0.0, 0.0, 0.0, 8.0]);
    names
        .iter()
        .map(|n| {
            let b = ui.add(Button::new(*n), bar);
            ui.control_mut(b).unwrap().set_align(Align::Left).set_width(width).set_margins([0.0, 0.0, 4.0, 0.0]).set_style("outline");
            b
        })
        .collect()
}

/// Uma área do tamanho dado, com o editor centralizado nela.
fn stage(ui: &mut Ui, parent: ComponentId, height: f32) -> ComponentId {
    let s = ui.add(Layout::default(), parent);
    ui.control_mut(s).unwrap().set_align(Align::Top).set_height(height).set_margins([0.0, 0.0, 0.0, 8.0]);
    s
}

// ---------------------------------------------------------------------------
// Ângulo
// ---------------------------------------------------------------------------

/// O relógio de ângulo: arrastar o ponteiro, digitar os graus ou escolher um atalho.
pub fn angle_editor(ui: &mut Ui, title: &str, initial: f64, on_ok: impl FnOnce(&mut Ui, f64) + 'static) {
    let d = new_dialog(ui, title, 460.0, 430.0);
    let s = stage(ui, d.body, 196.0);
    let dial = ui.add(AngleDial::new(initial as f32), s);
    ui.control_mut(dial).unwrap().set_rect((460.0 - 32.0 - 188.0) / 2.0, 4.0, 188.0, 188.0);
    let line = field(ui, d.body, "Graus");
    let value = edit(ui, line, &number(initial));
    hint(ui, d.body, "Arraste o ponteiro (Shift: de 15 em 15°), setas ou roda do mouse.");
    let presets = [0.0, 45.0, 90.0, 135.0, 180.0, 225.0, 270.0, 315.0];
    let names: Vec<String> = presets.iter().map(|p| format!("{p}°")).collect();
    let refs: Vec<&str> = names.iter().map(String::as_str).collect();
    for (b, p) in chips(ui, d.body, &refs, 49.0).into_iter().zip(presets) {
        ui.on_click(b, move |ui, _| {
            let _ = ui.set_prop(dial, "angle", p);
            let _ = ui.set_prop(value, "text", number(p));
        });
    }
    ui.on(dial, "on_change", move |ui, _, args| {
        if let EventArgs::Value(v) = args {
            // Sem disparar o `on_change` do campo (o valor pelo código avisa, como na VCL): senão o
            // texto digitado voltava reescrito ("400" virava "40").
            let _ = ui.set_prop_quiet(value, "text", number(v.to_f64().unwrap_or(0.0)));
        }
    });
    ui.on(value, "on_change", move |ui, _, _| {
        if let Some(v) = parse_number(&text_of(ui, value)) {
            let _ = ui.set_prop_quiet(dial, "angle", v.rem_euclid(360.0));
        }
    });
    run(
        ui,
        d,
        move |ui| parse_number(&text_of(ui, value)).map(|_| ()).ok_or_else(|| "informe os graus".to_string()),
        move |ui| {
            let v = parse_number(&text_of(ui, value)).unwrap_or(0.0);
            on_ok(ui, v);
        },
    );
}

// ---------------------------------------------------------------------------
// Fonte
// ---------------------------------------------------------------------------

/// Uma fonte escolhida no editor (o `Font` do Delphi): família (vazia = a do pai ou do
/// tema), tamanho (`None` = o herdado), peso (`default` = o herdado), itálico, sublinhado
/// e tachado.
#[derive(Clone, Debug, PartialEq)]
pub struct FontChoice {
    pub family: String,
    pub size: Option<f64>,
    pub weight: String,
    pub italic: bool,
    pub underline: bool,
    pub strikeout: bool,
}

impl Default for FontChoice {
    fn default() -> Self {
        Self { family: String::new(), size: None, weight: "default".into(), italic: false, underline: false, strikeout: false }
    }
}

/// Famílias que existem em qualquer sistema desktop comum (quando o sistema não diz quais
/// tem).
const COMMON_FAMILIES: [&str; 8] = ["Segoe UI", "Arial", "Helvetica", "Verdana", "Georgia", "Times New Roman", "Courier New", "Consolas"];

/// Os pesos no editor, com o nome mostrado.
const WEIGHT_CAPTIONS: [(&str, &str); 8] = [
    ("default", "(o herdado)"),
    ("thin", "Fino"),
    ("light", "Leve"),
    ("normal", "Normal"),
    ("medium", "Médio"),
    ("semibold", "Seminegrito"),
    ("bold", "Negrito"),
    ("black", "Extranegrito"),
];

/// O editor de fonte: `families` são as do projeto (primeiro) e `system` as instaladas.
pub fn font_editor(ui: &mut Ui, initial: FontChoice, families: Vec<String>, system: Vec<String>, on_ok: impl FnOnce(&mut Ui, FontChoice) + 'static) {
    let d = new_dialog(ui, "Fonte", 600.0, 580.0);
    // Prévia embaixo, a lista ocupa o resto.
    let preview_box = ui.add(Layout::default(), d.body);
    ui.control_mut(preview_box).unwrap().set_align(Align::Bottom).set_height(84.0).set_margins([0.0, 10.0, 0.0, 0.0]);
    let preview = ui.add(Label::new("Ação rápida: 0123 ÁÉÍÕÇ"), preview_box);
    ui.control_mut(preview).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(preview, "h_align", "center");
    let _ = ui.set_prop(preview, "v_align", "center");

    let line = field(ui, d.body, "Família");
    let family = edit(ui, line, &initial.family);
    let line = field(ui, d.body, "Tamanho");
    let size = edit(ui, line, &initial.size.map(number).unwrap_or_default());
    let line = field(ui, d.body, "Peso");
    let captions: Vec<&str> = WEIGHT_CAPTIONS.iter().map(|(_, c)| *c).collect();
    let weight = ui.add(ComboBox::new(&captions), line);
    ui.control_mut(weight).unwrap().set_align(Align::Client);
    let start = WEIGHT_CAPTIONS.iter().position(|(n, _)| n.eq_ignore_ascii_case(&initial.weight)).unwrap_or(0);
    let _ = ui.set_prop(weight, "item_index", start as i64);
    let line = field(ui, d.body, "Estilo");
    let check = |ui: &mut Ui, text: &str, on: bool| {
        let c = ui.add(CheckBox::new(text), line);
        ui.control_mut(c).unwrap().set_align(Align::Left).set_width(130.0);
        let _ = ui.set_prop(c, "checked", on);
        c
    };
    let italic = check(ui, "Itálico", initial.italic);
    let underline = check(ui, "Sublinhado", initial.underline);
    let strikeout = check(ui, "Tachado", initial.strikeout);
    hint(ui, d.body, "Vazio = a do pai (ou do tema). As fontes do projeto (Projeto → Opções do projeto → Fontes) vêm primeiro.");

    let mut list_items = vec!["(a herdada)".to_string()];
    list_items.extend(families.iter().cloned());
    let system: Vec<String> = if system.is_empty() { COMMON_FAMILIES.iter().map(|f| f.to_string()).collect() } else { system };
    for f in system {
        if !list_items.iter().any(|p| p.eq_ignore_ascii_case(&f)) {
            list_items.push(f);
        }
    }
    let list = ui.add(ListBox::default(), d.body);
    ui.control_mut(list).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(list, "items", list_items.join(";"));
    if let Some(i) = list_items.iter().position(|f| f.eq_ignore_ascii_case(&initial.family)) {
        let _ = ui.set_prop(list, "item_index", i as i64);
    }

    let checked = |ui: &Ui, c: ComponentId| ui.get_prop(c, "checked").ok().and_then(|v| v.to_bool().ok()).unwrap_or(false);
    let current = move |ui: &Ui| FontChoice {
        family: text_of(ui, family).trim().to_string(),
        size: parse_number(&text_of(ui, size)).filter(|v| *v > 0.0),
        weight: ui
            .get_prop(weight, "item_index")
            .ok()
            .and_then(|v| v.to_i64().ok())
            .and_then(|i| WEIGHT_CAPTIONS.get(i.max(0) as usize))
            .map_or("default", |(n, _)| *n)
            .to_string(),
        italic: checked(ui, italic),
        underline: checked(ui, underline),
        strikeout: checked(ui, strikeout),
    };
    let show = move |ui: &mut Ui| {
        let c = current(ui);
        let _ = ui.set_prop(preview, "font_family", c.family.clone());
        let _ = ui.set_prop(preview, "font_size", c.size.unwrap_or(22.0));
        let _ = ui.set_prop(preview, "font_weight", c.weight.as_str());
        let _ = ui.set_prop(preview, "font_italic", c.italic);
        let _ = ui.set_prop(preview, "font_underline", c.underline);
        let _ = ui.set_prop(preview, "font_strikeout", c.strikeout);
    };
    show(ui);
    ui.on(list, "on_change", move |ui, _, args| {
        if let EventArgs::Text(t) = args {
            let name = if t.starts_with('(') { "" } else { t.as_str() };
            let _ = ui.set_prop(family, "text", name);
            show(ui);
        }
    });
    for c in [family, size, weight, italic, underline, strikeout] {
        ui.on(c, "on_change", move |ui, _, _| show(ui));
    }
    run(
        ui,
        d,
        move |ui| {
            let t = text_of(ui, size);
            if t.trim().is_empty() || parse_number(&t).is_some_and(|v| v > 0.0) { Ok(()) } else { Err(format!("'{t}' não é um tamanho")) }
        },
        move |ui| {
            let c = current(ui);
            on_ok(ui, c);
        },
    );
}

// ---------------------------------------------------------------------------
// Caracteres (allowed_chars, blocked_chars...)
// ---------------------------------------------------------------------------

/// Os caracteres de um campo ou contêiner. `overrides` é `None` num campo (só os
/// contêineres mandam nos filhos).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CharsChoice {
    pub allowed: String,
    pub blocked: String,
    pub only_allowed: bool,
    pub overrides: Option<bool>,
    /// As trocas (`char_map`): pares "digitado, entregue" em seguida.
    pub map: String,
    /// O `char_case` (`normal`, `upper`, `lower`); `None` num contêiner (só os campos têm).
    pub case: Option<String>,
}

/// Modelos prontos: (nome, liberados, bloqueados, só os liberados).
pub const CHARS_PRESETS: [(&str, &str, &str, bool); 9] = [
    ("Personalizado", "", "", false),
    ("Só números", "0-9", "", true),
    ("Números com vírgula e sinal", "0-9,\\-", "", true),
    ("Só letras", "a-zA-ZÀ-ÿ\\s", "", true),
    ("Letras e números", "a-zA-ZÀ-ÿ0-9\\s", "", true),
    ("E-mail", "a-zA-Z0-9@._+\\-", "", true),
    ("Telefone", "0-9()+\\-\\s", "", true),
    ("Sem espaços", "", "\\s", false),
    ("Sem aspas e apóstrofo", "", "\"'", false),
];

/// Os símbolos da grade (os mais pedidos); o espaço aparece como "espaço".
const CHARS_GRID: &str = " @#$%&*()-_=+[]{};:'\",.<>/?\\|!~`^";

/// O caractere como ele entra num conjunto (`\s`, `\-`, `\\`).
fn set_char(c: char) -> String {
    match c {
        ' ' => "\\s".into(),
        '-' => "\\-".into(),
        '\\' => "\\\\".into(),
        c => c.to_string(),
    }
}

/// Tira `c` de um conjunto escrito como `set_char` (as faixas ficam).
fn remove_set_char(set: &str, c: char) -> String {
    set.replace(&set_char(c), "")
}

/// O editor dos caracteres (a lista de teclas liberadas e bloqueadas): um modelo pronto,
/// os dois conjuntos, a grade dos símbolos (clique: neutro → liberado → bloqueado) e um
/// campo de teste que já obedece.
pub fn chars_editor(ui: &mut Ui, initial: CharsChoice, on_ok: impl FnOnce(&mut Ui, CharsChoice) + 'static) {
    use aylis_controls::{Edit, FlowLayout};
    let d = new_dialog(ui, "Entrada de texto", 640.0, 700.0);
    let line = field(ui, d.body, "Modelo");
    let names: Vec<&str> = CHARS_PRESETS.iter().map(|p| p.0).collect();
    let preset = ui.add(ComboBox::new(&names), line);
    ui.control_mut(preset).unwrap().set_align(Align::Client);
    let start = CHARS_PRESETS.iter().position(|p| p.1 == initial.allowed && p.2 == initial.blocked && p.3 == initial.only_allowed).unwrap_or(0);
    let _ = ui.set_prop(preset, "item_index", start as i64);
    let line = field(ui, d.body, "Liberados");
    let allowed = edit(ui, line, &initial.allowed);
    let line = field(ui, d.body, "Bloqueados");
    let blocked = edit(ui, line, &initial.blocked);
    let line = field(ui, d.body, "");
    let only = ui.add(CheckBox::new("Só os liberados (o resto é bloqueado)"), line);
    ui.control_mut(only).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(only, "checked", initial.only_allowed);
    let overrides = initial.overrides.map(|on| {
        let line = field(ui, d.body, "");
        let c = ui.add(CheckBox::new("Este contêiner tem prioridade sobre os campos de dentro"), line);
        ui.control_mut(c).unwrap().set_align(Align::Client);
        let _ = ui.set_prop(c, "checked", on);
        c
    });
    hint(ui, d.body, "Faixas: 0-9, a-z, A-Z. \\s é o espaço, \\- o hífen. O campo decide primeiro; o form e os painéis valem para o que o campo não diz (ou mandam, com a prioridade).");
    // As trocas: uma por linha, o digitado e o entregue (`, .`).
    let line = field(ui, d.body, "Trocas");
    let pairs: String = initial.map.chars().collect::<Vec<char>>().chunks(2).filter(|p| p.len() == 2).map(|p| format!("{} {}\n", p[0], p[1])).collect();
    let map = ui.add(aylis_controls::Memo::new(&pairs), line);
    ui.control_mut(map).unwrap().set_align(Align::Client);
    ui.control_mut(line).unwrap().set_height(70.0);
    hint(ui, d.body, "Uma troca por linha: o caractere digitado, espaço, o que o campo recebe (\", .\" troca a vírgula por ponto). Vale ao digitar e ao colar.");
    let case = initial.case.as_ref().map(|value| {
        let line = field(ui, d.body, "Maiúsculas");
        let c = ui.add(ComboBox::new(&["Como digitado", "Tudo maiúsculo", "Tudo minúsculo"]), line);
        ui.control_mut(c).unwrap().set_align(Align::Client);
        let index = match value.as_str() { "upper" => 1i64, "lower" => 2, _ => 0 };
        let _ = ui.set_prop(c, "item_index", index);
        c
    });

    // A grade: cada símbolo com o estado dele.
    let grid = ui.add(FlowLayout::default(), d.body);
    ui.control_mut(grid).unwrap().set_align(Align::Top).set_height(150.0).set_margins([0.0, 6.0, 0.0, 6.0]);
    let cells: Vec<(char, ComponentId)> = CHARS_GRID
        .chars()
        .map(|c| {
            let b = ui.add(Button::new(if c == ' ' { "espaço".to_string() } else { c.to_string() }), grid);
            ui.control_mut(b).unwrap().set_width(if c == ' ' { 76.0 } else { 44.0 }).set_height(30.0);
            (c, b)
        })
        .collect();
    let test_line = field(ui, d.body, "Teste aqui");
    let test = ui.add(Edit::new(""), test_line);
    ui.control_mut(test).unwrap().set_align(Align::Client);

    let checked = |ui: &Ui, c: ComponentId| ui.get_prop(c, "checked").ok().and_then(|v| v.to_bool().ok()).unwrap_or(false);
    let current = move |ui: &Ui| CharsChoice {
        allowed: text_of(ui, allowed),
        blocked: text_of(ui, blocked),
        only_allowed: checked(ui, only),
        overrides: overrides.map(|c| checked(ui, c)),
        map: text_of(ui, map).lines().filter_map(|l| { let cs: Vec<char> = l.chars().filter(|c| !c.is_whitespace()).collect(); (cs.len() == 2).then(|| cs.iter().collect::<String>()) }).collect(),
        case: case.map(|c| match ui.get_prop(c, "item_index").ok().and_then(|v| v.to_i64().ok()) { Some(1) => "upper".to_string(), Some(2) => "lower".to_string(), _ => "normal".to_string() }),
    };
    // A grade e o campo de teste acompanham os conjuntos.
    let cells = Rc::new(cells);
    let show = {
        let cells = cells.clone();
        move |ui: &mut Ui| {
            let c = current(ui);
            let filter = aylis_core::CharFilter { allowed: c.allowed.clone(), blocked: c.blocked.clone(), only_allowed: c.only_allowed, overrides: false, ..Default::default() };
            for &(ch, b) in cells.iter() {
                let (mark, style) = match filter.opinion(ch) {
                    Some(true) => ("✓ ", ""),
                    Some(false) => ("✕ ", "danger"),
                    None => ("", "flat"),
                };
                let name = if ch == ' ' { "espaço".to_string() } else { ch.to_string() };
                let _ = ui.set_prop(b, "text", format!("{mark}{name}"));
                if let Some(cb) = ui.control_mut(b) {
                    cb.set_style(style);
                }
            }
            let _ = ui.set_prop(test, "allowed_chars", c.allowed);
            let _ = ui.set_prop(test, "blocked_chars", c.blocked);
            let _ = ui.set_prop(test, "only_allowed", c.only_allowed);
            let _ = ui.set_prop(test, "char_map", c.map);
            if let Some(case) = c.case {
                let _ = ui.set_prop(test, "char_case", PropValue::Enum(case));
            }
        }
    };
    show(ui);
    // Clique num símbolo: neutro → liberado → bloqueado → neutro.
    for &(ch, b) in cells.iter() {
        let show = show.clone();
        ui.on_click(b, move |ui, _| {
            let (a, k) = (text_of(ui, allowed), text_of(ui, blocked));
            let filter = aylis_core::CharFilter { allowed: a.clone(), blocked: k.clone(), only_allowed: false, overrides: false, ..Default::default() };
            let (a, k) = match filter.opinion(ch) {
                None => (format!("{a}{}", set_char(ch)), k),
                Some(true) => (remove_set_char(&a, ch), format!("{k}{}", set_char(ch))),
                Some(false) => (a, remove_set_char(&k, ch)),
            };
            let _ = ui.set_prop(allowed, "text", a);
            let _ = ui.set_prop(blocked, "text", k);
            let _ = ui.set_prop(preset, "item_index", 0i64);
            show(ui);
        });
    }
    {
        let show = show.clone();
        ui.on(preset, "on_change", move |ui, sender, _| {
            let i = ui.get_prop(sender, "item_index").ok().and_then(|v| v.to_i64().ok()).unwrap_or(0).max(0) as usize;
            if let Some(&(_, a, k, o)) = CHARS_PRESETS.get(i).filter(|_| i > 0) {
                let _ = ui.set_prop(allowed, "text", a);
                let _ = ui.set_prop(blocked, "text", k);
                let _ = ui.set_prop(only, "checked", o);
                show(ui);
            }
        });
    }
    for c in [allowed, blocked, only, map].into_iter().chain(case) {
        let show = show.clone();
        ui.on(c, "on_change", move |ui, _, _| show(ui));
    }
    run(ui, d, |_| Ok(()), move |ui| {
        let c = current(ui);
        on_ok(ui, c);
    });
}

// ---------------------------------------------------------------------------
// Sombra
// ---------------------------------------------------------------------------

/// Sombras prontas: (nome, valor; vazio = sem sombra).
pub const SHADOW_PRESETS: [(&str, &str); 6] = [
    ("Nenhuma", ""),
    ("Suave", "0 1 3 #0000001F"),
    ("Média", "0 4 12 #00000033"),
    ("Elevada", "0 12 28 #00000047"),
    ("Contorno", "0 0 0 1 #00000026"),
    ("Brilho", "0 0 16 4 #2563EB80"),
];

/// O editor de sombra. `on_ok` recebe o valor (`"dx dy desfoque [espalhamento] #cor"`) ou
/// vazio (sem sombra).
pub fn shadow_editor(ui: &mut Ui, initial: &str, tokens: Vec<(String, Color)>, on_ok: impl FnOnce(&mut Ui, String) + 'static) {
    let d = new_dialog(ui, "Sombra", 600.0, 470.0);
    let start = Shadow::parse(initial).unwrap_or(Shadow { color: Color::BLACK.with_opacity(0.2), blur: 12.0, offset: aylis_core::PointF::new(0.0, 4.0), spread: 0.0 });
    let editor = ui.add(ShadowEditor::new(start), d.body);
    ui.control_mut(editor).unwrap().set_align(Align::Top).set_height(206.0).set_margins([0.0, 0.0, 0.0, 10.0]);

    let line = field(ui, d.body, "Cor");
    let more = ui.add(Button::new("…"), line);
    ui.control_mut(more).unwrap().set_align(Align::Right).set_width(34.0).set_margins([6.0, 0.0, 0.0, 0.0]).set_style("flat");
    let swatch = ui.add(Rectangle::new(34.0, 0.0), line);
    ui.control_mut(swatch).unwrap().set_align(Align::Right).set_margins([6.0, 3.0, 0.0, 3.0]);
    let _ = ui.set_prop(swatch, "radius", 6);
    let border = ui.theme().color("border");
    let _ = ui.set_prop(swatch, "stroke", border);
    let _ = ui.set_prop(swatch, "fill", start.color);
    let color = edit(ui, line, &start.color.to_hex());

    let line = field(ui, d.body, "Valor");
    let spec = edit(ui, line, if initial.trim().is_empty() { "" } else { initial.trim() });
    hint(ui, d.body, "Arraste o ponto no quadro (Shift: um eixo só). Vazio = sem sombra.");

    // Editor → textos.
    let show = move |ui: &mut Ui| {
        let Some(s) = ui.get::<ShadowEditor>(editor).map(|e| e.shadow()) else { return };
        // Sem disparar o `on_change` dos campos (senão o texto digitado voltava reescrito).
        let _ = ui.set_prop_quiet(spec, "text", s.to_spec());
        let _ = ui.set_prop_quiet(color, "text", s.color.to_hex());
        let _ = ui.set_prop(swatch, "fill", s.color);
    };
    ui.on(editor, "on_change", move |ui, _, _| show(ui));
    // Textos → editor.
    let apply = move |ui: &mut Ui, s: Shadow| {
        if let Some(e) = ui.get_mut::<ShadowEditor>(editor) {
            e.set_shadow(s);
        }
        let _ = ui.set_prop(swatch, "fill", s.color);
        ui.request_repaint();
    };
    ui.on(spec, "on_change", move |ui, _, _| {
        if let Some(s) = Shadow::parse(&text_of(ui, spec)) {
            apply(ui, s);
            let _ = ui.set_prop_quiet(color, "text", s.color.to_hex());
        }
    });
    let set_color = move |ui: &mut Ui, c: Color| {
        if let Some(e) = ui.get_mut::<ShadowEditor>(editor) {
            e.color = c;
        }
        show(ui);
        ui.request_repaint();
    };
    ui.on(color, "on_change", move |ui, _, _| {
        if let Some(c) = Color::from_hex(&text_of(ui, color)) {
            set_color(ui, c);
        }
    });
    ui.on_click(more, move |ui, _| {
        let current = text_of(ui, color);
        dialogs::color_picker(ui, &current, tokens.clone(), move |ui, picked| {
            let c = Color::from_hex(&picked).or_else(|| ui.theme().colors.get(&picked).copied());
            if let Some(c) = c {
                set_color(ui, c);
            }
        });
    });

    let names: Vec<&str> = SHADOW_PRESETS.iter().map(|(n, _)| *n).collect();
    for (b, (_, value)) in chips(ui, d.body, &names, 86.0).into_iter().zip(SHADOW_PRESETS) {
        ui.on_click(b, move |ui, _| match Shadow::parse(value) {
            Some(s) => {
                apply(ui, s);
                show(ui);
            }
            None => {
                let _ = ui.set_prop(spec, "text", "");
            }
        });
    }
    run(
        ui,
        d,
        move |ui| {
            let t = text_of(ui, spec);
            if t.is_empty() || Shadow::parse(&t).is_some() { Ok(()) } else { Err(format!("'{t}' não é uma sombra: use \"dx dy desfoque [espalhamento] #cor\"")) }
        },
        move |ui| {
            let t = text_of(ui, spec);
            on_ok(ui, t);
        },
    );
}

// ---------------------------------------------------------------------------
// Curva de suavização
// ---------------------------------------------------------------------------

/// Curvas prontas: (nome mostrado, valor gravado).
pub const EASING_PRESETS: [(&str, &str); 16] = [
    ("Linear", "linear"),
    ("Entrada (quad)", "in_quad"),
    ("Saída (quad)", "out_quad"),
    ("Entrada e saída (quad)", "in_out_quad"),
    ("Saída (cúbica)", "out_cubic"),
    ("Entrada e saída (cúbica)", "in_out_cubic"),
    ("Saída com recuo", "out_back"),
    ("CSS ease", "ease"),
    ("CSS ease-in", "ease-in"),
    ("CSS ease-out", "ease-out"),
    ("CSS ease-in-out", "ease-in-out"),
    ("Material 3: padrão", "cubic-bezier(0.2, 0, 0, 1)"),
    ("Material 3: enfático, entrada", "cubic-bezier(0.05, 0.7, 0.1, 1)"),
    ("Material 3: enfático, saída", "cubic-bezier(0.3, 0, 0.8, 0.15)"),
    ("Fluent: desacelerar", "cubic-bezier(0.1, 0.9, 0.2, 1)"),
    ("Mola (passa e volta)", "cubic-bezier(0.34, 1.56, 0.64, 1)"),
];

/// Miniatura da curva (só o desenho; o clique passa para quem está embaixo).
pub fn easing_thumb(ui: &mut Ui, parent: ComponentId, easing: Easing, align: Align, width: f32) -> ComponentId {
    let t = ui.add(EasingEditor::new(easing), parent);
    let _ = ui.set_prop(t, "compact", true);
    let _ = ui.set_prop(t, "preview", false);
    ui.control_mut(t).unwrap().set_align(align).set_width(width).set_margins([4.0, 3.0, 0.0, 3.0]).set_cursor(CursorShape::Pointer);
    t
}

/// O editor da curva: arrastar os pontos de controle ou escolher uma pronta. `on_ok`
/// recebe o nome (das prontas do aylis e do CSS) ou `cubic-bezier(...)`.
pub fn easing_editor(ui: &mut Ui, initial: &str, on_ok: impl FnOnce(&mut Ui, String) + 'static) {
    let d = new_dialog(ui, "Suavização", 720.0, 620.0);
    let start = Easing::parse(initial).unwrap_or_default();
    let line = field(ui, d.body, "Valor");
    ui.control_mut(line).unwrap().set_align(Align::Bottom).set_margins([0.0, 8.0, 0.0, 0.0]);
    let spec = edit(ui, line, &start.to_spec());
    let editor = ui.add(EasingEditor::new(start), d.body);
    ui.control_mut(editor).unwrap().set_align(Align::Left).set_width(320.0).set_margins([0.0, 0.0, 16.0, 0.0]);
    let side = ui.add(Layout::default(), d.body);
    ui.control_mut(side).unwrap().set_align(Align::Client);
    caption(ui, side, "PRONTAS");
    let list = ui.add(ScrollBox::default(), side);
    ui.control_mut(list).unwrap().set_align(Align::Client);

    ui.on(editor, "on_change", move |ui, _, args| {
        if let EventArgs::Text(t) = args {
            let _ = ui.set_prop(spec, "text", t.clone());
        }
    });
    let apply = move |ui: &mut Ui, e: Easing| {
        if let Some(ed) = ui.get_mut::<EasingEditor>(editor) {
            ed.set_easing(e);
        }
        ui.request_repaint();
    };
    ui.on(spec, "on_change", move |ui, _, _| {
        if let Some(e) = Easing::parse(&text_of(ui, spec)) {
            apply(ui, e);
        }
    });
    let hover = ui.theme().color("text").with_opacity(0.07);
    for (title, value) in EASING_PRESETS {
        let Some(e) = Easing::parse(value) else { continue };
        let row = ui.add(Rectangle::new(0.0, 34.0), list);
        ui.control_mut(row).unwrap().set_align(Align::Top).set_margins([0.0, 0.0, 0.0, 2.0]).set_cursor(CursorShape::Pointer);
        let _ = ui.set_prop(row, "radius", 6);
        let _ = ui.set_prop(row, "fill", Color::TRANSPARENT);
        ui.add(aylis_core::PropertyAnimation::color("fill", hover).with_trigger(aylis_core::Trigger::Hover).with_duration(100.0), row);
        let thumb = easing_thumb(ui, row, e, Align::Left, 40.0);
        ui.control_mut(thumb).unwrap().set_hit_test(false);
        let l = ui.add(Label::new(title), row);
        ui.control_mut(l).unwrap().set_align(Align::Client).set_margins([10.0, 0.0, 4.0, 0.0]).set_hit_test(false);
        ui.on_click(row, move |ui, _| {
            apply(ui, e);
            let _ = ui.set_prop(spec, "text", value);
        });
    }
    run(
        ui,
        d,
        move |ui| {
            let t = text_of(ui, spec);
            Easing::parse(&t).map(|_| ()).ok_or_else(|| format!("'{t}' não é uma curva: use um nome ou cubic-bezier(x1, y1, x2, y2)"))
        },
        move |ui| {
            let t = text_of(ui, spec);
            on_ok(ui, t);
        },
    );
}

// ---------------------------------------------------------------------------
// Tempos da animação
// ---------------------------------------------------------------------------

/// Os tempos de uma animação. Os opcionais só aparecem no editor quando o componente tem
/// a propriedade (um `Timer` só tem a duração).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Timing {
    pub duration: f64,
    pub delay: Option<f64>,
    pub easing: Option<String>,
    pub auto_reverse: Option<bool>,
    pub looping: Option<bool>,
}

/// A linha do tempo: arrastar o começo (atraso), o fim (duração) ou a barra, digitar os
/// tempos, escolher a curva e ligar ida e volta / repetir, vendo a animação rodar.
pub fn timing_editor(ui: &mut Ui, title: &str, initial: Timing, on_ok: impl FnOnce(&mut Ui, Timing) + 'static) {
    let d = new_dialog(ui, title, 680.0, 500.0);
    let timeline = ui.add(TimingEditor::default(), d.body);
    ui.control_mut(timeline).unwrap().set_align(Align::Top).set_height(180.0).set_margins([0.0, 0.0, 0.0, 12.0]);
    let easing0 = initial.easing.as_deref().and_then(Easing::parse).unwrap_or(Easing::Linear);
    if let Some(t) = ui.get_mut::<TimingEditor>(timeline) {
        t.duration = initial.duration as f32;
        t.delay = initial.delay.unwrap_or(0.0) as f32;
        t.easing = easing0;
        t.auto_reverse = initial.auto_reverse.unwrap_or(false);
        t.looping = initial.looping.unwrap_or(false);
    }
    let repaint = |ui: &mut Ui| ui.request_repaint();

    let line = field(ui, d.body, "Duração (ms)");
    let duration = edit(ui, line, &number(initial.duration));
    let delay = initial.delay.map(|v| {
        let line = field(ui, d.body, "Atraso (ms)");
        edit(ui, line, &number(v))
    });
    let easing = initial.easing.as_ref().map(|v| {
        let line = field(ui, d.body, "Suavização");
        let more = ui.add(Button::new("…"), line);
        ui.control_mut(more).unwrap().set_align(Align::Right).set_width(34.0).set_margins([6.0, 0.0, 0.0, 0.0]).set_style("flat");
        let thumb = easing_thumb(ui, line, easing0, Align::Right, 46.0);
        let e = edit(ui, line, v);
        (e, thumb, more)
    });
    let flags = ui.add(Layout::default(), d.body);
    ui.control_mut(flags).unwrap().set_align(Align::Top).set_height(30.0).set_margins([130.0, 0.0, 0.0, 6.0]);
    let check = |ui: &mut Ui, text: &str, on: Option<bool>| {
        on.map(|on| {
            let c = ui.add(CheckBox::new(text), flags);
            ui.control_mut(c).unwrap().set_align(Align::Left).set_width(150.0);
            let _ = ui.set_prop(c, "checked", on);
            c
        })
    };
    let reverse = check(ui, "Ida e volta", initial.auto_reverse);
    let looping = check(ui, "Repetir", initial.looping);
    hint(ui, d.body, "Arraste o começo da barra (atraso), o fim (duração) ou a barra toda. Shift: de 1 em 1 ms.");

    // Linha do tempo → campos.
    ui.on(timeline, "on_change", move |ui, _, _| {
        let Some((dur, del)) = ui.get::<TimingEditor>(timeline).map(|t| (t.duration, t.delay)) else { return };
        let _ = ui.set_prop(duration, "text", number(dur as f64));
        if let Some(e) = delay {
            let _ = ui.set_prop(e, "text", number(del as f64));
        }
    });
    // Campos → linha do tempo.
    ui.on(duration, "on_change", move |ui, _, _| {
        if let (Some(v), Some(t)) = (parse_number(&text_of(ui, duration)), ui.get_mut::<TimingEditor>(timeline)) {
            t.duration = v.max(0.0) as f32;
            repaint(ui);
        }
    });
    if let Some(e) = delay {
        ui.on(e, "on_change", move |ui, _, _| {
            if let (Some(v), Some(t)) = (parse_number(&text_of(ui, e)), ui.get_mut::<TimingEditor>(timeline)) {
                t.delay = v.max(0.0) as f32;
                repaint(ui);
            }
        });
    }
    if let Some((e, thumb, more)) = easing {
        let set = move |ui: &mut Ui, curve: Easing| {
            if let Some(t) = ui.get_mut::<TimingEditor>(timeline) {
                t.easing = curve;
            }
            if let Some(t) = ui.get_mut::<EasingEditor>(thumb) {
                t.set_easing(curve);
            }
            repaint(ui);
        };
        ui.on(e, "on_change", move |ui, _, _| {
            if let Some(curve) = Easing::parse(&text_of(ui, e)) {
                set(ui, curve);
            }
        });
        let open = move |ui: &mut Ui| {
            let current = text_of(ui, e);
            easing_editor(ui, &current, move |ui, value| {
                let _ = ui.set_prop(e, "text", value.clone());
                if let Some(curve) = Easing::parse(&value) {
                    set(ui, curve);
                }
            });
        };
        ui.on_click(more, move |ui, _| open(ui));
        ui.on_click(thumb, move |ui, _| open(ui));
    }
    for (c, prop) in [(reverse, "auto_reverse"), (looping, "looping")] {
        if let Some(c) = c {
            ui.on(c, "on_change", move |ui, _, args| {
                let on = matches!(args, EventArgs::Value(PropValue::Bool(true)));
                let _ = ui.set_prop(timeline, prop, on);
            });
        }
    }

    let read = Rc::new(RefCell::new(None::<Timing>));
    let collect = {
        let read = read.clone();
        move |ui: &mut Ui| -> Result<(), String> {
            let duration = parse_number(&text_of(ui, duration)).ok_or("informe a duração em ms")?;
            let delay = match delay {
                Some(e) => Some(parse_number(&text_of(ui, e)).ok_or("informe o atraso em ms")?),
                None => None,
            };
            let easing = match easing {
                Some((e, _, _)) => {
                    let t = text_of(ui, e);
                    Easing::parse(&t).ok_or(format!("'{t}' não é uma curva"))?;
                    Some(t)
                }
                None => None,
            };
            let checked = |c: Option<ComponentId>| c.map(|c| matches!(ui.get_prop(c, "checked"), Ok(PropValue::Bool(true))));
            *read.borrow_mut() = Some(Timing {
                duration: duration.max(0.0),
                delay: delay.map(|v| v.max(0.0)),
                easing,
                auto_reverse: checked(reverse),
                looping: checked(looping),
            });
            Ok(())
        }
    };
    run(ui, d, collect, move |ui| {
        if let Some(t) = read.borrow_mut().take() {
            on_ok(ui, t);
        }
    });
}
