//!
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/6/29
//! @version:0.0.1
//! @description:
//! ```

pub mod core_tests;

use lang_core::error::MyError;
use lang_core::my_core::{ParseIntError, IntErrorKind};

fn main() {
    let a = ParseIntError {
        kind: IntErrorKind::Zero
    };
    println!("ParseIntError-description: {}", a.description());
    println!("IntErrorKind: {:?}",a.kind());
}