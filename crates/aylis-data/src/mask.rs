//! Máscaras de edição.
//!
//! | Caractere | Aceita |
//! |---|---|
//! | `0` | dígito (obrigatório) |
//! | `9` | dígito (opcional) |
//! | `L` | letra (obrigatória) |
//! | `l` | letra (opcional) |
//! | `A` | letra ou dígito (obrigatório) |
//! | `a` | letra ou dígito (opcional) |
//! | `C` | qualquer caractere (obrigatório) |
//! | `c` | qualquer caractere (opcional) |
//! | `>` / `<` | o que vem depois em maiúsculas / minúsculas |
//! | `\` | o próximo é literal |
//! | outros | literais (`.`, `-`, `/`, `(`...) |
//!
//! Depois do formato, `;0` grava só o que foi digitado (sem os literais) e `;1` (o padrão)
//! grava com eles: `000.000.000-00;0` guarda `12345678901` e mostra `123.456.789-01`.
//!
//! Prontas: [`CPF`], [`CNPJ`], [`CEP`], [`PHONE`].

pub const CPF: &str = "000.000.000-00;0";
pub const CNPJ: &str = "00.000.000/0000-00;0";
pub const CEP: &str = "00000-000;0";
pub const PHONE: &str = "(00) 00000-0000;0";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Slot {
    Literal(char),
    Input { kind: char, required: bool, case: Case },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Case {
    Keep,
    Upper,
    Lower,
}

/// Uma máscara lida.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mask {
    slots: Vec<Slot>,
    /// Grava os literais junto (`;1`).
    pub save_literals: bool,
}

impl Mask {
    pub fn parse(mask: &str) -> Option<Mask> {
        if mask.trim().is_empty() {
            return None;
        }
        let mut parts = mask.split(';');
        let format = parts.next().unwrap_or_default();
        let save_literals = parts.next().is_none_or(|s| s.trim() != "0");
        let mut slots = Vec::new();
        let mut case = Case::Keep;
        let mut chars = format.chars();
        while let Some(c) = chars.next() {
            match c {
                '\\' => {
                    if let Some(next) = chars.next() {
                        slots.push(Slot::Literal(next));
                    }
                }
                '>' => case = Case::Upper,
                '<' => case = Case::Lower,
                '0' | 'L' | 'A' | 'C' => slots.push(Slot::Input { kind: c.to_ascii_lowercase(), required: true, case }),
                '9' => slots.push(Slot::Input { kind: '0', required: false, case }),
                'l' | 'a' | 'c' => slots.push(Slot::Input { kind: c, required: false, case }),
                other => slots.push(Slot::Literal(other)),
            }
        }
        Some(Mask { slots, save_literals })
    }

    fn accepts(kind: char, c: char) -> bool {
        match kind {
            '0' => c.is_ascii_digit(),
            'l' => c.is_alphabetic(),
            'a' => c.is_alphanumeric(),
            _ => !c.is_control(),
        }
    }

    /// Só os caracteres digitados (sem literais).
    pub fn raw(&self, text: &str) -> String {
        let literals: Vec<char> = self.slots.iter().filter_map(|s| if let Slot::Literal(c) = s { Some(*c) } else { None }).collect();
        text.chars().filter(|c| !literals.contains(c) && !c.is_whitespace()).collect()
    }

    /// Encaixa o que foi digitado na máscara (para mostrar enquanto digita): os
    /// literais aparecem sozinhos conforme os dígitos chegam; o que não serve é ignorado.
    pub fn apply(&self, typed: &str) -> String {
        let mut out = String::new();
        let raw = self.raw(typed);
        let mut input = raw.chars().peekable();
        let mut pending = String::new();
        for slot in &self.slots {
            match slot {
                Slot::Literal(c) => pending.push(*c),
                Slot::Input { kind, case, .. } => {
                    // Pula o que não serve para esta posição.
                    while input.peek().is_some_and(|&c| !Self::accepts(*kind, c)) {
                        input.next();
                    }
                    let Some(c) = input.next() else { break };
                    out.push_str(&pending);
                    pending.clear();
                    match case {
                        Case::Upper => out.extend(c.to_uppercase()),
                        Case::Lower => out.extend(c.to_lowercase()),
                        Case::Keep => out.push(c),
                    }
                }
            }
        }
        out
    }

    /// Está completa (todas as posições obrigatórias preenchidas)?
    pub fn is_complete(&self, text: &str) -> bool {
        let formatted = self.apply(text);
        let filled = self.raw(&formatted).chars().count();
        let required = self.slots.iter().filter(|s| matches!(s, Slot::Input { required: true, .. })).count();
        filled >= required
    }

    /// O valor a gravar: com ou sem literais (`;1`/`;0`). Vazio continua vazio.
    pub fn stored(&self, text: &str) -> String {
        let formatted = self.apply(text);
        if self.save_literals { formatted } else { self.raw(&formatted) }
    }

    /// Texto para mostrar de um valor gravado (com ou sem literais).
    pub fn display(&self, stored: &str) -> String {
        self.apply(stored)
    }
}
