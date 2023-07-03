// +:43
// -:45
// 0:48
// 9:57
fn main() {
    let src = "+";
    let src_byte = src.as_bytes();
    match src_byte[0] {
        b'+' | b'-' if src_byte[1..].is_empty() => {
            println!("{}", src_byte[0])
        }
        b'+' => {
            println!("+")
        }
        b'-' => {
            println!("-")
        }
        _ => {
            println!("{}", src_byte[0]);
            for i in src_byte {
                println!("{}", i);
            }
        }
    }
    let src = "+";
    let index = 5;
    match src {
        "+" if index == 5 => {
            println!("first 5")
        }
        _ => {}
    }
}