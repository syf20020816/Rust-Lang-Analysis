//!
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/6/29
//! @version:0.0.1
//! @description:
//! ```
#![feature(const_trait_impl)]
pub mod core_tests;

use lang_core::my_core::Add;


fn main() {
    println!("{}",Add::add(8_u8,1_u8))
}