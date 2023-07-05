use std::char::from_u32;
use std::mem;
use std::ops::Add;


pub mod practice;
pub mod tests;


fn main() {
    //01
    println!("{:?}",1_u8.count_ones());
    //10
    println!("{:?}",2_u8.count_ones());
    //
    println!("{:?}",53_u8.count_ones());
    println!("{:?}",5_u8.count_ones());
}