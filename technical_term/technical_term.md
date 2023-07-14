# Technical Term

- author : syf20020816@outlook.com
- date : 2023-06-29

## 旋转操作

旋转操作（rotation operation）指的是对二进制数进行循环位移的操作。旋转操作在计算机编程中常常用于位操作、密码学、数据压缩等领域

旋转操作通常涉及两个参数：要旋转的二进制数和旋转的位数。例如，如果我们将 10101100 向左旋转 2 位，则结果应为 10110001。向左旋转意味着最左边的位将移动到最右边，同时保持其它位的相对顺序不变。

同样，如果我们将 10101100 向右旋转 3 位，则结果应为 10010101。向右旋转意味着最右边的位将移动到最左边，同时保持其它位的相对顺序不变。

> note ： 向左旋转变大，向右旋转变小

<img src="https://github.com/syf20020816/Rust-Lang-Analysis/blob/main/imgs/ro_example.png">

## 交换操作

交换操作（swap operation）指的是将二进制数高位与低位进行交换

例如：1011 0101 交换后 0101 1011

## Const Trait

在Rust中，`impl MyTrait`和`impl const MyTrait`之间有一些区别。

1. 功能特性：`impl MyTrait`用于实现一个普通的trait，该trait可以包含任意的方法和关联类型。而`impl const MyTrait`用于实现一个特殊的trait，该trait只能包含具有`const`修饰符的方法。
2. 方法调用：对于`impl MyTrait`实现的trait，其方法可以通过实例对象进行调用，即可以使用动态分发。而`impl const MyTrait`实现的trait中的方法只能通过类型名直接调用，即使用静态分发。
3. 特化常量：`impl MyTrait`可以在实现中使用特定的常量值。对于`impl const MyTrait`，它允许在trait的方法中使用const常量，并保证在编译时进行计算。这可以用于在编译期间优化代码。

需要注意的是，`impl const`是Rust 1.50版本引入的新功能，因此如果你使用较早的Rust版本，可能不支持`impl const`。

## 宏实现(常用形式)

先看问题：为什么要实现三种
1. `impl<'a> const $imp<$u> for &'a $t `
2. `impl const $imp<&$u> for $t `
3. `impl const $imp<&$u> for &$t `

这三种宏实现形式的目的是为了提供更灵活的宏使用方式和代码复用。

1. `impl<'a> const $imp<$u> for &'a $t`：这种形式的宏实现允许将 trait 实现关联到一个引用类型上，并且在编译时进行常量计算。通过这种方式，我们可以对一个引用类型实现常量级的方法。这在某些情况下很有用，特别是当我们需要对引用进行操作但又不想复制实际数据时。
2. `impl const $imp<&$u> for $t`：这种形式的宏实现允许将 trait 实现关联到一个值类型上，并且在编译时进行常量计算。通过这种方式，我们可以在编译时为值类型实现常量级的方法。这对于需要在编译期执行某些操作的场景非常有用，例如在编译时计算某些值或执行某些验证。
3. `impl const $imp<&$u> for &$t`：这种形式的宏实现允许将 trait 实现关联到一个引用类型的引用上，并且在编译时进行常量计算。通过这种方式，我们可以在编译时为引用类型的引用实现常量级的方法。这在需要在编译时操作引用的场景下很有用，例如对多层引用进行操作时。

这三种宏实现形式提供了更多的灵活性和代码复用性，能够满足不同场景下的需求，并且在编译时进行常量计算可以提高性能和代码质量。

## 进制数

进制数是一种表示数字的方式，它决定了数字中各个位置上可以使用的符号和其对应的权重。常见的进制包括二进制（base 2）、十进制（base 10）、八进制（base 8）和十六进制（base 16）。进制数由基数（radix）确定，基数表示一个进制系统中可用的不同符号的数量。

例如，十进制中的基数为 10，因此使用了 0 到 9 的十个数字字符来表示数字。而二进制中的基数为 2，因此只能使用 0 和 1 两个数字字符来表示数字。类似地，八进制的基数为 8，十六进制的基数为 16。

进制数通过使用权重的方式来表示数字。在十进制中，每个数字的权重是以 10 的幂递增的，最右边的数字权重为 10^0，向左依次递增。例如，数字 123 在十进制中表示为 1 * 10^2 + 2 * 10^1 + 3 * 10^0。而在二进制中，每个数字的权重是以 2 的幂递增的，最右边的数字权重为 2^0，向左依次递增。

进制数的选择取决于所需表示的值的特定需求。十进制是最常用的进制，因为它直观且易于理解。二进制常用于计算机科学和数字电路中，因为计算机使用的是二进制逻辑。八进制和十六进制经常用于表示内存地址、字节序列和编码表示等。进制数的范围通常限制在 2 到 36，这是因为该范围内有足够的符号（数字和字母）可以表示各个位上的值，而且涵盖了大多数常见的需求。

## 数字截断

出自源码`fn from_u32(u: u32) -> Self { u as Self }`的思考（将u32转为各种整数类型比如u8，但是若输入的u32大于u8的最大值是否会出现错误？答案：不会，因为采用了数字截断）

使用 `as` 关键字将一个整数类型转换为另外一个整数类型时，如果目标类型无法表示源类型的值范围，会发生截断行为。对于无符号整数类型（比如 `u32` 和 `u8`），截断行为意味着超出目标类型范围的值会被截断，只保留低位部分。

因此，如果将一个大于 `u8` 最大值的 `u32` 值转换为 `u8` 类型，截断行为会让结果等于原始值对 `u8` 取模的结果。例如，如果 `u` 是一个大于 `u8` 最大值的 `u32` 值，那么 `u as u8` 的结果将是 `u % 256`。

这种截断行为并不会引发错误或产生编译时错误，而是由程序员负责确保安全性。在某些情况下，可能需要进行溢出检查或处理。

## match with judge

出自源码：`core/src/num/mod.rs`中的`from_str_radix()`

match 进行匹配时加判断直接跟在后面写即可

```rust
fn main(){
    let src = "+";
    let index = 5;
    match src {
        "+" if index == 5 => {
            println!("first 5")
        }
        _ => {}
    }
}
```

## 位移相比数字计算的优点

左移和右移是位运算中常见的操作，主要用于对二进制数进行移位操作。与数值运算相比，左移和右移具有以下优点：

1. 效率更高：位移操作通常比数值运算更快，因为它们是底层的位级操作，直接对二进制数据进行操作，不涉及乘法、除法等复杂的数学运算。
2. 结果可预测：位移操作的结果是确定的且可预测的，不受机器字长的影响。无论是在32位系统还是64位系统上执行位移，结果都一致。
3. 逻辑清晰：在某些情况下，使用位移操作可以使代码更加直观和易于理解。比如，将一个整数左移 n 位，相当于将该整数乘以 2 的 n 次方；将一个整数右移 n 位，相当于将该整数除以 2 的 n 次方并向下取整。
4. 位掩码操作：位移操作经常与位掩码操作结合使用，用于提取或设置特定的位。通过左移和右移配合使用，可以轻松地实现对指定位的读取、修改或写入。

需要注意的是，左移和右移是位级操作，其结果也是按位计算的。在进行位移操作时，需要明确考虑数据类型和移动的位数，以避免溢出或逻辑错误。

## ^运算和&运算

- ^ : 异 1 同 0 （1^1 = 0 , 0^0 = 0 , 1^0 = 1）
- & : 1 1 other 0 (1&1=1 , 1&0=0 , 0&0=0)

## 无分支算法(branchless algorithm)

`(a + b) / 2` == `((a ^ b) >> 1) + (a & b)`.

例如：

```
a = 5
b = 7
(a+b)/2 = 6
a^b == 101^111 = 010
010 >> 1 = 01
a&b == 101&111 = 101
01 + 101 = 110 == 2+4 = 6
```

## 🦀编译器内部函数说明

编译器内部函数。相应的定义在https://github.comrust-langrustblobmastercompilerrustc_codegen_llvmsrcintrinic.rs中。相应的const实现在https://github.comrust-angrustbloBMastercompile rrustc_const_evalsrcinterprisetriniscs.rs中。

源码位置：`core::intrinsics`

```rust
    /// Returns the number of bits set in an integer type `T`
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `count_ones` method. For example,
    /// [`u32::count_ones`]
    #[rustc_const_stable(feature = "const_ctpop", since = "1.40.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn ctpop<T: Copy>(x: T) -> T;
```

## 🦀Rust 非绑定模式(non-binding)

在 Rust 的 match 表达式中，每个模式匹配的分支都会对变量进行模式解构，并根据需要移动、借用或复制变量。

然而，在 Rust 1.26 版本之后，Rust 引入了非绑定（non-binding）的模式，允许在 match 表达式中使用未绑定的变量。这些未绑定的变量在模式匹配过程中不会引起任何移动、借用或复制操作。

在如下代码示例中:

```rust
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
    //这里的match编辑器会提示你:Use of moved value 
    //但实际上并没有错误
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
```

这里`(b & MAN_MASK, b & EXP_MASK)`是一个非绑定的模式，其中的 b 并不会被移动，因为它没有被使用。

## 🦀内嵌函数（Nested Function）

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

## 🦀Rust编译器推断执行函数

这里要介绍一个内部函数：编译器推断执行函数`intrinsics::const_eval_select()`

``` rust
core::intrinsics
pub extern "rust-intrinsic" fn const_eval_select<ARG: Tuple, F, G, RET>(arg: ARG, called_in_const: F, called_at_rt: G) -> RET where G: FnOnce<ARG, Output = RET>,F: FnOnce<ARG, Output = RET>,
```
### 作用

根据上下文选择要调用的函数。 如果在编译时对该函数求值，则对该内在函数的调用将被对called_in_const的调用所取代。否则，它将被一个对called_at_rt的调用所取代。 类型要求 这两个函数必须都是函数项。它们不能是函数指针或闭包。第一个函数必须是常量fn。 arg将是传递给两个函数中任意一个的元组参数，因此，两个函数必须接受相同类型的参数。两个函数都必须返回RET。

### 引入

由于这个推断函数是rust的unstable的所以你需要在其中使用如下特性宏

```rust
//! unstable 核心内部函数使用特性
#![feature(core_intrinsics)]
#![feature(const_eval_select)]
```

### 为什么要用?

他的主要使用原因是为了在不同的编译环境和编译器优化的情况下提供更好的兼容性和性能。但是这引入了一个unsafe的操作,所以开发者需要保证在使用这些功能前后的上下文中，代码的行为是安全的，并且符合 Rust 的内存安全规则。若你无法做到优秀的控制，我们更应该分离实现，不去使用编译器推断执行函数。

```rust
unsafe{
    intrinsics::const_eval_select((data,), const_fn, dyn_fn)
}
```

## 🦀impl [] and impl () 自定义复合类型实现

Rust 不允许为原生类型（primitive types）实现直接的 inherent impl。原生类型包括像 u8、char 这样的基本类型。 Rust 提供了扩展特性（extension trait）的机制。为原生类型创建一个 trait，然后在该 trait 上实现方法。无论是元组还是数组都支持这种写法。

### impl []

```rust
pub trait AsciiCharSlice<Rhs = AsciiChar> {
    type Origin;
    fn as_str(&self) -> Vec<&str>;
}

impl AsciiCharSlice for [AsciiChar] {
    type Origin = AsciiChar;

    fn as_str(&self) -> Vec<&str> {
        let mut res:Vec<&str> = Vec::new();
        for item in self {
            match item {
                AsciiChar::Null => res.push("Null"),
                AsciiChar::Delete => res.push("Delete"),
                _ => ()
            }
        }
        res
    }
}
```

### impl ()

```rust
trait TupleTrait{
    fn do_change()->();
}

impl TupleTrait for (f32,f32,f32){
    fn do_change() -> () {
        println!("{}","do change");
    }
}

fn main() {
    let a:(f32, f32, f32) = (1.0_f32, 1.2_f32, 25.6_f32);
    let _ = <(f32, f32, f32)>::do_change();
}
```

## 🦀`?Sized`非固定大小数据

关键字 `?Sized` 用于表示一个类型可能是不定大小（unsized）的类型。不定大小类型是指在编译时无法确定具体大小的类型，例如动态分配的数组或 trait 对象。

常用于方法的泛型约束上，`T: ?Sized` 表示泛型类型 `T` 可能是不定大小的类型。这意味着函数可以接受任意大小的 `T` 类型作为参数，包括不定大小的类型。

为什么要使用 `?Sized` 前缀呢？这是因为 Rust 默认情况下要求泛型类型是大小已知的（sized），这样才能在栈上分配内存。但是有些类型的大小是在运行时才能确定的，例如具有动态数组长度的数组类型，或者 trait 对象。在这种情况下，我们需要在类型前面加上 `?Sized` 来表明它是不定大小的，从而允许它们作为泛型参数。
