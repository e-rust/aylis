//! Painéis da IDE refeitos a cada mudança: paleta, estrutura, projeto, abas e mensagens.

use aylis_controls::{IconView, Label, MenuItem, PopupMenu, Rectangle, ScrollBox, Tab, TabStrip, TreeNode, TreeView};
use aylis_core::{Align, Color, ComponentId, CursorShape, EventArgs, PointF, PropertyAnimation, Trigger, Ui};

use crate::actions;
use crate::app::{Ids, refresh};
use crate::state::Shared;
use crate::widgets::{caption, clear, label, muted, row, set_row_selected};

pub const CATEGORY_ORDER: [&str; 17] = ["Standard", "Additional", "Menus", "Layouts", "System", "Shapes", "Data", "Data Controls", "Reports", "BI", "Trader", "Industrial", "Electrical", "Media", "Quasar", "Animations", "Providers"];

/// Paleta de componentes. Uma classe com subtipos (`Button` → padrão, ícone, flutuante...)
/// tem uma seta que abre a lista deles; escolher um subtipo cria `Classe.variante`.
pub fn palette(ui: &mut Ui, state: &Shared, ids: &Ids) {
    // Nada mudou (a aba, a escolha, a busca, o que está aberto): a paleta fica como está.
    // Refazer as centenas de linhas a cada clique deixava a IDE lenta.
    let key = {
        use std::hash::{Hash, Hasher};
        let st = state.borrow();
        let mut h = std::collections::hash_map::DefaultHasher::new();
        let mut open: Vec<&String> = st.palette_open.iter().collect();
        open.sort();
        (&st.palette_tab, &st.pending, &st.palette_filter, open, st.doc.is_data_module(), st.doc.ui.registry().iter().count()).hash(&mut h);
        // Os pacotes do projeto.
        if let Some(project) = st.project.as_ref() {
            aylis_project::packages::used(&aylis_project::Files::Dir(project.dir())).iter().map(|m| (&m.name, &m.version)).for_each(|p| p.hash(&mut h));
        }
        // As cores das linhas vêm do tema da IDE.
        ui.theme().name.hash(&mut h);
        h.finish()
    };
    if state.borrow().palette_key == key && !ui.children(ids.palette).is_empty() {
        return;
    }
    state.borrow_mut().palette_key = key;
    clear(ui, ids.palette);
    let tab = state.borrow().palette_tab.clone();
    palette_tabs(ui, state, ids, &tab);
    if let Some((_, families)) = PLATFORM_TABS.iter().find(|(name, _)| *name == tab) {
        platform_palette(ui, state, ids, families);
        non_visual_section(ui, state, ids);
        return;
    }
    let (classes, pending, filter, open, variants) = {
        let st = state.borrow();
        let classes: Vec<(String, String)> = st
            .doc
            .ui
            .registry()
            .iter()
            // Os não visuais têm a seção deles no fim (com ícone).
            .filter(|c| c.name != "Form" && c.name != "DataModule" && st.doc.class_is_visual(c.name))
            .map(|c| (c.category.to_string(), c.name.to_string()))
            .collect();
        // Os pacotes que o projeto usa (`packages/`), na aba que o autor escolheu.
        let mut classes = classes;
        if let Some(project) = st.project.as_ref().filter(|_| !st.doc.is_data_module()) {
            for m in aylis_project::packages::used(&aylis_project::Files::Dir(project.dir())) {
                let category = if m.category.is_empty() { "Pacotes".to_string() } else { m.category.clone() };
                classes.push((category, format!("{}{}", aylis_project::packages::PREFIX, m.name)));
            }
        }
        let variants: std::collections::HashMap<String, Vec<(String, String)>> = classes
            .iter()
            .map(|(_, n)| (n.clone(), st.doc.ui.variants_of(n).into_iter().map(|v| (v.name, v.caption)).collect()))
            .collect();
        (classes, st.pending.clone(), st.palette_filter.to_lowercase(), st.palette_open.clone(), variants)
    };
    let mut categories: Vec<String> = CATEGORY_ORDER.iter().map(|s| s.to_string()).collect();
    for (category, _) in &classes {
        if !categories.contains(category) {
            categories.push(category.clone());
        }
    }
    for category in categories {
        // A busca também acha os subtipos ("ícone" mostra o Button aberto).
        let matches_variant = |n: &String| {
            variants[n].iter().any(|(name, caption)| name.to_lowercase().contains(&filter) || caption.to_lowercase().contains(&filter))
        };
        let mut names: Vec<&String> = classes
            .iter()
            .filter(|(c, n)| *c == category && (filter.is_empty() || n.to_lowercase().contains(&filter) || matches_variant(n)))
            .map(|(_, n)| n)
            .collect();
        if names.is_empty() {
            continue;
        }
        names.sort();
        if !category_header(ui, state, ids, palette_label(&category), 0.0, format!("closed:aylis/{category}"), !filter.is_empty()) {
            continue;
        }
        for name in names {
            let subtypes = &variants[name];
            let expanded = !subtypes.is_empty() && (open.contains(name) || (!filter.is_empty() && matches_variant(name)));
            let shown = name.strip_prefix(aylis_project::packages::PREFIX).unwrap_or(name);
            let r = row(ui, ids.palette, shown, 14.0, pending.as_deref() == Some(name.as_str()), 28.0);
            if !subtypes.is_empty() {
                toggle(ui, r, expanded, name.clone(), state, ids);
            }
            platform_icons(ui, r, name);
            if subtypes.is_empty() {
                pick(ui, r, name.clone(), state, ids);
            } else {
                pick_and_open(ui, r, name.clone(), state, ids);
            }
            if expanded {
                for (variant, caption) in subtypes {
                    let key = format!("{name}.{variant}");
                    let v = row(ui, ids.palette, caption, 32.0, pending.as_deref() == Some(key.as_str()), 26.0);
                    pick(ui, v, key, state, ids);
                }
            }
        }
    }
    non_visual_section(ui, state, ids);
}

/// As abas de plataforma da paleta e as versões de cada uma (as famílias de tema). A aba
/// "aylis" é a paleta por categoria, sem tema próprio.
pub const PLATFORM_TABS: [(&str, &[&str]); 4] = [
    ("Windows", &["WinUI 3", "Windows 11", "Windows 10", "Win32", "Windows 7", "Windows Phone"]),
    ("Apple", &["iOS 26", "iOS 13", "iOS 7"]),
    ("Android", &["Material 3", "Material 2", "Android 7"]),
    // Os frameworks visuais da web (os controles com a cara de cada um).
    ("Web", &aylis_core::themes::web::FAMILIES),
];

/// As abas Windows / Apple / Android / Web / aylis em cima da lista.
fn palette_tabs(ui: &mut Ui, state: &Shared, ids: &Ids, current: &str) {
    clear(ui, ids.palette_tabs);
    let names = PLATFORM_TABS.iter().map(|(n, _)| *n).chain(["aylis"]);
    for name in names {
        // A largura pelo texto: as cinco cabem na largura da paleta.
        let t = tab(ui, ids.palette_tabs, name, name == current, (name.chars().count() as f32 * 6.5 + 10.0).max(34.0));
        let (state, ids, name) = (state.clone(), *ids, name.to_string());
        ui.on_click(t, move |ui, _| {
            state.borrow_mut().palette_tab = name.clone();
            palette(ui, &state, &ids);
        });
    }
}

/// Uma aba (texto num retângulo, realçada quando `selected`), à esquerda em `parent`: as da
/// paleta e as da galeria.
pub fn tab(ui: &mut Ui, parent: ComponentId, text: &str, selected: bool, width: f32) -> ComponentId {
    let t = ui.add(Rectangle::new(0.0, 0.0), parent);
    ui.control_mut(t).unwrap().set_align(Align::Left).set_width(width).set_margins([0.0, 0.0, 2.0, 0.0]).set_cursor(CursorShape::Pointer);
    let _ = ui.set_prop(t, "radius", 5);
    let fill = if selected { ui.theme().color("primary").with_opacity(0.28) } else { Color::TRANSPARENT };
    let _ = ui.set_prop(t, "fill", fill);
    if !selected {
        let hover = ui.theme().color("text").with_opacity(0.07);
        ui.add(PropertyAnimation::color("fill", hover).with_trigger(Trigger::Hover).with_duration(100.0), t);
    }
    let l = ui.add(Label::new(text), t);
    ui.control_mut(l).unwrap().set_align(Align::Client).set_hit_test(false);
    let _ = ui.set_prop(l, "h_align", aylis_core::PropValue::Enum("center".into()));
    let _ = ui.set_prop(l, "font_size", 12);
    t
}

/// A paleta de uma plataforma: cada versão (Windows 11, iOS 26...) com os componentes
/// visuais; o escolhido aqui entra no form com o tema daquela versão (`Classe@Família`).
fn platform_palette(ui: &mut Ui, state: &Shared, ids: &Ids, families: &[&str]) {
    let (mut classes, pending, filter, open) = {
        let st = state.borrow();
        let classes: Vec<(String, String)> = st
            .doc
            .ui
            .registry()
            .iter()
            .filter(|c| c.name != "Form" && c.name != "DataModule" && st.doc.class_is_visual(c.name))
            .map(|c| (c.category.to_string(), c.name.to_string()))
            .collect();
        (classes, st.pending.clone(), st.palette_filter.to_lowercase(), st.palette_open.clone())
    };
    classes.retain(|(_, n)| filter.is_empty() || n.to_lowercase().contains(&filter));
    if classes.is_empty() {
        return;
    }
    // Por categoria, na ordem da aba aylis (as fora da lista no fim), e por nome dentro dela.
    let rank = |c: &str| CATEGORY_ORDER.iter().position(|o| *o == c).unwrap_or(CATEGORY_ORDER.len());
    classes.sort_by(|(ca, na), (cb, nb)| rank(ca).cmp(&rank(cb)).then(ca.cmp(cb)).then(na.cmp(nb)));
    for (i, &family) in families.iter().enumerate() {
        // A versão abre e fecha pelo clique no cabeçalho (na linha toda, não só na seta): só
        // a mais nova começa aberta (cada uma lista todos os visuais); a busca abre todas.
        let key = if i == 0 { format!("closed:{family}") } else { format!("open:{family}") };
        let expanded = (open.contains(&key) != (i == 0)) || !filter.is_empty();
        let header = row(ui, ids.palette, family, 0.0, false, 28.0);
        toggle(ui, header, expanded, key.clone(), state, ids);
        // A seta deixa o clique passar para a linha (senão abriria e fecharia de uma vez).
        for c in ui.children(header).to_vec() {
            if ui.class_name(c) == Some("Rectangle") {
                ui.control_mut(c).unwrap().set_hit_test(false);
            }
        }
        let (st, ids_copy) = (state.clone(), *ids);
        ui.on_click(header, move |ui, _| {
            {
                let mut s = st.borrow_mut();
                if !s.palette_open.remove(&key) {
                    s.palette_open.insert(key.clone());
                }
            }
            palette(ui, &st, &ids_copy);
        });
        if let Some(&label) = ui.children(header).iter().find(|&&c| ui.class_name(c) == Some("Label")) {
            let _ = ui.set_prop(label, "bold", true);
        }
        if !expanded {
            continue;
        }
        // Mais um nível: as categorias da versão, cada uma recolhível.
        let mut last: Option<&str> = None;
        let mut category_open = true;
        for (category, name) in &classes {
            if last != Some(category.as_str()) {
                category_open = category_header(ui, state, ids, category, 14.0, format!("closed:{family}/{category}"), !filter.is_empty());
                last = Some(category.as_str());
            }
            if !category_open {
                continue;
            }
            let key = format!("{name}@{family}");
            let r = row(ui, ids.palette, name, 28.0, pending.as_deref() == Some(key.as_str()), 26.0);
            platform_icons(ui, r, name);
            pick(ui, r, key, state, ids);
        }
    }
}

/// Os não visuais no fim da paleta (em todas as abas: não têm estilo de plataforma).
fn non_visual_section(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let (pending, filter) = {
        let st = state.borrow();
        (st.pending.clone(), st.palette_filter.to_lowercase())
    };
    // Não visuais: um ícone próprio para cada um, a descrição na dica.
    let non_visual: Vec<(String, String)> = crate::nonvisual::classes(state)
        .into_iter()
        .filter(|(n, _)| filter.is_empty() || n.to_lowercase().contains(&filter) || crate::nonvisual::description(n).to_lowercase().contains(&filter))
        .collect();
    if !non_visual.is_empty() {
        caption(ui, ids.palette, "NÃO VISUAIS");
        let color = ui.theme().color("primary");
        for (name, _) in non_visual {
            let r = row(ui, ids.palette, &name, 22.0, pending.as_deref() == Some(name.as_str()), 28.0);
            let icon = ui.add(IconView::new(crate::nonvisual::icon(&name).unwrap_or("plug")), r);
            ui.control_mut(icon).unwrap().set_align(Align::Left).set_width(16.0).set_margins([10.0, 6.0, 0.0, 6.0]);
            let _ = ui.set_prop(icon, "color", color);
            // Os que dependem de plataforma mostram onde funcionam, como os visuais.
            platform_icons(ui, r, &name);
            let platforms = match ui.get_prop(r, "hint") {
                Ok(aylis_core::PropValue::String(h)) if !h.is_empty() => format!(" · {h}"),
                _ => String::new(),
            };
            let _ = ui.set_prop(r, "hint", format!("{}{platforms}", crate::nonvisual::description(&name)));
            pick(ui, r, name, state, ids);
        }
    }
}

/// Os ícones das plataformas onde a classe funciona, à direita da linha (só nas que não
/// funcionam em todas). A dica lista os nomes.
fn platform_icons(ui: &mut Ui, row: ComponentId, class: &str) {
    let Some(platforms) = aylis_build::class_platforms(class) else { return };
    let color = ui.theme().color("text").with_opacity(0.55);
    let hint = ui.tr("Funciona em: {}").replace("{}", &platforms.iter().map(|p| p.name()).collect::<Vec<_>>().join(", "));
    // Da direita para a esquerda: a ordem de `Align::Right` empilha ao contrário.
    for p in platforms.iter().rev() {
        let icon = match p {
            aylis_build::Platform::Windows => "platform:desktop_windows",
            aylis_build::Platform::MacOs => "platform:laptop_mac",
            aylis_build::Platform::Linux => "platform:terminal",
            aylis_build::Platform::Android => "platform:android",
            aylis_build::Platform::Ios => "platform:ios",
            aylis_build::Platform::Web => "platform:language",
        };
        let i = ui.add(IconView::new(icon), row);
        ui.control_mut(i).unwrap().set_align(Align::Right).set_width(13.0).set_margins([1.0, 7.0, 1.0, 7.0]);
        let _ = ui.set_prop(i, "color", color);
    }
    let _ = ui.set_prop(row, "hint", hint);
}

/// Seta à direita da linha que abre e fecha os subtipos.
fn toggle(ui: &mut Ui, row: ComponentId, expanded: bool, class: String, state: &Shared, ids: &Ids) {
    let t = ui.add(Rectangle::new(26.0, 0.0), row);
    ui.control_mut(t).unwrap().set_align(Align::Right).set_cursor(CursorShape::Pointer);
    let _ = ui.set_prop(t, "radius", 5);
    let hover = ui.theme().color("text").with_opacity(0.1);
    ui.add(PropertyAnimation::color("fill", hover).with_trigger(Trigger::Hover).with_duration(100.0), t);
    let l = ui.add(Label::new(if expanded { "▾" } else { "▸" }), t);
    ui.control_mut(l).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(l, "h_align", aylis_core::PropValue::Enum("center".into()));
    let (state, ids) = (state.clone(), *ids);
    ui.on_click(t, move |ui, _| {
        {
            let mut st = state.borrow_mut();
            if !st.palette_open.remove(&class) {
                st.palette_open.insert(class.clone());
            }
        }
        palette(ui, &state, &ids);
    });
}

/// O cabeçalho de uma categoria (`Standard`, `Data`...): o nome menor e apagado, com a seta;
/// clicar na linha recolhe e abre (começa aberta; `key` é a marca de fechada). Com busca,
/// fica aberta. Devolve se está aberta.
/// O nome mostrado de uma categoria da paleta. "Data" é também, em português, o rótulo da
/// propriedade `date`, e a tradução trocava a categoria por "Date" em inglês: ela aparece como
/// "Dados" (traduzido "Data", "Datos"...). A chave (a ordem, o que está recolhido) fica "Data".
pub fn palette_label(category: &str) -> &str {
    match category {
        "Data" => "Dados",
        other => other,
    }
}

fn category_header(ui: &mut Ui, state: &Shared, ids: &Ids, text: &str, indent: f32, key: String, searching: bool) -> bool {
    let expanded = !state.borrow().palette_open.contains(&key) || searching;
    let r = row(ui, ids.palette, text, indent, false, 24.0);
    if let Some(&label) = ui.children(r).iter().find(|&&c| ui.class_name(c) == Some("Label")) {
        muted(ui, label);
        let _ = ui.set_prop(label, "font_size", 11);
    }
    let arrow = ui.add(Label::new(if expanded { "▾" } else { "▸" }), r);
    ui.control_mut(arrow).unwrap().set_align(Align::Right).set_width(26.0).set_hit_test(false);
    let _ = ui.set_prop(arrow, "h_align", aylis_core::PropValue::Enum("center".into()));
    muted(ui, arrow);
    let (state, ids) = (state.clone(), *ids);
    ui.on_click(r, move |ui, _| {
        {
            let mut st = state.borrow_mut();
            if !st.palette_open.remove(&key) {
                st.palette_open.insert(key.clone());
            }
        }
        palette(ui, &state, &ids);
    });
    expanded
}

/// Clique numa classe com subtipos: escolhe a classe e abre os subtipos embaixo dela (clicar
/// de novo desmarca e fecha).
fn pick_and_open(ui: &mut Ui, r: ComponentId, class: String, state: &Shared, ids: &Ids) {
    let (state, ids) = (state.clone(), *ids);
    ui.on_click(r, move |ui, _| {
        choose(&state, &class);
        {
            let mut st = state.borrow_mut();
            if st.pending.as_deref() == Some(class.as_str()) {
                st.palette_open.insert(class.clone());
            } else {
                st.palette_open.remove(&class);
            }
        }
        refresh(ui, &state, &ids);
    });
}

/// Clique numa linha da paleta: escolhe a classe (ou `Classe.variante`) para colocar no form.
fn pick(ui: &mut Ui, r: ComponentId, class: String, state: &Shared, ids: &Ids) {
    let (state, ids) = (state.clone(), *ids);
    ui.on_click(r, move |ui, _| {
        choose(&state, &class);
        refresh(ui, &state, &ids);
    });
}

/// Escolhe a classe na paleta (ou na galeria): uma visual espera o clique no form; uma não
/// visual entra direto. Escolher de novo a que já estava escolhida desmarca.
pub fn choose(state: &Shared, class: &str) {
    // Escolher um componente na tela inicial vai para o designer (o form vazio de partida).
    if crate::home::visible(state) {
        state.borrow_mut().home = false;
    }
    let mut st = state.borrow_mut();
    // `Button@Windows 11`: a classe com o tema da plataforma escolhida na paleta.
    let (base, look) = class.split_once('@').unwrap_or((class, ""));
    let shown = if look.is_empty() { base.to_string() } else { format!("{base} ({look})") };
    if st.pending.as_deref() == Some(class) {
        st.pending = None;
        st.status.clear();
    } else if st.doc.class_is_visual(base) && st.doc.is_data_module() {
        st.pending = None;
        st.status = "Um DataModule só aceita componentes não visuais".into();
    } else if st.doc.class_is_visual(base) {
        st.pending = Some(class.to_string());
        st.status = format!("Clique no form para colocar um {shown}");
    } else {
        st.pending = None;
        st.status = match st.doc.add(class, None) {
            Ok(id) => format!("{} adicionado", st.doc.ui.name(id).unwrap_or_default()),
            Err(e) => e,
        };
    }
}

thread_local! {
    /// A árvore que a Estrutura mostra agora (componente, nível e texto) e a linha de cada um:
    /// mudando só a seleção, as linhas ficam e só o destaque troca. Refazer as linhas a cada
    /// clique custava 3 ms em release e 300 ms em debug com os 231 componentes da Landing
    /// (relatado pelo autor em 07/11; `landing_selection_cost`).
    static STRUCTURE: std::cell::RefCell<(Vec<(ComponentId, usize, String)>, Vec<ComponentId>)> = const { std::cell::RefCell::new((Vec::new(), Vec::new())) };
}

thread_local! {
    /// Os componentes recolhidos no painel ESTRUTURA, por form (pedido do autor em 08/11): a seta
    /// (ou ←/→) recolhe e abre os filhos, e o form lembra ao voltar a ele.
    static COLLAPSED: std::cell::RefCell<std::collections::HashMap<String, std::collections::HashSet<String>>> = std::cell::RefCell::new(std::collections::HashMap::new());
}

/// A chave de um componente no recolher: o nome (o mesmo ao reabrir o form).
fn structure_key(doc: &crate::document::Document, id: ComponentId) -> String {
    doc.ui.name(id).filter(|n| !n.is_empty()).map_or_else(|| format!("#{}", id.to_u64()), str::to_string)
}

/// Recolhe (`Some(false)`), abre (`Some(true)`) ou alterna (`None`) os filhos de `id` no painel.
pub fn structure_toggle(state: &Shared, id: ComponentId, open: Option<bool>) {
    let st = state.borrow();
    let (form, key) = (st.doc.form_name(), structure_key(&st.doc, id));
    COLLAPSED.with(|c| {
        let mut c = c.borrow_mut();
        let set = c.entry(form).or_default();
        let collapsed = set.contains(&key);
        match open.unwrap_or(collapsed) {
            true => set.remove(&key),
            false => set.insert(key),
        };
    });
}

/// As setas no painel ESTRUTURA (com o foco nele), como numa árvore: ↑/↓ escolhem a linha de
/// cima ou de baixo; → abre os filhos (aberto, vai ao primeiro); ← recolhe (recolhido ou sem
/// filhos, vai ao pai). Devolve se a tecla era dele.
pub fn structure_arrow(ui: &mut Ui, state: &Shared, ids: &Ids, key: aylis_core::Key) -> bool {
    use aylis_core::Key;
    if !matches!(key, Key::Up | Key::Down | Key::Left | Key::Right) {
        return false;
    }
    let rows: Vec<(ComponentId, usize, String)> = STRUCTURE.with(|s| s.borrow().0.clone());
    let selected = state.borrow().doc.selection;
    let Some(i) = selected.and_then(|id| rows.iter().position(|(r, _, _)| *r == id)) else {
        if let Some((first, _, _)) = rows.first() {
            state.borrow_mut().doc.select(Some(*first));
            refresh(ui, state, ids);
        }
        return true;
    };
    let (id, depth, text) = rows[i].clone();
    let collapsed = text.starts_with("▸ ");
    let expanded = text.starts_with("▾ ");
    let choose = |target: Option<ComponentId>| {
        if let Some(t) = target {
            state.borrow_mut().doc.select(Some(t));
        }
    };
    match key {
        Key::Up => choose(i.checked_sub(1).map(|j| rows[j].0)),
        Key::Down => choose(rows.get(i + 1).map(|r| r.0)),
        Key::Right if collapsed => structure_toggle(state, id, Some(true)),
        Key::Right if expanded => choose(rows.get(i + 1).map(|r| r.0)),
        Key::Left if expanded => structure_toggle(state, id, Some(false)),
        Key::Left => choose(rows[..i].iter().rev().find(|(_, d, _)| *d < depth).map(|r| r.0)),
        _ => {}
    }
    refresh(ui, state, ids);
    true
}

pub fn structure(ui: &mut Ui, state: &Shared, ids: &Ids) {
    // (componente, nível, texto, escolhido, tem filhos)
    let rows: Vec<(ComponentId, usize, String, bool, bool)> = {
        let st = state.borrow();
        let tree = if st.no_form { Vec::new() } else { st.doc.tree() };
        let form = st.doc.form_name();
        let keys: Vec<String> = tree.iter().map(|(id, _)| structure_key(&st.doc, *id)).collect();
        // O escolhido dentro de um recolhido: os de cima dele abrem (senão a seleção some).
        COLLAPSED.with(|c| {
            let mut c = c.borrow_mut();
            let Some(set) = c.get_mut(&form) else { return };
            for (i, (id, depth)) in tree.iter().enumerate() {
                if !st.doc.is_selected(*id) {
                    continue;
                }
                let mut level = *depth;
                for j in (0..i).rev() {
                    if tree[j].1 < level {
                        set.remove(&keys[j]);
                        level = tree[j].1;
                    }
                }
            }
        });
        let collapsed = COLLAPSED.with(|c| c.borrow().get(&form).cloned().unwrap_or_default());
        let mut out = Vec::with_capacity(tree.len());
        let mut hide_below: Option<usize> = None;
        for (i, (id, depth)) in tree.iter().enumerate() {
            if hide_below.is_some_and(|d| *depth > d) {
                continue;
            }
            hide_below = None;
            let has_children = tree.get(i + 1).is_some_and(|(_, d)| d > depth);
            let closed = has_children && collapsed.contains(&keys[i]);
            if closed {
                hide_below = Some(*depth);
            }
            let name = st.doc.ui.name(*id).unwrap_or_default();
            let class = st.doc.ui.class_name(*id).unwrap_or_default();
            let arrow = match (has_children, closed) {
                (false, _) => "   ",
                (true, true) => "▸ ",
                (true, false) => "▾ ",
            };
            let text = if name.is_empty() { format!("{arrow}({class})") } else { format!("{arrow}{name}: {class}") };
            out.push((*id, *depth, text, st.doc.is_selected(*id), has_children));
        }
        out
    };
    // A mesma árvore, com as mesmas linhas ainda na tela: só a seleção muda.
    let shape: Vec<(ComponentId, usize, String)> = rows.iter().map(|(id, depth, text, _, _)| (*id, *depth, text.clone())).collect();
    let kept = STRUCTURE.with(|s| {
        let s = s.borrow();
        (s.0 == shape && ui.children(ids.structure) == s.1.as_slice()).then(|| s.1.clone())
    });
    if let Some(lines) = kept {
        let mut selected_row = None;
        for (r, (_, _, _, selected, _)) in lines.iter().zip(&rows) {
            set_row_selected(ui, *r, *selected);
            if *selected {
                selected_row = Some(*r);
            }
        }
        if let Some(r) = selected_row {
            ScrollBox::scroll_into_view(ui, ids.structure, r);
        }
        return;
    }
    clear(ui, ids.structure);
    let mut selected_row = None;
    let mut lines = Vec::with_capacity(rows.len());
    for (doc_id, depth, text, selected, has_children) in rows {
        let indent = depth as f32 * 14.0;
        let r = row(ui, ids.structure, &text, indent, selected, 26.0);
        lines.push(r);
        if selected {
            selected_row = Some(r);
        }
        let (state, ids) = (state.clone(), *ids);
        ui.on(r, "on_pointer_down", move |ui, _, args| {
            // A seta (o começo da linha, onde ela fica) recolhe e abre os filhos.
            if has_children && matches!(args, aylis_core::EventArgs::Pointer(p) if p.position.x < 10.0 + indent + 14.0) {
                structure_toggle(&state, doc_id, None);
                refresh(ui, &state, &ids);
                return;
            }
            let additive = matches!(args, aylis_core::EventArgs::Pointer(p) if p.modifiers.ctrl || p.modifiers.shift);
            // O painel fica com o foco: as setas andam nele (`structure_arrow`).
            ui.set_focus(ids.structure);
            {
                let mut st = state.borrow_mut();
                if additive {
                    st.doc.toggle(doc_id);
                } else {
                    st.doc.select(Some(doc_id));
                }
            }
            refresh(ui, &state, &ids);
        });
    }
    STRUCTURE.with(|s| *s.borrow_mut() = (shape, lines));
    ui.layout();
    if let Some(r) = selected_row {
        ScrollBox::scroll_into_view(ui, ids.structure, r);
    }
}

/// O gerenciador de projeto: os forms, quem é o principal e quem nasce no início.
pub fn project(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let project = state.borrow().project.clone();
    let Some(project) = project else {
        clear(ui, ids.project);
        // A lite do navegador: o form fica guardado no navegador e sai em ZIP.
        if !crate::lite::can(crate::lite::Capability::Disk) {
            let l = label(ui, ids.project, "No navegador, o form fica guardado aqui mesmo. O ZIP abre na IDE de desktop.", Align::Top, 56.0);
            let _ = ui.set_prop(l, "wrap", true);
            muted(ui, l);
            let r = row(ui, ids.project, "Baixar projeto (ZIP)", 0.0, false, 28.0);
            let (state, ids) = (state.clone(), *ids);
            ui.on_click(r, move |ui, _| {
                crate::lite::download_project(ui, &state, &ids);
                refresh(ui, &state, &ids);
            });
            return;
        }
        let recent = state.borrow().recent.clone();
        let l = label(ui, ids.project, "Nenhum projeto aberto.", Align::Top, 26.0);
        muted(ui, l);
        for (text, action) in [("Novo projeto…", 0), ("Abrir projeto…", 1)] {
            let r = row(ui, ids.project, text, 0.0, false, 28.0);
            let (state, ids) = (state.clone(), *ids);
            ui.on_click(r, move |ui, _| {
                if action == 0 {
                    actions::new_project(ui, &state, &ids);
                } else {
                    actions::open_project_dialog(&state, &ids);
                }
                refresh(ui, &state, &ids);
            });
        }
        if !recent.is_empty() {
            caption(ui, ids.project, "RECENTES");
            for path in recent {
                let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                let r = row(ui, ids.project, &name, 0.0, false, 26.0);
                let (state, ids) = (state.clone(), *ids);
                ui.on_click(r, move |ui, _| {
                    actions::open_recent(ui, &state, &ids, &path);
                    refresh(ui, &state, &ids);
                });
            }
        }
        return;
    };
    // O item atual: no código, o fonte da aba ativa (o de um form conta como o form); senão o
    // form no designer.
    let current = {
        let st = state.borrow();
        let file = st.code_view.then(|| crate::codewindow::active_location(ui).map(|(f, _)| f)).flatten();
        match file {
            Some(f) => {
                let form = project.forms.iter().find(|e| crate::debugger::norm(&aylis_project::codegen::entry_source(&project, e)) == f);
                match form {
                    Some(e) => format!("form:{}", e.name),
                    None => format!("file:{}", f.display()),
                }
            }
            None => format!("form:{}", st.doc.form_name()),
        }
    };
    let scan = project_scan(ui, state, ids, &project);
    let build = |st: &crate::state::State| {
        project_nodes(&project, &scan, &current, &st.project_tree_open)
    };
    let (nodes, selected) = build(&state.borrow());
    // A árvore começa fechada e fica como o usuário deixar (guardada por projeto; I10 do
    // Tabernáculo): o item atual não abre as pastas sozinho.
    if state.borrow().project_tree_current != current {
        state.borrow_mut().project_tree_current = current.clone();
    }
    let title_text = format!("{}  ·  {}", project.name, project.language.name());
    // A árvore já está lá com os mesmos nós: só marca o form aberto (refazer perderia a
    // rolagem e piscaria a cada mudança no form).
    let existing = ui.children(ids.project).iter().copied().find(|&c| ui.get::<TreeView>(c).is_some());
    let title_same = ui.children(ids.project).first().and_then(|&c| ui.get::<Label>(c)).is_some_and(|l| l.text == title_text);
    if let Some(tree) = existing.filter(|_| title_same) {
        if let Some(t) = ui.get_mut::<TreeView>(tree) {
            if t.nodes() == nodes.as_slice() {
                t.set_selected_path(selected);
                return;
            }
        }
    }
    clear(ui, ids.project);
    let title = label(ui, ids.project, &title_text, Align::Top, 26.0);
    let _ = ui.set_prop(title, "font_size", 14);
    let tree = ui.add(TreeView::default(), ids.project);
    ui.control_mut(tree).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(tree, "hint", "★ principal   ● criado no início   ⛁ DataModule   (botão direito: opções)");
    if let Some(t) = ui.get_mut::<TreeView>(tree) {
        t.item_height = 26.0;
        t.set_nodes(nodes);
        t.set_selected_path(selected);
    }
    // Clique: o form abre no designer (a variante, na aba dela); o fonte, no editor de código.
    {
        let (state, ids) = (state.clone(), *ids);
        ui.on(tree, "on_change", move |ui, sender, _| {
            let data = ui.get::<TreeView>(sender).and_then(|t| t.selected()).map(|n| n.data.clone()).unwrap_or_default();
            if let Some(form) = data.strip_prefix("form:") {
                actions::open_project_form(&state, form);
            } else if let Some((form, variant)) = data.strip_prefix("variant:").and_then(|v| v.split_once('|')) {
                actions::open_project_variant(&state, form, Some(variant));
            } else if let Some(file) = data.strip_prefix("file:") {
                if let Err(e) = crate::codewindow::open(ui, std::path::Path::new(file), 0) {
                    state.borrow_mut().status = format!("⚠ {e}");
                }
            } else {
                return;
            }
            refresh(ui, &state, &ids);
        });
    }
    // Abrir e fechar pastas fica guardado (o painel é refeito a cada mudança).
    for (event, open) in [("on_expand", true), ("on_collapse", false)] {
        let state = state.clone();
        ui.on(tree, event, move |ui, sender, _| {
            let Some(t) = ui.get::<TreeView>(sender) else { return };
            let Some(key) = t.last_toggled().and_then(|p| t.node(&p).map(|n| n.data.clone())) else { return };
            let mut st = state.borrow_mut();
            if open {
                st.project_tree_open.insert(key);
            } else {
                st.project_tree_open.remove(&key);
            }
            if let Some(project) = st.project.as_ref() {
                crate::actions::remember_project_tree(&project.path, &st.project_tree_open);
            }
        });
    }
    // O menu de contexto age sobre o form do clique direito.
    let menu = ui.add(PopupMenu::default(), ids.project);
    ui.control_mut(tree).unwrap().set_popup_menu(Some(menu));
    {
        let state = state.clone();
        ui.on(tree, "on_pointer_down", move |ui, sender, _| {
            let form = ui.get::<TreeView>(sender).and_then(|t| t.context_node()).and_then(|n| n.data.strip_prefix("form:").map(str::to_string));
            if form.is_some() {
                state.borrow_mut().context_form = form;
            }
        });
    }
    type FormAction = fn(&mut Ui, &Shared, &Ids, &str);
    let items: [(&str, FormAction); 6] = [
        ("Abrir", |_, s, _, f| actions::open_project_form(s, f)),
        ("Definir como principal", |_, s, _, f| actions::set_main_form(s, f)),
        ("Criar no início (liga/desliga)", |_, s, _, f| actions::toggle_auto_create(s, f)),
        ("Abrir código (F12)", |ui, s, _, f| {
            actions::open_project_form(s, f);
            actions::open_code(ui, s, None);
        }),
        ("Abrir pasta do projeto", |_, s, _, _| actions::open_project_folder(s)),
        ("Remover do projeto…", |ui, s, i, f| actions::remove_form(ui, s, i, f)),
    ];
    for (text, action) in items {
        let item = ui.add(MenuItem::new(text), menu);
        let (state, ids) = (state.clone(), *ids);
        ui.on_click(item, move |ui, _| {
            let form = state.borrow().context_form.clone();
            if let Some(form) = form {
                action(ui, &state, &ids, &form);
            }
            refresh(ui, &state, &ids);
        });
    }
}

/// Os nós do painel do projeto: os forms nas pastas deles (a de `forms/`), com as
/// variantes embaixo, e os fontes da linguagem do projeto nas pastas do código — sem os de
/// cada form (abrem pelo F12) nem os gerados pela IDE. Devolve também o caminho do form
/// atual (`current`: `form:nome` ou `file:caminho`), para marcar. `opened` são as pastas
/// abertas (as outras ficam recolhidas).
/// O que a árvore do projeto lê do disco: quais forms são DataModules (o JSON de cada um),
/// as variantes que existem, a unit de cada form e os fontes da linguagem. Ler tudo a cada
/// clique custava ~12 ms num projeto de 16 forms (mais numa pasta do OneDrive).
#[derive(Clone, Default, PartialEq)]
struct ProjectScan {
    modules: std::collections::HashSet<String>,
    variants: std::collections::HashMap<String, Vec<String>>,
    units: std::collections::HashMap<String, std::path::PathBuf>,
    sources: Vec<std::path::PathBuf>,
}

impl ProjectScan {
    fn read(project: &aylis_project::Project) -> Self {
        let mut scan = Self::default();
        // Cada pasta lida uma vez e a classe da raiz guardada (veja `FormFiles`): ler e
        // consultar o disco form a form pesava num projeto grande, a cada refresh.
        let mut files = aylis_project::FormFiles::default();
        for form in &project.forms {
            if form.data_module {
                scan.modules.insert(form.name.clone());
            }
            scan.variants.insert(form.name.clone(), project.entry_variants(form, &mut files));
            if let Some(unit) = Some(aylis_project::codegen::entry_source(project, form)).filter(|p| files.exists(p)) {
                scan.units.insert(form.name.clone(), unit);
            }
        }
        scan.sources = aylis_project::symbols::sources(project);
        scan
    }
}

thread_local! {
    /// A última leitura, por projeto (o `.aylisproj`), e se há outra em andamento.
    static SCAN: std::cell::RefCell<Option<(std::path::PathBuf, ProjectScan)>> = const { std::cell::RefCell::new(None) };
    static SCANNING: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    /// Quem redesenha o painel quando uma leitura em segundo plano traz novidade.
    static PANEL: std::cell::RefCell<Option<(Shared, Ids)>> = const { std::cell::RefCell::new(None) };
}

/// O disco do projeto para a árvore, sem esperar: a primeira vez lê na hora; depois devolve
/// a última leitura e lê de novo numa thread (no desktop), redesenhando o painel se algo
/// mudou (um form novo, um fonte criado fora da IDE).
fn project_scan(ui: &Ui, state: &Shared, ids: &Ids, project: &aylis_project::Project) -> ProjectScan {
    let key = project.path.clone();
    let cached = SCAN.with(|s| s.borrow().as_ref().filter(|(p, _)| *p == key).map(|(_, scan)| scan.clone()));
    let Some(cached) = cached else {
        let fresh = ProjectScan::read(project);
        SCAN.with(|s| *s.borrow_mut() = Some((key, fresh.clone())));
        return fresh;
    };
    // No navegador o "disco" é a memória: ler de novo não custa.
    if !crate::lite::can(crate::lite::Capability::Disk) {
        let fresh = ProjectScan::read(project);
        SCAN.with(|s| *s.borrow_mut() = Some((key, fresh.clone())));
        return fresh;
    }
    PANEL.with(|p| *p.borrow_mut() = Some((state.clone(), *ids)));
    if !SCANNING.with(|s| s.replace(true)) {
        let (project, poster) = (project.clone(), ui.poster());
        std::thread::spawn(move || {
            let fresh = ProjectScan::read(&project);
            poster.post(move |ui| {
                SCANNING.with(|s| s.set(false));
                let changed = SCAN.with(|s| {
                    let mut s = s.borrow_mut();
                    let changed = s.as_ref().is_some_and(|(p, old)| *p == project.path && *old != fresh);
                    if changed {
                        *s = Some((project.path.clone(), fresh));
                    }
                    changed
                });
                if changed {
                    if let Some((state, ids)) = PANEL.with(|p| p.borrow().clone()) {
                        self::project(ui, &state, &ids);
                    }
                }
            });
        });
    }
    cached
}

fn project_nodes(
    project: &aylis_project::Project,
    scan: &ProjectScan,
    current: &str,
    opened: &std::collections::HashSet<String>,
) -> (Vec<TreeNode>, Option<Vec<usize>>) {
    use std::path::{Path, PathBuf};
    // Uma pasta (criada se preciso) no caminho `dirs`, dentro de `nodes`.
    fn folder<'a>(nodes: &'a mut Vec<TreeNode>, dirs: &[String], key: &str, opened: &std::collections::HashSet<String>) -> &'a mut Vec<TreeNode> {
        let Some((first, rest)) = dirs.split_first() else { return nodes };
        let data = format!("{key}/{first}");
        let i = match nodes.iter().position(|n| n.data == data) {
            Some(i) => i,
            None => {
                let mut n = TreeNode::new(format!("📁 {first}"));
                n.data = data.clone();
                n.expanded = opened.contains(&data);
                // As pastas antes dos arquivos.
                let at = nodes.iter().position(|n| !n.data.starts_with("dir:")).unwrap_or(nodes.len());
                nodes.insert(at, n);
                at
            }
        };
        folder(&mut nodes[i].children, rest, &data, opened)
    }
    let mut roots = Vec::new();
    let mut active_path = None;

    // Os forms.
    let forms_key = "dir:forms".to_string();
    let mut forms_root = TreeNode::new("Forms");
    forms_root.data = forms_key.clone();
    forms_root.expanded = opened.contains(&forms_key);
    for form in &project.forms {
        let rel = form.file.replace('\\', "/");
        let rel = rel.strip_prefix("forms/").unwrap_or(&rel);
        let dirs: Vec<String> = rel.split('/').collect::<Vec<_>>().split_last().map(|(_, d)| d.iter().map(|s| s.to_string()).collect()).unwrap_or_default();
        let main = form.name == project.main_form;
        let module = scan.modules.contains(&form.name);
        let marks = format!("{}{}{}", if main { "★ " } else { "" }, if form.auto_create && !main { "● " } else { "" }, if module { "⛁ " } else { "" });
        // Só o nome: o "(fechado)" de quem não está aberto saiu a pedido do autor (13/11); o
        // aberto já aparece nas abas.
        let text = format!("{marks}{}", form.name);
        let mut node = TreeNode::new(text);
        node.data = format!("form:{}", form.name);
        for variant in scan.variants.get(&form.name).into_iter().flatten() {
            let mut v = TreeNode::new(format!("↳ {variant}"));
            v.data = format!("variant:{}|{variant}", form.name);
            node.children.push(v);
        }
        // O form com a unit dele (o código) e o JSON (o visual).
        if let Some(unit) = scan.units.get(&form.name) {
            let mut u = TreeNode::new(unit.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default());
            u.data = format!("file:{}", unit.display());
            node.children.push(u);
        }
        let json = project.dir().join(&form.file);
        let mut j = TreeNode::new(json.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default());
        j.data = format!("file:{}", json.display());
        node.children.push(j);
        node.expanded = opened.contains(&node.data);
        folder(&mut forms_root.children, &dirs, &forms_key, opened).push(node);
    }
    roots.push(forms_root);

    // Os fontes da linguagem por dono: o programa (o ponto de entrada), as units do usuário e
    // o que o aylis refaz a cada geração (recolhido, com cadeado). As units dos forms ficam
    // dentro de cada form; o cliente do aylis não aparece (os arquivos-ponte só apontam para o
    // da instalação).
    let code = project.code_dir();
    // Num conjunto, pelos componentes do caminho: comparar cada fonte com a lista inteira era
    // quadrático (o ERP de 1009 forms gastava 165 ms a cada refresh da IDE, e ela é refeita a
    // cada mensagem da compilação).
    let path_key = |p: &Path| p.components().map(|c| c.as_os_str().to_string_lossy().into_owned()).collect::<Vec<_>>().join("/");
    let mut skip: std::collections::HashSet<String> =
        project.forms.iter().map(|f| path_key(&aylis_project::codegen::entry_source(project, f))).collect();
    skip.extend(["src/aylis.ts", "src/aylis_client.py", "src/aylis.hpp", "src/aylis.pas"].iter().map(|f| path_key(&code.join(f))));
    let generated: Vec<PathBuf> = aylis_project::hostlang::generated_files(project.language).iter().map(|f| code.join(f)).chain([code.join(aylis_project::bindings::file(project.language))]).collect();
    let program: Vec<PathBuf> = aylis_project::hostlang::program_files(project.language).iter().map(|f| code.join(f)).collect();
    let same = |a: &Path, b: &Path| a.components().eq(b.components());
    let group = |key: &str, title: String| {
        let mut n = TreeNode::new(title);
        n.data = key.to_string();
        n.expanded = opened.contains(key);
        n
    };
    let mut program_root = group("dir:program", "Programa".to_string());
    let code_key = "dir:code".to_string();
    let mut code_root = group(&code_key, format!("Units ({})", project.language.name()));
    let generated_dir = code.join("generated");
    let mut generated_root = group("dir:generated", "Gerado pelo Aylis (somente leitura)".to_string());
    for file in scan.sources.iter().cloned() {
        let leaf = |file: &Path| {
            let mut node = TreeNode::new(file.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default());
            node.data = format!("file:{}", file.display());
            node
        };
        if skip.contains(&path_key(&file)) {
            continue;
        }
        // Os pedaços do acesso tipado (um por form e por classe) são internos da geração: a árvore
        // mostra o arquivo que os junta.
        if file.starts_with(generated_dir.join("components")) {
            continue;
        }
        if file.starts_with(&generated_dir) || generated.iter().any(|g| same(g, &file)) {
            generated_root.children.push(leaf(&file));
            continue;
        }
        if program.iter().any(|p| same(p, &file)) {
            program_root.children.push(leaf(&file));
            continue;
        }
        let rel = file.strip_prefix(&code).unwrap_or(&file).to_string_lossy().replace('\\', "/");
        let mut parts: Vec<String> = rel.split('/').map(str::to_string).collect();
        let name = parts.pop().unwrap_or_default();
        let mut node = leaf(&file);
        node.text = name;
        folder(&mut code_root.children, &parts, &code_key, opened).push(node);
    }
    if !program_root.children.is_empty() {
        roots.push(program_root);
    }
    if !code_root.children.is_empty() {
        roots.push(code_root);
    }
    if !generated_root.children.is_empty() {
        roots.push(generated_root);
    }

    // O caminho do item atual (só se as pastas dele estão abertas: senão não aparece).
    fn find(nodes: &[TreeNode], data: &str, path: &mut Vec<usize>) -> bool {
        for (i, n) in nodes.iter().enumerate() {
            path.push(i);
            if same_item(&n.data, data) || (n.expanded && find(&n.children, data, path)) {
                return true;
            }
            path.pop();
        }
        false
    }
    let mut path = Vec::new();
    if !current.is_empty() && find(&roots, current, &mut path) {
        active_path = Some(path);
    }
    (roots, active_path)
}

/// O mesmo item da árvore (`file:` compara os caminhos como a IDE compara).
fn same_item(a: &str, b: &str) -> bool {
    match (a.strip_prefix("file:"), b.strip_prefix("file:")) {
        (Some(x), Some(y)) => crate::debugger::norm(std::path::Path::new(x)) == crate::debugger::norm(std::path::Path::new(y)),
        _ => a == b,
    }
}

/// O que uma aba da faixa de cima mostra: um form (o designer e o código dele) ou um
/// arquivo de código que não é de nenhum form aberto.
#[derive(Clone)]
enum TabKind {
    Form(usize),
    Code(std::path::PathBuf),
}

impl TabKind {
    /// O que identifica a aba entre uma atualização e outra (para guardar a ordem que o
    /// usuário arrastou): o form pelo nome, o código pelo caminho.
    fn key(&self, name: &str) -> String {
        match self {
            TabKind::Form(_) => format!("form:{name}"),
            TabKind::Code(file) => format!("code:{}", file.display()),
        }
    }
}

thread_local! {
    /// O que é cada aba da faixa agora, na ordem dela (os handlers da faixa leem daqui).
    static KINDS: std::cell::RefCell<Vec<TabKind>> = const { std::cell::RefCell::new(Vec::new()) };
    /// A ordem que o usuário deixou arrastando (as chaves de [`TabKind::key`]); as abas
    /// novas vão para o fim.
    static ORDER: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// As abas em cima do designer e do código, juntas: cada form aberto é uma
/// aba só (o "Design | Código" embaixo alterna entre o form e o código dele), e os outros
/// arquivos de código abertos vêm depois, cada um na sua. É a `TabStrip` (as abas do
/// navegador): arrastar troca a ordem, com as vizinhas deslizando; o × e o clique do meio
/// fecham; a aba de código arrastada para fora vai para a janela de código.
pub fn tabs(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let strip = strip_of(ui, state, ids);
    let code_tabs = crate::codewindow::embedded_tabs(ui);
    let code_active = crate::codewindow::embedded_active(ui);
    let mut tabs: Vec<(String, bool, bool, TabKind, String)> = Vec::new();
    {
        let st = state.borrow();
        let code_view = st.code_view && !code_tabs.is_empty();
        let count = if st.no_form { 0 } else { st.tab_count() };
        let mut form_files = Vec::new();
        for i in 0..count {
            let Some(d) = st.tab(i) else { continue };
            let file = actions::code_file_of(&st, d).map(|f| crate::debugger::norm(&f));
            let code_modified = file.as_ref().is_some_and(|f| code_tabs.iter().any(|(t, m)| t == f && *m));
            let active = if code_view { file.is_some() && file == code_active } else { i == st.active };
            let hint = file.as_ref().map(|f| f.display().to_string()).unwrap_or_default();
            tabs.push((d.form_name(), d.modified || code_modified, active, TabKind::Form(i), hint));
            form_files.extend(file);
        }
        for (file, modified) in &code_tabs {
            if form_files.contains(file) {
                continue;
            }
            let name = file.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            let active = code_view && code_active.as_ref() == Some(file);
            tabs.push((name, *modified, active, TabKind::Code(file.clone()), file.display().to_string()));
        }
    }
    // A ordem arrastada pelo usuário vale; o que não está nela fica no fim, na ordem natural.
    // Por mapa, não procurando cada aba na lista (com os 1009 forms do ERP abertos, 1 milhão de
    // comparações a cada refresh).
    ORDER.with(|order| {
        let order = order.borrow();
        let position: std::collections::HashMap<&str, usize> = order.iter().enumerate().map(|(i, k)| (k.as_str(), i)).collect();
        tabs.sort_by_cached_key(|(name, _, _, kind, _)| position.get(kind.key(name).as_str()).copied().unwrap_or(usize::MAX));
    });
    let index = tabs.iter().position(|t| t.2).unwrap_or_else(|| ui.get::<TabStrip>(strip).map_or(0, |s| s.tab_index.max(0) as usize));
    let list = tabs.iter().map(|(name, modified, _, _, hint)| Tab { title: name.clone(), hint: hint.clone(), modified: *modified, ..Default::default() }).collect();
    KINDS.with(|k| *k.borrow_mut() = tabs.iter().map(|t| t.3.clone()).collect());
    let keys: Vec<String> = tabs.iter().map(|(name, _, _, kind, _)| kind.key(name)).collect();
    ORDER.with(|order| {
        let mut order = order.borrow_mut();
        let current: std::collections::HashSet<&String> = keys.iter().collect();
        order.retain(|k| current.contains(k));
        let mut known: std::collections::HashSet<String> = order.iter().cloned().collect();
        for k in keys.iter() {
            if known.insert(k.clone()) {
                order.push(k.clone());
            }
        }
    });
    if let Some(s) = ui.get_mut::<TabStrip>(strip) {
        s.set_tabs(list, index);
    }
    ui.request_repaint();
}

/// A faixa de abas (criada na primeira vez, com os handlers).
fn strip_of(ui: &mut Ui, state: &Shared, ids: &Ids) -> ComponentId {
    if let Some(s) = ui.children(ids.tabs).iter().copied().find(|&c| ui.get::<TabStrip>(c).is_some()) {
        return s;
    }
    clear(ui, ids.tabs);
    let mut s = TabStrip::new();
    s.show_add = false;
    s.auto_close = false;
    s.max_tab_width = 220.0;
    let strip = ui.add(s, ids.tabs);
    ui.control_mut(strip).unwrap().set_align(Align::Client);
    let kind_at = |args: &EventArgs| -> Option<TabKind> {
        let EventArgs::Value(v) = args else { return None };
        let i = v.to_i64().ok()?;
        KINDS.with(|k| k.borrow().get(i.max(0) as usize).cloned())
    };
    let (state2, ids2) = (state.clone(), *ids);
    ui.on(strip, "on_change", move |ui, _, args| {
        let Some(kind) = kind_at(args) else { return };
        match &kind {
            TabKind::Form(i) => {
                let code_view = state2.borrow().code_view && crate::codewindow::embedded_has_tabs(ui);
                state2.borrow_mut().switch_to(*i);
                // No código, a aba do form mostra o código dele (sem arquivo ainda: o form).
                if code_view {
                    actions::open_code(ui, &state2, None);
                    let file = {
                        let st = state2.borrow();
                        actions::code_file_of(&st, &st.doc).map(|f| crate::debugger::norm(&f))
                    };
                    if file.is_none() || file != crate::codewindow::embedded_active(ui) {
                        state2.borrow_mut().code_view = false;
                        ui.set_focus(ids2.surface);
                    }
                }
            }
            TabKind::Code(file) => {
                if let Err(e) = crate::codewindow::open(ui, file, 0) {
                    state2.borrow_mut().status = format!("⚠ {e}");
                }
            }
        }
        refresh(ui, &state2, &ids2);
    });
    let (state3, ids3) = (state.clone(), *ids);
    ui.on(strip, "on_close", move |ui, _, args| {
        let Some(kind) = kind_at(args) else { return };
        match &kind {
            TabKind::Form(i) => {
                // Fecha o form e, se não há nada por salvar, o código dele junto.
                let (form_modified, file) = {
                    let st = state3.borrow();
                    let doc = st.tab(*i);
                    (doc.is_some_and(|d| d.modified), doc.and_then(|d| actions::code_file_of(&st, d)).map(|f| crate::debugger::norm(&f)))
                };
                actions::close_tab(ui, &state3, &ids3, *i);
                if let Some(file) = file.filter(|_| !form_modified) {
                    if crate::codewindow::embedded_tabs(ui).iter().any(|(f, m)| *f == file && !m) {
                        crate::codewindow::close_embedded(ui, &file);
                    }
                }
            }
            TabKind::Code(file) => crate::codewindow::close_embedded(ui, file),
        }
        refresh(ui, &state3, &ids3);
    });
    // Arrastou: a ordem nova fica guardada (a faixa já trocou as abas de lugar).
    ui.on(strip, "on_reorder", move |ui, sender, args| {
        let EventArgs::Text(t) = args else { return };
        let mut parts = t.split(',').filter_map(|p| p.trim().parse::<usize>().ok());
        let (Some(from), Some(to)) = (parts.next(), parts.next()) else { return };
        let titles: Vec<String> = ui.get::<TabStrip>(sender).map(|s| s.tabs().iter().map(|t| t.title.clone()).collect()).unwrap_or_default();
        KINDS.with(|k| {
            let mut kinds = k.borrow_mut();
            if from < kinds.len() && to < kinds.len() {
                let kind = kinds.remove(from);
                kinds.insert(to, kind);
            }
            let keys: Vec<String> = kinds.iter().zip(&titles).map(|(kind, name)| kind.key(name)).collect();
            ORDER.with(|order| *order.borrow_mut() = keys);
        });
    });
    // A aba de código puxada para fora vai para a janela de código (como "Abrir em outra
    // janela"); a de um form fica.
    let state4 = state.clone();
    let ids4 = *ids;
    ui.on(strip, "on_tear_off", move |ui, _, args| {
        if let Some(TabKind::Code(file)) = kind_at(args) {
            if crate::codewindow::open(ui, &file, 0).is_ok() {
                crate::codewindow::detach_active(ui);
            }
            refresh(ui, &state4, &ids4);
        }
    });
    strip
}

/// Mensagens (compilação, saída do programa, publicação), na [`MessageList`](crate::messages::MessageList):
/// o duplo clique num erro com local abre o código na linha (e a pasta publicada).
pub fn messages(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let (show, version) = {
        let st = state.borrow();
        (st.show_messages, st.output_version)
    };
    if let Some(cb) = ui.control_mut(ids.messages_panel) {
        cb.set_visible(show);
    }
    if let Some(cb) = ui.control_mut(ids.messages_splitter) {
        cb.set_visible(show);
    }
    if !show || state.borrow().output_shown.get() == version {
        return;
    }
    state.borrow().output_shown.set(version);
    // Todas as linhas: a lista só desenha as que estão à vista.
    let lines = state.borrow().output.clone();
    if let Some(list) = ui.get_mut::<crate::messages::MessageList>(ids.messages) {
        list.set_lines(lines);
    }
    ui.request_repaint();
}

/// Posição (no form da IDE) para abrir um menu de contexto a partir de um evento.
pub fn popup_at(ui: &Ui, source: ComponentId, local: PointF) -> PointF {
    let origin = ui.absolute_rect(source).map_or(PointF::ZERO, |r| r.origin());
    PointF::new(origin.x + local.x, origin.y + local.y)
}
