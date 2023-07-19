//! # Option的实现
//!
//! Rust 设计了 `Option` 这个枚举类型，是为了解决可能会出现的空值（`null`）或缺失值的问题。
//!
//! 在其他一些编程语言中，通常使用 `null` 表示一个值的缺失，但这种方式容易引发空指针异常等错误。
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/19
//! @version:0.0.1
//! @description:
//! ```

pub enum MyOption<T> {
    Some(T),
    ///无值
    None,
}

/// 其实这里的设计并没有什么
/// 真正需要吸收的是对于其中使用很多闭包作为传入参数的实现
impl<T> MyOption<T> {
    pub fn is_some(&self) -> bool {
        matches!(*self,MyOption::Some(_))
    }
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
    ///如果选项是Some并且其中的值与谓词匹配，则返回true
    pub fn is_some_and(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            MyOption::Some(x) => f(x),
            MyOption::None => false
        }
    }
    ///获取可变引用
    pub const fn as_mut(&mut self)->MyOption<&mut T>{
        match *self {
            MyOption::Some(ref mut x) => MyOption::Some(x),
            MyOption::None => MyOption::None
        }
    }
    pub fn unwrap(self)->T{
        match self {
            MyOption::Some(val) => val,
            MyOption::None => panic!("error unwrap")
        }
    }
}