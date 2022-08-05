
use std::{ops::Drop, mem::drop, fmt::{Display, Formatter, Result}};

#[derive(Debug)]
struct MyBox {
    data: String
}
// automatic deref method
// clean up process
impl Drop for MyBox {
    fn drop(&mut self) {
        println!("Dropping {}", self.data);
    }
}
impl Display for MyBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.data, self.data)
    }
}

pub fn main() {
    // manual drop
    let d = MyBox{data: String::from("arbitrary manual drop")};
    drop(d);

    // drop is stack
    let b = MyBox{data: String::from("world")};
    let c = MyBox{data: String::from("hello")};
    println!("{} {}", b.data, c.data);

}