
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

fn print(s: &str) {
    println!("{}", s);
}