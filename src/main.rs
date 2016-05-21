use std::fmt::{Display, Formatter, Result};

struct My {
    field1: i32
}

impl Drop for My {
    fn drop(&mut self) {
        println!("{} is dropping", self)    
    }
}

impl Display for My {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "My(field1 = {})", self.field1)
    }
}

fn main() {
    let _my = My { field1: 1 };
    println!("Done.")
}
