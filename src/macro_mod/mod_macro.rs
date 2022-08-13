

pub trait Greet {
    fn greet();
}

struct Empty;

impl Greet for Empty {
    fn greet() {
        println!("Have a nice day");
    }
}

pub fn main() {
    Empty::greet();
}