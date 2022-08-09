
mod modules;
mod interface;
mod cli;
mod lib;
mod practice;

fn main() {
    // CLI
    cli::command::main();

    // interface
    // interface::user_interface::main();

    // basic
    basic();
    test();

    modules::root::main();

    practice::closure::main();
    practice::closure_generics::main();
}


// basic
fn basic() {
    let s = String::from("rusted metal");
    let mut s2 = separate(&s);
    println!("{}", s);  // rusted metal
    println!("{}", s2); // rusted

    s2 = "rust";
    println!("{}", s2); // rust
}

fn separate(s: &String) -> &str {
    let buf = s.as_bytes();

    for (i, &v) in buf.iter().enumerate() {
        if v == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}

fn test() {
    let mut s = String::from("coffee");
    let s2 = add(&mut s);
    println!("{}", s2);

    s2.push_str("foo");
    println!("{}", s2);
    println!("{}", s);
}

fn add(s: &mut String) -> &mut String {
    *s = String::from("white coffee");
    s
}