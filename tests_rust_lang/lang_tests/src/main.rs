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

use lang_core::my_core::num::f32::MyF32;

pub fn outer(data: u32) -> () {
    fn inner(data: u32) -> String {
        println!("inner");
        let res = data.to_string();
        println!("{}", &res);
        res
    }
    inner(data);
}


fn main() {
    // let a = MyF32::new(7.0_f32);
    // println!("{}", MyF32::NAN);
    // println!("{}", a.is_nan());
    // println!("{}",)
    outer(56_u32);
}
