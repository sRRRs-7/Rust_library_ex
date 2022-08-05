
use std::rc::Rc;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn main() {
    let a = Rc::new(
        Cons(7, Rc::new(
            Cons(5, Rc::new(
                Cons(3, Rc::new(Nil)))))));
    let b = Cons(1, Rc::clone(&a));
    let c = Cons(0, Rc::clone(&a));
    println!("{:?} {:?} {:?}", a, b, c);

    println!("{}", Rc::strong_count(&a));

    main2();
}

use List2::{Cons2, Nil2};
#[derive(Debug)]
enum List2<'a> {
    Cons2(i32, &'a List2<'a>),
    Nil2,
}

pub fn main2() {
    let a =Cons2(7, &(Cons2(5, &(Cons2(3, &(Nil2))))));
    let b = Cons2(1, &a);
    let c = Cons2(0, &a);
    println!("{:?} {:?} {:?}", a, b, c);

    println!("{:?}", a);
}