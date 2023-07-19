use lang_core::my_core;
use lang_core::my_core::default::Default;

#[derive(Debug)]
struct A {
    data: u32,
}

impl my_core::default::Default for A {
    fn default() -> Self {
        A {
            data: 56_u32
        }
    }
}

fn main() {
    println!("{:?}", A::default());
}
