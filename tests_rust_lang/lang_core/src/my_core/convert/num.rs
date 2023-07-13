//! convert num
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/13
//! @version:0.0.1
//! @description:
//! ```


///这个从机箱外无法访问的特性阻止了FloatToInt特性的其他实现
///
/// 这允许在特性#[stable]之后潜在地添加更多的特性方法。
mod private {
    pub trait Sealed {}
}

/// 支持f32和f64的固有方法的特性
/// 即从浮点数转化为整数类型，如：
/// 1. f32 -> u32
/// 2. f64 -> u64
pub trait FloatToInt<Int>: private::Sealed + Sized {
    unsafe fn to_int_unchecked(self) -> Int;
}


/// $($Int:ident)+ : 意思是可以匹配多个
///
/// 如：impl_float_to_int!(f32 => u8 u16 u32 u64 u128);
///
/// 表现形式:
/// ``` code
/// impl private::Sealed for f32{
///     impl FloatToInt<u32> for $Float{
///         unsafe fn to_int_unchecked(self) -> u32{
///             unsafe{intrinsics::float_to_int_unchecked(self)}
///         }
///     }
/// }
/// ```
macro_rules! impl_float_to_int {
    ($Float:ident=>$($Int:ident)+) => {
        impl private::Sealed for $Float{
            $(
                impl FloatToInt<$Int> for $Float {
                    unsafe fn to_int_unchecked(self) -> $Int {
                        unsafe { crate::intrinsics::float_to_int_unchecked(self) }
                    }
                }
            )+
        }
    };
}



// impl_float_to_int!(f32 => u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
