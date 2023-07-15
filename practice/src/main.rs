use std::char::from_u32;
use std::collections::HashMap;
use std::mem;
use std::ops::Add;


pub mod practice;
pub mod tests;

use std::fmt;
use std::marker::PhantomData;

struct Foo<T>(i32, String, PhantomData<T>);

impl<T> fmt::Debug for Foo<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple("Foo2")
            .field(&self.0)
            .field(&self.1)
            .field(&format_args!("_"))
            .finish()
    }
}


fn main() {
    println!("{}", format!("{:?}", Foo(10, "Hello".to_string(), PhantomData::<u8>)));
}


