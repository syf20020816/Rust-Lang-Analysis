//! Test my_i8
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/6/29
//! @version:0.0.1
//! @description: 解释core/src/shells/int_macros.rs
//! ```
//!
//! # int_macros
//!
//! 请参看`tests_rust_lang/lang_tests/src/core_tests/num_shells_int_macros.rs`
//!
//! - package : `src/num/shells/int_macros.rs`
//!
//! ## Preface
//!
//! 在这里所有的i和u类型，即`i8,u8,i16...`构建的代码形如：
//!
//! ```rust
//! #![stable(feature = "rust1", since = "1.0.0")]
//! #![deprecated(
//!     since = "TBD",
//!     note = "all constants in this module replaced by associated constants on `isize`"
//! )]
//!
//! int_module! { isize }
//! ```
//!
//! ❗：所以最主要看的应该是`int_module!`这个宏
//!
//! <hr />
//!
//! ## Source
//!
//! 在看源码时，我们需要删除一些不需要的东西，比如doc相关的，最后我们就会得到下面的内容：
//!
//! ```rust
//! ① macro_rules! int_module {
//!     ② ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
//!     ③ ($T:ident, #[$attr:meta]) => (
//!
//!         #[$attr]
//!         #[deprecated(since = "TBD", note = "replaced by the `MIN` associated constant on this type")]
//!         pub const MIN: $T = $T::MIN;
//!
//!         #[$attr]
//!         #[deprecated(since = "TBD", note = "replaced by the `MAX` associated constant on this type")]
//!         pub const MAX: $T = $T::MAX;
//!     )
//! }
//!
//! ```
//!
//! - ① int_module是int_macros中所实现的声明宏
//!
//! - ②输入为`$T:ident`说明接收的是标识符，然后转化为`int_module!($T, #[stable(feature = "rust1", since = "1.0.0")])`，例如：`int_module!(u8) --> int_module!(u8,#[stable(feature = "rust1", since = "1.0.0")])`
//!
//! - ③识别输入`($T:ident, #[$attr:meta])`生成MAX和MIN，MAX和MIN就是输入标识符的类型最后得到如（以i8为例）：
//!
//!   ​
//!
//!   ```rust
//!           #[stable(feature = "rust1", since = "1.0.0")]
//!           #[deprecated(since = "TBD", note = "replaced by the `MIN` associated constant on this type")]
//!           pub const MIN: i8 = i8::MIN;
//!
//!           #[stable(feature = "rust1", since = "1.0.0")]
//!           #[deprecated(since = "TBD", note = "replaced by the `MAX` associated constant on this type")]
//!           pub const MAX: i8 = i8::MAX;
//!   ```
//!
//!   所以实际上我们看到i8.rs中的代码应该是：
//!
//!   ```rust
//!   #![stable(feature = "rust1", since = "1.0.0")]
//!   #![deprecated(
//!       since = "TBD",
//!       note = "all constants in this module replaced by associated constants on `isize`"
//!   )]
//!
//!   #[stable(feature = "rust1", since = "1.0.0")]
//!   #[deprecated(since = "TBD", note = "replaced by the `MIN` associated constant on this type")]
//!   pub const MIN: i8 = i8::MIN;
//!
//!   #[stable(feature = "rust1", since = "1.0.0")]
//!   #[deprecated(since = "TBD", note = "replaced by the `MAX` associated constant on this type")]
//!   pub const MAX: i8 = i8::MAX;
//!   ```
//!
//!

use lang_core::my_core::my_i8;

fn main() {
    /// 127
    println!("{}",my_i8::MAX);
    /// -128
    println!("{}",my_i8::MIN);
}
