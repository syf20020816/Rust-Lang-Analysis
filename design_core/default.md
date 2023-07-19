# Default

其实主要来说就是一个Default trait实现

Default用于生成一个默认的初始值

这个trait十分简单，里面只包含一个需要实现的方法

```rust
trait Default: Sized {
    fn default() -> Self;
}
```

其实写到这里已经基本上结束了，因为Default这个trait就是让调用者调用default方法生成一个默认值罢了，其中还有一个default_impl!宏单纯就是为了让默认的一些类型直接实现Default。

```rust
default_impl! { (), (), "Returns the default value of `()`" }
default_impl! { bool, false, "Returns the default value of `false`" }
default_impl! { char, '\x00', "Returns the default value of `\\x00`" }

default_impl! { usize, 0, "Returns the default value of `0`" }
default_impl! { u8, 0, "Returns the default value of `0`" }
default_impl! { u16, 0, "Returns the default value of `0`" }
default_impl! { u32, 0, "Returns the default value of `0`" }
default_impl! { u64, 0, "Returns the default value of `0`" }
default_impl! { u128, 0, "Returns the default value of `0`" }

default_impl! { isize, 0, "Returns the default value of `0`" }
default_impl! { i8, 0, "Returns the default value of `0`" }
default_impl! { i16, 0, "Returns the default value of `0`" }
default_impl! { i32, 0, "Returns the default value of `0`" }
default_impl! { i64, 0, "Returns the default value of `0`" }
default_impl! { i128, 0, "Returns the default value of `0`" }

default_impl! { f32, 0.0f32, "Returns the default value of `0.0`" }
default_impl! { f64, 0.0f64, "Returns the default value of `0.0`" }
```

