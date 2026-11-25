//! O menu de texto padrão dos campos (`Edit`, `Memo` e quem os usa por dentro): Recortar,
//! Copiar, Colar e Selecionar tudo, no botão direito e no toque longo, quando nem o campo nem
//! um pai tem `popup_menu` (o gancho `Component::context_menu`). E as alças da seleção feita
//! pelo toque, como nos campos do Android.
//!
//! Cada item manda ao campo a mesma tecla do atalho (Ctrl+X, Ctrl+C, Ctrl+V, Ctrl+A): o
//! `on_change`, o `on_paste`, o desfazer, o `max_length`, os caracteres liberados e a máscara
//! são os do teclado.

use aylis_core::messages::t;
use aylis_core::{Brush, Canvas, Color, ComponentId, EventCtx, Key, KeyEvent, Modifiers, PointF, RectF, Stroke, Ui};

/// O que o menu oferece. Recortar e Copiar ficam desligados sem seleção.
#[derive(Clone, Copy, Debug)]
pub(crate) struct TextMenu {
    /// Há texto selecionado.
    pub selected: bool,
    /// Há texto no campo (Selecionar tudo).
    pub has_text: bool,
    /// Só leitura: só Copiar e Selecionar tudo.
    pub read_only: bool,
    /// Senha: sem Copiar e Recortar.
    pub password: bool,
}

/// O menu fechou (com ou sem escolha): o campo desliga a marca de menu aberto (com ela, a
/// seleção continua à vista e o foco que volta não seleciona tudo). Cada campo tem a sua (os
/// campos dele são privados).
pub(crate) type MenuClosed = fn(&mut Ui, ComponentId);

/// O `PopupMenu` do campo (filho dele, refeito a cada abertura), vazio.
pub(crate) fn menu_slot(ctx: &mut EventCtx<'_>, slot: &mut Option<ComponentId>) -> ComponentId {
    let id = ctx.id;
    let menu = match slot.filter(|&m| ctx.ui.contains(m)) {
        Some(m) => m,
        None => {
            let m = ctx.ui.add(crate::PopupMenu::default(), id);
            *slot = Some(m);
            m
        }
    };
    let children = ctx.ui.children(menu).to_vec();
    ctx.ui.free_many(&children);
    menu
}

/// Põe no `menu` os itens de texto do campo `field`.
pub(crate) fn add_text_items(ui: &mut Ui, menu: ComponentId, field: ComponentId, what: TextMenu, closed: MenuClosed) {
    let paste = can_paste(ui);
    let item = |ui: &mut Ui, text: &str, enabled: bool, key: char| {
        let id = ui.add(crate::MenuItem::new(t(text)), menu);
        if !enabled {
            let _ = ui.set_prop(id, "enabled", false);
        }
        ui.on_click(id, move |ui, _| act(ui, field, closed, key));
    };
    if !what.read_only && !what.password {
        item(ui, "Recortar", what.selected, 'x');
    }
    if !what.password {
        item(ui, "Copiar", what.selected, 'c');
    }
    if !what.read_only {
        item(ui, "Colar", paste, 'v');
    }
    item(ui, "Selecionar tudo", what.has_text, 'a');
}

/// Colar só vale com algo para colar. Saber se a área de transferência da plataforma tem texto
/// exige ler o texto (no Android 12+ isso mostra o aviso "colado da área de transferência"; no
/// navegador, pede permissão): com ela, Colar fica ligado; sem ela, vale a cópia do processo.
fn can_paste(ui: &mut Ui) -> bool {
    if ui.services_mut().clipboard.is_some() {
        return true;
    }
    aylis_core::clipboard::text(ui).is_some()
}

/// Um item escolhido: o foco volta ao campo (com a seleção de quando o menu abriu) e a tecla
/// do atalho vai a ele (o mesmo caminho do teclado).
fn act(ui: &mut Ui, field: ComponentId, closed: MenuClosed, key: char) {
    if !ui.contains(field) {
        return;
    }
    ui.set_focus(field);
    closed(ui, field);
    let event = KeyEvent { key: Key::Char(key), modifiers: Modifiers { ctrl: true, ..Default::default() }, ..Default::default() };
    ui.with_component(field, |c, ctx| c.key_down(ctx, &event));
    ui.process_events();
    ui.request_repaint();
}

/// Abre o menu em `at` (local do campo). Quando ele fecha sem escolha (Esc, toque fora), o
/// foco volta ao campo se ninguém ficou com ele, com a mesma seleção.
pub(crate) fn popup(ctx: &mut EventCtx<'_>, menu: ComponentId, at: PointF, closed: MenuClosed) {
    let field = ctx.id;
    ctx.ui.on(menu, "on_close", move |ui, _, _| {
        if !ui.contains(field) {
            return;
        }
        let root = ui.root_of(field);
        if ui.focused(root).is_none() {
            ui.set_focus(field);
        }
        closed(ui, field);
        ui.request_repaint();
    });
    let origin = ctx.ui.absolute_rect(field).map_or(PointF::ZERO, |r| r.origin());
    crate::PopupMenu::popup(ctx.ui, menu, PointF::new(origin.x + at.x, origin.y + at.y));
}

// ---------------------------------------------------------------------------
// As alças da seleção pelo toque

/// O raio da bolinha da alça.
pub(crate) const HANDLE_RADIUS: f32 = 8.0;
/// O traço entre o texto e a bolinha.
const STEM: f32 = 3.0;
/// A área que o dedo pega em volta da bolinha (lado do quadrado).
const HANDLE_TOUCH: f32 = 40.0;
/// Quanto o campo recebe o toque além do próprio retângulo com as alças à vista.
pub(crate) const HANDLE_REACH: f32 = HANDLE_TOUCH;
/// Abaixo das alças, onde o menu do toque abre (a partir do pé da linha).
pub(crate) const BELOW_HANDLES: f32 = STEM + HANDLE_RADIUS * 2.0 + 6.0;

/// As áreas de toque das alças no pé da linha (`bottom`) em `x1` (começo) e `x2` (fim).
pub(crate) fn handle_rects(x1: f32, y1: f32, x2: f32, y2: f32) -> (RectF, RectF) {
    let rect = |x: f32, bottom: f32| {
        let cy = bottom + STEM + HANDLE_RADIUS;
        RectF::new(x - HANDLE_TOUCH / 2.0, cy - HANDLE_TOUCH / 2.0, HANDLE_TOUCH, HANDLE_TOUCH)
    };
    (rect(x1, y1), rect(x2, y2))
}

/// Desenha uma alça: o traço que sai do pé da linha em `x` e a bolinha embaixo, na cor
/// `color` (o `primary` do tema).
pub(crate) fn draw_handle(canvas: &mut dyn Canvas, x: f32, bottom: f32, color: Color) {
    canvas.draw_line(PointF::new(x, bottom - 1.0), PointF::new(x, bottom + STEM + 1.0), &Stroke::new(color, 2.0));
    let d = HANDLE_RADIUS * 2.0;
    canvas.fill_ellipse(RectF::new(x - HANDLE_RADIUS, bottom + STEM, d, d), &Brush::Solid(color));
}

/// O ponto acerta uma alça: `Some(true)` a do começo, `Some(false)` a do fim.
pub(crate) fn handle_at(handles: Option<(RectF, RectF)>, p: PointF) -> Option<bool> {
    let (start, end) = handles?;
    // As duas perto (seleção curta): a mais perto do dedo.
    match (start.contains(p), end.contains(p)) {
        (true, true) => {
            let (cs, ce) = (start.x + start.width / 2.0, end.x + end.width / 2.0);
            Some((p.x - cs).abs() <= (p.x - ce).abs())
        }
        (true, false) => Some(true),
        (false, true) => Some(false),
        (false, false) => None,
    }
}

/// O cursor arrastando a alça (a outra ponta fica na âncora): não cruza a outra ponta.
pub(crate) fn drag_handle(anchor: usize, index: usize, start: bool, len: usize) -> usize {
    if start { index.min(anchor.saturating_sub(1)) } else { index.max(anchor + 1).min(len) }
}
