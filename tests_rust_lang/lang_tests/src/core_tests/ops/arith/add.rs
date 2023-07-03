//! Add实现测试
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/2
//! @version:0.0.1
//! @description:
//! ```

use lang_core::my_core::Add;

#[derive(Debug)]
struct TA1 {
    num: i8,
}

impl Add for TA1 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            num: self.num + rhs.num
        }
    }
}

fn main() {
    let a = TA1 {
        num: 100
    };
    let b = TA1 {
        num: -56
    };
    let c = TA1 {
        num: 24
    };
    let d = TA1 {
        num: 5
    };
    println!("{}", Add::add(8_u8, 1_u8));
    println!("{}", Add::add(8_i8, 102_i8));
    println!("{}", Add::add(8_i32, 100_i32));
    println!("{:?}", a.add(b));
    println!("{:?}", Add::add(c, d));
}