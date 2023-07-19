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

use lang_core::my_core::option::MyOption;

fn main() {
    let a = MyOption::Some(56);
    let b: MyOption<u32> = MyOption::<u32>::None;
    println!("{}", a.is_some());
    println!("{}", b.is_none());
    println!("{}", a.is_some_and(|x| x > 100))
}