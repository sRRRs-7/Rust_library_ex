use std::{rc::{Rc, Weak}, cell::RefCell};

#[derive(Debug)]
struct Node {
    parent: RefCell<Weak<Node>>,
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn main() {
    let leaf = Rc::new(Node {
        parent: RefCell::new(Weak::new()),
        value: 7,
        children: RefCell::new(vec![]),
    });

    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        parent: RefCell::new(Weak::new()),
        value: 77,
        children: RefCell::new(vec!(Rc::clone(&leaf))),
    });
    // create weak pointer
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent: {:?}", *leaf.parent.borrow().upgrade().unwrap());

    println!("leaf strong reference: {} leaf weak reference: {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    println!("branch strong reference: {} branch weak reference: {}", Rc::strong_count(&branch), Rc::weak_count(&branch));

}