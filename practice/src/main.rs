use std::ops::Add;


pub mod practice;
pub mod tests;

struct Ts {
    num: u8,
}

trait MyAdd<Rhs = Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output {
        todo!()
    }
}

impl MyAdd for Ts {
    type Output = Ts;

    fn add(self, rhs: Rhs) -> Self::Output {
        todo!()
    }
}


fn main() {


}