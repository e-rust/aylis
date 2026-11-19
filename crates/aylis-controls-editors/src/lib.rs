//! # aylis-controls-editors
//!
//! Os editores visuais ([`AngleDial`], [`ShadowEditor`], [`EasingEditor`], [`TimingEditor`]),
//! a pintura ([`PaintCanvas`]) e o texto animado ([`AnimatedText`]). Os apps usam pelo
//! `aylis-controls`, que registra as classes.

mod animated_text;
mod editors;
mod paintbox;

pub use animated_text::{AnimatedText, TEXT_EFFECT};
pub use editors::{AngleDial, EasingEditor, ShadowEditor, TimingEditor};
pub use paintbox::{Bitmap, IMAGE_SIZE, ImageSize, PaintCanvas, PaintTool};

pub(crate) use aylis_core::leaf_control;
