use crate::practice::error::{CuckoldError, CuckoldKind};

fn main() {
    let e = CuckoldError{
        kind:CuckoldKind::Impulse
    };
    println!("{:?}",e);
}
