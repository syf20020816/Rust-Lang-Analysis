use std::char::from_u32;
use std::collections::HashMap;
use std::mem;
use std::ops::Add;


pub mod practice;
pub mod tests;

#[derive(Debug)]
enum MatchU32 {
    Zero,
    NotNaN,
}

fn match_u32(jd_num: u32) -> MatchU32 {
    match (jd_num * 2, jd_num * 5) {
        (0_u32, 0_u32) => MatchU32::Zero,
        _ => MatchU32::NotNaN
    }
}

fn main() {
    println!("{:?}",match_u32(0_u32));
    println!("{:?}",match_u32(6_u32));
}
