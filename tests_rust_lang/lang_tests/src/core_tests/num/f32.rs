use lang_core::my_core::num::f32::MyF32;


fn main() {
    let a = MyF32::new(7.0_f32);
    println!("{}", MyF32::NAN);
    println!("{}", a.is_nan());
}

