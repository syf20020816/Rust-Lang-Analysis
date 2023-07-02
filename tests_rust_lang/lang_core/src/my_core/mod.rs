pub mod num;
pub mod ops;
pub mod internal_macros;

pub use num::shells::my_i8;
/// pay attention to here!
/// Actually my_impl_i8 has MAX and MIN
pub use num::my_impl_i8::{MAX, MyImplI8, MIN, BITS};
pub use num::error::{ParseIntError, IntErrorKind};
