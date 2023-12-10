use std::fmt::{Debug, Formatter, Result};

struct Structure(i32);

struct Deep(Structure);

impl Debug for Deep {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0 .0)
    }
}

fn main() {
    println!("Now {:?} will print!", Deep(Structure(7)));
}
