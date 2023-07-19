pub mod num;
pub mod ops;
pub mod internal_macros;
pub mod ascii;
pub mod convert;
pub mod any;
// pub mod fmt;
pub mod default;
pub mod option;

pub use num::shells::my_i8;
/// pay attention to here!
/// Actually my_impl_i8 has MAX and MIN
pub use num::my_impl_i8::{MyImplI8};
pub use num::error::{ParseIntError, IntErrorKind};
/// output Add
pub use ops::Add;
pub use num::MidPoint;
pub use convert::FloatToInt;