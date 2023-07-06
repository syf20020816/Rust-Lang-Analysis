# Core Knowledge

- author : syf20020816@outlook.com
- date : 2023-06-28

## Des

在Core Knowledge 中，将会解析说明`Rust-std`库中的一些我们没怎么见过的Lib中的内容，进行学习、思考、模仿、创新，提升自我层次。

## Macro

### #![stable]

在 Rust 中，`#![stable(feature = "rust1", since = "1.0.0")]` 是一个用于标记稳定 API 特性的属性（attribute）。这个属性告诉 Rust 编译器该特性自 Rust 1.0.0 版本以来一直稳定存在，并且可以被公开使用。

通过使用 `#![stable(feature = "rust1", since = "1.0.0")]` 属性，在 Rust 代码中可以清晰地标记哪些特性是稳定的，并提供给其他开发者使用。

### #[cfg(target_pointer_width = "64")]

用于根据目标指针的位宽选择性地编译代码块。

这个属性用于条件编译（Conditional Compilation），在不同的平台上根据指针的位宽选择是否包含或排除某些代码。具体来说，`#[cfg(target_pointer_width = "64")]` 的作用是判断当前编译目标是否为 64 位平台，如果是，则编译对应的代码块；如果不是，则忽略该代码块。

下面是一个示例：

```rust
rust
#[cfg(target_pointer_width = "64")]
fn print_message() {
    println!("This is a 64-bit platform");
}

#[cfg(not(target_pointer_width = "64"))]
fn print_message() {
    println!("This is not a 64-bit platform");
}

fn main() {
    print_message();
}
```

在上述示例中，我们定义了两个函数 `print_message()`，使用了 `#[cfg(target_pointer_width = "64")]` 和 `#[cfg(not(target_pointer_width = "64"))]` 属性进行条件编译。如果编译目标是 64 位平台，则会选择编译并执行第一个函数，打印 "This is a 64-bit platform"；如果不是 64 位平台，则会选择编译并执行第二个函数，打印 "This is not a 64-bit platform"。

使用 `#[cfg(target_pointer_width = "64")]` 属性可以根据不同平台的特性编写针对性的代码，以确保代码能够在特定平台上正确运行。
