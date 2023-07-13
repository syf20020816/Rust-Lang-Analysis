//! Rust非绑定模式示例
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/13
//! @version:0.0.1
//! @description:
//! ```

/// 浮点数的枚举类型
#[derive(Debug)]
pub enum FpCategory {
    Nan,
    Infinite,
    Zero,
    Subnormal,
    Normal,
}

const fn classify_bits(b: u32) -> FpCategory {
    //0111 1111 1000 0000 0000 0000 0000 0000
    const EXP_MASK: u32 = 0x7f800000;
    //0111 1111 1111 1111 1111 1111
    const MAN_MASK: u32 = 0x007fffff;
    match (b & MAN_MASK, b & EXP_MASK) {
        (0, EXP_MASK) => FpCategory::Infinite,
        (_, EXP_MASK) => FpCategory::Nan,
        (0, 0) => FpCategory::Zero,
        (_, 0) => FpCategory::Subnormal,
        _ => FpCategory::Normal,
    }
}


fn main() {
    let a: u32 = 0x1ff;
    let res = classify_bits(a);
    dbg!(res);
}