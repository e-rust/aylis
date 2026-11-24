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

/// Uma cor para o acesso tipado (`set_fill`): a cor ([`crate::Color`]) ou um nome do tema
/// ([`ColorRef::token`]), que acompanha a troca de tema.
#[derive(Clone, Debug, PartialEq)]
pub struct ColorRef(PropValue);

impl ColorRef {
    /// Um nome de cor do tema ou do projeto (`primary`, `primary/40`).
    pub fn token(name: &str) -> Self {
        Self(PropValue::String(name.trim().to_string()))
    }
}

impl From<crate::Color> for ColorRef {
    fn from(c: crate::Color) -> Self {
        Self(PropValue::Color(c))
    }
}

impl From<ColorRef> for PropValue {
    fn from(c: ColorRef) -> Self {
        c.0
    }
}

/// Uma referência a outro componente no acesso tipado (`set_popup_menu(ui, self.pmMenu)`):
/// um handle qualquer, um id ou [`Ref::NONE`] (nenhum).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Ref(pub Option<ComponentId>);

impl Ref {
    pub const NONE: Ref = Ref(None);
}

impl<H: Handle> From<H> for Ref {
    fn from(h: H) -> Self {
        Ref(Some(h.id()))
    }
}

impl From<ComponentId> for Ref {
    fn from(id: ComponentId) -> Self {
        Ref(Some(id))
    }
}

impl From<Option<ComponentId>> for Ref {
    fn from(id: Option<ComponentId>) -> Self {
        Ref(id)
    }
}

impl From<Ref> for PropValue {
    fn from(r: Ref) -> Self {
        PropValue::Component(r.0)
    }
}

/// Lê um enum (o primeiro valor, se o que está lá não for um dos nomes).
pub fn read_enum<E: crate::prop::PropEnum>(ui: &Ui, id: ComponentId, prop: &str) -> E {
    read(ui, id, prop).and_then(|v| v.to_enum::<E>().ok()).or_else(|| E::from_index(0)).expect("enum sem valores")
}

/// O `font_weight` do acesso tipado é o [`crate::FontWeight`] do núcleo: `None` é o `default`
/// (o peso do pai ou do tema), e os outros nomes seguem [`crate::FONT_WEIGHTS`]. Assim o app usa
/// um tipo só (`Some(FontWeight::SemiBold)`), e não um enum gerado parecido.
impl crate::prop::PropEnum for Option<crate::FontWeight> {
    const NAMES: &'static [&'static str] = &crate::FONT_WEIGHTS;

    fn from_index(index: usize) -> Option<Self> {
        use crate::FontWeight::*;
        match index {
            0 => Some(None),
            i => [Thin, Light, Normal, Medium, SemiBold, Bold, Black].get(i - 1).map(|w| Some(*w)),
        }
    }

    fn index(self) -> usize {
        self.map_or(0, |w| w as usize + 1)
    }
}

/// Lê um conjunto de enums (os nomes que não são do enum ficam de fora).
pub fn read_set<E: crate::prop::PropEnum>(ui: &Ui, id: ComponentId, prop: &str) -> Vec<E> {
    let names: Vec<String> = match read(ui, id, prop) {
        Some(PropValue::Set(names)) => names,
        Some(PropValue::String(s) | PropValue::Enum(s)) => s.split(',').map(|p| p.trim().to_string()).filter(|p| !p.is_empty()).collect(),
        _ => Vec::new(),
    };
    names.iter().filter_map(|n| E::from_name(n)).collect()
}

/// Um conjunto de enums como a tela grava.
pub fn set_value<E: crate::prop::PropEnum>(values: &[E]) -> PropValue {
    PropValue::Set(values.iter().map(|v| v.name().to_string()).collect())
}

fn read_text(ui: &Ui, id: ComponentId, prop: &str) -> String {
    read(ui, id, prop).and_then(|v| v.to_str().ok().map(str::to_string)).unwrap_or_default()
}

/// Lê uma data (`None`: vazia ou inválida).
pub fn read_date(ui: &Ui, id: ComponentId, prop: &str) -> Option<crate::Date> {
    crate::Date::parse_iso(&read_text(ui, id, prop))
}

/// Lê uma hora (`None`: vazia ou inválida).
pub fn read_time(ui: &Ui, id: ComponentId, prop: &str) -> Option<crate::Time> {
    crate::Time::parse_iso(&read_text(ui, id, prop))
}

/// Lê o dia e a hora (`None`: vazio ou inválido).
pub fn read_datetime(ui: &Ui, id: ComponentId, prop: &str) -> Option<crate::DateTime> {
    crate::DateTime::parse_iso(&read_text(ui, id, prop))
}

/// A data como a tela grava (`None`: vazia).
pub fn date_value(value: Option<crate::Date>) -> PropValue {
    PropValue::String(value.map(|d| d.to_string()).unwrap_or_default())
}

/// A hora como a tela grava: `HH:MM`, com os segundos só quando há (`None`: vazia).
pub fn time_value(value: Option<crate::Time>) -> PropValue {
    PropValue::String(value.map(time_text).unwrap_or_default())
}

/// O dia e a hora como a tela grava (`2026-11-12T14:30`; `None`: vazio).
pub fn datetime_value(value: Option<crate::DateTime>) -> PropValue {
    PropValue::String(value.map(|d| format!("{}T{}", d.date, time_text(d.time))).unwrap_or_default())
}

fn time_text(t: crate::Time) -> String {
    if t.second() == 0 { format!("{:02}:{:02}", t.hour(), t.minute()) } else { format!("{:02}:{:02}:{:02}", t.hour(), t.minute(), t.second()) }
}

/// Os itens de uma lista separada por `;` (vazia: nenhum).
fn items(text: &str) -> impl Iterator<Item = &str> {
    text.split(';').map(str::trim).filter(|s| !s.is_empty())
}

/// Lê uma lista de textos (`a;b;c`) como os controles a leem: texto vazio é nenhum item, e
/// fora isso cada pedaço é um item, os vazios e os espaços inclusive (um `ComboBox` com o item
/// "nenhum" vazio no começo, e o `item_index` alinhado com a lista).
pub fn read_list(ui: &Ui, id: ComponentId, prop: &str) -> Vec<String> {
    split_list(&read_text(ui, id, prop))
}

/// Os itens de um texto `a;b;c` sem descartar nada (vazio: nenhum).
pub fn split_list(text: &str) -> Vec<String> {
    if text.is_empty() { Vec::new() } else { text.split(';').map(str::to_string).collect() }
}

/// Lê uma lista de números (`0.4;0.9`; o que não é número fica de fora).
pub fn read_numbers(ui: &Ui, id: ComponentId, prop: &str) -> Vec<f64> {
    items(&read_text(ui, id, prop)).filter_map(|s| s.parse().ok()).collect()
}

/// Lê uma lista de cores (`#FF0000;#00AA00`; o que não é cor fica de fora).
pub fn read_colors(ui: &Ui, id: ComponentId, prop: &str) -> Vec<crate::Color> {
    items(&read_text(ui, id, prop)).filter_map(crate::Color::from_hex).collect()
}

/// Uma lista de textos como a tela grava.
pub fn list_value<S: AsRef<str>>(values: &[S]) -> PropValue {
    PropValue::String(values.iter().map(|s| s.as_ref()).collect::<Vec<_>>().join(";"))
}

/// Uma lista de números como a tela grava.
pub fn numbers_value(values: &[f64]) -> PropValue {
    PropValue::String(values.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(";"))
}

/// Uma lista de cores como a tela grava.
pub fn colors_value(values: &[crate::Color]) -> PropValue {
    PropValue::String(values.iter().map(|c| c.to_hex()).collect::<Vec<_>>().join(";"))
}

/// Lê a sombra (`None`: sem sombra).
pub fn read_shadow(ui: &Ui, id: ComponentId, prop: &str) -> Option<crate::Shadow> {
    crate::Shadow::parse(&read_text(ui, id, prop))
}

/// A sombra como a tela grava (`None`: sem sombra).
pub fn shadow_value(value: Option<crate::Shadow>) -> PropValue {
    PropValue::String(value.map(|s| s.to_spec()).unwrap_or_default())
}

/// Lê a curva (a linear, se o texto não é uma curva).
pub fn read_easing(ui: &Ui, id: ComponentId, prop: &str) -> crate::Easing {
    crate::Easing::parse(&read_text(ui, id, prop)).unwrap_or_default()
}

/// A curva como a tela grava (`out_quad`, `cubic-bezier(...)`).
pub fn easing_value(value: crate::Easing) -> PropValue {
    PropValue::String(value.to_spec())
}

/// Lê os pontos `x,y` separados por espaço (os que não são pontos ficam de fora).
pub fn read_points(ui: &Ui, id: ComponentId, prop: &str) -> Vec<crate::PointF> {
    read_text(ui, id, prop)
        .split_whitespace()
        .filter_map(|p| {
            let (x, y) = p.split_once(',')?;
            Some(crate::PointF::new(x.trim().parse().ok()?, y.trim().parse().ok()?))
        })
        .collect()
}

/// Os pontos como a tela grava (`0.5,0 1,1 0,1`).
pub fn points_value(values: &[crate::PointF]) -> PropValue {
    PropValue::String(values.iter().map(|p| format!("{},{}", p.x, p.y)).collect::<Vec<_>>().join(" "))
}

/// Lê a referência a outro componente (`None`: nenhum).
pub fn read_ref(ui: &Ui, id: ComponentId, prop: &str) -> Option<ComponentId> {
    read(ui, id, prop).and_then(|v| v.to_component().ok().flatten())
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
        // `margins`/`padding` e `anchors` como o `set_` aceita: "12, 4, 12, 0" e "left,top".
        Some(PropValue::Bounds(b)) => format!("{}, {}, {}, {}", b.left, b.top, b.right, b.bottom),
        Some(PropValue::Set(names)) => names.join(","),
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

#[cfg(test)]
mod tests {
    use super::*;

    /// O item vazio do começo e os espaços continuam na lista: o `item_index` fica alinhado.
    #[test]
    fn the_list_keeps_the_empty_items_and_the_spaces() {
        let text = match list_value(&["", "A", " B "]) {
            PropValue::String(s) => s,
            _ => unreachable!(),
        };
        assert_eq!(split_list(&text), ["", "A", " B "]);
        assert!(split_list("").is_empty());
    }

    /// Os nomes do `font_weight` e o `FontWeight` do núcleo, ida e volta (`default` é `None`).
    #[test]
    fn the_font_weight_names_match_the_core_type() {
        use crate::prop::PropEnum;
        for (i, name) in crate::FONT_WEIGHTS.iter().enumerate() {
            let w = <Option<crate::FontWeight>>::from_index(i).unwrap();
            assert_eq!(w.name(), *name);
        }
        assert_eq!(<Option<crate::FontWeight>>::from_name("semibold"), Some(Some(crate::FontWeight::SemiBold)));
        assert_eq!(<Option<crate::FontWeight>>::from_name("default"), Some(None));
    }
}
