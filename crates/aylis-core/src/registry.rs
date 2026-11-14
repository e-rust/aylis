//! Registro de classes: permite criar componentes pelo
//! nome da classe, o que é usado pelos arquivos de layout, pela paleta da IDE e pelo
//! controle remoto.

use crate::component::Component;
use crate::prop::PropValue;

/// Informações de uma classe registrada.
#[derive(Clone, Copy, Debug)]
pub struct ClassInfo {
    pub name: &'static str,
    /// Aba da paleta de componentes ("Standard", "Layouts", "Data"...).
    pub category: &'static str,
    pub create: fn() -> Box<dyn Component>,
}

/// Um subtipo de uma classe (na paleta, debaixo dela): o mesmo componente com uma
/// variante de estilo do tema e valores iniciais próprios. `Button.icon` é um `Button`
/// com `style = "icon"`, sem texto, com ícone e quadrado.
#[derive(Clone, Debug, PartialEq)]
pub struct VariantInfo {
    pub class: String,
    /// Nome da variante (o `style` do controle e o sufixo no tema: `Button.icon`).
    pub name: String,
    /// Texto da paleta ("Botão de ícone").
    pub caption: String,
    /// Propriedades aplicadas ao criar (antes do `style`).
    pub props: Vec<(String, PropValue)>,
}

#[derive(Default)]
pub struct ClassRegistry {
    classes: Vec<ClassInfo>,
    variants: Vec<VariantInfo>,
}

impl ClassRegistry {
    /// Registra (ou substitui) uma classe.
    pub fn register(&mut self, info: ClassInfo) {
        match self.classes.iter_mut().find(|c| c.name.eq_ignore_ascii_case(info.name)) {
            Some(existing) => *existing = info,
            None => self.classes.push(info),
        }
    }

    /// Procura pelo nome, sem diferenciar maiúsculas.
    pub fn get(&self, name: &str) -> Option<&ClassInfo> {
        self.classes.iter().find(|c| c.name.eq_ignore_ascii_case(name))
    }

    pub fn iter(&self) -> impl Iterator<Item = &ClassInfo> {
        self.classes.iter()
    }

    /// Registra (ou substitui) um subtipo.
    pub fn register_variant(&mut self, info: VariantInfo) {
        let same = |v: &VariantInfo| v.class.eq_ignore_ascii_case(&info.class) && v.name.eq_ignore_ascii_case(&info.name);
        match self.variants.iter_mut().find(|v| same(v)) {
            Some(existing) => *existing = info,
            None => self.variants.push(info),
        }
    }

    /// Os subtipos registrados de uma classe, na ordem de registro.
    pub fn variants_of(&self, class: &str) -> impl Iterator<Item = &VariantInfo> {
        let class = class.to_string();
        self.variants.iter().filter(move |v| v.class.eq_ignore_ascii_case(&class))
    }

    pub fn variant(&self, class: &str, name: &str) -> Option<&VariantInfo> {
        self.variants.iter().find(|v| v.class.eq_ignore_ascii_case(class) && v.name.eq_ignore_ascii_case(name))
    }
}

/// Separa `"Button.icon"` em `("Button", Some("icon"))`.
pub fn split_variant(name: &str) -> (&str, Option<&str>) {
    match name.split_once('.') {
        Some((class, variant)) if !variant.is_empty() => (class, Some(variant)),
        _ => (name, None),
    }
}

/// Função de criação usada por [`Ui::register_class`](crate::Ui::register_class).
pub(crate) fn create_default<T: Component + Default>() -> Box<dyn Component> {
    Box::new(T::default())
}
