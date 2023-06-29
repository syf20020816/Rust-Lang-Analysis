use lang_core::my_core::MyImplI8;
use lang_core::my_core;

fn main() {
    let mut my_impl_i8 = MyImplI8{
        max:0,
        min:0
    };
    my_impl_i8.set_max();
    let a = 5_i8;

    /// 127
    println!("{:?}",my_impl_i8);
    println!("MAX = {}",my_core::MAX);
    println!("MIN = {}",my_core::MIN);
    println!("BITS = {}",my_core::BITS);
}
