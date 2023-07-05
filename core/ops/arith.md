# arith

请参看：

1. `tests_rust_lang/lang_tests/src/core_tests/ops/arith/add.rs`

- package : `src/ops/arith.rs`

<hr />

## Ⓜ️ arith Add

<img src="https://github.com/syf20020816/Rust-Lang-Analysis/blob/main/imgs/arith_add.png">

<img src="https://github.com/syf20020816/Rust-Lang-Analysis/blob/main/imgs/add_trait.png">

### Add

```rust
use crate::forward_ref_binop;

/// 加法运算符+
/// > 请注意，默认情况下Rhs是Self，但这不是强制性的。
///
/// 例如，std:：time::SystemTime实现了Add<Duration>,它允许SystemTime=SystemTime+Duration形式的操作。
/// 实际上我们需要为数字类型甚至字符串实现Add trait，其实事实也如此，在self中为所有u/i num type都进行了实现
/// 其实加法对于这些num type的工作是一样的，所以，通过宏快速生成
/// `add_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }`
///
/// please see:
/// 1. Add(trait)
/// 2. add_impl!(macro)
#[const_trait]
pub trait Add<Rhs = Self> {
    /// the type after add
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

/// impl add for num types
/// see : forward_ref_binop!(macro)
macro_rules! add_impl {
    ($($t:ty)*) => ($(
     /// why use const Add for $t, please read : technical_term.md
        impl const Add for $t{
            type Output = $t;

            fn add(self, rhs:$t) -> $t {
                self + rhs
            }
        }
        forward_ref_binop!{impl const Add, add for $t, $t }
    )*);
}

/// you can add more
add_impl! {u8 i8 u32 i32}
```

### forward_ref_binop

该宏的目的就是做兼容，兼容多种不同的借用规则（对所有权的思考）

参看：`technical_term.md`的宏实现(常用形式)

```rust
//! 实现一元运算符“op&T”
//!  基于“操作T”，其中T应为“可复制”
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/2
//! @version:0.0.1
//! @description:
//! ```

/// you may see the source:
/// ``` code
/// (impl const $imp:ident, $method:ident for $t:ty, $u:ty) => {
///         forward_ref_binop!(impl const $imp, $method for $t, $u,
///                 #[stable(feature = "rust1", since = "1.0.0")]);
///     };
/// ```
/// but actually! `#[stable(feature = "rust1", since = "1.0.0")]);` is no useful to us
// #![feature(const_trait_impl)]
#[macro_export]
macro_rules! forward_ref_binop {
    (impl const $impl:ident, $method:ident for $t:ty,$u:ty) => {
        /// force:`forward_ref_binop!{impl const Add, add for $t, $t }`
        /// <hr>
        /// so we know : $t == $u

        /// such as:
        /// ``` code
        /// impl<'a> const Add<u8> for &'a u8{
        ///   // just like Self::Output
        ///   type Output = <u8 as Add<u8>>::Output;
        ///   fn add(self, rhs: u8)-><u8 as Add<u8>>::Output{
        ///       Add::add(*self,rhs)
        ///   }
        /// }
        /// ```
        impl<'a> const $impl<$u> for &'a $t {
            type Output = <$t as $impl<$u>>::Output;

            fn $method(self, rhs: $u)-><$t as $impl<$u>>::Output{
                $impl::$method(*self, rhs)
            }
        }
        // such as:
        // impl const $imp<&u8> for u8 {}
        impl const $impl<&$u> for $t{
            type Output = <$t as $impl<$u>>::Output;

            fn $method(self, rhs: &$u)-><$t as $impl<$u>>::Output{
                $impl::$method(self, *rhs)
            }
        }
        // such as:
        // impl const $imp<&u8> for &u8{}
        impl const $impl<&$u> for &$t {
            type Output = <$t as $impl<$u>>::Output;

            fn $method(self, rhs: &$u)-><$t as $impl<$u>>::Output{
                $impl::$method(*self, *rhs)
            }
        }
    };
    (impl $impl:ident, $method:ident for $t:ty,$u:ty) => {
        impl<'a>  $impl<$u> for &'a $t {
            type Output = <$t as $impl<$u>>::Output;

            fn $method(self, rhs: $u)-><$t as $impl<$u>>::Output{
                $impl::$method(*self, rhs)
            }
        }
        impl $impl<&$u> for $t{
            type Output = <$t as $impl<$u>>::Output;

            fn $method(self, rhs: &$u)-><$t as $impl<$u>>::Output{
                $impl::$method(self, *rhs)
            }
        }
        impl $imp<&$u> for &$t {
            type Output = <$t as $impl<$u>>::Output;

            fn $method(self, rhs: &$u)-><$t as $impl<$u>>::Output{
                $impl::$method(*self, *rhs)
            }
        }
    };
}
```

