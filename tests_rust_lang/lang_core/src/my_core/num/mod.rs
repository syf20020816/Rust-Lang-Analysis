use core::mem;

use std::ops::RangeBounds;

pub mod shells;
mod int_macros;
pub mod my_impl_i8;
pub mod error;

use self::error::{ParseIntError,IntErrorKind};
use crate::my_core::Add;


// pub fn can_not_overflow<T>(radix: u32, is_signed_ty: bool, digits: &[u8]) -> bool {
//     radix <= 16 && digits.len() <= mem::size_of::<T>() * 2 - is_signed_ty as usize
// }

// #[doc(hidden)]
// trait FromStrRadixHelper:
// PartialOrd + Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self>
// {
//     const MIN: Self;
//     fn from_u32(u: u32) -> Self;
//     fn checked_mul(&self, other: u32) -> Option<Self>;
//     fn checked_sub(&self, other: u32) -> Option<Self>;
//     fn checked_add(&self, other: u32) -> Option<Self>;
// }
//
// /// if you see this func,that means you are trying to read `int_impl::from_str_radix()`
// ///
// /// this func needs :
// /// 1. ParseIntError
// /// 2. IntErrorKind
// /// 3. FromStrRadixHelper
// /// 4. can_not_overflow()
// ///
// /// first we need to write error mod! see `lang_core/src/error.rs` and `self::error.rs`
// /// 确定该基数长度的文本字符串是否可以保证存储在给定的类型T中。请注意，如果编译器知道基数，则只需在运行时检查digitals.len即可
// fn from_str_radix<T: FromStrRadixHelper>(src: &str, radix: u32) -> Result<T, ParseIntError> {
//
//     // just like source (Don't mind this):
//     // - use self::IntErrorKind::*;
//     // - use self::ParseIntError as PIE;
//     // Use this way is cheaper and more efficient
//     use self::error::IntErrorKind::*;
//     use self::error::ParseIntError as PIE;
//
//
//     assert!(
//         (2..=36).contains(&radix),
//         "from_str_radix_int: must lie in the range `[2, 36]` - found {}",
//         radix
//     );
//     // can't parse if empty
//     if src.is_empty() {
//         return Err(PIE { kind: Empty });
//     }
//
//     let is_signed_ty = T::from_u32(0) > T::MIN;
//
//     // all valid digits are ascii, so we will just iterate over the utf8 bytes
//     // and cast them to chars. .to_digit() will safely return None for anything
//     // other than a valid ascii digit for the given radix, including the first-byte
//     // of multi-byte sequences
//     let src = src.as_bytes();
//
//     let (is_positive, digits) = match src[0] {
//         b'+' | b'-' if src[1..].is_empty() => {
//             return Err(PIE { kind: InvalidDigit });
//         }
//         b'+' => (true, &src[1..]),
//         b'-' if is_signed_ty => (false, &src[1..]),
//         _ => (true, src),
//     };
//
//     let mut result = T::from_u32(0);
//
//     if can_not_overflow::<T>(radix, is_signed_ty, digits) {
//         // If the len of the str is short compared to the range of the type
//         // we are parsing into, then we can be certain that an overflow will not occur.
//         // This bound is when `radix.pow(digits.len()) - 1 <= T::MAX` but the condition
//         // above is a faster (conservative) approximation of this.
//         //
//         // Consider radix 16 as it has the highest information density per digit and will thus overflow the earliest:
//         // `u8::MAX` is `ff` - any str of len 2 is guaranteed to not overflow.
//         // `i8::MAX` is `7f` - only a str of len 1 is guaranteed to not overflow.
//         macro_rules! run_unchecked_loop {
//             ($unchecked_additive_op:expr) => {
//                 for &c in digits {
//                     result = result * T::from_u32(radix);
//                     let x = (c as char).to_digit(radix).ok_or(PIE { kind: InvalidDigit })?;
//                     result = $unchecked_additive_op(result, T::from_u32(x));
//                 }
//             };
//         }
//         if is_positive {
//             run_unchecked_loop!(<T as core::ops::Add>::add)
//         } else {
//             run_unchecked_loop!(<T as core::ops::Sub>::sub)
//         };
//     } else {
//         macro_rules! run_checked_loop {
//             ($checked_additive_op:ident, $overflow_err:expr) => {
//                 for &c in digits {
//                     // When `radix` is passed in as a literal, rather than doing a slow `imul`
//                     // the compiler can use shifts if `radix` can be expressed as a
//                     // sum of powers of 2 (x*10 can be written as x*8 + x*2).
//                     // When the compiler can't use these optimisations,
//                     // the latency of the multiplication can be hidden by issuing it
//                     // before the result is needed to improve performance on
//                     // modern out-of-order CPU as multiplication here is slower
//                     // than the other instructions, we can get the end result faster
//                     // doing multiplication first and let the CPU spends other cycles
//                     // doing other computation and get multiplication result later.
//                     let mul = result.checked_mul(radix);
//                     let x = (c as char).to_digit(radix).ok_or(PIE { kind: InvalidDigit })?;
//                     result = mul.ok_or_else($overflow_err)?;
//                     result = T::$checked_additive_op(&result, x).ok_or_else($overflow_err)?;
//                 }
//             };
//         }
//         if is_positive {
//             run_checked_loop!(checked_add, || PIE { kind: PosOverflow })
//         } else {
//             run_checked_loop!(checked_sub, || PIE { kind: NegOverflow })
//         };
//     }
//     Ok(result)
// }
