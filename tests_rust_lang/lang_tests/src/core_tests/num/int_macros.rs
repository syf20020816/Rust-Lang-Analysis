use lang_core::my_core::MyImplI8;

fn main() {
    let t = MyImplI8;
    println!("{}", MyImplI8::MAX);
    println!("{}", MyImplI8::MIN);
    println!("{}", MyImplI8::BITS);
    println!("{:?}", MyImplI8::from_str_radix("16", 8));
    println!("{:?}",t.count_ones(56));
}