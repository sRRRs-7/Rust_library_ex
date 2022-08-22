
mod modules;
mod interface;
mod cli;
mod practice;
mod cargo;
mod smart_pointer;
mod thread;
mod trait_relation;
mod macro_mod;
mod trait_associated_generic;


fn main() {
    // // CLI
    // cli::command::main();

    // // interface
    // // interface::user_interface::main();

    // // basic
    // basic();
    // test();

    // modules::root::main();
    // // cargo module
    // cargo::mod_cargo::main();
    // // smt_ptr module
    // smart_pointer::smt_ptr::main();
    // smart_pointer::ref_cell::main();
    // smart_pointer::rc_weak::main();
    // // thread
    // thread::concurrent::main();
    // // trait relation
    // trait_relation::mod_trait::main();
    // // practice module
    // practice::closure::main();
    // practice::closure_generics::main();
    // practice::if_let::main();
    // practice::arg_fn::main();
    // // macro
    // macro_mod::mod_macro::main();

    // associated_generic
    trait_associated_generic::mod_associated_generic::main();
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