//! O controle `Barcode` (código de barras e QR Code) e o provedor padrão dele.

use std::cell::RefCell;

use aylis_core::providers::{BarcodeImage, BarcodeOptions, BarcodeProvider, ErrorCorrection, Provided};
use aylis_core::{Brush, Canvas, Color, Component, ComponentId, ControlBase, EventCtx, PaintCtx, RectF, TextAlign, properties};

use crate::barcodes::BuiltinBarcodes;

/// Código de barras ou QR Code: o dado em `text` (no relatório, um campo entre colchetes vira o valor de cada
/// registro), o formato em `format` (`code128`, `ean13`, `ean8`, `upca`, `code39`, `itf`,
/// `qr`; no QR, a correção de erro em `error_correction`). Quem gera é o provedor ligado em `provider` (um formato que o Aylis não tem vem de
/// outro provedor); sem ele, o do Aylis. As barras são escuras sobre o fundo claro mesmo no
/// tema escuro (o leitor precisa do contraste), e `fill` e `text_color` trocam as duas cores.
pub struct Barcode {
    pub base: ControlBase,
    /// O dado codificado (no relatório, um campo entre colchetes vira o valor de cada
    /// registro).
    pub text: String,
    /// O formato: `code128`, `ean13`, `ean8`, `upca`, `code39`, `itf` ou `qr`.
    pub format: String,
    /// O provedor que gera o código (um formato que o Aylis não tem vem de outro); vazio: o
    /// do Aylis.
    pub provider: Option<ComponentId>,
    /// O texto legível embaixo das barras (nos códigos de barras; o QR não tem).
    pub show_text: bool,
    /// A correção de erro do QR Code (`low`, `medium`, `quartile`, `high`: L, M, Q e H), que
    /// vai para o provedor; os códigos de barras não têm.
    pub error_correction: ErrorCorrection,
    /// O código pronto (ou o motivo de não sair), refeito quando o dado, o formato ou o
    /// provedor mudam (a pintura não tem o `Ui` para achar o provedor).
    image: RefCell<Option<Result<BarcodeImage, String>>>,
}

impl Default for Barcode {
    fn default() -> Self {
        Self { base: ControlBase::new(220.0, 80.0), text: "123456789012".into(), format: "code128".into(), provider: None, show_text: true, error_correction: ErrorCorrection::default(), image: RefCell::new(None) }
    }
}

impl Barcode {
    pub fn new(format: &str, text: &str) -> Self {
        Self { format: format.into(), text: text.into(), ..Default::default() }
    }

    /// O código (ou o motivo), pelo provedor dado ou pelo do Aylis.
    pub fn encode(&self, provider: Option<&dyn BarcodeProvider>) -> Result<BarcodeImage, String> {
        provider.unwrap_or(&BuiltinBarcodes).encode(&self.format, &self.text, &self.options())
    }

    /// As opções que vão para o provedor.
    pub fn options(&self) -> BarcodeOptions {
        BarcodeOptions { error_correction: self.error_correction }
    }

    fn refresh(&self, provider: Option<&dyn BarcodeProvider>) {
        *self.image.borrow_mut() = Some(self.encode(provider));
    }
}

impl Component for Barcode {
    aylis_core::leaf_control!("Barcode");

    properties! {
        text: String => "Data",
        format: String => "Data",
        error_correction: ErrorCorrection => "Data",
        provider: Option<ComponentId> => "Data",
        show_text: bool => "Appearance",
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if matches!(name, "text" | "format" | "error_correction" | "provider") {
            match self.provider.and_then(|p| ctx.ui.provided(p)) {
                Some(Provided::Barcode(b)) => self.refresh(Some(b)),
                _ => self.refresh(None),
            }
            ctx.repaint();
        }
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.provider == Some(freed) {
            self.provider = None;
            *self.image.borrow_mut() = None;
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        if self.image.borrow().is_none() {
            self.refresh(None);
        }
        let r = ctx.rect;
        let light = ctx.colors.fill.unwrap_or(Color::WHITE);
        let dark = ctx.colors.text.unwrap_or(Color::BLACK);
        canvas.fill_rect(r, 0.0, &Brush::Solid(light));
        let image = self.image.borrow();
        match image.as_ref() {
            Some(Ok(BarcodeImage::Linear { modules, text })) => {
                let mut style = ctx.text_style();
                style.color = dark;
                style.h_align = TextAlign::Center;
                let text_h = if self.show_text && !text.is_empty() { (style.font.size * 1.4).min(r.height * 0.35) } else { 0.0 };
                // A margem branca (10 módulos de cada lado) e a largura de cada módulo.
                let quiet = 10.0;
                let unit = r.width / (modules.len() as f32 + quiet * 2.0);
                let bars = RectF::new(r.x + quiet * unit, r.y + 4.0, unit * modules.len() as f32, (r.height - text_h - 8.0).max(1.0));
                let mut i = 0;
                while i < modules.len() {
                    if modules[i] {
                        let start = i;
                        while i < modules.len() && modules[i] {
                            i += 1;
                        }
                        let x = bars.x + start as f32 * unit;
                        canvas.fill_rect(RectF::new(x, bars.y, (i - start) as f32 * unit, bars.height), 0.0, &Brush::Solid(dark));
                    } else {
                        i += 1;
                    }
                }
                if text_h > 0.0 {
                    canvas.draw_text(text, RectF::new(r.x, bars.bottom(), r.width, text_h + 4.0), &style);
                }
            }
            Some(Ok(BarcodeImage::Matrix { width, modules })) => {
                // Quadrado, no centro, com a margem de 4 módulos.
                let side = r.width.min(r.height);
                let unit = side / (*width as f32 + 8.0);
                let x0 = r.x + (r.width - unit * *width as f32) / 2.0;
                let y0 = r.y + (r.height - unit * *width as f32) / 2.0;
                for (k, &on) in modules.iter().enumerate() {
                    if on {
                        let (col, row) = (k % width, k / width);
                        // Um pouco maior que o módulo: sem fresta entre os vizinhos.
                        canvas.fill_rect(RectF::new(x0 + col as f32 * unit, y0 + row as f32 * unit, unit + 0.4, unit + 0.4), 0.0, &Brush::Solid(dark));
                    }
                }
            }
            Some(Err(reason)) => {
                let mut style = ctx.text_style();
                style.color = ctx.theme.color("danger");
                style.h_align = TextAlign::Center;
                style.wrap = true;
                canvas.draw_text(&format!("⚠ {reason}"), r.inflate(-6.0), &style);
            }
            None => {}
        }
    }
}

/// O provedor de códigos de barras do Aylis como componente (para ligar explicitamente; sem
/// provedor ligado, o `Barcode` já usa este).
#[derive(Default)]
pub struct AylisBarcodeProvider;

impl Component for AylisBarcodeProvider {
    fn class_name(&self) -> &'static str {
        "AylisBarcodeProvider"
    }


    fn provided(&self) -> Option<Provided<'_>> {
        Some(Provided::Barcode(&BuiltinBarcodes))
    }
}
