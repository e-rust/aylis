//! # aylis-controls-industrial
//!
//! Os controles industriais e elétricos (telas de supervisão). Os apps usam pelo
//! `aylis-controls` (`aylis_controls::industrial`), que chama o [`register`] daqui.

mod industrial;

pub use industrial::*;

pub(crate) use aylis_core::leaf_control;
