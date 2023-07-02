use std::ops::Add;

pub mod practice;
pub mod tests;


// struct MyT1;
// struct MyT2;
//
// impl Add for MyT{
//     type Output = ();
//
//     fn add(self, rhs: Self) -> Self::Output {
//         todo!()
//     }
// }

fn main() {
    let a = 8_u8;
    let b = 16;

    println!("{}", a + b);
}