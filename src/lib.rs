#![deny(
unstable_features,
unused_must_use,
unused_mut,
unused_imports,
unused_import_braces)]

mod client;
mod model;
pub mod exchangerate;
pub mod metadata;
pub mod ohlcv;
pub mod api;

#[macro_use]
extern crate serde_derive;
