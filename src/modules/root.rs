
use crate::modules::{
    mod_drop,
    mod_rc,
    mod_unsafe,
    mod_ffi,
    mod_lifetime,
    mod_match,
    mod_vec,
 };

use core::str;
use std::ops::Deref;
use std::ops::Drop;

#[derive(Debug)]
struct MyBox<T> (T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping");
    }
}

pub fn main() {
    {
        let a = 5;
        let b = Box::new(a);

        assert_eq!(a, 5);
        assert_eq!(*b, 5);

        let c = Box::new(String::from("substitute"));
        let d = &c;
        println!("{}", c);
        println!("{}", d);

        let e = MyBox::new(String::from("OCN"));
        print(&e);

        // drop
        let f = MyBox::new(String::from("drop"));
        println!("{:?}", f);
    }

    {
        mod_drop::main();
        mod_rc::main();
        mod_unsafe::main();
        mod_ffi::main();
        mod_lifetime::main();
        mod_match::main();
        mod_vec::main();
    }
}

fn print(s: &str) {
    println!("{}", s);
}