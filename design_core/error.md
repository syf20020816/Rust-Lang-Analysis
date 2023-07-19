# Error

其实Error是十分简单的，需要做的就是打印错误❌，所以显而易见，Error trait需要实现Debug和Display

```rust
use std::fmt::{Debug, Display};

/// Error is a trait!
/// <hr>
/// design:
/// - Error need to be print
/// - Error need contain some msg (but sometimes it is None)
pub trait MyError: Debug + Display {
    /// 低级源信息
    fn source(&self) -> Option<&(dyn MyError + 'static)> {
        None
    }
    /// 获取TypeId
    fn type_id(&self, _: private::Internal) -> TypeId
        where
            Self: 'static,
    {
        TypeId::of::<Self>()
    }

    fn description(&self) -> &str {
        "My Error Des; use Display"
    }
    fn cause(&self) -> Option<&dyn MyError> {
        self.source()
    }
    // fn provide<'a>(&'a self,demand:&mut Demand<'a>){}
}
```

其中包含4个方法，而我们如果要自定义一个Error其实只要实现这里面的`description`这个方法即可，只要出现错误，就会默认打印这个方法中的输出

值得注意的是TypeId这个东西，每种不同的类型都会有一个唯一的ID值