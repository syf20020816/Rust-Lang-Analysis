//! Num Errors
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/6/29
//! @version:0.0.1
//! @description:
//! ```

use std::any::TypeId;
use std::fmt::{Debug, Display, Formatter};
use crate::error::MyError;

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum IntErrorKind{
    /// ""
    Empty,
    /// Contains an invalid digit
    /// a1 !=> 1 Error!
    InvalidDigit,
    /// num > num_type::MAX
    /// 500 > i8::MAX
    PosOverflow,
    /// nun < num_type::MIN
    /// -500 < i8::MIN
    NegOverflow,
    /// 0
    Zero
}

/// Parse Int Error
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct ParseIntError {
    /// if parse error
    pub kind: IntErrorKind,
}

impl ParseIntError{
    pub fn kind(&self)->&IntErrorKind{
        &self.kind
    }
}
/// MyError : Debug+Display
/// Need Display!
impl Display for ParseIntError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       /// don't use -> self.description().fmt(f)
       /// use -> Debug::fmt(&self.description(),f)
       /// use -> std::fmt::Display::fmt(&self.description(), f)
        std::fmt::Display::fmt(&self.description(), f)
    }
}

impl MyError for ParseIntError{
    fn description(&self) -> &str {
        match self.kind {
            IntErrorKind::Empty => "cannot parse empty",
            IntErrorKind::InvalidDigit => "contains invalid digit",
            IntErrorKind::PosOverflow => "too big",
            IntErrorKind::NegOverflow => "too small",
            IntErrorKind::Zero => "0 cannot be parsed!0不能转化"
        }
    }
}