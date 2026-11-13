//! "Projeto → Opções do projeto → Fontes" e "Projeto → Ícones…": os recursos offline da IDE (Google Fonts,
//! Material Symbols, Fluent) entram no projeto só no que ele usa.

use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;

use aylis_controls::{Button, CheckBox, ComboBox, Edit, Layout, Rectangle, ScrollBox};
use aylis_core::icons::IconPack;
use aylis_core::{Align, ComponentId, Ui};
use aylis_project::Project;
use aylis_project::assets::{self, FontEntry};

use crate::dialogs::{combo_text, field, new_dialog, run, text_of};
use crate::widgets::{caption, label, muted};

const SAMPLE: &str = "Árvores ágeis pulam sobre o cão 0123456789";

/// O que o diálogo de fontes decidiu.
pub struct FontsResult {
    /// Famílias (pastas) que o projeto passa a ter.
    pub folders: BTreeSet<String>,
    /// Fonte do projeto inteiro (vazio = a do tema).
    pub font: String,
    /// Famílias em uso a trocar em todo o projeto: (de, para).
    pub replacements: Vec<(String, String)>,
    /// As fontes nomeadas do projeto: (nome, família).
    pub names: Vec<(String, String)>,
}

/// Registra os arquivos de uma família do catálogo (para a prévia). Devolve o nome real.
fn register_entry(entry: &FontEntry) -> Option<String> {
    let dir = aylis_project::codegen::resources_dir().join("fonts").join(&entry.folder);
    // O arquivo "normal" primeiro (sem Italic).
    let mut files = entry.files.clone();
    files.sort_by_key(|f| f.contains("Italic"));
    let mut name = None;
    for f in files.iter().take(2) {
        if let Ok(data) = std::fs::read(dir.join(f)) {
            name = name.or(aylis_core::fonts::register(data));
        }
    }
    name
}

const THEME_FONT: &str = "(a do tema)";
/// No "trocar por": deixar como está.
const KEEP: &str = "(manter)";

/// O catálogo de fontes: marcar as que vão para o projeto, prévia e a fonte do projeto.
pub fn fonts(ui: &mut Ui, project: &Project, on_ok: impl FnOnce(&mut Ui, FontsResult) + 'static) {
    let d = new_dialog(ui, "Fontes do projeto", 1060.0, 820.0);
    let catalog = Rc::new(RefCell::new(assets::font_catalog()));
    let chosen: Rc<RefCell<BTreeSet<String>>> = Rc::new(RefCell::new(assets::project_fonts(project).into_iter().collect()));

    // ----- esquerda: catálogo -----
    let left = ui.add(Layout::default(), d.body);
    ui.control_mut(left).unwrap().set_align(Align::Left).set_width(430.0).set_margins([0.0, 0.0, 20.0, 0.0]);
    crate::widgets::section(ui, left, "CATÁLOGO OFFLINE (Google Fonts) · marque as que o app leva");
    let search = ui.add(Edit::new(""), left);
    ui.control_mut(search).unwrap().set_align(Align::Top).set_height(32.0).set_margins([0.0, 0.0, 0.0, 6.0]);
    let _ = ui.set_prop(search, "placeholder", "Procurar família…");
    let _ = ui.set_prop(search, "icon", "search");
    let download_line = ui.add(Layout::default(), left);
    ui.control_mut(download_line).unwrap().set_align(Align::Bottom).set_height(32.0).set_margins([0.0, 8.0, 0.0, 0.0]);
    let download = ui.add(Button::new("Baixar"), download_line);
    ui.control_mut(download).unwrap().set_align(Align::Right).set_width(90.0).set_margins([6.0, 0.0, 0.0, 0.0]).set_style("secondary");
    let to_download = ui.add(Edit::new(""), download_line);
    ui.control_mut(to_download).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(to_download, "placeholder", "Outra do Google Fonts (ex.: Fira Sans)");
    let list = crate::widgets::framed_list(ui, left);

    // ----- direita: prévia e fonte do projeto -----
    let right = ui.add(Layout::default(), d.body);
    ui.control_mut(right).unwrap().set_align(Align::Client);
    crate::widgets::section(ui, right, "FONTE DO PROJETO (troca a do tema; as monoespaçadas ficam)");
    let line = field(ui, right, "Família");
    let project_font = ui.add(ComboBox::new(&[THEME_FONT]), line);
    ui.control_mut(project_font).unwrap().set_align(Align::Client);
    // ----- fontes nomeadas (como as cores do projeto) -----
    let all_families: Vec<String> = {
        let mut all: Vec<String> = aylis_core::fonts::families();
        all.extend(project.fonts.values().cloned());
        all.extend(crate::host::system_font_families());
        all.sort_by_key(|f| f.to_lowercase());
        all.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
        all
    };
    crate::widgets::section(ui, right, "FONTES NOMEADAS · use o nome no font_family dos componentes; trocar a família aqui troca em todos");
    let names_box = ui.add(ScrollBox::default(), right);
    ui.control_mut(names_box).unwrap().set_align(Align::Top).set_height(122.0);
    let add_name = ui.add(Button::new("+ Nova fonte nomeada"), right);
    ui.control_mut(add_name).unwrap().set_align(Align::Top).set_height(30.0).set_margins([0.0, 4.0, 0.0, 8.0]).set_style("flat");
    // Cada linha: (linha, nome, família).
    let name_rows: Rc<RefCell<Vec<(aylis_core::ComponentId, aylis_core::ComponentId, aylis_core::ComponentId)>>> = Rc::new(RefCell::new(Vec::new()));
    let add_row: Rc<dyn Fn(&mut Ui, &str, &str)> = {
        let (rows, families) = (name_rows.clone(), all_families.clone());
        Rc::new(move |ui: &mut Ui, name: &str, family: &str| {
            let row = ui.add(Layout::default(), names_box);
            ui.control_mut(row).unwrap().set_align(Align::Top).set_height(36.0).set_margins([0.0, 0.0, 0.0, 4.0]);
            let remove = ui.add(Button::new("×"), row);
            ui.control_mut(remove).unwrap().set_align(Align::Right).set_width(32.0).set_margins([6.0, 0.0, 0.0, 0.0]).set_style("flat");
            let _ = ui.set_prop(remove, "hint", "Tirar este nome (os componentes que o usam voltam à fonte do tema)");
            let edit = ui.add(Edit::new(name), row);
            ui.control_mut(edit).unwrap().set_align(Align::Left).set_width(150.0).set_margins([0.0, 0.0, 8.0, 0.0]);
            let _ = ui.set_prop(edit, "placeholder", "nome (titulo)");
            let combo = ui.add(ComboBox::new(&[""]), row);
            ui.control_mut(combo).unwrap().set_align(Align::Client);
            let _ = ui.set_prop(combo, "items", families.join(";"));
            let index = families.iter().position(|f| f.eq_ignore_ascii_case(family)).map_or(-1, |i| i as i64);
            let _ = ui.set_prop(combo, "item_index", index);
            rows.borrow_mut().push((row, edit, combo));
            let rows = rows.clone();
            ui.on_click(remove, move |ui, _| {
                rows.borrow_mut().retain(|(r, _, _)| *r != row);
                ui.free(row);
            });
        })
    };
    for (name, family) in &project.fonts {
        add_row(ui, name, family);
    }
    {
        let add_row = add_row.clone();
        ui.on_click(add_name, move |ui, _| add_row(ui, "", ""));
    }

    // ----- em uso no projeto: cada família com onde aparece e "trocar por" -----
    let in_use = assets::fonts_in_use(project);
    crate::widgets::section(ui, right, &format!("EM USO NO PROJETO ({}) · troque uma família em todos os lugares de uma vez", in_use.len()));
    let uses_box = ui.add(ScrollBox::default(), right);
    let rows_height = (in_use.len().max(1) as f32 * 40.0).min(200.0);
    ui.control_mut(uses_box).unwrap().set_align(Align::Top).set_height(rows_height).set_margins([0.0, 0.0, 0.0, 8.0]);
    // As opções de troca: as do projeto (assets/fonts), as já usadas e as do sistema.
    let replace_options: Vec<String> = {
        let mut all: Vec<String> = aylis_core::fonts::families();
        all.extend(in_use.iter().map(|u| u.family.clone()).filter(|f| !f.is_empty() && f != "monospace" && f != "sans-serif" && f != "serif"));
        all.extend(crate::host::system_font_families());
        all.sort_by_key(|f| f.to_lowercase());
        all.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
        std::iter::once(KEEP.to_string()).chain(all).collect()
    };
    let mut replace_combos: Vec<(String, aylis_core::ComponentId)> = Vec::new();
    let mut name_buttons: Vec<(aylis_core::ComponentId, String)> = Vec::new();
    if in_use.is_empty() {
        let none = label(ui, uses_box, "Nenhuma família própria: os componentes usam a fonte do tema.", Align::Top, 30.0);
        muted(ui, none);
    }
    for usage in &in_use {
        let row = ui.add(Layout::default(), uses_box);
        ui.control_mut(row).unwrap().set_align(Align::Top).set_height(36.0).set_margins([0.0, 0.0, 0.0, 4.0]);
        let combo = ui.add(ComboBox::new(&[KEEP]), row);
        ui.control_mut(combo).unwrap().set_align(Align::Right).set_width(220.0);
        let _ = ui.set_prop(combo, "items", replace_options.join(";"));
        let _ = ui.set_prop(combo, "item_index", 0i64);
        let arrow = label(ui, row, "→", Align::Right, 0.0);
        ui.control_mut(arrow).unwrap().set_width(24.0);
        let name = ui.add(Button::new(&usage.label), row);
        ui.control_mut(name).unwrap().set_align(Align::Left).set_width(170.0).set_style("flat");
        let _ = ui.set_prop(name, "font_family", usage.family.as_str());
        // Cada lugar traduzido ("tema Aylis Dark" pelo modelo "tema {}") antes de entrar na frase.
        let shown: Vec<String> = usage.places.iter().map(|p| ui.tr(p)).collect();
        let places = if shown.len() <= 2 { shown.join(", ") } else { format!("{}, {} e mais {}", shown[0], shown[1], shown.len() - 2) };
        let where_ = label(ui, row, &format!("{} lugar(es): {places}", usage.places.len()), Align::Client, 0.0);
        muted(ui, where_);
        let _ = ui.set_prop(where_, "font_size", 11i64);
        // A monoespaçada do tema só muda num tema próprio (a fonte do projeto não troca ela).
        if assets::is_monospace(&usage.family) && usage.places.iter().all(|p| p.starts_with("tema ")) {
            let _ = ui.set_prop(combo, "enabled", false);
            let _ = ui.set_prop(combo, "hint", "A monoespaçada do tema muda num tema próprio (Projeto → Opções do projeto → Aparência)");
        }
        let _ = ui.set_prop(where_, "trimming", "character");
        let _ = ui.set_prop(where_, "hint", usage.places.join("\n"));
        replace_combos.push((usage.family.clone(), combo));
        name_buttons.push((name, usage.family.clone()));
    }

    crate::widgets::section(ui, right, "PRÉVIA");
    let preview = ui.add(Rectangle::default(), right);
    ui.control_mut(preview).unwrap().set_align(Align::Client).set_padding(16.0);
    let _ = ui.set_prop(preview, "radius", 8);
    let title = label(ui, preview, "", Align::Top, 30.0);
    let info = label(ui, preview, "", Align::Top, 22.0);
    muted(ui, info);
    let mut samples = Vec::new();
    for (size, bold) in [(13.0, false), (16.0, false), (22.0, false), (22.0, true), (34.0, false)] {
        let l = label(ui, preview, SAMPLE, Align::Top, size * 1.6);
        let _ = ui.set_prop(l, "font_size", size as i64);
        let _ = ui.set_prop(l, "bold", bold);
        let _ = ui.set_prop(l, "trimming", "character");
        samples.push(l);
    }

    let show = {
        let samples = samples.clone();
        move |ui: &mut Ui, entry: &FontEntry| {
            let family = register_entry(entry).unwrap_or_else(|| entry.family.clone());
            let _ = ui.set_prop(title, "text", entry.family.as_str());
            let _ = ui.set_prop(title, "font_family", family.as_str());
            let _ = ui.set_prop(title, "font_size", 20i64);
            let category = entry.category.to_lowercase().replace('_', " ");
            let _ = ui.set_prop(info, "text", format!("{} · {} · {} arquivo(s)", entry.license, category, entry.files.len()));
            for l in &samples {
                let _ = ui.set_prop(*l, "font_family", family.as_str());
            }
        }
    };
    let show = Rc::new(show);
    // Tocar numa família em uso: a prévia dela (do projeto ou do sistema).
    for (button, family) in name_buttons {
        let samples = samples.clone();
        ui.on_click(button, move |ui, _| {
            let _ = ui.set_prop(title, "text", assets::font_label(&family));
            let _ = ui.set_prop(title, "font_family", family.as_str());
            let _ = ui.set_prop(info, "text", "em uso no projeto");
            for l in &samples {
                let _ = ui.set_prop(*l, "font_family", family.as_str());
            }
        });
    }

    // O combo da fonte do projeto: as famílias marcadas.
    let fill_font_combo = {
        let (catalog, chosen) = (catalog.clone(), chosen.clone());
        move |ui: &mut Ui, selected: &str| {
            let mut items = vec![THEME_FONT.to_string()];
            let catalog = catalog.borrow();
            for folder in chosen.borrow().iter() {
                let family = catalog.iter().find(|e| e.folder == *folder).map_or(folder.clone(), |e| e.family.clone());
                items.push(family);
            }
            let _ = ui.set_prop(project_font, "items", items.join(";"));
            let index = items.iter().position(|i| i == selected).unwrap_or(0);
            let _ = ui.set_prop(project_font, "item_index", index as i64);
        }
    };
    let fill_font_combo = Rc::new(fill_font_combo);
    fill_font_combo(ui, &project.font);

    // Quantos lugares usam cada família (para marcar no catálogo).
    let used: Rc<Vec<(String, usize)>> = Rc::new(in_use.iter().map(|u| (u.family.to_lowercase(), u.places.len())).collect());

    // As linhas do catálogo (filtradas pela busca), as famílias em uso primeiro.
    let rebuild: Rc<dyn Fn(&mut Ui)> = {
        let (catalog, chosen, show, fill, used) = (catalog.clone(), chosen.clone(), show.clone(), fill_font_combo.clone(), used.clone());
        Rc::new(move |ui: &mut Ui| {
            let children = ui.children(list).to_vec();
            ui.free_many(&children);
            let filter = text_of(ui, search).to_lowercase();
            let uses_of = |family: &str| used.iter().find(|(f, _)| *f == family.to_lowercase()).map(|(_, n)| *n);
            let mut entries: Vec<FontEntry> =
                catalog.borrow().iter().filter(|e| filter.is_empty() || e.family.to_lowercase().contains(&filter)).cloned().collect();
            entries.sort_by_key(|e| uses_of(&e.family).is_none());
            for entry in entries {
                let row = ui.add(Layout::default(), list);
                ui.control_mut(row).unwrap().set_align(Align::Top).set_height(40.0).set_margins([0.0, 0.0, 0.0, 2.0]);
                let check = ui.add(CheckBox::new(""), row);
                ui.control_mut(check).unwrap().set_align(Align::Left).set_width(30.0);
                let _ = ui.set_prop(check, "checked", chosen.borrow().contains(&entry.folder));
                // O nome alinhado à esquerda (um botão centralizava; pedido do autor em 08/11); o
                // clique mostra a prévia.
                let lic = label(ui, row, &entry.license, Align::Right, 110.0);
                muted(ui, lic);
                let _ = ui.set_prop(lic, "font_size", 11i64);
                // Só a falta: a família que os forms usam e o projeto não embute (decidido pelo
                // autor em 08/11; o "em uso · N" saiu). Some ao marcar.
                let missing = uses_of(&entry.family).map(|_| {
                    let badge = label(ui, row, "usada, mas não embutida", Align::Right, 150.0);
                    let warning = ui.theme().color("warning");
                    let _ = ui.set_prop(badge, "color", warning);
                    let _ = ui.set_prop(badge, "font_size", 11i64);
                    let _ = ui.set_prop(badge, "bold", true);
                    ui.control_mut(badge).unwrap().set_visible(!chosen.borrow().contains(&entry.folder));
                    badge
                });
                let see = label(ui, row, &entry.family, Align::Client, 0.0);
                ui.control_mut(see).unwrap().set_margins([6.0, 0.0, 0.0, 0.0]).set_cursor(aylis_core::CursorShape::Pointer);
                {
                    let (show, entry) = (show.clone(), entry.clone());
                    ui.on(see, "on_pointer_down", move |ui, _, _| show(ui, &entry));
                }
                let (chosen, fill, folder) = (chosen.clone(), fill.clone(), entry.folder.clone());
                ui.on(check, "on_change", move |ui, id, _| {
                    let on = matches!(ui.get_prop(id, "checked"), Ok(aylis_core::PropValue::Bool(true)));
                    if let Some(badge) = missing {
                        if let Some(cb) = ui.control_mut(badge) {
                            cb.set_visible(!on);
                        }
                    }
                    if on {
                        chosen.borrow_mut().insert(folder.clone());
                    } else {
                        chosen.borrow_mut().remove(&folder);
                    }
                    let current = combo_text(ui, project_font);
                    fill(ui, &current);
                });
            }
        })
    };
    rebuild(ui);
    if let Some(first) = catalog.borrow().first() {
        show(ui, first);
    }
    {
        let rebuild = rebuild.clone();
        ui.on(search, "on_change", move |ui, _, _| rebuild(ui));
    }
    {
        let (catalog, show) = (catalog.clone(), show.clone());
        let error = info;
        ui.on_click(download, move |ui, _| {
            let family = text_of(ui, to_download);
            if family.is_empty() {
                return;
            }
            match assets::download_google_font(&family) {
                Ok(entry) => {
                    *catalog.borrow_mut() = assets::font_catalog();
                    let _ = ui.set_prop(search, "text", entry.family.as_str());
                    rebuild(ui);
                    show(ui, &entry);
                }
                Err(e) => {
                    let _ = ui.set_prop(error, "text", format!("⚠ {e}"));
                }
            }
        });
    }

    run(ui, d, |_| Ok(()), move |ui| {
        // O primeiro item ("(a do tema)", "(manter)") pela posição: o texto vem traduzido na
        // tela (em inglês, "(the theme's)" virava o nome da família; achado em 07/11).
        let first = |ui: &Ui, combo: ComponentId| ui.get::<ComboBox>(combo).is_some_and(|c| c.item_index <= 0);
        let font = if first(ui, project_font) { String::new() } else { combo_text(ui, project_font) };
        let folders = chosen.borrow().clone();
        let replacements = replace_combos
            .iter()
            .filter_map(|(from, combo)| {
                let to = combo_text(ui, *combo);
                (!first(ui, *combo) && !to.is_empty() && to != *from).then(|| (from.clone(), to))
            })
            .collect();
        let names = name_rows
            .borrow()
            .iter()
            .map(|(_, edit, combo)| (text_of(ui, *edit).trim().to_string(), combo_text(ui, *combo)))
            .filter(|(n, f)| !n.is_empty() && !f.is_empty())
            .collect();
        on_ok(ui, FontsResult { folders, font, replacements, names });
    });
}

// ---------------------------------------------------------------------------

/// Um pacote aberto no diálogo de ícones.
struct OpenPack {
    prefix: String,
    pack: IconPack,
}

/// Quantos ícones a grade mostra de uma vez (a busca filtra o resto).
const GRID_LIMIT: usize = 240;
const CELL: f32 = 48.0;

/// O nome do pacote do aylis (os ícones embutidos, sem prefixo).
const BUILTIN: &str = "Aylis (embutidos)";

/// Navegar nos pacotes de ícones: buscar, ver preenchido, copiar o nome, usar no componente
/// selecionado e importar SVG para o pacote do app. `on_ok` recebe o ícone escolhido
/// (`"material:home"`), `Some("")` para tirar o ícone, ou `None`. Sem projeto não há o pacote
/// do app nem a importação de SVG. `initial` é o ícone atual (já vem escolhido).
pub fn icons(
    ui: &mut Ui,
    project: Option<&Project>,
    target: Option<String>,
    initial: &str,
    on_ok: impl FnOnce(&mut Ui, Option<String>) + 'static,
) {
    let title = match &target {
        Some(t) => format!("Ícones · OK usa no {t}"),
        None => "Ícones".to_string(),
    };
    let d = new_dialog(ui, &title, 980.0, 700.0);
    let mut packs = assets::icon_packs();
    let dir = project.map(Project::dir).unwrap_or_default();
    if project.is_some() && dir.join("assets/icons/app.txt").exists() {
        packs.insert(0, "app".to_string());
    }
    packs.push(BUILTIN.to_string());
    let open: Rc<RefCell<Option<OpenPack>>> = Rc::new(RefCell::new(None));
    let selected: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));

    let top = ui.add(Layout::default(), d.body);
    ui.control_mut(top).unwrap().set_align(Align::Top).set_height(34.0).set_margins([0.0, 0.0, 0.0, 8.0]);
    let refs: Vec<&str> = packs.iter().map(String::as_str).collect();
    let pack_combo = ui.add(ComboBox::new(&refs), top);
    ui.control_mut(pack_combo).unwrap().set_align(Align::Left).set_width(200.0).set_margins([0.0, 0.0, 8.0, 0.0]);
    // O pacote do ícone atual (`material:home`; sem prefixo, os do aylis).
    let initial_pack = match initial.split_once(':') {
        Some((pack, _)) => pack.to_string(),
        None if !initial.is_empty() => BUILTIN.to_string(),
        None => "material".to_string(),
    };
    let start = packs.iter().position(|p| *p == initial_pack).or_else(|| packs.iter().position(|p| p == "material")).unwrap_or(0);
    let _ = ui.set_prop(pack_combo, "item_index", start as i64);
    let fill = ui.add(CheckBox::new("Preenchido"), top);
    ui.control_mut(fill).unwrap().set_align(Align::Left).set_width(120.0);
    let import = ui.add(Button::new("Importar SVG…"), top);
    ui.control_mut(import).unwrap().set_align(Align::Right).set_width(130.0).set_margins([8.0, 0.0, 0.0, 0.0]).set_style("secondary");
    let search = ui.add(Edit::new(""), top);
    ui.control_mut(search).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(search, "placeholder", "Procurar ícone (inglês: home, person, arrow…)");
    let _ = ui.set_prop(search, "icon", "search");

    // Direita: o escolhido.
    let side = ui.add(Layout::default(), d.body);
    ui.control_mut(side).unwrap().set_align(Align::Right).set_width(260.0).set_margins([16.0, 0.0, 0.0, 0.0]);
    caption(ui, side, "ESCOLHIDO");
    let big = ui.add(aylis_controls::IconView::new(""), side);
    ui.control_mut(big).unwrap().set_align(Align::Top).set_height(120.0).set_margins([0.0, 8.0, 0.0, 8.0]);
    let name_label = label(ui, side, "", Align::Top, 24.0);
    let copy = ui.add(Button::new("Copiar nome"), side);
    ui.control_mut(copy).unwrap().set_align(Align::Top).set_height(32.0).set_margins([0.0, 8.0, 0.0, 0.0]).set_style("secondary");
    let clear = ui.add(Button::new("Sem ícone"), side);
    ui.control_mut(clear).unwrap().set_align(Align::Top).set_height(32.0).set_margins([0.0, 8.0, 0.0, 0.0]).set_style("secondary");
    let pack_info = label(ui, side, "", Align::Top, 60.0);
    muted(ui, pack_info);
    let _ = ui.set_prop(pack_info, "font_size", 11i64);
    let _ = ui.set_prop(pack_info, "wrap", true);
    let hint = label(
        ui,
        side,
        "O app leva só os ícones citados: \"Gerar código\" grava em assets/icons/ um pacote com eles.",
        Align::Top,
        80.0,
    );
    muted(ui, hint);
    let _ = ui.set_prop(hint, "font_size", 11i64);
    let _ = ui.set_prop(hint, "wrap", true);

    let count = label(ui, d.body, "", Align::Bottom, 22.0);
    muted(ui, count);
    let grid = ui.add(ScrollBox::default(), d.body);
    ui.control_mut(grid).unwrap().set_align(Align::Client);

    let choose = {
        let selected = selected.clone();
        Rc::new(move |ui: &mut Ui, name: &str| {
            *selected.borrow_mut() = Some(name.to_string());
            let _ = ui.set_prop(big, "icon", name);
            let _ = ui.set_prop(name_label, "text", name);
        })
    };

    // Abre o pacote do combo (o texto inteiro é lido uma vez por troca).
    let load = {
        let open = open.clone();
        move |ui: &mut Ui, project_dir: &std::path::Path| {
            let name = combo_text(ui, pack_combo);
            let text = if name == "app" {
                std::fs::read_to_string(project_dir.join("assets/icons/app.txt")).ok()
            } else if name == BUILTIN {
                None
            } else {
                assets::icon_pack_text(&name)
            };
            *open.borrow_mut() = text.and_then(|t| aylis_core::icons::parse_pack(&t).ok()).map(|pack| OpenPack { prefix: name.clone(), pack });
            let info = match &*open.borrow() {
                Some(p) => format!("{} · {} · {} ícones", p.pack.title, p.pack.license, p.pack.icons.len()),
                None => format!("Ícones do próprio Aylis · {} ícones", aylis_core::icons::builtin_names().len()),
            };
            let _ = ui.set_prop(pack_info, "text", info);
        }
    };

    let rebuild: Rc<dyn Fn(&mut Ui)> = {
        let (open, choose) = (open.clone(), choose.clone());
        Rc::new(move |ui: &mut Ui| {
            let children = ui.children(grid).to_vec();
            ui.free_many(&children);
            let filter = text_of(ui, search).to_lowercase();
            let filled = matches!(ui.get_prop(fill, "checked"), Ok(aylis_core::PropValue::Bool(true)));
            let open = open.borrow();
            // (nome mostrado, nome registrado)
            let mut names: Vec<(String, String)> = Vec::new();
            let mut total = 0;
            match &*open {
                Some(p) => {
                    let has_fill = p.pack.icons.iter().any(|(n, _)| n.ends_with("-fill"));
                    for (n, d) in &p.pack.icons {
                        let is_fill = n.ends_with("-fill");
                        if has_fill && is_fill != filled {
                            continue;
                        }
                        if !filter.is_empty() && !n.contains(&filter) {
                            continue;
                        }
                        total += 1;
                        if names.len() < GRID_LIMIT {
                            let full = format!("{}:{n}", p.prefix);
                            if ui.icon(&full).is_none() {
                                if let Some(icon) = p.pack.build(d) {
                                    ui.register_icon(&full, icon);
                                }
                            }
                            names.push((n.clone(), full));
                        }
                    }
                }
                None => {
                    for n in aylis_core::icons::builtin_names() {
                        if filter.is_empty() || n.contains(&filter) {
                            total += 1;
                            if names.len() < GRID_LIMIT {
                                names.push((n.to_string(), n.to_string()));
                            }
                        }
                    }
                }
            }
            let width = grid_width(ui, grid);
            let columns = ((width / CELL).floor() as usize).max(1);
            for (i, (shown, full)) in names.iter().enumerate() {
                let b = ui.add(Button::with_icon(full.as_str()), grid);
                let (x, y) = ((i % columns) as f32 * CELL, (i / columns) as f32 * CELL);
                ui.control_mut(b).unwrap().set_rect(x + 2.0, y + 2.0, CELL - 4.0, CELL - 4.0).set_style("icon");
                let _ = ui.set_prop(b, "icon_size", 24.0);
                let _ = ui.set_prop(b, "hint", shown.as_str());
                let (choose, full) = (choose.clone(), full.clone());
                ui.on_click(b, move |ui, _| choose(ui, &full));
            }
            let more = if total > names.len() { format!(" (mostrando {}; refine a busca)", names.len()) } else { String::new() };
            let _ = ui.set_prop(count, "text", format!("{total} ícone(s){more}"));
        })
    };

    load(ui, &dir);
    rebuild(ui);
    if !initial.is_empty() {
        choose(ui, initial);
    }
    {
        let (rebuild, dir) = (rebuild.clone(), dir.clone());
        ui.on(pack_combo, "on_change", move |ui, _, _| {
            load(ui, &dir);
            rebuild(ui);
        });
    }
    for id in [search, fill] {
        let rebuild = rebuild.clone();
        ui.on(id, "on_change", move |ui, _, _| rebuild(ui));
    }
    {
        let selected = selected.clone();
        ui.on_click(copy, move |ui, _| {
            if let Some(name) = selected.borrow().clone() {
                aylis_core::clipboard::set_text(ui, &name);
            }
        });
    }
    {
        let selected = selected.clone();
        ui.on_click(clear, move |ui, _| {
            *selected.borrow_mut() = Some(String::new());
            let _ = ui.set_prop(big, "icon", "");
            let _ = ui.set_prop(name_label, "text", "(sem ícone)");
        });
    }
    // Importar SVG grava no pacote do app: só com projeto.
    match project.cloned() {
        None => {
            let _ = ui.set_prop(import, "enabled", false);
            let _ = ui.set_prop(import, "hint", "Abra um projeto para importar SVG");
        }
        Some(project) => {
            let choose = choose.clone();
            ui.on_click(import, move |ui, _| {
                let Some(file) = crate::files::FileDialog::new().add_filter("SVG", &["svg"]).pick_file() else { return };
                let stem = file.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
                let result = std::fs::read_to_string(&file).map_err(|e| e.to_string()).and_then(|svg| assets::import_svg(&project, &stem, &svg));
                match result {
                    Ok(name) => {
                        if let Ok(text) = std::fs::read_to_string(project.dir().join("assets/icons/app.txt")) {
                            let _ = ui.register_icon_pack("app", &text);
                        }
                        choose(ui, &name);
                        let _ = ui.set_prop(pack_info, "text", format!("Importado: {name} (assets/icons/app.txt)"));
                    }
                    Err(e) => {
                        let _ = ui.set_prop(pack_info, "text", format!("⚠ {e}"));
                    }
                }
            });
        }
    }

    run(ui, d, |_| Ok(()), move |ui| {
        let chosen = selected.borrow().clone();
        on_ok(ui, chosen);
    });
}

/// Os ícones de pacote citados no form `form` (o arquivo dele) e nos forms que ele cita pelo
/// nome (frames e DataModules) ficam disponíveis no `ui` do designer (só eles; o pacote inteiro
/// tem milhares), e o pacote do app (`assets/icons/app.txt`) inteiro. Antes eram os citados no
/// projeto todo, lido a cada form aberto: 320 ms por form no ERP de 1009 forms (07/11).
pub fn register_used_icons(ui: &mut Ui, project: &Project, form: Option<&std::path::Path>) {
    let mut used: std::collections::BTreeMap<String, std::collections::BTreeSet<String>> = Default::default();
    if let Some(text) = form.and_then(|f| std::fs::read_to_string(f).ok()) {
        let cited = project.forms.iter().filter(|e| text.contains(&format!("\"{}\"", e.name))).map(|e| project.file(&e.file));
        let texts = std::iter::once(text.clone()).chain(cited.filter(|p| Some(p.as_path()) != form).filter_map(|p| std::fs::read_to_string(p).ok()));
        for t in texts {
            for (pack, names) in aylis_project::codegen::icons_cited_in(&t) {
                used.entry(pack).or_default().extend(names);
            }
        }
    }
    for (pack, names) in used {
        if names.iter().all(|n| ui.icon(&format!("{pack}:{n}")).is_some()) {
            continue;
        }
        let Some(parsed) = assets::icon_pack_text(&pack).and_then(|t| aylis_core::icons::parse_pack(&t).ok()) else { continue };
        for name in &names {
            if let Some(icon) = parsed.icon(name) {
                ui.register_icon(&format!("{pack}:{name}"), icon);
            }
        }
    }
    if let Ok(text) = std::fs::read_to_string(project.dir().join("assets/icons/app.txt")) {
        let _ = ui.register_icon_pack("app", &text);
    }
}

// ---------------------------------------------------------------------------

/// Os arquivos do projeto para o designer; o que ainda não foi copiado para
/// `assets/lottie/<pacote>/` vem dos recursos da IDE (a cópia acontece ao salvar).
pub struct IdeResources(pub std::path::PathBuf);

impl aylis_core::platform::ResourceService for IdeResources {
    fn read(&self, path: &str) -> Option<Vec<u8>> {
        if let Some(data) = aylis_project::DirResources(self.0.clone()).read(path) {
            return Some(data);
        }
        let rest = path.strip_prefix("assets/lottie/")?;
        aylis_project::DirResources(aylis_project::codegen::resources_dir().join("lottie")).read(rest)
    }
}

/// Um pacote de animações da IDE (`resources/lottie/<pacote>/catalog.json`).
pub struct AnimationPack {
    pub name: String,
    pub title: String,
    pub license: String,
    /// Nome e título de cada animação.
    pub animations: Vec<(String, String)>,
}

/// Os pacotes de animação da IDE.
pub fn animation_packs() -> Vec<AnimationPack> {
    let dir = aylis_project::codegen::resources_dir().join("lottie");
    let mut out = Vec::new();
    for e in std::fs::read_dir(&dir).into_iter().flatten().flatten().filter(|e| e.path().is_dir()) {
        let Ok(text) = std::fs::read_to_string(e.path().join("catalog.json")) else { continue };
        let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
        let animations = v["animations"]
            .as_array()
            .map(|a| a.iter().filter_map(|x| Some((x["name"].as_str()?.to_string(), x["title"].as_str().unwrap_or("").to_string()))).collect())
            .unwrap_or_default();
        out.push(AnimationPack {
            name: e.file_name().to_string_lossy().to_string(),
            title: v["title"].as_str().unwrap_or("").to_string(),
            license: v["license"].as_str().unwrap_or("").to_string(),
            animations,
        });
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

/// Uma animação de um pacote da IDE.
pub fn load_animation(pack: &str, name: &str) -> Option<aylis_vector::Animation> {
    let file = aylis_project::codegen::resources_dir().join("lottie").join(pack).join(format!("{name}.json"));
    aylis_vector::Animation::parse(&std::fs::read(file).ok()?).ok()
}

const ANIM_CELL: f32 = 92.0;

type AnimCache = Rc<RefCell<std::collections::HashMap<String, aylis_vector::Animation>>>;

fn cached(cache: &AnimCache, pack: &str, name: &str) -> Option<aylis_vector::Animation> {
    let key = format!("{pack}:{name}");
    if let Some(a) = cache.borrow().get(&key) {
        return Some(a.clone());
    }
    let anim = load_animation(pack, name)?;
    cache.borrow_mut().insert(key, anim.clone());
    Some(anim)
}

/// "Projeto → Animações": as animações dos pacotes tocando; escolher uma põe no `Lottie`
/// selecionado (ou só copia o nome, `"noto:fogo"`).
pub fn animations(ui: &mut Ui, target: Option<String>, on_ok: impl FnOnce(&mut Ui, Option<String>) + 'static) {
    let title = match &target {
        Some(t) => format!("Animações · OK usa no {t}"),
        None => "Animações".to_string(),
    };
    let d = new_dialog(ui, &title, 980.0, 700.0);
    let packs = Rc::new(animation_packs());
    let selected: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));

    let top = ui.add(Layout::default(), d.body);
    ui.control_mut(top).unwrap().set_align(Align::Top).set_height(34.0).set_margins([0.0, 0.0, 0.0, 8.0]);
    let names: Vec<&str> = packs.iter().map(|p| p.name.as_str()).collect();
    let pack_combo = ui.add(ComboBox::new(&names), top);
    ui.control_mut(pack_combo).unwrap().set_align(Align::Left).set_width(200.0).set_margins([0.0, 0.0, 8.0, 0.0]);
    let _ = ui.set_prop(pack_combo, "item_index", 0i64);
    let search = ui.add(Edit::new(""), top);
    ui.control_mut(search).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(search, "placeholder", "Procurar animação…");
    let _ = ui.set_prop(search, "icon", "search");

    let side = ui.add(Layout::default(), d.body);
    ui.control_mut(side).unwrap().set_align(Align::Right).set_width(260.0).set_margins([16.0, 0.0, 0.0, 0.0]);
    caption(ui, side, "ESCOLHIDA");
    let big = ui.add(aylis_vector::Lottie::default(), side);
    ui.control_mut(big).unwrap().set_align(Align::Top).set_height(180.0).set_margins([0.0, 8.0, 0.0, 8.0]);
    let name_label = label(ui, side, "", Align::Top, 24.0);
    let copy = ui.add(Button::new("Copiar nome"), side);
    ui.control_mut(copy).unwrap().set_align(Align::Top).set_height(32.0).set_margins([0.0, 8.0, 0.0, 0.0]).set_style("secondary");
    let credit = label(ui, side, "", Align::Top, 70.0);
    muted(ui, credit);
    let _ = ui.set_prop(credit, "font_size", 11i64);
    let _ = ui.set_prop(credit, "wrap", true);
    let hint = label(ui, side, "O app leva só as animações citadas: ao salvar, elas vão para assets/lottie/ com a licença.", Align::Top, 60.0);
    muted(ui, hint);
    let _ = ui.set_prop(hint, "font_size", 11i64);
    let _ = ui.set_prop(hint, "wrap", true);

    let grid = ui.add(ScrollBox::default(), d.body);
    ui.control_mut(grid).unwrap().set_align(Align::Client);

    let cache: AnimCache = Rc::default();
    let choose = {
        let (selected, cache) = (selected.clone(), cache.clone());
        Rc::new(move |ui: &mut Ui, pack: &str, name: &str, title: &str| {
            *selected.borrow_mut() = Some(format!("{pack}:{name}"));
            let anim = cached(&cache, pack, name);
            if let Some(l) = ui.get_mut::<aylis_vector::Lottie>(big) {
                l.set_animation(anim);
            }
            let _ = ui.set_prop(name_label, "text", format!("{pack}:{name} · {title}"));
            ui.request_repaint();
        })
    };

    let rebuild: Rc<dyn Fn(&mut Ui)> = {
        let (choose, cache, packs) = (choose.clone(), cache.clone(), packs.clone());
        Rc::new(move |ui: &mut Ui| {
            let children = ui.children(grid).to_vec();
            ui.free_many(&children);
            let pack_name = combo_text(ui, pack_combo);
            let Some(pack) = packs.iter().find(|p| p.name == pack_name) else { return };
            let _ = ui.set_prop(credit, "text", format!("{} · {}", pack.title, pack.license));
            let filter = text_of(ui, search).to_lowercase();
            let width = grid_width(ui, grid);
            let columns = ((width / ANIM_CELL).floor() as usize).max(1);
            let shown = pack.animations.iter().filter(|(n, t)| filter.is_empty() || n.contains(&filter) || t.to_lowercase().contains(&filter));
            for (i, (name, title)) in shown.enumerate() {
                let (x, y) = ((i % columns) as f32 * ANIM_CELL, (i / columns) as f32 * ANIM_CELL);
                let b = ui.add(Button::new(""), grid);
                ui.control_mut(b).unwrap().set_rect(x + 2.0, y + 2.0, ANIM_CELL - 4.0, ANIM_CELL - 4.0).set_style("flat");
                let _ = ui.set_prop(b, "hint", format!("{}:{name} · {title}", pack.name));
                let l = ui.add(aylis_vector::Lottie::default(), b);
                ui.control_mut(l).unwrap().set_align(Align::Client).set_margins([8.0, 8.0, 8.0, 8.0]);
                let anim = cached(&cache, &pack.name, name);
                if let Some(lottie) = ui.get_mut::<aylis_vector::Lottie>(l) {
                    lottie.set_animation(anim);
                }
                let (choose, pack, name, title) = (choose.clone(), pack.name.clone(), name.clone(), title.clone());
                ui.on_click(b, move |ui, _| choose(ui, &pack, &name, &title));
            }
        })
    };
    rebuild(ui);
    for id in [pack_combo, search] {
        let rebuild = rebuild.clone();
        ui.on(id, "on_change", move |ui, _, _| rebuild(ui));
    }
    {
        let selected = selected.clone();
        ui.on_click(copy, move |ui, _| {
            if let Some(name) = selected.borrow().clone() {
                aylis_core::clipboard::set_text(ui, &name);
            }
        });
    }
    run(ui, d, |_| Ok(()), move |ui| {
        let chosen = selected.borrow().clone();
        on_ok(ui, chosen);
    });
}

/// Largura útil da grade dos diálogos (antes do primeiro layout ela ainda não tem
/// o tamanho padrão do ScrollBox: vale a do diálogo, 980 menos as bordas e a coluna da
/// direita).
fn grid_width(ui: &Ui, grid: aylis_core::ComponentId) -> f32 {
    ui.laid_out_bounds(grid).map(|b| b.width).filter(|w| *w > 300.0).unwrap_or(980.0 - 32.0 - 276.0) - 14.0
}
