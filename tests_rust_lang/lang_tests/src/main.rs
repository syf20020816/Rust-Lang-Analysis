//! Test my_i8
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/6/29
//! @version:0.0.1
//! @description: 解释core/src/shells/int_macros.rs
//! ```

use lang_core::my_core::my_i8;
pub mod core_tests;

fn main() {
    /// 127
    println!("{}",my_i8::MAX);
    /// -128
    println!("{}",my_i8::MIN);
}
