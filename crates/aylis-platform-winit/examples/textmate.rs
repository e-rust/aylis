//! CodeEditor com a gramática oficial do VS Code (a do Rust, a mesma que a IDE embute) e as
//! regras por escopo do Dark+. `AYLIS_AUTO_CLOSE=1` fecha sozinho (para a captura do
//! `AYLIS_SNAPSHOT`).
//!
//! ```text
//! cargo run -p aylis-platform-winit --example textmate --features cpu
//! ```

use aylis_controls::*;
use aylis_core::*;

const GRAMMAR: &str = include_str!("../../aylis-ide/resources/grammars/rust.tmLanguage.json");
const CONFIG: &str = include_str!("../../aylis-ide/resources/grammars/rust.language-configuration.json");
const DARK_PLUS: &str = include_str!("../../aylis-ide/resources/themes/dark_plus.json");
const DARK_VS: &str = include_str!("../../aylis-ide/resources/themes/dark_vs.json");

const CODE: &str = r#"// Um exemplo de Rust com a gramática do VS Code.
use std::collections::HashMap;

/// Soma os valores por nome.
#[derive(Debug, Clone)]
pub struct Totais {
    por_nome: HashMap<String, f64>,
}

impl Totais {
    pub fn somar(&mut self, nome: &str, valor: f64) -> f64 {
        let total = self.por_nome.entry(nome.to_string()).or_insert(0.0);
        *total += valor * 2.5;
        println!("{nome}: {total:.2}");
        *total
    }
}

fn main() {
    let mut t = Totais { por_nome: HashMap::new() };
    let s = r"texto cru \n";
    t.somar("vendas", 42.0);
}
"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    textmate::register_grammar("rust", GRAMMAR)?;
    textmate::register_language_config("rust", CONFIG)?;
    textmate_theme::register_theme(DARK_PLUS, "Dark+", &|_| Some(DARK_VS.to_string()))?;
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Gramática TextMate", 760.0, 560.0));
    let mut editor = CodeEditor::new(CODE).with_syntax("rust");
    editor.set_color_theme("Dark+".into());
    let editor = ui.add(editor, form);
    ui.control_mut(editor).unwrap().set_align(Align::Client);
    if std::env::var("AYLIS_AUTO_CLOSE").is_ok() {
        let timer = ui.add(Timer::new(1500), form);
        ui.on(timer, "on_timer", |ui, _, _| ui.request_close());
    }
    aylis_platform_winit::run(ui, form)
}
