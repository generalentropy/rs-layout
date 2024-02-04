//! Crate prelude

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// Genereic Wrapper tuple  struct for newtype pattern

pub struct W<T>(pub T);

// Preference

pub use std::format as f;