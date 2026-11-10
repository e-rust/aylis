//! CodeEditor: o editor de código (como o `TSynEdit` do Delphi). É o [`Memo`] com o que um
//! editor de programação tem: as cores da linguagem, a calha com os números e os pontos de
//! parada, as sugestões e as dicas de um servidor de linguagem, desfazer e refazer, as
//! operações de linha do VS Code, vários cursores, dobrar blocos e o minimapa.
//!
//! O `Memo` comum fica só caixa de texto; o que é de código liga aqui. Por código, o texto, o
//! cursor e a seleção são os do `Memo` de dentro (`editor.memo`, ou `ui.get::<Memo>(id)`).
//!
//! Eventos próprios (além dos do `Memo`): `on_complete_request` (pediu sugestões: o índice
//! do cursor), `on_signature_request` (abriu um parêntese), `on_hover_index` (a palavra sob
//! o mouse; -1 fora) e `on_word_click` (Ctrl+clique numa palavra).

use aylis_core::{Component, properties};

use crate::Memo;

pub struct CodeEditor {
    pub memo: Memo,
}

impl Default for CodeEditor {
    fn default() -> Self {
        let mut memo = Memo::default();
        memo.code_editing = true;
        memo.line_numbers = true;
        memo.want_tabs = true;
        memo.wrap = false;
        memo.folding = true;
        memo.minimap = true;
        Self { memo }
    }
}

impl CodeEditor {
    pub fn new(text: impl Into<String>) -> Self {
        let mut editor = Self::default();
        editor.memo.text = text.into();
        editor
    }

    /// A linguagem das cores (`rust`, `python`, `pascal`...; o `syntax` do `Memo`).
    pub fn with_syntax(mut self, syntax: &str) -> Self {
        self.memo.syntax = syntax.to_string();
        self
    }

    pub fn minimap(&self) -> bool {
        self.memo.minimap
    }

    pub fn set_minimap(&mut self, on: bool) {
        self.memo.minimap = on;
    }

    /// O tema de cores do código (`Dark+`, `Light+`, `Monokai`, `Dracula`, `One Dark`,
    /// `Solarized Dark`, `Solarized Light`, `GitHub Light`, `GitHub Dark`, `Nord`); vazio:
    /// segue o tema do app.
    pub fn color_theme(&self) -> String {
        self.memo.color_theme.clone()
    }

    pub fn set_color_theme(&mut self, name: String) {
        self.memo.color_theme = name;
    }

    pub fn folding(&self) -> bool {
        self.memo.folding
    }

    pub fn set_folding(&mut self, on: bool) {
        self.memo.folding = on;
        if !on {
            self.memo.unfold_all();
        }
    }
}

impl Component for CodeEditor {
    fn class_name(&self) -> &'static str {
        "CodeEditor"
    }

    fn inner(&self) -> Option<&dyn Component> {
        Some(&self.memo)
    }

    fn inner_mut(&mut self) -> Option<&mut dyn Component> {
        Some(&mut self.memo)
    }

    properties! {
        ; events: on_word_click, on_complete_request, on_signature_request, on_hover_index
        ; computed: minimap: bool => minimap / set_minimap,
            folding: bool => folding / set_folding,
            color_theme: String => color_theme / set_color_theme
    }
}
