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

### 👍内嵌函数（Nested Function）

这里我们要学习的是一种内嵌函数的写法

优点：

1. 封装性和私有性：内嵌函数只在其父函数内部可见，而对于外部代码来说是隐藏的。这可以增强代码的封装性和模块化，使得内部逻辑对外部代码是不可见的，提高了代码的安全性和可维护性。
2. 代码复用：通过将功能相似的代码包装在内部函数中，可以避免代码重复。内嵌函数可以被父函数多次调用，提供了一种代码复用的机制。
3. 逻辑清晰性：内嵌函数可以将复杂的逻辑分解成更小的、更易理解的部分。通过将代码分割成逻辑相关的块，可以提高代码的可读性和可理解性。

缺点：

1. 可见范围限制：内嵌函数只能在其父函数内部调用，无法在其它函数或模块中使用。如果需要在其它地方复用该功能，就需要将内嵌函数提取到一个可公开调用的位置。
2. 函数间通信限制：内嵌函数无法直接与父函数外的代码进行交互，因为它们与外部作用域隔离。如果需要与父函数外的代码进行通信，可能需要使用闭包或参数传递等机制。

```rust
pub fn outer(data: u32) -> () {
    fn inner(data: u32) -> String {
        println!("inner");
        let res = data.to_string();
        println!("{}", &res);
        res
    }
    inner(data);
}


fn main() {
    outer(56_u32);
}
```

