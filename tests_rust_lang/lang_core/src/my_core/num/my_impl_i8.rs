//! Num Type 的实现 👍更加推荐的方式
//!
//! Output use:
//! - MyImplI8::MAX
//! - MyImplI8::MIN
//! - MyImplI8::BITS
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/3
//! @version:0.0.1
//! @description:
//! ```


use crate::int_impl;
use crate::{ParseIntError, from_str_radix};

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct MyImplI8 {
    data: i8,
}

impl MyImplI8 {
    int_impl! {
        SelfT = MyImplI8,
        ActualT = i8,
        UnsignedT = u8,
        BITS = 8,
        BITS_MINUS_ONE = 7,
        Min = -128,
        Max = 127,
        rot = 2,
        rot_op = "-0x7e",
        rot_result = "0xa",
        swap_op = "0x12",
        swapped = "0x12",
        reversed = "0x48",
        le_bytes = "[0x12]",
        be_bytes = "[0x12]",
        to_xe_bytes_doc = "",
        from_xe_bytes_doc = "",
        bound_condition = "",
    }
}