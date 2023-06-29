use crate::int_impl;

#[derive(Debug)]
pub struct MyImplI8 {
    pub max: u8,
    pub min: i8,
}

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