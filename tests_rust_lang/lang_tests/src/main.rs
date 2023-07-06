//!
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/6/29
//! @version:0.0.1
//! @description:
//! ```
#![feature(const_trait_impl)]
#![feature(num_midpoint)]

pub mod core_tests;

use lang_core::my_core::ascii::{AsciiChar,AsciiCharSlice};

fn main() {
    println!("{:?}", AsciiChar::from_u8(0));
    println!("{:?}", AsciiChar::from_u8(6));
    let ac_slice = [AsciiChar::Null, AsciiChar::Delete, AsciiChar::Acknowledge];
    println!("{:?}", ac_slice.as_str());
}
