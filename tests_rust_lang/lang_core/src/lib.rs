//! `#![feature(const_trait_impl)]`
//!
//! when write `impl const Trait for XXX`
//!
//! you need to use Rust nightly and add this feature
//!
//! what's more add `#[const_trait]` on trait
#![feature(const_trait_impl)]
//! `#![feature(lang_items)]`
//! means design a lang items
#![feature(lang_items)]
pub mod my_core;
pub mod error;

pub use my_core::ParseIntError;
pub use my_core::num::from_str_radix;