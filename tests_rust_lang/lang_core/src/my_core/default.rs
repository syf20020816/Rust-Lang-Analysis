//! # Default trait
//! Default用于生成一个默认的初始值
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/19
//! @version:0.0.1
//! @description:
//! ```

pub trait Default: Sized {
    fn default() -> Self;
}
/// 是一个 Rust 内置的宏声明。
/// 该宏在 Rust 编译器内部使用，用于为一些结构体和枚举类型生成默认的实现
/// 为了让代码通过编译，这里就直接去除了
/// pub macro Default($item:item){}
macro_rules! default_impl {
    ($t:ty,$v:expr) => {
        impl Default for $t{
            fn default()->$t{$v}
        }
    };
}

default_impl! {u32,10086_u32}