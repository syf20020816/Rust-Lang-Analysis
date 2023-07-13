use std::char::from_u32;
use std::collections::HashMap;
use std::mem;
use std::ops::Add;
use std::str::FromStr;


pub mod practice;
pub mod tests;


fn do_something(v: &mut Vec<&str>) {
    let a = "t";
    v.push(a);
}

fn main() {
    let mut c: Vec<&str> = vec![];
    do_something(&mut c);
    println!("{:?}", c);
}
