use std::{ops::Deref, fmt::{Debug, Display}, rc::Rc};
use std::fmt;



// Box
#[derive(Debug)]
struct MyBox<T>{
    num: T
}

impl<T> MyBox<T> {
    fn new(num: T) -> MyBox<T> {
        MyBox { num }
    }
}

impl<T> Display for MyBox<T>
where T: Display{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.num)
    }
}

// Deref impl
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.num
    }
}


// drop
#[derive(Debug)]
struct Structure {
    data: String
}

// Drop impl
impl Drop for Structure{
    fn drop(&mut self) {
        println!("drop now {:?}", self.data);
    }
}


// reference count
fn rc_func() {
    let r1 = Rc::new(7);
    let r2 = Rc::clone(&r1);
    println!("Rc: {}", Rc::strong_count(&r1));
    let r3 = Rc::clone(&r1);
    println!("Rc: {}", Rc::strong_count(&r1));

    {
        let r4 = Rc::clone(&r1);
        println!("{}", r4);
        println!("Rc: {}", Rc::strong_count(&r1));
    }
    println!("Rc: {}", Rc::strong_count(&r1));

    println!("{} {} {}", r1, r2, r3);
}


pub fn main() {
    let x = 7;
    let y = MyBox::new(x);
    assert_eq!(x, *y);

    let mut s = Structure{data: String::from("hello")};
    println!("{:?}", s);
    s = Structure{data: String::from("world")};
    println!("{:?}", s);

    let n = 9;
    drop(n);
    let sum = n + 1;
    println!("{}", sum);

    rc_func();
}
