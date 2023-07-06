use lang_core::my_core::ascii::{AsciiChar,AsciiCharSlice};

fn main() {
    println!("{:?}", AsciiChar::from_u8(0));
    println!("{:?}", AsciiChar::from_u8(6));
    let ac_slice = [AsciiChar::Null, AsciiChar::Delete, AsciiChar::Acknowledge];
    println!("{:?}", ac_slice.as_str());
}