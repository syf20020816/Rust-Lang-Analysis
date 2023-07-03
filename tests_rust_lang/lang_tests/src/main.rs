//!
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/6/29
//! @version:0.0.1
//! @description:
//! ```
#![feature(const_trait_impl)]

pub mod core_tests;

use lang_core::my_core::MyImplI8;

fn main() {
    println!("{}", MyImplI8::MAX);
    println!("{}", MyImplI8::MIN);
    println!("{}", MyImplI8::BITS);
    println!("{:?}",u8::from_str_radix("55",10))
}
