//!  impl ()
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/14
//! @version:0.0.1
//! @description:
//! ```


trait TupleTrait{
    fn do_change()->();
}

impl TupleTrait for (f32,f32,f32){
    fn do_change() -> () {
        println!("{}","do change");
    }
}

fn main() {
    let a:(f32, f32, f32) = (1.0_f32, 1.2_f32, 25.6_f32);
    let _ = <(f32, f32, f32)>::do_change();
}