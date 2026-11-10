//! # aylis-vector
//!
//! Desenhos vetoriais desenhados pelo próprio `Canvas` do aylis (iguais no Skia, no
//! Direct2D e no navegador, sem biblioteca nativa):
//!
//! - [`Lottie`]: animações Lottie (After Effects/Bodymovin). O pacote padrão da IDE são
//!   os emojis animados Noto do Google (`"noto:fogo"`);
//! - [`Svg`]: imagens SVG (ícones, logos, ilustrações), com `currentColor` seguindo o tema.
//!
//! [`Animation`] e [`SvgImage`] também servem sozinhos, para desenhar num `paint` próprio.

pub mod controls;
pub mod geom;
pub mod lottie;
pub mod svg;

pub use controls::{Lottie, Svg, lottie_path};
pub use lottie::Animation;
pub use svg::SvgImage;

use aylis_core::Ui;

/// Registra `Lottie` e `Svg` (paleta "Media" e "Additional").
pub fn register(ui: &mut Ui) {
    ui.register_class::<Lottie>("Media");
    ui.register_class::<Svg>("Additional");
}
