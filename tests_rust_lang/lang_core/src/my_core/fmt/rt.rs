use crate::my_core::fmt::{Formatter,Result};

pub struct Placeholder {
    pub position: usize,
    pub fill: char,
    pub align: Alignment,
    pub flags: u32,
    pub precision: Count,
    pub width: Count,
}

impl Placeholder {
    pub const fn new(
        position: usize,
        fill: char,
        align: Alignment,
        flags: u32,
        precision: Count,
        width: Count,
    ) -> Self {
        Placeholder {
            position,
            fill,
            align,
            flags,
            precision,
            width,
        }
    }
}

/// 这里我十分奇怪，明明在super中已经有了一个Alignment
/// 所以我觉得可以直接省略
pub enum Alignment {
    Left,
    Right,
    Center,
    Unknown,
}

/// 使用到width和precision
pub enum Count {
    ///用文字数字指定，存储值
    Is(usize),
    ///使用$和*语法指定，将索引存储到参数中
    Param(usize),
    ///未指定
    Implied,
}

/// Argument本质上是一个优化的部分应用的格式化函数
/// 此结构表示format_args!()所采用的泛型“参数”。
/// 它包含一个用于格式化给定值的函数。
/// 在编译时，确保函数和值具有正确的类型，然后使用此结构将参数规范化为一个类型。
pub struct Argument<'a> {
    a:&'a str
    // value: &'a Opaque,
    // formatter: fn(&Opaque, &mut Formatter<'_>) -> Result,
}


pub(super) enum Flag{
    //+
    SignPlus,
    //-
    SignMinus,
    Alternate,
    SignAwareZeroPad,
    DebugLowerHex,
    DebugUpperHex,
}