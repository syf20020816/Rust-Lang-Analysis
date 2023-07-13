# Num Design

## 👎shells

在这个目录下实现了i/u两种系列的数字类型以及一个int_macros中的`int_module!`而这里做的事情很简单就是生成对应数字类型的 MAX和MIN，以如下形式:

```rust
pub const MIN:i8 = i8::MIN
pub const MAX:i8 = i8::MAX
```

### int_module!

目的是统一的，简便的生成对应每个i/u数字类型的最大最小值引用

这些常量模块提供了与基本类型相关的有用常量，设计这些冗余的常量模块有几个主要原因：

1. **方便导入**: 使用 `std::i8::MIN` 和 `std::i8::MAX` 这样的全名路径可以明确指定要使用的常量。这在代码中可以提供更清晰的语义，并且能够避免命名冲突。
2. **跨模块可见性**: 通过将常量放在常量模块中，可以在不同的模块中进行访问，而无需显式地引入常量。这对于库作者和用户来说都很方便，因为他们可以直接使用这些常量，而无需显式导入它们。
3. **示例和学习**: 这些常量模块提供了一个方便的位置，可以查看和学习与特定类型相关的常量和约束。这对于初学者来说尤其有帮助，因为它们可以在标准库文档中找到与整数类型相关的常量。

## 👍int_impl

### constants

```rust
MyImplI8::MAX
MyImplI8::MIN
MyImplI8::BITS
```

### from_str_radix()

将给定基数中的字符串切片转换为整数
意思就是:
- u8::from_str_radix("16",10) == 16 :表示将16这个10进制数字转为整数
- u8::from_str_radix("16",8) == 14 :表示将16这个8进制数字转为整数

在int_impl中使用：`tests_rust_lang/lang_core/src/my_core/num/mod.rs`中的`from_str_radix()`

处理步骤：

1. 判断进制数（range:[2,36]）
2. 检查Empty
3. 检查类型（true：i type number，false：u type number）
4. 目标字符串转字节切片（[u8]）
5. 匹配字节切片首位检查正负值
6. 从u32转适配类型
7. 确定该基数长度的文本字符串是否可以保证存储在给定的类型T中(if can_not_overflow())
  - 检查溢出
8. 结束包装为Result

#### 使用

```rust
println!("{:?}", MyImplI8::from_str_radix("16", 8));
```

### count_ones()

用于统计一个数转为2进制数后其中1个个数，例如：5 → 0101 则结果为2

### 这些方法在哪？（🦀编译器内部函数说明）

见：`technical_term.md`

在 Rust 源码中，`count_ones()` 方法的实现位于 Unsigned 整数类型（如 `u8`、`u16`、`u32` 等）的定义之中。这个方法是由编译器为每个 Unsigned 整数类型自动生成的。

具体来说，整数类型的 `count_ones()` 方法是一个内建方法（Intrinsic），由编译器直接实现并优化。这意味着编译器会根据平台和目标架构提供高效的底层实现。你在代码中直接调用 `count_ones()` 方法，实际上是调用了编译器提供的底层实现。

这也解释了为什么可以直接调用 `count_ones()` 方法，即使其定义并不在你的代码中。Rust 编译器会根据上下文和类型推导，在编译期间将这个调用替换为相应的底层实现。

需要注意的是，`count_ones()` 作为一个内建方法，它的行为是与对应的平台和底层硬件相关的。因此，不同的运行环境下，`count_ones()` 可能会有不同的实现和性能特征。这就是为什么编译器会为每个 Unsigned 整数类型自动生成 `count_ones()` 方法，以便进行底层优化和适配。

## 👍midpoint()

使用branchless algorithm算法（see : technical_term.md）

👎calc : 5+6 = 11 -> 11/2 = 5...1 -> judge(..1)舍弃余数 -> Result == 5

👍move : ((5^6)>>1) + (5&6) -> Result == 5

 ``` rust
///重写实现并非源码
pub struct MidPoint<T>{
    data:T
}

macro_rules! my_midpoint_impl {
    ($($NumT:ty)*) => ($(
        impl MidPoint<$NumT>{
            pub const fn new(data:$NumT)->Self{
                MidPoint{
                    data
                }
            }
            pub const fn calc_midpoint(self,rhs:$NumT)->$NumT{
                ((self.data ^ rhs) >> 1) + (self.data & rhs)
            }
        }
    )*);
}

my_midpoint_impl!{u8 u16 u32 u128}
 ```

## f32

f32 类型是 IEEE 754 标准中定义的单精度浮点数类型，它用于表示带有单精度精度的浮点数值。该类型使用 32 位（4 字节）内存来存储浮点数。

在 f32 类型中，基数（即底数）对有效位数的影响是由浮点数的表示方法决定的。浮点数使用科学计数法表示，包括一个尾数和一个指数。尾数中的有效位数（即尾数的精度）表示浮点数可以准确表示的数字的个数。

### constants

```rust
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
```

### 👍非绑定模式(non-binding)

请参看:`technical_term.md`**🦀Rust 非绑定模式(non-binding)**

```rust
#[rustc_const_unstable(feature = "const_float_classify", issue = "72505")]
    const fn classify_bits(b: u32) -> FpCategory {
        const EXP_MASK: u32 = 0x7f800000;
        const MAN_MASK: u32 = 0x007fffff;

        match (b & MAN_MASK, b & EXP_MASK) {
            (0, EXP_MASK) => FpCategory::Infinite,
            (_, EXP_MASK) => FpCategory::Nan,
            (0, 0) => FpCategory::Zero,
            (_, 0) => FpCategory::Subnormal,
            _ => FpCategory::Normal,
        }
}
```

### 👍from_bits() | to_bits()

参看:`technical_term.md`**🦀Rust编译器推断执行函数**

先看这段内嵌函数的代码：

```rust
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
```

其中两个内嵌的函数的作用都是为了将u32转为f32，但这样用内嵌函数到底有什么意义呢？这种设计可能是为了结合不同的编译环境和编译器优化的需求。

首先，第一个嵌套函数 `ct_u32_to_f32` 使用了 `mem::transmute::<u32, f32>(ct)` 进行类型转换。这种显式指定源类型和目标类型的转换被称为 "const transmute"。使用 `const transmute` 可以在编译时计算和优化转换，但要求 `ct` 必须是编译时常量。

而第二个嵌套函数 `rt_u32_to_f32` 使用了 `mem::transmute(x)` 进行类型转换，省去了类型参数的指定，而是根据变量 `x` 的类型进行推断。这种转换被称为 "runtime transmute"，它在运行时进行类型转换。与 `const transmute` 不同，运行时转换无法进行编译时优化。

为什么要设计这两个嵌套函数并使用 `intrinsics::const_eval_select` 进行选择呢？这可能是为了兼容在不同编译环境下的需求。由于不同的编译器或编译选项对于编译时计算和优化的支持程度不同，使用 `const transmute` 可能在某些环境下无法正常工作。因此，提供了两种选择，并通过 `intrinsics::const_eval_select` 让编译器根据具体情况进行选择。对于普通的使用场景，直接使用 `rt_u32_to_f32(x: u32) -> f32` 可以更方便简洁。
