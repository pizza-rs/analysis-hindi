#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

mod indic;
mod normalization;
mod register;
mod stem;
mod stop;

pub use indic::IndicNormalizationFilter;
pub use normalization::HindiNormalizationFilter;
pub use register::register_all;
pub use stem::HindiStemFilter;
pub use stop::HindiStopFilter;
