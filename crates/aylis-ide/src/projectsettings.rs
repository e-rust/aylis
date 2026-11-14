//! Projeto → Opções do projeto: o que é do projeto e muda pouco (geral, aparência, fontes,
//! ícone do app, Android, memória, DataModules usados, traduções, confiança) numa janela só:
//! as seções numa lista à esquerda e a escolhida à
//! direita. O menu fica com o que se usa no dia a dia.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use aylis_controls::{Button, Layout, ListBox, ScrollBox};
use aylis_core::{Align, Ui};

use crate::app::{Ids, refresh};
use crate::dialogs::{Embedded, embed, new_dialog, run};
use crate::state::Shared;
use crate::widgets::{label, muted};

type Open = fn(&mut Ui, &Shared, &Ids);

/// Como a seção aparece na página: o editor dela montado ali ou, para a que só pergunta (a
/// confiança), um botão que a abre.
#[derive(Clone, Copy, PartialEq)]
enum Page {
    Embedded,
    Button(&'static str),
}

/// As seções: o título (também o texto que a IDE lite usa para saber se a seção vale lá),
/// a descrição, o editor e como ele aparece.
const SECTIONS: [(&str, &str, Open, Page); 10] = [
    (
        "Geral",
        "Nome, linguagem, plataformas, form principal, idioma, fuso, renderizador e texto por plataforma, placa de vídeo e onde sai o executável.",
        crate::actions::project_options,
        Page::Embedded,
    ),
    ("Aparência", "Tema, modo claro ou escuro, cores do projeto e a mistura de temas por classe.", crate::actions::project_appearance, Page::Embedded),
    ("Fontes", "As famílias embutidas no app, a fonte do projeto e as fontes nomeadas.", crate::actions::project_fonts, Page::Embedded),
    // Abre escolhendo o arquivo: um botão, para não pedir a imagem só por passar pela aba.
    ("Ícone do app", "O ícone de cada plataforma, gerado a partir de um PNG ou SVG.", crate::iconview::open, Page::Button("Escolher a imagem do ícone…")),
    ("Android", "Versões do SDK, orientação, tela cheia, permissões extras, links que abrem o app e o manifesto.", crate::androidview::open, Page::Embedded),
    ("Windows", "O instalador do Publicar (.msi, NSIS ou Inno Setup) e o que vai nele.", crate::windowsview::open, Page::Embedded),
    (
        "Memória",
        "Quanto o app guarda de imagens prontas (vídeo, prévias, ícones, miniaturas) para não refazer a cada pintura.",
        crate::actions::project_memory,
        Page::Embedded,
    ),
    ("DataModules usados", "Os DataModules que o form aberto usa.", crate::actions::edit_uses, Page::Embedded),
    ("Traduções", "Os idiomas do app e os textos traduzidos dos forms e do código.", crate::tooldialogs::translations, Page::Embedded),
    (
        "Confiar neste projeto",
        "Liga compilar, executar e o servidor de linguagem num projeto que veio de fora.",
        |_, s, _| crate::trust::trust_open_project(s),
        Page::Button("Confiar neste projeto…"),
    ),
];

/// O texto que a IDE lite usa para cada seção (o mesmo do item de menu de antes).
fn lite_name(title: &str) -> &str {
    match title {
        "Ícone do app" => "Ícone do app…",
        "Android" => "Android…",
        other => other,
    }
}

/// Os títulos das seções, na ordem da lista (as que valem nesta IDE).
pub fn section_titles() -> Vec<&'static str> {
    sections().iter().map(|s| s.0).collect()
}

fn sections() -> Vec<(&'static str, &'static str, Open, Page)> {
    SECTIONS.iter().copied().filter(|(t, ..)| crate::lite::needs(lite_name(t)).is_none_or(crate::lite::can)).collect()
}

pub fn open(ui: &mut Ui, state: &Shared, ids: &Ids) {
    open_at(ui, state, ids, 0);
}

/// Abre as Opções do projeto na seção `start`. Trocar de seção aplica a que estava aberta
/// (se ela estiver certa; senão, fica nela com o erro); OK aplica e fecha; Cancelar descarta
/// só o que mudou na seção aberta.
pub fn open_at(ui: &mut Ui, state: &Shared, ids: &Ids, start: usize) {
    let title = state.borrow().project.as_ref().map(|p| format!("Opções do projeto · {}", p.name)).unwrap_or_else(|| "Opções do projeto".into());
    let sections = sections();
    let d = new_dialog(ui, &title, 1320.0, 880.0);
    // Aplicar uma seção ao trocar de aba não refaz a IDE atrás (bloqueada): uma vez, ao fechar.
    crate::app::defer_refresh(true);
    {
        let (state, ids) = (state.clone(), *ids);
        ui.on(d.form, "on_close", move |ui, _, args| {
            let pending = crate::app::defer_refresh(false);
            // Com OK, o aplicar logo depois já refaz.
            let ok = matches!(args, aylis_core::EventArgs::Value(aylis_core::PropValue::Enum(r)) if r == "ok");
            if pending && !ok {
                refresh(ui, &state, &ids);
            }
        });
    }
    let nav = ui.add(ListBox::default(), d.body);
    ui.control_mut(nav).unwrap().set_align(Align::Left).set_width(220.0).set_margins([0.0, 0.0, 16.0, 0.0]);
    let _ = ui.set_prop(nav, "item_height", 38.0);
    let _ = ui.set_prop(nav, "items", sections.iter().map(|s| s.0).collect::<Vec<_>>().join(";"));
    let host = ui.add(ScrollBox::default(), d.body);
    // O recuo à direita deixa a barra de rolagem fora dos campos (pedido do autor em 07/11).
    ui.control_mut(host).unwrap().set_align(Align::Client).set_padding([0.0, 0.0, 18.0, 0.0]);
    let current: Rc<RefCell<Option<Embedded>>> = Rc::default();
    let shown = Rc::new(Cell::new(usize::MAX));
    let show = {
        let (state, ids, current, shown, sections) = (state.clone(), *ids, current.clone(), shown.clone(), sections.clone());
        Rc::new(move |ui: &mut Ui, index: usize| {
            let Some(&(title, description, open, page_kind)) = sections.get(index) else { return };
            let children = ui.children(host).to_vec();
            ui.free_many(&children);
            let header = label(ui, host, title, Align::Top, 30.0);
            let _ = ui.set_prop(header, "font_size", 18);
            let text = label(ui, host, description, Align::Top, 40.0);
            muted(ui, text);
            let _ = ui.set_prop(text, "wrap", true);
            let page = ui.add(Layout::default(), host);
            ui.control_mut(page).unwrap().set_align(Align::Top).set_height(120.0);
            let embedded = match page_kind {
                Page::Embedded => {
                    let before = state.borrow().status.clone();
                    let embedded = embed(page, || open(ui, &state, &ids));
                    // Sem editor (um aviso no lugar, como "abra um form"): o aviso fica na página.
                    let status = state.borrow().status.clone();
                    if embedded.is_none() && status != before {
                        let l = label(ui, page, &status, Align::Top, 30.0);
                        muted(ui, l);
                    }
                    embedded
                }
                Page::Button(text) => {
                    let b = ui.add(Button::new(text), page);
                    ui.control_mut(b).unwrap().set_align(Align::Left).set_width(240.0).set_margins([0.0, 8.0, 0.0, 80.0]).set_style("secondary");
                    let (state, ids) = (state.clone(), ids);
                    ui.on_click(b, move |ui, _| open(ui, &state, &ids));
                    None
                }
            };
            if page_kind == Page::Embedded {
                fit_page(ui, host, page);
            }
            *current.borrow_mut() = embedded;
            shown.set(index);
            let _ = ui.set_prop(nav, "item_index", index as i64);
            crate::lang::translate(ui);
        })
    };
    // Trocar de seção: a aberta passa pela conferência e é aplicada antes.
    {
        let (state, ids, current, shown, show) = (state.clone(), *ids, current.clone(), shown.clone(), show.clone());
        ui.on(nav, "on_change", move |ui, _, _| {
            let index = ui.get::<ListBox>(nav).map_or(-1, |l| l.item_index);
            let Ok(index) = usize::try_from(index) else { return };
            if index == shown.get() {
                return;
            }
            let taken = current.borrow_mut().take();
            if let Some(open) = taken {
                if let Err(message) = (open.validate)(ui) {
                    let _ = ui.set_prop(open.error, "text", format!("⚠ {message}"));
                    *current.borrow_mut() = Some(open);
                    let _ = ui.set_prop(nav, "item_index", shown.get() as i64);
                    return;
                }
                (open.apply)(ui);
                refresh(ui, &state, &ids);
            }
            show(ui, index);
        });
    }
    show(ui, start.min(sections.len().saturating_sub(1)));
    let validate = {
        let current = current.clone();
        move |ui: &mut Ui| match current.borrow().as_ref() {
            Some(open) => (open.validate)(ui),
            None => Ok(()),
        }
    };
    let (state, ids) = (state.clone(), *ids);
    run(ui, d, validate, move |ui| {
        let taken = current.borrow_mut().take();
        if let Some(open) = taken {
            (open.apply)(ui);
        }
        refresh(ui, &state, &ids);
    });
}

/// A altura da página embutida: a área visível (abaixo do título e da descrição) e, se o
/// conteúdo não couber nela, o que falta a mais, para rolar até o fim. A altura pedida pelo
/// diálogo (pensada para a janela dele) sobrava ou faltava: em Fontes a barra aparecia sem nada
/// para rolar, e o fim da Geral e da Memória ficava cortado (achado pelo autor em 07/11).
fn fit_page(ui: &mut Ui, host: aylis_core::ComponentId, page: aylis_core::ComponentId) {
    ui.layout();
    let Some(hb) = ui.control(host) else { return };
    let pad = hb.padding();
    let mut available = hb.bounds().height - pad.top - pad.bottom;
    for &c in ui.children(host) {
        if c == page {
            continue;
        }
        if let Some(cb) = ui.control(c).filter(|cb| cb.visible()) {
            let m = cb.margins();
            available -= cb.bounds().height + m.top + m.bottom;
        }
    }
    let available = available.max(200.0);
    ui.control_mut(page).unwrap().set_height(available);
    ui.layout();
    // O corpo do diálogo (o que preenche a página): o que os filhos dele ocupam além dele.
    let body = ui.children(page).iter().copied().find(|&c| ui.control(c).is_some_and(|cb| cb.align() == Align::Client));
    let Some(body) = body.and_then(|b| ui.control(b)) else { return };
    let missing = body.content_size().height - body.bounds().height;
    if missing > 0.5 {
        ui.control_mut(page).unwrap().set_height(available + missing);
        ui.layout();
    }
}
