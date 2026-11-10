//! O apoio do acesso tipado gerado (`self.edtNome.set_text(ui, ...)`): cada componente de um
//! form é um handle com o `ComponentId` dele, resolvido uma vez ao abrir o form. Ler e
//! gravar não falham calados: a falha (o componente foi liberado, o valor não serve) aparece
//! no log em modo dev e vai para a atividade do app (a gravação, o relato). Quem quer tratar
//! usa os `try_` do [`Handle`].

use crate::activity::Activity;
use crate::component::ComponentId;
use crate::error::Result;
use crate::prop::PropValue;
use crate::ui::Ui;

/// Um componente de um form, pelo id (o que o código gerado cria para cada um).
pub trait Handle: Copy {
    fn id(&self) -> ComponentId;

    /// Lê a propriedade, com o erro (componente liberado, propriedade que a classe não tem).
    fn try_get(&self, ui: &Ui, prop: &str) -> Result<PropValue> {
        ui.get_prop(self.id(), prop)
    }

    /// Grava a propriedade, com o erro (o valor não serve, o componente foi liberado).
    fn try_set(&self, ui: &mut Ui, prop: &str, value: impl Into<PropValue>) -> Result<()> {
        ui.set_prop(self.id(), prop, value)
    }

    /// Chama um método do componente, com o erro.
    fn try_invoke(&self, ui: &mut Ui, method: &str, args: &[PropValue]) -> std::result::Result<PropValue, String> {
        ui.invoke(self.id(), method, args)
    }
}

/// Lê a propriedade; a falha vai para o log em modo dev (a leitura não tem como avisar a
/// atividade: ela só vê o `Ui`).
pub fn read(ui: &Ui, id: ComponentId, prop: &str) -> Option<PropValue> {
    match ui.get_prop(id, prop) {
        Ok(v) => Some(v),
        Err(e) => {
            if cfg!(debug_assertions) {
                crate::stderr_line!("aylis: ler {}.{prop}: {e}", ui.path_of(id));
            }
            None
        }
    }
}

/// Grava a propriedade; a falha vai para o log em modo dev e para a atividade do app.
pub fn write(ui: &mut Ui, id: ComponentId, prop: &str, value: impl Into<PropValue>) {
    if let Err(e) = ui.set_prop(id, prop, value) {
        failed(ui, id, &format!("gravar {prop}"), &e.to_string());
    }
}

/// Chama um método; a falha vai para o log em modo dev e para a atividade do app.
pub fn call(ui: &mut Ui, id: ComponentId, method: &str, args: &[PropValue]) -> Option<PropValue> {
    match ui.invoke(id, method, args) {
        Ok(v) => Some(v),
        Err(e) => {
            failed(ui, id, &format!("chamar {method}"), &e.to_string());
            None
        }
    }
}

/// Chama um método e devolve o erro (o que os métodos do acesso tipado usam: `save_to`
/// volta `Err` com o motivo, não um `false`); a falha também vai para o log em modo dev e
/// para a atividade do app.
pub fn try_call(ui: &mut Ui, id: ComponentId, method: &str, args: &[PropValue]) -> std::result::Result<PropValue, String> {
    ui.invoke(id, method, args).inspect_err(|e| failed(ui, id, &format!("chamar {method}"), e))
}

fn failed(ui: &mut Ui, id: ComponentId, what: &str, error: &str) {
    let message = format!("{what} em {}: {error}", ui.path_of(id));
    if cfg!(debug_assertions) {
        crate::stderr_line!("aylis: {message}");
    }
    ui.report(&Activity::Error { message: &message });
}

/// O valor como texto (a cor em `#RRGGBB`, a referência pelo caminho).
pub fn text(ui: &Ui, value: Option<PropValue>) -> String {
    match value {
        Some(PropValue::Color(c)) => c.to_hex(),
        Some(PropValue::Component(Some(id))) => ui.path_of(id),
        Some(v) => v.to_str().map(str::to_string).unwrap_or_default(),
        None => String::new(),
    }
}

/// Resolve o componente `name` de classe `class` no form `root` para a struct do form
/// gerada; o que não bate vai para `problems` (e o form não abre).
pub fn resolve(ui: &Ui, root: ComponentId, name: &str, class: &str, problems: &mut Vec<String>) -> ComponentId {
    match ui.find_in(root, name) {
        Some(id) => {
            let actual = ui.class_name(id).unwrap_or_default();
            if actual != class {
                problems.push(format!("{name} é {actual} no form e {class} no código"));
            }
            id
        }
        None => {
            problems.push(format!("falta o componente {name} ({class})"));
            root
        }
    }
}
