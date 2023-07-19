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

use lang_core::my_core::any::TypeId;

use std::any::{Any};
use std::fmt::{Debug, Display, format, Formatter};

struct A{
    data:u32
}

impl Default for A{
    fn default() -> Self {
        A{
            data: 0
        }
    }
}

fn main() {
    // // String: 160456767258664830752912317615608621363
    // println!("{:?}", is_string(&"sdsd".to_string()));
    // // u32: 25882202575019293479932656973818029271
    // println!("{:?}", is_string(&16_u32));
    // // println!("{}",TypeId::of()::<>())
    // println!("{}", TypeId::type_name_of_val(&"sdsd".to_string()));
    // // println!("{:?}",56.type_id());
    let mut a = Y::new();
    let m = Vec::new();
    a.inner_new();
    println!("{:?}",a);
}
