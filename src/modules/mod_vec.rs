
#[derive(Debug)]
enum List {
    Num(i64),
    Float(f64),
    Text(String),
}
use List::{Num, Float, Text};

pub fn main() {
    let arr: Vec<List> = vec![
        Num(256),
        Float(7.0),
        Text(String::from("Rust")),
    ];

    let mut stg: Vec<String> = Vec::new();
    for i in arr.into_iter() {
        let st = subtract(&i);
        stg.push(st);
    }
    println!("{:?}", stg)
}

fn subtract(v: &List) -> String {
    match v {
        Num(x) => x.to_string(),
        Float(x) => x.to_string(),
        Text(x) => x.to_string(),
    }
}