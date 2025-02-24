//! `x-map` is a crate intended to add some more map and vector implementations which are fast,
//! and flexible in their usage environments.

pub mod maps;
pub mod error;
pub mod result;

#[cfg(test)]
mod tests;
mod util;