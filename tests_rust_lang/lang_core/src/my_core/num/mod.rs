use core::mem;

/// Use `std::ops`
/// The Add impl is in `use crate::my_core::Add;`
/// 但是为了功能的完整性则需要使用std库中的实现
///
/// 原因：重写的ADD并无法实现通过运算符进行计算
use std::ops::{Mul, RangeBounds, Sub, Add};
use crate::my_core::{
    MyImplI8,
    ascii::AsciiChar,
};
use std::ascii::Char;

pub mod shells;
mod int_macros;
pub mod my_impl_i8;
pub mod error;
pub mod f32;

use self::error::{ParseIntError, IntErrorKind};
// use crate::my_core::Add;

/// 确定该基数长度的文本字符串是否可以保证存储在给定的类型T中。
/// 请注意，如果编译器知道基数，则只需在运行时检查digit.len即可。
/// if T == u32 :
/// ``` code
/// mem::size_of::<u32>() == 4 //u32为4个字节
/// is_signed_ty as usize == 1 | 0 //true == 1 , false == 0
/// digits.len()
/// ```
pub fn can_not_overflow<T>(radix: u32, is_signed_ty: bool, digits: &[u8]) -> bool {
    radix <= 16 && digits.len() <= mem::size_of::<T>() * 2 - is_signed_ty as usize
}


pub trait FromStrRadixHelper:
PartialOrd + Copy + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self>
{
    const MIN: Self;
    fn from_u32(u: u32) -> Self;
    fn checked_mul(&self, other: u32) -> Option<Self>;
    fn checked_sub(&self, other: u32) -> Option<Self>;
    fn checked_add(&self, other: u32) -> Option<Self>;
}

/// 为u/i num types实现FromStrRadixHelper
macro_rules! impl_helper_for {
    ($($t:ty)*) => ($(impl FromStrRadixHelper for $t {
        /// 获取num type的最小值
        const MIN: Self = Self::MIN;
        ///使用 `as` 关键字将一个整数类型转换为另外一个整数类型时，如果目标类型无法表示源类型的值范围，会发生截断行为。对于无符号整数类型（比如 `u32` 和 `u8`），截断行为意味着超出目标类型范围的值会被截断，只保留低位部分。
        ///
        /// 因此，如果将一个大于 `u8` 最大值的 `u32` 值转换为 `u8` 类型，截断行为会让结果等于原始值对 `u8` 取模的结果。例如，如果 `u` 是一个大于 `u8` 最大值的 `u32` 值，那么 `u as u8` 的结果将是 `u % 256`。
        ///
        /// 这种截断行为并不会引发错误或产生编译时错误，而是由程序员负责确保安全性。在某些情况下，可能需要进行溢出检查或处理。
        fn from_u32(u: u32) -> Self { u as Self }

        fn checked_mul(&self, other: u32) -> Option<Self> {
            Self::checked_mul(*self, other as Self)
        }

        fn checked_sub(&self, other: u32) -> Option<Self> {
            Self::checked_sub(*self, other as Self)
        }

        fn checked_add(&self, other: u32) -> Option<Self> {
            Self::checked_add(*self, other as Self)
        }
    })*)
}

impl_helper_for! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }

/// 将给定基数中的字符串切片转换为整数
///
/// 意思就是:
/// - u8::from_str_radix("16",10) == 16 :表示将16这个10进制数字转为整数
/// - u8::from_str_radix("16",8) == 14 :表示将16这个8进制数字转为整数
///
/// if you see this func,that means you are trying to read `int_impl::from_str_radix()`
///
/// this func needs :
/// 1. ParseIntError
/// 2. IntErrorKind
/// 3. FromStrRadixHelper
/// 4. can_not_overflow()
///
/// first we need to write error mod! see `lang_core/src/error.rs` and `self::error.rs`
/// 确定该基数长度的文本字符串是否可以保证存储在给定的类型T中。请注意，如果编译器知道基数，则只需在运行时检查digit.len即可
pub fn from_str_radix<T: FromStrRadixHelper>(src: &str, radix: u32) -> Result<T, ParseIntError> {
    // just like source (Don't mind this):
    // - use self::IntErrorKind::*;
    // - use self::ParseIntError as PIE;
    // Use this way is cheaper and more efficient
    use self::error::IntErrorKind::*;
    use self::error::ParseIntError as PIE;

    /// 关于进制数:`technical_term.md`
    ///
    /// 因为 radix 表示用于解析字符串的进制数。进制数的范围通常限制在 2 到 36，这是因为该范围内有足够的符号（数字和字母）可以表示各个位上的值，而且涵盖了大多数常见的需求。
    ///
    /// 在这个特定的断言中，使用了 contains 方法来判断基数是否在 2 到 36 的范围内。如果基数不在这个范围内，断言条件为 false，会触发 panic。
    ///
    /// 这种限制的原因是为了确保解析字符串为整数时使用的基数是有效且合理的。进制数大于 1 且小于等于 36 是普遍可接受的范围
    ///
    /// 因为数字（0-9）和字母（A-Z）共有 36 个字符可用于表示数字值。超出这个范围的基数将导致无效的解析或无法正确表示整数值。
    assert!(
        (2..=36).contains(&radix),
        "from_str_radix_int: must lie in the range `[2, 36]` - found {}",
        radix
    );

    /// 检查空值 can't parse if empty
    if src.is_empty() {
        return Err(PIE { kind: Empty });
    }
    /// 检查0是否大于数字类型的最小值
    /// - true : i type number
    /// - false: u type number
    let is_signed_ty = T::from_u32(0) > T::MIN;

    // all valid digits are ascii, so we will just iterate over the utf8 bytes
    // and cast them to chars. .to_digit() will safely return None for anything
    // other than a valid ascii digit for the given radix, including the first-byte
    // of multi-byte sequences
    /// 字符串转字节切片
    let src = src.as_bytes();
    /// 匹配字节切片首位检查正负值
    let (is_positive, digits) = match src[0] {
        b'+' | b'-' if src[1..].is_empty() => {
            return Err(PIE { kind: InvalidDigit });
        }
        b'+' => (true, &src[1..]),
        b'-' if is_signed_ty => (false, &src[1..]),
        _ => (true, src),
    };
    /// 从u32转适配类型
    let mut result = T::from_u32(0);
    /// 确定该基数长度的文本字符串是否可以保证存储在给定的类型T中
    if can_not_overflow::<T>(radix, is_signed_ty.clone(), digits) {
        // If the len of the str is short compared to the range of the type
        // we are parsing into, then we can be certain that an overflow will not occur.
        // This bound is when `radix.pow(digits.len()) - 1 <= T::MAX` but the condition
        // above is a faster (conservative) approximation of this.
        //
        // Consider radix 16 as it has the highest information density per digit and will thus overflow the earliest:
        // `u8::MAX` is `ff` - any str of len 2 is guaranteed to not overflow.
        // `i8::MAX` is `7f` - only a str of len 1 is guaranteed to not overflow.
        macro_rules! run_unchecked_loop {
            ($unchecked_additive_op:expr) => {
                for &c in digits {
                    result = result * T::from_u32(radix);
                    let x = (c as char).to_digit(radix).ok_or(PIE { kind: InvalidDigit })?;
                    result = $unchecked_additive_op(result, T::from_u32(x));
                }
            };
        }
        if is_positive {
            run_unchecked_loop!(<T as core::ops::Add>::add)
        } else {
            run_unchecked_loop!(<T as core::ops::Sub>::sub)
        };
    } else {
        macro_rules! run_checked_loop {
            ($checked_additive_op:ident, $overflow_err:expr) => {
                for &c in digits {
                    // When `radix` is passed in as a literal, rather than doing a slow `imul`
                    // the compiler can use shifts if `radix` can be expressed as a
                    // sum of powers of 2 (x*10 can be written as x*8 + x*2).
                    // When the compiler can't use these optimisations,
                    // the latency of the multiplication can be hidden by issuing it
                    // before the result is needed to improve performance on
                    // modern out-of-order CPU as multiplication here is slower
                    // than the other instructions, we can get the end result faster
                    // doing multiplication first and let the CPU spends other cycles
                    // doing other computation and get multiplication result later.
                    let mul = result.checked_mul(radix);
                    let x = (c as char).to_digit(radix).ok_or(PIE { kind: InvalidDigit })?;
                    result = mul.ok_or_else($overflow_err)?;
                    result = T::$checked_additive_op(&result, x).ok_or_else($overflow_err)?;
                }
            };
        }
        if is_positive {
            run_checked_loop!(checked_add, || PIE { kind: PosOverflow })
        } else {
            run_checked_loop!(checked_sub, || PIE { kind: NegOverflow })
        };
    }
    Ok(result)
}


/// # midpoint (branchless algorithm)
/// 获取2个数字的中点,使用右移
/// ## why
/// see : technical_term.md
/// ## 中点
/// 👎calc : 5+6 = 11 -> 11/2 = 5...1 -> judge(..1)舍弃余数 -> Result == 5
/// 👍move : ((5^6)>>1) + (5&6) -> Result == 5
/// ``` code
/// macro_rules! midpoint_impl {
///     ($($NumT:ty)*) => ($(
///         pub const fn midpoint(self,rhs:$NumT)->$NumT{
///             ((self^rhs)>>1) + (self&rhs)
///         }
///     )*);
/// }
///
/// midpoint_impl!{u8 u16 u32 u64 u128}
/// ```
pub struct MidPoint<T> {
    data: T,
}

macro_rules! my_midpoint_impl {
    ($($NumT:ty)*) => ($(
        impl MidPoint<$NumT>{
            pub const fn new(data:$NumT)->Self{
                MidPoint{
                    data
                }
            }
            pub const fn calc_midpoint(self,rhs:$NumT)->$NumT{
                ((self.data ^ rhs) >> 1) + (self.data & rhs)
            }
        }
    )*);
}

my_midpoint_impl! {u8 u16 u32 u128}


/// 如果此字节的值在ASCII范围内，则将其作为ASCII字符返回。否则，返回None
/// 对应源码:
///``` code
/// pub const fn as_ascii(&self) -> Option<ascii::Char> {
///         ascii::Char::from_u8(*self)
///     }
/// ```
pub const fn as_ascii(target: u8) -> Option<Char> {
    Char::from_u8(target)
}

// 其他很多方法都是1.72.0开始增加的新方法，和上方的as_ascii万变不离其宗就不继续写了
// 看到这里请移步下一个实现f32,f64