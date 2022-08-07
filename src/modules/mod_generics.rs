use std::fmt::Display;


pub fn main() {
    let v = gen(1);
    let s = gen("hello");
    let c = gen('a');

    println!("{} {} {}", v,s,c);
}

fn gen<T>(x: T) -> String
where T: PartialOrd + PartialEq + Display {
    let v = x.to_string();
    v
}