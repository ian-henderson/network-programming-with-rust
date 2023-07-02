use std::fmt::{self, Display, Formatter, Result};

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> fmt::Display for Point<T> where T: Display {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 4u32, y: 2u32 };

    // uses Display
    println!("{}", p);

    // uses Debug
    println!("{:?}", p);
}

