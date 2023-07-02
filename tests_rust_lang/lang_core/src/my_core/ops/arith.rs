//! 运算符实现
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/2
//! @version:0.0.1
//! @description:
//! ```
use crate::forward_ref_binop;

/// 加法运算符+
/// > 请注意，默认情况下Rhs是Self，但这不是强制性的。
///
/// 例如，std:：time::SystemTime实现了Add<Duration>,它允许SystemTime=SystemTime+Duration形式的操作。
/// 实际上我们需要为数字类型甚至字符串实现Add trait，其实事实也如此，在self中为所有u/i num type都进行了实现
/// 其实加法对于这些num type的工作是一样的，所以，通过宏快速生成
/// `add_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }`
///
/// please see:
/// 1. Add(trait)
/// 2. add_impl!(macro)
pub trait Add<Rhs = Self> {
    /// the type after add
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

/// impl add for num types
/// see : forward_ref_binop!(macro)
macro_rules! add_impl {
    ($($t:ty)*) => {
        /// why use const Add for $t, please read : technical_term.md
        impl const Add for $t{
            type Output = $t;

            fn add(self, rhs:$t) -> $t {
                self + rhs
            }
        }
        forward_ref_binop!{impl const Add, add for $t, $t }
    };
}

add_impl! {i8,u8}