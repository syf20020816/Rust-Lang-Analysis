# Ops Design

## 👍Arith Design

Arith是各类运算符号的设计(`+,-,*,/,%`)

整体上来看单个运算符由两个部分组成：

1. Arith Trait（运算符实现的Trait）
2. Arith Macro（生成类型运算符的宏）

通过`Arith Macro`将统一的生成2类`impl Trait for Type`

1. `impl const Trait for Type`
2. `impl Trait for Type`

在Rust中，`impl MyTrait`和`impl const MyTrait`之间有一些区别。

1. 功能特性：`impl MyTrait`用于实现一个普通的trait，该trait可以包含任意的方法和关联类型。而`impl const MyTrait`用于实现一个特殊的trait，该trait只能包含具有`const`修饰符的方法。
2. 方法调用：对于`impl MyTrait`实现的trait，其方法可以通过实例对象进行调用，即可以使用动态分发。而`impl const MyTrait`实现的trait中的方法只能通过类型名直接调用，即使用静态分发。
3. 特化常量：`impl MyTrait`可以在实现中使用特定的常量值。对于`impl const MyTrait`，它允许在trait的方法中使用const常量，并保证在编译时进行计算。这可以用于在编译期间优化代码。

需要注意的是，`impl const`是Rust 1.50版本引入的新功能，因此如果你使用较早的Rust版本，可能不支持`impl const`。

> ❗note: 1.72.0中移除了const

然后就是内部的对方法的实现同样分为3类(实现于`core/src/internal_macros.rs`)：

1. `fn $method(self, rhs: $u)->Self::Output`
2. `fn $method(self, rhs: &$u)->Self::Output`
3. `fn $method(self, rhs: &$u)->Self::Output`

这是因为需要关注数据所有权，但是最终这三种实现都会去调用`core/src/ops/arith.rs`中的原始方法，主要还是为了兼容（这个兼容的写法是很重要的，在日常开发中很值得借鉴）

原始方法以`arith::Add`为例：

```rust
        impl const Add for $t{
            type Output = $t;

            fn add(self, rhs:$t) -> $t {
                self + rhs
            }
        }
```

然后在其中使用`forward_ref_binop!{impl const Add, add for $t, $t }`生成了如下代码进行兼容

```rust
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

