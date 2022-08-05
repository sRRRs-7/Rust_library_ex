use std::fmt::Display;


pub fn main() {
    scope();

    // lifetime define function
    let s1 = String::from("prime number");
    let s2 = "number";
    let s = lifetime(s1.as_str(), s2);
    println!("compare s1 & s2 as the result {} is longer", s);

    let recv = lifetime2("xx", "yy", 135711);
    println!("longest is {}", recv);
}

fn scope() {
    let mut r = 0;
    println!("{}", r);
    {
        let x = 7;
        r = x;
        println!("{}", r)
    }
    println!("{}", r);
}

fn lifetime<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lifetime2<'a, T>(x: &'a str, y: &'a str, s: T) -> &'a str
    where T: Display {
        println!("{}", s);
        if x.len() > y.len() {
            x
        } else {
            y
        }
}