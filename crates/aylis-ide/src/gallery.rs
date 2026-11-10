//! Galeria de componentes: uma janela com cada componente da paleta criado de verdade, no
//! tamanho padrão dele e com o tema do form (passe o mouse, clique, digite, abra a lista).
//! O botão "Usar" escolhe o componente, como clicar na paleta.
//!
//! Os componentes que mexem no sistema (bandeja, barra de tarefas, câmera, microfone...)
//! não são criados aqui: aparecem desenhados por um `Ui` à parte, em modo de design e sem
//! serviços do sistema (como no designer). Os controles de dados vêm ligados a um
//! ClientDataSet de exemplo.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use aylis_controls::{Button, Edit, Form, Label, Layout, Rectangle, ScrollBox};
use aylis_core::{
    Align, Brush, Canvas, Color, Component, ComponentId, ControlBase, ModalResult, PaintCtx, PropValue, PropertyAnimation,
    RectF, Trigger, Ui,
};
use aylis_data::{ClientDataSet, DataSource, Decimal, Field, FieldType, data_set};

use crate::app::{Ids, refresh};
use crate::dialogs::new_dialog;
use crate::document::new_document_ui;
use crate::panels::{CATEGORY_ORDER, PLATFORM_TABS, choose, tab};
use crate::state::Shared;
use crate::widgets::{clear, muted, row};

/// O tema do form aberto, registrado na IDE com este nome para os cartões.
const THEME: &str = "aylis-galeria";
/// Largura útil da lista (a janela menos margens e a barra de rolagem).
const LIST_WIDTH: f32 = 1040.0;
/// Margem em volta do componente dentro do cartão e altura da barra do título.
const PAD: f32 = 14.0;
const TITLE: f32 = 30.0;
const GAP: f32 = 10.0;
/// Tamanho de exemplo para quem não tem tamanho próprio (alinhado ao pai, zerado).
const SAMPLE: (f32, f32) = (240.0, 140.0);

/// Classes que não são criadas de verdade na galeria (mexem no sistema ou precisam de GPU):
/// as que dependem de plataforma e o 3D.
fn drawn_only(class: &str) -> bool {
    aylis_build::class_platforms(class).is_some() || class == "Model3D"
}

enum Kind {
    /// Criado de verdade no cartão.
    Live,
    /// Desenhado pelo `Ui` das prévias (o form dele).
    Drawn(ComponentId),
    NonVisual,
}

/// Uma classe (ou subtipo) da galeria.
struct Entry {
    /// O que vai para a paleta: `Button` ou `Button.icon`.
    class: String,
    /// A classe base (`Button`), para o texto inicial.
    base: String,
    /// O texto do cartão: `Button` ou `Button · Ícone`.
    title: String,
    category: String,
    kind: Kind,
    /// Tamanho do componente (o padrão dele).
    size: (f32, f32),
}

impl Entry {
    fn card_size(&self) -> (f32, f32) {
        match self.kind {
            Kind::NonVisual => (190.0, 96.0),
            _ => ((self.size.0 + 2.0 * PAD).max(220.0).min(LIST_WIDTH), self.size.1 + 2.0 * PAD + TITLE),
        }
    }
}

/// Abre a galeria; devolve o form dela.
pub fn open(ui: &mut Ui, state: &Shared, ids: &Ids) -> ComponentId {
    // A IDE só registra os controles básicos; a galeria cria de tudo.
    if ui.registry().get("DataGrid").is_none() {
        aylis_remote::register_all(ui);
    }
    let mut theme = (*state.borrow().doc.theme).clone();
    let background = theme.color("background");
    theme.name = THEME.into();
    ui.register_theme(theme);

    let (previews, entries) = build_entries(state);
    let d = new_dialog(ui, "Componentes", 1100.0, 760.0);
    // Só o "Fechar": escolher é o "Usar" de cada cartão.
    for child in ui.children(d.footer).to_vec() {
        if ui.class_name(child) == Some("Button") {
            ui.free(child);
        }
    }
    let close = ui.add(Button::new("Fechar"), d.footer);
    ui.control_mut(close).unwrap().set_align(Align::Right).set_width(110.0);
    let _ = ui.set_prop(close, "modal_result", "cancel");
    // As abas de plataforma (como as da paleta) e, numa plataforma, as versões dela.
    let tabs = ui.add(Layout::default(), d.body);
    ui.control_mut(tabs).unwrap().set_align(Align::Top).set_height(30.0).set_margins([0.0, 0.0, 0.0, 6.0]);
    let versions = ui.add(Layout::default(), d.body);
    ui.control_mut(versions).unwrap().set_align(Align::Top).set_height(30.0).set_margins([0.0, 0.0, 0.0, 8.0]);
    let search = ui.add(Edit::new(""), d.body);
    ui.control_mut(search).unwrap().set_align(Align::Top).set_height(32.0).set_margins([0.0, 0.0, 0.0, 10.0]);
    let _ = ui.set_prop(search, "placeholder", "Buscar componente");
    let list = ui.add(ScrollBox::default(), d.body);
    ui.control_mut(list).unwrap().set_align(Align::Client);
    let (dataset, data) = sample_data(ui, d.form);

    // Começa na aba em que a paleta está, na versão mais nova.
    let (tab, dark) = {
        let st = state.borrow();
        (st.palette_tab.clone(), st.doc.dark)
    };
    let family = PLATFORM_TABS.iter().find(|(n, _)| *n == tab).map_or(String::new(), |(_, f)| f[0].to_string());
    let view = Rc::new(RefCell::new(View { tab, family, filter: String::new(), dark, closed: Default::default() }));
    let entries = Rc::new(entries);
    let form = d.form;
    let ctx = Ctx { list, tabs, versions, dialog: form, background, data, previews, state: state.clone(), ids: *ids, view: view.clone() };
    // Refeito a cada troca de aba, de versão ou de busca. Os registros entram depois dos
    // cartões: os DB* se atualizam quando os dados chegam.
    let render: Rc<RefCell<Option<Rc<dyn Fn(&mut Ui)>>>> = Rc::default();
    let fill: Rc<dyn Fn(&mut Ui)> = {
        let render = render.clone();
        Rc::new(move |ui: &mut Ui| {
            header(ui, &ctx, &render);
            cards(ui, &ctx, &entries, &render);
            load_sample_rows(ui, dataset);
        })
    };
    *render.borrow_mut() = Some(fill.clone());
    fill(ui);
    {
        let fill = fill.clone();
        ui.on(search, "on_change", move |ui, _, args| {
            view.borrow_mut().filter = args.text().unwrap_or_default().to_lowercase();
            fill(ui);
        });
    }
    crate::lang::translate(ui);
    crate::dialogs::close_on_escape(ui, form);
    let _ = ui.show_modal(form, move |ui, _| {
        ui.free(form);
    });
    form
}

/// A aba, a versão (a família de tema; vazia na aba aylis) e a busca da galeria.
struct View {
    tab: String,
    family: String,
    filter: String,
    /// O form está escuro: a versão escura da família.
    dark: bool,
    /// As categorias recolhidas (o cabeçalho de cada uma abre e fecha).
    closed: std::collections::HashSet<String>,
}

impl View {
    /// O tema da versão escolhida pelo nome completo ("iOS 26 Light"); `None` na aba aylis.
    fn theme_name(&self) -> Option<String> {
        (!self.family.is_empty()).then(|| aylis_core::themes::of_family(&self.family, self.dark).map_or(self.family.clone(), |t| t.name.clone()))
    }

    /// O que o "Usar" escolhe: `Button` ou, numa plataforma, `Button@iOS 26`.
    fn key(&self, class: &str) -> String {
        if self.family.is_empty() { class.to_string() } else { format!("{class}@{}", self.family) }
    }
}

/// As abas de plataforma e, numa plataforma, as versões dela.
fn header(ui: &mut Ui, ctx: &Ctx, render: &Rc<RefCell<Option<Rc<dyn Fn(&mut Ui)>>>>) {
    clear(ui, ctx.tabs);
    clear(ui, ctx.versions);
    let (current, family) = {
        let v = ctx.view.borrow();
        (v.tab.clone(), v.family.clone())
    };
    let redo = |render: &Rc<RefCell<Option<Rc<dyn Fn(&mut Ui)>>>>, ui: &mut Ui| {
        let f = render.borrow().clone();
        if let Some(f) = f {
            f(ui);
        }
    };
    for name in PLATFORM_TABS.iter().map(|(n, _)| *n).chain(["aylis"]) {
        let t = tab(ui, ctx.tabs, name, name == current, if name == "aylis" { 54.0 } else { 80.0 });
        let (view, render, name) = (ctx.view.clone(), render.clone(), name.to_string());
        ui.on_click(t, move |ui, _| {
            {
                let mut v = view.borrow_mut();
                v.family = PLATFORM_TABS.iter().find(|(n, _)| *n == name).map_or(String::new(), |(_, f)| f[0].to_string());
                v.tab = name.clone();
            }
            redo(&render, ui);
        });
    }
    let families = PLATFORM_TABS.iter().find(|(n, _)| *n == current).map_or(&[][..], |(_, f)| *f);
    ui.control_mut(ctx.versions).unwrap().set_visible(!families.is_empty());
    for &f in families {
        let t = tab(ui, ctx.versions, f, f == family, 110.0);
        let (view, render, f) = (ctx.view.clone(), render.clone(), f.to_string());
        ui.on_click(t, move |ui, _| {
            view.borrow_mut().family = f.clone();
            redo(&render, ui);
        });
    }
}

/// O que os cartões precisam.
struct Ctx {
    list: ComponentId,
    tabs: ComponentId,
    versions: ComponentId,
    view: Rc<RefCell<View>>,
    dialog: ComponentId,
    /// O fundo do form (atrás de cada componente, como no form de verdade).
    background: Color,
    data: ComponentId,
    previews: Rc<RefCell<Ui>>,
    state: Shared,
    ids: Ids,
}

/// Um ClientDataSet de exemplo e o DataSource dele (os registros: `load_sample_rows`).
fn sample_data(ui: &mut Ui, form: ComponentId) -> (ComponentId, ComponentId) {
    let cds = ui.add(ClientDataSet::new().with_key("id"), form);
    ui.add(Field::new("id", FieldType::Integer).label("Código"), cds);
    ui.add(Field::new("nome", FieldType::String).label("Nome").size(40), cds);
    ui.add(Field::new("cidade", FieldType::String).label("Cidade").size(30), cds);
    ui.add(Field::new("valor", FieldType::Currency).label("Valor").format("#,##0.00"), cds);
    ui.add(Field::new("ativo", FieldType::Boolean).label("Ativo"), cds);
    ui.add(Field::new("obs", FieldType::Memo).label("Observação"), cds);
    let ds = ui.add(DataSource::new(cds), form);
    (cds, ds)
}

/// Alguns registros no ClientDataSet de exemplo (troca os que houver).
fn load_sample_rows(ui: &mut Ui, cds: ComponentId) {
    let rows = [
        ("Ana Souza", "São Paulo", 1_250_00, true, "Cliente desde 2019."),
        ("Bruno Lima", "Recife", 980_50, true, "Prefere contato por e-mail."),
        ("Carla Dias", "Curitiba", 15_300_00, false, "Pedido em análise."),
        ("Diego Alves", "Porto Alegre", 420_00, true, ""),
        ("Elisa Rocha", "Belo Horizonte", 7_890_90, true, "Desconto de 5%."),
        ("Fábio Nunes", "Salvador", 3_100_00, false, ""),
    ];
    let data = rows
        .iter()
        .enumerate()
        .map(|(i, (name, city, value, active, notes))| {
            vec![(i as i64 + 1).into(), (*name).into(), (*city).into(), Decimal::new(*value as i128, 2).into(), (*active).into(), (*notes).into()]
        })
        .collect();
    let _ = data_set(ui, cds).load_rows(data);
}

/// As entradas (classes e subtipos) com o tamanho padrão de cada uma, medido no `Ui` das
/// prévias; os desenhados ficam com o form deles lá.
fn build_entries(state: &Shared) -> (Rc<RefCell<Ui>>, Vec<Entry>) {
    let st = state.borrow();
    let mut ui = new_document_ui(st.doc.dark);
    ui.set_shared_theme(st.doc.theme.clone());
    let classes: Vec<(String, String, Vec<(String, String)>)> = st
        .doc
        .ui
        .registry()
        .iter()
        .filter(|c| c.name != "Form" && c.name != "DataModule")
        .map(|c| (c.category.to_string(), c.name.to_string(), st.doc.ui.variants_of(c.name).into_iter().map(|v| (v.name, v.caption)).collect()))
        .collect();
    let mut entries = Vec::new();
    for (category, base, variants) in classes {
        let visual = st.doc.class_is_visual(&base);
        // Os não visuais têm a janela deles (o botão ao lado da galeria).
        if !visual {
            continue;
        }
        let all = std::iter::once((base.clone(), base.clone())).chain(variants.into_iter().map(|(v, caption)| (format!("{base}.{v}"), format!("{base} · {caption}"))));
        for (class, title) in all {
            let (kind, size) = if !visual {
                (Kind::NonVisual, (0.0, 0.0))
            } else {
                let Some(form) = preview_form(&mut ui, &class, &base) else { continue };
                let size = ui.bounds(form).map_or(SAMPLE, |b| (b.width, b.height));
                if drawn_only(&base) {
                    (Kind::Drawn(form), size)
                } else {
                    ui.free(form);
                    (Kind::Live, size)
                }
            };
            entries.push(Entry { class, base: base.clone(), title, category: category.clone(), kind, size });
        }
    }
    (Rc::new(RefCell::new(ui)), entries)
}

/// Cria o componente em `parent` com o texto inicial do designer (o nome da classe, menos
/// quando o subtipo já define o texto). Devolve o componente e o tamanho padrão dele.
fn create(ui: &mut Ui, class: &str, base: &str, parent: ComponentId) -> Option<(ComponentId, (f32, f32))> {
    let id = ui.create(class, Some(parent)).ok()?;
    let preset: Vec<String> = aylis_core::split_variant(class)
        .1
        .and_then(|v| ui.registry().variant(base, v))
        .map(|v| v.props.iter().map(|(k, _)| k.clone()).collect())
        .unwrap_or_default();
    let bound = base.starts_with("DB");
    for prop in ["text", "caption"] {
        if bound || preset.iter().any(|p| p == prop) {
            continue;
        }
        if let Ok(PropValue::String(current)) = ui.get_prop(id, prop) {
            if current.is_empty() {
                let _ = ui.set_prop(id, prop, base);
            }
        }
    }
    // O conteúdo de exemplo (os itens de uma lista, as linhas de uma grade, os filhos de um
    // contêiner): o mesmo das demonstrações ao vivo da documentação, no tamanho delas.
    let plain = aylis_core::split_variant(class).0;
    if aylis::samples::has_sample(plain) {
        aylis::samples::apply(ui, id, plain, parent);
        if let Some(size) = aylis::samples::sample_size(plain) {
            return Some((id, size));
        }
    }
    let (w, h, align) = ui.control(id).map(|cb| (cb.width(), cb.height(), cb.align())).unwrap_or((0.0, 0.0, Align::None));
    // Alinhado ao pai: a medida que o alinhamento não estica fica, a outra vem do exemplo
    // (uma barra no topo tem a altura dela e a largura de um form pequeno).
    let own = |v: f32, sample: f32| if v >= 1.0 { v } else { sample };
    let size = match align {
        Align::None => (own(w, SAMPLE.0), own(h, SAMPLE.1)),
        Align::Top | Align::Bottom => (360.0, own(h, 40.0)),
        Align::Left | Align::Right => (own(w, 200.0), SAMPLE.1),
        _ => SAMPLE,
    };
    Some((id, size))
}

/// Um form do tamanho do componente, com ele dentro (no `Ui` das prévias).
pub(crate) fn preview_form(ui: &mut Ui, class: &str, base: &str) -> Option<ComponentId> {
    let form = ui.add_root(Form::new("", 200.0, 100.0));
    let Some((id, (w, h))) = create(ui, class, base, form) else {
        ui.free(form);
        return None;
    };
    if let Some(cb) = ui.control_mut(id) {
        cb.set_align(Align::None).set_rect(0.0, 0.0, w, h);
    }
    if let Some(cb) = ui.control_mut(form) {
        cb.set_width(w).set_height(h);
    }
    ui.layout();
    Some(form)
}

/// Monta os cartões que passam no filtro, por categoria. Cada cartão tem o tamanho do seu
/// componente; as linhas quebram quando não cabem.
fn cards(ui: &mut Ui, ctx: &Ctx, entries: &[Entry], render: &Rc<RefCell<Option<Rc<dyn Fn(&mut Ui)>>>>) {
    clear(ui, ctx.list);
    let (filter, look) = {
        let v = ctx.view.borrow();
        (v.filter.clone(), v.theme_name())
    };
    let filter = filter.as_str();
    // Os desenhados (prévias de outro `Ui`) também com o tema da versão.
    {
        let mut previews = ctx.previews.borrow_mut();
        for e in entries {
            if let Kind::Drawn(form) = e.kind {
                if let Some(cb) = previews.control_mut(form) {
                    cb.set_theme_name(look.as_deref().unwrap_or(""));
                }
            }
        }
    }
    let mut by_category: HashMap<&str, Vec<&Entry>> = HashMap::new();
    for e in entries.iter().filter(|e| filter.is_empty() || e.title.to_lowercase().contains(filter)) {
        by_category.entry(e.category.as_str()).or_default().push(e);
    }
    let mut categories: Vec<&str> = CATEGORY_ORDER.iter().copied().filter(|c| by_category.contains_key(c)).collect();
    let mut others: Vec<&str> = by_category.keys().copied().filter(|c| !CATEGORY_ORDER.contains(c)).collect();
    others.sort();
    categories.extend(others);
    let pending = ctx.state.borrow().pending.clone();
    for category in categories {
        let mut items = by_category.remove(category).unwrap_or_default();
        items.sort_by(|a, b| a.title.cmp(&b.title));
        // O cabeçalho da categoria recolhe e abre (com busca, fica aberta).
        let open = !ctx.view.borrow().closed.contains(category) || !filter.is_empty();
        category_header(ui, ctx, category, open, render);
        if !open {
            continue;
        }
        let mut rows: Vec<Vec<&Entry>> = Vec::new();
        let mut used = LIST_WIDTH;
        for e in items {
            let w = e.card_size().0;
            if used + w > LIST_WIDTH {
                rows.push(Vec::new());
                used = 0.0;
            }
            rows.last_mut().unwrap().push(e);
            used += w + GAP;
        }
        for row in rows {
            let height = row.iter().map(|e| e.card_size().1).fold(0.0, f32::max);
            let line = ui.add(Layout::default(), ctx.list);
            ui.control_mut(line).unwrap().set_align(Align::Top).set_height(height).set_margins([0.0, 0.0, 0.0, GAP]);
            for e in row {
                let key = ctx.view.borrow().key(&e.class);
                card(ui, ctx, line, e, pending.as_deref() == Some(key.as_str()));
            }
        }
    }
    ui.layout();
}

/// O cabeçalho de uma categoria (como as versões na paleta): o nome com a seta; clicar
/// recolhe e abre os cartões dela.
fn category_header(ui: &mut Ui, ctx: &Ctx, category: &str, open: bool, render: &Rc<RefCell<Option<Rc<dyn Fn(&mut Ui)>>>>) {
    let r = row(ui, ctx.list, category, 0.0, false, 30.0);
    if let Some(&label) = ui.children(r).iter().find(|&&c| ui.class_name(c) == Some("Label")) {
        let _ = ui.set_prop(label, "bold", true);
    }
    let arrow = ui.add(Label::new(if open { "▾" } else { "▸" }), r);
    ui.control_mut(arrow).unwrap().set_align(Align::Right).set_width(30.0).set_hit_test(false);
    let _ = ui.set_prop(arrow, "h_align", PropValue::Enum("center".into()));
    ui.control_mut(r).unwrap().set_margins([0.0, 4.0, 0.0, 6.0]);
    let (view, render, category) = (ctx.view.clone(), render.clone(), category.to_string());
    ui.on_click(r, move |ui, _| {
        {
            let mut v = view.borrow_mut();
            if !v.closed.remove(&category) {
                v.closed.insert(category.clone());
            }
        }
        let f = render.borrow().clone();
        if let Some(f) = f {
            f(ui);
        }
    });
}

fn card(ui: &mut Ui, ctx: &Ctx, line: ComponentId, e: &Entry, selected: bool) {
    let (w, h) = e.card_size();
    let r = ui.add(Rectangle::new(w, h), line);
    // Align::Left estica na altura da linha: a margem de baixo devolve a altura do cartão.
    let line_height = ui.control(line).map_or(h, |cb| cb.height());
    ui.control_mut(r).unwrap().set_align(Align::Left).set_margins([0.0, 0.0, GAP, (line_height - h).max(0.0)]);
    let _ = ui.set_prop(r, "radius", 8);
    let stroke = if selected { ui.theme().color("primary") } else { ui.theme().color("border") };
    let _ = ui.set_prop(r, "stroke", stroke);
    let _ = ui.set_prop(r, "fill", ui.theme().color("text").with_opacity(0.03));

    // Barra do título: o nome e o "Usar".
    let bar = ui.add(Layout::default(), r);
    ui.control_mut(bar).unwrap().set_align(Align::Bottom).set_height(TITLE).set_padding([10.0, 0.0, 4.0, 4.0]);
    let use_button = ui.add(Button::new("Usar"), bar);
    ui.control_mut(use_button).unwrap().set_align(Align::Right).set_width(64.0).set_style("flat");
    let title = ui.add(Label::new(&e.title), bar);
    ui.control_mut(title).unwrap().set_align(Align::Client);
    let (class, state, ids, dialog) = (ctx.view.borrow().key(&e.class), ctx.state.clone(), ctx.ids, ctx.dialog);
    ui.on_click(use_button, move |ui, _| {
        choose(&state, &class);
        ui.set_modal_result(dialog, ModalResult::Ok);
        refresh(ui, &state, &ids);
    });

    match e.kind {
        Kind::NonVisual => {
            let l = ui.add(Label::new("não visual"), r);
            ui.control_mut(l).unwrap().set_align(Align::Client);
            let _ = ui.set_prop(l, "text_align", "center");
            muted(ui, l);
        }
        Kind::Drawn(form) => {
            let p = ui.add(Preview { base: ControlBase::new(e.size.0, e.size.1), ui: ctx.previews.clone(), form }, r);
            ui.control_mut(p).unwrap().set_align(Align::Client).set_margins([4.0, 4.0, 4.0, 0.0]);
            let _ = ui.set_prop(p, "hint", "Prévia: este componente usa recursos do sistema");
        }
        Kind::Live => {
            // A área do form (fundo e tema do form), cortando o que passar.
            // Numa plataforma, o tema da versão (e o fundo dela).
            let look = ctx.view.borrow().theme_name();
            let background = look.as_deref().and_then(|t| ui.theme_named(t)).map_or(ctx.background, |t| t.color("background"));
            let host = ui.add(Rectangle::default(), r);
            ui.control_mut(host).unwrap().set_align(Align::Client).set_margins([4.0, 4.0, 4.0, 0.0]).set_theme_name(look.as_deref().unwrap_or(THEME)).set_clip_children(true);
            let _ = ui.set_prop(host, "fill", background);
            let _ = ui.set_prop(host, "radius", 6);
            let _ = ui.set_prop(host, "stroke", Color::TRANSPARENT);
            let Some((id, (cw, ch))) = create(ui, &e.class, &e.base, host) else { return };
            if let Some(cb) = ui.control_mut(id) {
                cb.set_align(Align::None).set_rect(PAD - 4.0, PAD - 4.0, cw, ch);
            }
            bind(ui, id, ctx.data);
        }
    }
    if !selected {
        let hover = ui.theme().color("primary");
        ui.add(PropertyAnimation::color("stroke", hover).with_trigger(Trigger::Hover).with_duration(100.0), r);
    }
}

/// Liga um controle de dados ao ClientDataSet de exemplo (os de um campo, ao `nome`).
fn bind(ui: &mut Ui, id: ComponentId, data: ComponentId) {
    if ui.set_prop(id, "data_source", data).is_err() {
        return;
    }
    let _ = ui.set_prop(id, "data_field", "nome");
}

/// Desenha um form do `Ui` das prévias, reduzido para caber (nunca ampliado) e centralizado.
struct Preview {
    base: ControlBase,
    ui: Rc<RefCell<Ui>>,
    form: ComponentId,
}

impl Component for Preview {
    fn class_name(&self) -> &'static str {
        "ComponentPreview"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let Ok(mut ui) = self.ui.try_borrow_mut() else { return };
        let Some(size) = ui.bounds(self.form).map(|b| b.size()) else { return };
        if size.width < 1.0 || size.height < 1.0 {
            return;
        }
        let scale = (r.width / size.width).min(r.height / size.height).min(1.0);
        let (w, h) = (size.width * scale, size.height * scale);
        let (x, y) = (r.x + (r.width - w) / 2.0, r.y + (r.height - h) / 2.0);
        canvas.save();
        canvas.clip_rect(r);
        canvas.fill_rect(RectF::new(x, y, w, h).inflate(1.0), 3.0, &Brush::Solid(Color::BLACK.with_opacity(0.08)));
        canvas.translate(x, y);
        canvas.scale(scale, scale);
        canvas.clip_rect(RectF::new(0.0, 0.0, size.width, size.height));
        ui.paint(self.form, canvas);
        canvas.restore();
    }
}
