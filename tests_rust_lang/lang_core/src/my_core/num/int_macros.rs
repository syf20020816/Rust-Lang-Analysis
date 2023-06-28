//! The Main Macro for u/i num types
//! such as :
//! - i8
//! - u8
//! - u32
//! - ...
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/6/29
//! @version:0.0.1
//! @description: The main macro for u/i num types
//! ```

/// 属于声明宏
///
/// 最后我们会得到
/// ``` txt
///          pub const MIN: $T = $T::MIN;
///          pub const MAX: $T = $T::MAX;
/// ```
#[macro_export]
macro_rules! int_module {
    ($T:ident) =>(
         pub const MIN: $T = $T::MIN;
         pub const MAX: $T = $T::MAX;
    );
}