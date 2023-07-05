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

use lang_core::my_core::MidPoint;

fn main() {
    let m = MidPoint::<u8>::new(46_u8);
    println!("{}", m.calc_midpoint(16_u8));
}
