//! # 对f32的实现
//!
//! f32 类型是 IEEE 754 标准中定义的单精度浮点数类型，它用于表示带有单精度精度的浮点数值。
//!
//! 该类型使用 32 位（4 字节）内存来存储浮点数
//!
//! 在 f32 类型中，基数（即底数）对有效位数的影响是由浮点数的表示方法决定的。
//!
//! 浮点数使用科学计数法表示，包括一个尾数和一个指数。尾数中的有效位数（即尾数的精度）表示浮点数可以准确表示的数字的个数。
//!
//! ## f32
//! ``` code
//! |   0   |   0 * 8   |  0 * 23  |
//! |   ↑   |     ↑     |     ↑    |
//! |  sign |  exponent | fraction |
//!
//! sign: 符号位 +-
//! exponent: 指数位
//! fraction: 尾数位
//! ```
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/12
//! @version:0.0.1
//! @description:
//! ```

use std::{intrinsics, mem};

pub struct MyF32 {
    data: f32,
}


impl MyF32 {
    /// f32内部表示的基数或基数
    /// `f32::RADIX`
    pub const RADIX: u32 = 2;
    /// 以2为基数的有效位数
    /// 其中第一位为符号位所以是24位
    pub const MANTISSA_DIGITS: u32 = 24;
    /// 最大值
    pub const MAX: f32 = 3.40282347e+38_f32;
    ///最小正规化值（最接近于零的正数）
    pub const MIN_POSITIVE: f32 = 1.17549435e-38_f32;
    /// 最小值
    pub const MIN: f32 = -3.40282347e-38_f32;
    /// 非正规化值和特殊值（如无穷大和 NaN）。
    /// 非正规化值的范围比最小正规化值还小，而特殊值则表示无穷大或无效的操作数
    /// NAN : 无效值（NOT A Number）
    /// 结果就是NaN
    pub const NAN: f32 = 0.0_f32 / 0.0_f32;
    /// 无穷大 Lim(1/0)
    pub const INFINITY: f32 = 1.0_f32 / 0.0_f32;
    /// 无穷小 Lim(-1/0)
    pub const NEG_INFINITY: f32 = -1.0_f32 / 0.0_f32;

    pub const fn new(data: f32) -> Self {
        MyF32 {
            data
        }
    }
    /// 判断是否为NAN
    pub const fn is_nan(self) -> bool {
        self.data != self.data
    }

    //下面省略一些方法如：
    // 1. is_finite : 判断既不是无穷也不是NaN
    // 2. is_infinite : 判断是否为无穷(无穷大|无穷小)
    // 3. is_subnormal : 判断是否为非正常值(0~3.40282347e-38)
    // 4. is_normal : 判断是否为正常值(非无穷,非非正常值,非NaN)

    /// 判断f32属于那种类型返回 FpCategory
    pub const fn classify(self) -> FpCategory {
        // 通过对于is_finite,is_infinite,is_subnormal,is_normal进行处理
        FpCategory::Nan
    }

    /// 判断传入的binary类型属于那种f32类型
    const fn classify_bits(b: u32) -> FpCategory {
        //0111 1111 1000 0000 0000 0000 0000 0000
        const EXP_MASK: u32 = 0x7f800000;
        //0111 1111 1111 1111 1111 1111
        const MAN_MASK: u32 = 0x007fffff;
        // 这里并不是错误！
        // see ： technical_term.md Rust非绑定模式
        match (b & MAN_MASK, b & EXP_MASK) {
            (0, EXP_MASK) => FpCategory::Infinite,
            (_, EXP_MASK) => FpCategory::Nan,
            (0, 0) => FpCategory::Zero,
            (_, 0) => FpCategory::Subnormal,
            _ => FpCategory::Normal,
        }
    }


    // 当前f32值的后一个f32
    // pub const fn next_up(self)->Self{
    //     // 最小的f32
    //     // 2进制:0001
    //     const TINY_BITS:u32 = 0x1;
    //     // 2进制:0111 1111 1111 1111 1111 1111 1111 1111
    //     const CLEAR_SIGN_MASK:u32 = 0x7fff_ffff;
    //
    // }

    /// 通过内部嵌套方法进行
    pub const fn from_bits(v:u32) -> Self {
        const fn ct_u32_to_f32(ct: u32) -> f32 {
            match Self::classify_bits(ct){
                FpCategory::Nan => {
                    panic!("nan err")
                }
                FpCategory::Subnormal => {
                    panic!("subnormal err")
                }
                FpCategory::Infinite | FpCategory::Normal | FpCategory::Zero => {
                    unsafe { mem::transmute::<u32, f32>(ct) }
                }
            }
        }
        const fn rt_u32_to_f32(x:u32)->f32{
            unsafe { mem::transmute(x) }
        }
        unsafe { intrinsics::const_eval_select((v,), ct_u32_to_f32, rt_u32_to_f32) }
    }

}

/// 浮点数的枚举类型
pub enum FpCategory {
    Nan,
    Infinite,
    Zero,
    Subnormal,
    Normal,
}