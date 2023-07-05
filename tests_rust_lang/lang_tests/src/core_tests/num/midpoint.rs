use lang_core::my_core::MidPoint;

fn main() {
    let m = MidPoint::<u8>::new(46_u8);
    println!("{}", m.calc_midpoint(16_u8));
}