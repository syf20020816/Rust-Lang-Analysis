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
//! support AsciiChar!
#![feature(ascii_char)]
//! float feature for f32,f64
//! 浮点数特性
#![feature(const_fn_floating_point_arithmetic)]
//! unstable 核心内部函数使用特性
#![feature(core_intrinsics)]
#![feature(const_eval_select)]
#![feature(const_type_id)]
#![feature(const_type_name)]
pub mod my_core;
pub mod error;

pub use my_core::ParseIntError;
pub use my_core::num::from_str_radix;