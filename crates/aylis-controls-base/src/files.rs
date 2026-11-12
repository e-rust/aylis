//! Arquivos: `FileInput` (campo que abre o diálogo do sistema, o `v-file-input`) e
//! `FileDropZone` (área para soltar arquivos ou clicar, o `v-file-upload`).
//!
//! `files` tem os caminhos escolhidos, separados por `;`. O `on_change` leva só os que
//! entraram (separados por `;`; vazio quando um sai pelo "×"); a lista inteira fica em `files`.
//! `filter` é o do `OpenDialog` (`Imagens|*.png;*.jpg|Todos|*.*`); o arquivo solto que não passa
//! nele dispara `on_reject`, com os caminhos recusados.

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

/// Junta `paths` à lista `current` (ou troca, sem `multiple`): a lista nova e os que entraram.
fn add_files(current: &str, paths: Vec<String>, multiple: bool) -> (String, String) {
    let mut all: Vec<String> = if multiple { split(current).into_iter().map(str::to_string).collect() } else { Vec::new() };
    let mut added = Vec::new();
    for p in paths {
        if !all.contains(&p) {
            all.push(p.clone());
            added.push(p);
        }
    }
    if !multiple {
        all.truncate(1);
        added.truncate(1);
    }
    (all.join(";"), added.join(";"))
}

/// Abre o diálogo do sistema e grava a escolha em `files` do controle `id`.
pub(crate) fn choose(ui: &mut Ui, id: ComponentId, filter: &str, multiple: bool, append: bool) {
    let options = FileDialogOptions { filters: crate::parse_filter(filter), multiple, owner: Some(ui.root_of(id)), ..Default::default() };
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
            let (all, added) = add_files(&before, paths, true);
            if added.is_empty() && all == before {
                return;
            }
            let _ = ui.set_prop_quiet(id, "files", all);
            ui.fire(id, "on_change", &EventArgs::Text(added));
        });
    });
    if let Some(service) = ui.services_mut().file_dialogs.as_mut() {
        service.show(options, done);
    }
}

/// Separa os caminhos que passam no filtro (extensões; `*` é todos) dos recusados; sem
/// filtro, todos passam.
fn accepted(filter: &str, paths: &[String]) -> (Vec<String>, Vec<String>) {
    let exts: Vec<String> = crate::parse_filter(filter).into_iter().flat_map(|(_, e)| e).map(|e| e.to_lowercase()).collect();
    paths
        .iter()
        .cloned()
        .partition(|p| exts.is_empty() || exts.iter().any(|e| e == "*" || p.to_lowercase().ends_with(&format!(".{e}"))))
}

/// O arquivo solto num `FileInput` ou num `FileDropZone`: os aceitos entram, os recusados
/// disparam `on_reject` (o relatório do Tabernáculo, F02). Devolve `true` se algum entrou.
fn drop_into(ctx: &mut EventCtx<'_>, files: &mut String, filter: &str, multiple: bool, paths: &[String]) -> bool {
    let (ok, rejected) = accepted(filter, paths);
    if !rejected.is_empty() {
        ctx.fire("on_reject", EventArgs::Text(rejected.join(";")));
    }
    if ok.is_empty() {
        return false;
    }
    let (all, added) = add_files(files, ok, multiple);
    if all != *files {
        *files = all;
        ctx.fire("on_change", EventArgs::Text(added));
        ctx.repaint();
    }
    true
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
        events: on_change, on_reject
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
    /// Arquivos do sistema passando por cima (o destaque e o `on_file_enter`).
    hovering: bool,
}

impl FileDropZone {
    /// Com o filtro (`Imagens|*.png;*.jpg|Todos|*.*`): a montagem sem a sintaxe de struct (os
    /// campos internos não deixam; o relatório do Tabernáculo, F05).
    pub fn with_filter(mut self, filter: impl Into<String>) -> Self {
        self.filter = filter.into();
        self
    }

    pub fn with_multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
}

impl Default for FileDropZone {
    fn default() -> Self {
        let mut base = ControlBase::new(360.0, 200.0);
        base.set_cursor(CursorShape::Pointer);
        Self {
            base,
            files: String::new(),
            filter: String::new(),
            multiple: true,
            title: "Arraste arquivos para cá ou clique para escolher".into(),
            removes: RefCell::new(Vec::new()),
            hovering: false,
        }
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
        events: on_change, on_reject, on_file_enter, on_file_leave
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        let remove = self.removes.borrow().iter().find(|(r, _)| r.contains(event.position)).map(|(_, p)| p.clone());
        if let Some(path) = remove {
            let rest: Vec<&str> = split(&self.files).into_iter().filter(|p| *p != path).collect();
            self.files = rest.join(";");
            ctx.fire("on_change", EventArgs::Text(String::new()));
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
        let hover = ctx.state.hover || self.hovering;
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
        assert_eq!(accepted("Imagens|*.png;*.jpg", &paths), (vec!["a.PNG".to_string()], vec!["b.txt".to_string()]));
        assert_eq!(accepted("", &paths).0.len(), 2);
        assert_eq!(accepted("Imagens|*.png|Todos|*.*", &paths).0.len(), 2, "o *.* aceita todos");
        assert_eq!(add_files("a;b", vec!["b".into(), "c".into()], true), ("a;b;c".to_string(), "c".to_string()));
    }
}

impl aylis_core::PlatformHooks for FileInput {
    fn files_dropped(&mut self, ctx: &mut EventCtx<'_>, paths: &[String]) -> bool {
        let (filter, multiple) = (self.filter.clone(), self.multiple);
        drop_into(ctx, &mut self.files, &filter, multiple, paths)
    }
}

impl aylis_core::PlatformHooks for FileDropZone {
    fn files_dropped(&mut self, ctx: &mut EventCtx<'_>, paths: &[String]) -> bool {
        let (filter, multiple) = (self.filter.clone(), self.multiple);
        drop_into(ctx, &mut self.files, &filter, multiple, paths)
    }

    /// Arquivos passando por cima: destaca e avisa (`on_file_enter` com os que passam no
    /// filtro; `on_file_leave` ao sair, soltar ou cancelar).
    fn files_hovering(&mut self, ctx: &mut EventCtx<'_>, paths: Option<&[String]>) -> bool {
        match paths {
            Some(paths) => {
                let (ok, _) = accepted(&self.filter, paths);
                if ok.is_empty() {
                    return false;
                }
                if !self.hovering {
                    self.hovering = true;
                    ctx.fire("on_file_enter", EventArgs::Text(ok.join(";")));
                    ctx.repaint();
                }
                true
            }
            None => {
                if self.hovering {
                    self.hovering = false;
                    ctx.fire("on_file_leave", EventArgs::None);
                    ctx.repaint();
                }
                false
            }
        }
    }
}
