# Core Knowledge

- author : syf20020816@outlook.com
- date : 2023-06-28

## Des

在Core Knowledge 中，将会解析说明`Rust-std`库中的一些我们没怎么见过的Lib中的内容，进行学习、思考、模仿、创新，提升自我层次。

## Macro

### #![stable]

在 Rust 中，`#![stable(feature = "rust1", since = "1.0.0")]` 是一个用于标记稳定 API 特性的属性（attribute）。这个属性告诉 Rust 编译器该特性自 Rust 1.0.0 版本以来一直稳定存在，并且可以被公开使用。

通过使用 `#![stable(feature = "rust1", since = "1.0.0")]` 属性，在 Rust 代码中可以清晰地标记哪些特性是稳定的，并提供给其他开发者使用。

