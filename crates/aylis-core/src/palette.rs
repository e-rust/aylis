//! Cores com nome: as do tema (`primary`, `text_3`...) e as do projeto (a paleta do
//! `.aylisproj`), usadas em qualquer propriedade de cor e nos estilos.
//!
//! Uma cor pode ser escrita como:
//! - `#RRGGBB` / `#RRGGBBAA` — a cor em si;
//! - `nome` — uma cor do tema em uso ou da paleta do projeto (`success`, `brand`);
//! - `nome/NN` — a mesma cor com NN % de opacidade (`text/85`, `primary/20`, `white/12`);
//! - `white`, `black`, `transparent`.
//!
//! A paleta do projeto entra em todo tema que o [`Ui`](crate::Ui) usa (por cima das
//! cores do tema, que ela pode citar). Uma entrada pode ter um valor por modo: no tema
//! escuro vale `dark`, no claro `light`.
//!
//! Numa propriedade de cor com um nome, o `Ui` lembra o nome: ao trocar de tema a cor é
//! recalculada, e o form salvo guarda o nome (não o hex).

use std::collections::HashMap;

use crate::{Color, Theme};

/// Uma cor da paleta do projeto: a mesma nos dois modos ou uma por modo.
#[derive(Clone, Debug, PartialEq)]
pub struct PaletteColor {
    pub dark: String,
    pub light: String,
}

impl PaletteColor {
    pub fn same(spec: impl Into<String>) -> Self {
        let spec = spec.into();
        Self { dark: spec.clone(), light: spec }
    }

    pub fn by_mode(dark: impl Into<String>, light: impl Into<String>) -> Self {
        Self { dark: dark.into(), light: light.into() }
    }

    /// O valor para o modo do tema.
    pub fn for_theme(&self, dark: bool) -> &str {
        if dark { &self.dark } else { &self.light }
    }
}

/// A paleta do projeto, na ordem em que foi escrita (uma entrada pode citar as anteriores).
pub type Palette = Vec<(String, PaletteColor)>;

/// Interpreta uma cor escrita com [`resolve`] nas cores de `colors`.
pub fn resolve_in(colors: &HashMap<String, Color>, spec: &str) -> Option<Color> {
    let spec = spec.trim();
    let (name, alpha) = match spec.rsplit_once('/') {
        Some((name, pct)) => (name.trim(), Some(pct.trim().trim_end_matches('%').parse::<f32>().ok()? / 100.0)),
        None => (spec, None),
    };
    // O hex também aceita a opacidade (`#2563EB/15`), como os nomes (relatório do InsightDB,
    // 11/11: só valia para os nomes).
    let base = match name {
        _ if name.starts_with('#') => Color::from_hex(name),
        "white" => Some(Color::WHITE),
        "black" => Some(Color::rgb(0, 0, 0)),
        "transparent" => Some(Color::TRANSPARENT),
        _ => colors.get(name).copied().or_else(|| colors.get(&name.replace('-', "_")).copied()),
    }?;
    Some(match alpha {
        Some(a) => base.with_opacity(a.clamp(0.0, 1.0)),
        None => base,
    })
}

/// É um nome (ou nome/NN), não um hex? Vazio não é.
pub fn is_named(spec: &str) -> bool {
    let s = spec.trim();
    !s.is_empty() && !s.starts_with('#')
}

impl Theme {
    /// Uma cor escrita como `#hex`, `nome` ou `nome/NN` (veja o módulo [`palette`](crate::palette)).
    pub fn resolve_color(&self, spec: &str) -> Option<Color> {
        resolve_in(&self.colors, spec)
    }

    /// Os nomes de cor que os estilos citam e o tema não tem: `(estilo, nome)`, em ordem. Um
    /// nome desconhecido pinta transparente, sem erro (o relatório do Tabernáculo, C07); a
    /// verificação do projeto avisa.
    pub fn unknown_color_tokens(&self) -> Vec<(String, String)> {
        let mut out = Vec::new();
        for (key, style) in &self.styles {
            for props in [&style.base, &style.hover, &style.pressed, &style.focused, &style.checked, &style.disabled] {
                for c in [&props.fill, &props.text, &props.border_color].into_iter().flatten() {
                    if let crate::theme::ColorRef::Token(name) = c
                        && name != "transparent"
                        && self.resolve_color(name).is_none()
                        && !out.iter().any(|(k, n)| k == key && n == name)
                    {
                        out.push((key.clone(), name.clone()));
                    }
                }
            }
        }
        out.sort();
        out
    }

    /// Põe a paleta do projeto no tema (no modo dele). Uma entrada pode citar outra, em
    /// qualquer ordem; a que não se resolve (nome desconhecido) fica de fora.
    pub fn apply_palette(&mut self, palette: &Palette) {
        let mut pending: Vec<&(String, PaletteColor)> = palette.iter().collect();
        while !pending.is_empty() {
            let before = pending.len();
            pending.retain(|(name, color)| {
                let spec = color.for_theme(self.dark);
                // Um nome que é ele mesmo (`"primary": "primary/90"`) olha a cor do tema.
                match self.resolve_color(spec) {
                    Some(c) => {
                        self.colors.insert(name.clone(), c);
                        false
                    }
                    None => true,
                }
            });
            if pending.len() == before {
                break;
            }
        }
    }
}
