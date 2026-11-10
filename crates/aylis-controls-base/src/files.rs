//! Arquivos: `FileInput` (campo que abre o diálogo do sistema, o `v-file-input`) e
//! `FileDropZone` (área para soltar arquivos ou clicar, o `v-file-upload`).
//!
//! `files` tem os caminhos escolhidos, separados por `;`; `on_change` a cada mudança.
//! `filter` é o do `OpenDialog` (`Imagens|*.png;*.jpg`).

use std::cell::{Cell, RefCell};

use aylis_core::platform::FileDialogOptions;
use aylis_core::{
    Brush, Canvas, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, PaintCtx, PointerButton, PointerEvent,
    RectF, Stroke, TextAlign, Ui, properties,
};

use crate::leaf_control;

fn split(items: &str) -> Vec<&str> {
    if items.is_empty() { Vec::new() } else { items.split(';').collect() }
}

fn file_name(path: &str) -> &str {
    path.rsplit(['/', '\\']).next().unwrap_or(path)
}

/// Abre o diálogo do sistema e grava a escolha em `files` do controle `id`.
pub(crate) fn choose(ui: &mut Ui, id: ComponentId, filter: &str, multiple: bool, append: bool) {
    let options = FileDialogOptions { filters: crate::parse_filter(filter), multiple, ..Default::default() };
    let poster = ui.poster();
    let done = Box::new(move |paths: Vec<String>| {
        if paths.is_empty() {
            return;
        }
        poster.post(move |ui| {
            let before = match ui.get_prop(id, "files") {
                Ok(aylis_core::PropValue::String(s)) if append => s,
                _ => String::new(),
            };
            let mut all: Vec<String> = split(&before).into_iter().map(str::to_string).collect();
            for p in paths {
                if !all.contains(&p) {
                    all.push(p);
                }
            }
            let joined = all.join(";");
            let _ = ui.set_prop_quiet(id, "files", joined.clone());
            ui.fire(id, "on_change", &EventArgs::Text(joined));
        });
    });
    if let Some(service) = ui.services_mut().file_dialogs.as_mut() {
        service.show(options, done);
    }
}

/// Aceita os caminhos que passam no filtro (extensões); sem filtro, todos.
fn accepted(filter: &str, paths: &[String]) -> Vec<String> {
    let exts: Vec<String> = crate::parse_filter(filter).into_iter().flat_map(|(_, e)| e).map(|e| e.to_lowercase()).collect();
    paths
        .iter()
        .filter(|p| exts.is_empty() || exts.iter().any(|e| e == "*" || p.to_lowercase().ends_with(&format!(".{e}"))))
        .cloned()
        .collect()
}

// ===========================================================================
// FileInput
// ===========================================================================

/// Campo de arquivo: mostra os nomes escolhidos; o clique abre o diálogo do sistema; o
/// "×" limpa. Também aceita arquivos soltos sobre ele.
pub struct FileInput {
    pub base: ControlBase,
    pub files: String,
    pub filter: String,
    pub multiple: bool,
    pub placeholder: String,
    clear: Cell<RectF>,
}

impl Default for FileInput {
    fn default() -> Self {
        let mut base = ControlBase::new(260.0, 34.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self { base, files: String::new(), filter: String::new(), multiple: false, placeholder: "Escolher arquivo…".into(), clear: Cell::new(RectF::default()) }
    }
}

impl Component for FileInput {
    // O `on_change` do valor mudado pelo código leva o mesmo que o do usuário.
    fn change_args(&self, _name: &str, _value: &aylis_core::PropValue) -> Option<EventArgs> {
        Some(EventArgs::Text(self.files.clone()))
    }

    leaf_control!("FileInput");

    properties! {
        files: String => "Data",
        filter: String => "Behavior",
        multiple: bool => "Behavior",
        placeholder: String => "Appearance";
        events: on_change
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        if !self.files.is_empty() && self.clear.get().inflate(4.0).contains(event.position) {
            self.files.clear();
            ctx.fire("on_change", EventArgs::Text(String::new()));
            ctx.repaint();
            return;
        }
        let (id, filter, multiple) = (ctx.id, self.filter.clone(), self.multiple);
        ctx.ui.poster().post(move |ui| choose(ui, id, &filter, multiple, false));
    }

    fn platform_hooks(&self) -> Option<&dyn aylis_core::PlatformHooks> {
        Some(self)
    }

    fn platform_hooks_mut(&mut self) -> Option<&mut dyn aylis_core::PlatformHooks> {
        Some(self)
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let s = ctx.style;
        canvas.fill_rect(r, s.radius.max(4.0), &Brush::Solid(ctx.theme.color("surface")));
        let border = if ctx.state.focused { ctx.theme.color("primary") } else { ctx.theme.color("border") };
        canvas.stroke_rect(r.inflate(-0.5), s.radius.max(4.0), &Stroke::new(border, 1.0));
        let muted = ctx.theme.color("text_muted");
        ctx.draw_icon(canvas, "attach", RectF::new(r.x + 8.0, r.y + (r.height - 18.0) / 2.0, 18.0, 18.0), muted);
        let names: Vec<&str> = split(&self.files).into_iter().map(file_name).collect();
        let mut t = ctx.text_style();
        let text_r = RectF::new(r.x + 32.0, r.y, r.width - 64.0, r.height);
        if names.is_empty() {
            t.color = muted;
            canvas.draw_text(&ctx.ui.tr(&self.placeholder), text_r, &t);
        } else {
            let label = if names.len() == 1 { names[0].to_string() } else { format!("{} (+{} arquivos)", names[0], names.len() - 1) };
            canvas.draw_text(&label, text_r, &t);
            let c = RectF::new(r.right() - 26.0, r.y + (r.height - 16.0) / 2.0, 16.0, 16.0);
            ctx.draw_icon(canvas, "close", c, muted);
            self.clear.set(c);
        }
    }
}

// ===========================================================================
// FileDropZone
// ===========================================================================

/// Área para soltar arquivos (ou clicar e escolher): borda tracejada, ícone de envio e a
/// lista do que foi escolhido, cada um com "×".
pub struct FileDropZone {
    pub base: ControlBase,
    pub files: String,
    pub filter: String,
    pub multiple: bool,
    pub title: String,
    removes: RefCell<Vec<(RectF, String)>>,
}

impl Default for FileDropZone {
    fn default() -> Self {
        let mut base = ControlBase::new(360.0, 200.0);
        base.set_cursor(CursorShape::Pointer);
        Self { base, files: String::new(), filter: String::new(), multiple: true, title: "Arraste arquivos para cá ou clique para escolher".into(), removes: RefCell::new(Vec::new()) }
    }
}

impl Component for FileDropZone {
    // O `on_change` do valor mudado pelo código leva o mesmo que o do usuário.
    fn change_args(&self, _name: &str, _value: &aylis_core::PropValue) -> Option<EventArgs> {
        Some(EventArgs::Text(self.files.clone()))
    }

    leaf_control!("FileDropZone");

    properties! {
        files: String => "Data",
        filter: String => "Behavior",
        multiple: bool => "Behavior",
        title: String => "Appearance";
        events: on_change
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        let remove = self.removes.borrow().iter().find(|(r, _)| r.contains(event.position)).map(|(_, p)| p.clone());
        if let Some(path) = remove {
            let rest: Vec<&str> = split(&self.files).into_iter().filter(|p| *p != path).collect();
            self.files = rest.join(";");
            ctx.fire("on_change", EventArgs::Text(self.files.clone()));
            ctx.repaint();
            return;
        }
        let (id, filter, multiple) = (ctx.id, self.filter.clone(), self.multiple);
        ctx.ui.poster().post(move |ui| choose(ui, id, &filter, multiple, multiple));
    }

    fn platform_hooks(&self) -> Option<&dyn aylis_core::PlatformHooks> {
        Some(self)
    }

    fn platform_hooks_mut(&mut self) -> Option<&mut dyn aylis_core::PlatformHooks> {
        Some(self)
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        let color = theme.color("primary");
        let hover = ctx.state.hover;
        canvas.fill_rect(r, 12.0, &Brush::Solid(if hover { color.with_opacity(0.06) } else { theme.color("surface") }));
        canvas.stroke_rect(r.inflate(-1.0), 12.0, &Stroke::new(if hover { color } else { theme.color("border") }, 1.5).dashed(vec![6.0, 4.0]));
        let files = split(&self.files);
        let top = if files.is_empty() { r.y + (r.height - 80.0) / 2.0 } else { r.y + 16.0 };
        ctx.draw_icon(canvas, "upload", RectF::new(r.x + (r.width - 32.0) / 2.0, top, 32.0, 32.0), color);
        let mut t = ctx.text_style();
        t.h_align = TextAlign::Center;
        t.color = theme.color("text_muted");
        canvas.draw_text(&ctx.ui.tr(&self.title), RectF::new(r.x + 12.0, top + 38.0, r.width - 24.0, 22.0), &t);
        let mut removes = Vec::new();
        let mut y = top + 68.0;
        for path in files {
            if y + 26.0 > r.bottom() - 6.0 {
                break;
            }
            let row = RectF::new(r.x + 16.0, y, r.width - 32.0, 26.0);
            canvas.fill_rect(row, 6.0, &Brush::Solid(theme.color("surface_2")));
            ctx.draw_icon(canvas, "file", RectF::new(row.x + 6.0, row.y + 5.0, 16.0, 16.0), theme.color("text_muted"));
            let mut n = ctx.text_style();
            n.font.size = (n.font.size * 0.9).round();
            canvas.draw_text(file_name(path), RectF::new(row.x + 28.0, row.y, row.width - 56.0, row.height), &n);
            let x = RectF::new(row.right() - 22.0, row.y + 5.0, 16.0, 16.0);
            ctx.draw_icon(canvas, "close", x, theme.color("text_muted"));
            removes.push((x.inflate(4.0), path.to_string()));
            y += 30.0;
        }
        *self.removes.borrow_mut() = removes;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_and_names() {
        assert_eq!(file_name("C:\\fotos\\a.png"), "a.png");
        assert_eq!(file_name("/home/u/b.jpg"), "b.jpg");
        let paths = vec!["a.PNG".to_string(), "b.txt".to_string()];
        assert_eq!(accepted("Imagens|*.png;*.jpg", &paths), vec!["a.PNG".to_string()]);
        assert_eq!(accepted("", &paths).len(), 2);
    }
}

impl aylis_core::PlatformHooks for FileInput {
    fn files_dropped(&mut self, ctx: &mut EventCtx<'_>, paths: &[String]) -> bool {
        let ok = accepted(&self.filter, paths);
        if ok.is_empty() {
            return false;
        }
        let mut all: Vec<String> = if self.multiple { split(&self.files).into_iter().map(str::to_string).collect() } else { Vec::new() };
        for p in ok {
            if !all.contains(&p) {
                all.push(p);
            }
        }
        if !self.multiple {
            all.truncate(1);
        }
        self.files = all.join(";");
        ctx.fire("on_change", EventArgs::Text(self.files.clone()));
        ctx.repaint();
        true
    }
}

impl aylis_core::PlatformHooks for FileDropZone {
    fn files_dropped(&mut self, ctx: &mut EventCtx<'_>, paths: &[String]) -> bool {
        let ok = accepted(&self.filter, paths);
        if ok.is_empty() {
            return false;
        }
        let mut all: Vec<String> = if self.multiple { split(&self.files).into_iter().map(str::to_string).collect() } else { Vec::new() };
        for p in ok {
            if !all.contains(&p) {
                all.push(p);
            }
        }
        if !self.multiple {
            all.truncate(1);
        }
        self.files = all.join(";");
        ctx.fire("on_change", EventArgs::Text(self.files.clone()));
        ctx.repaint();
        true
    }
}
