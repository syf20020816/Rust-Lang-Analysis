# Ascii Design

## 👍AsciiChar enum

该枚举是从U+0000到U+007F的128个Unicode字符之一(range:[0,127])，通常称为ASCII子集。其中实现大量使用编译器内部函数，需要参考编译器内部函数源码：

- `core::intrinsics.rs`
- `https://github.com/rust-lang/rust/blob/master/compiler/rustc_codegen_llvm/src/intrinsic.rs`

```rust
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AsciiChar {
    /// U+0000
    Null = 0,
    /// U+0001
    StartOfHeading = 1,
    //...
    Delete = 127,
}

/// 大量使用编译器内部函数
/// 参照源码`core/src/ascii/ascii_char.rs`
impl AsciiChar {
    pub const fn from_u8(char_u8: u8) -> Option<Self> {
        //range:[0,127]
        if char_u8 <= 127 {
            match char_u8 {
                0 => Some(AsciiChar::Null),
                _ => Some(AsciiChar::Delete)
            }
        } else {
            None
        }
    }
}
```

## `impl []{}`对切片类型的实现

参见 : `technical_term.md`**🦀impl [] and impl () 自定义复合类型实现**

Rust 不允许为原生类型（primitive types）实现直接的 inherent impl。原生类型包括像 u8、char 这样的基本类型。 Rust 提供了扩展特性（extension trait）的机制。为原生类型创建一个 trait，然后在该 trait 上实现方法。

```rust
pub trait AsciiCharSlice<Rhs = AsciiChar> {
    type Origin;
    fn as_str(&self) -> Vec<&str>;
}

/// # impl []{}
///Rust 不允许为原生类型（primitive types）实现直接的 inherent impl。原生类型包括像 u8、char 这样的基本类型。
///
/// Rust 提供了扩展特性（extension trait）的机制。为原生类型创建一个 trait，然后在该 trait 上实现方法。
///
/// 对应源码:
/// ``` code
/// impl [AsciiChar] {
///     pub const fn as_str(&self) -> &str {
///         let ascii_ptr: *const Self = self;
///         let str_ptr = ascii_ptr as *const str;
///         unsafe { &*str_ptr }
///     }
///     pub const fn as_bytes(&self) -> &[u8] {
///         self.as_str().as_bytes()
///     }
/// }
/// ```
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

