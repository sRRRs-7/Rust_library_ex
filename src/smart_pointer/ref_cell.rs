use std::{cell::{RefCell, Cell}, rc::Rc, ops::Deref, fmt::Display, borrow::Borrow};

trait Messenger {
    fn send(&self, msg: &str);
}

#[derive(Debug)]
struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    count: usize,
    max: usize,
}

impl <'a, T> LimitTracker<'a, T>
    where T: Messenger {
        fn new(messenger: &T, max: usize) -> LimitTracker<T> {
            LimitTracker {
                messenger,
                count: 0,
                max
            }
        }
        fn set_count(&mut self, count: usize) {
            self.count = count;

            let percentage = self.count as f64 / self.max as f64;

            if percentage > 0.7 && percentage < 0.9 {
                self.messenger.send("You have used up over 70% - 90% of the quota")
            } else if percentage > 0.4 && percentage < 0.7 {
                self.messenger.send("You have used up over 40% - 70% of the quota")
            } else if percentage > 0.1 && percentage < 0.4 {
                self.messenger.send("You have used up over 10% - 40% of the quota")
            } else if percentage == 1.0 {
                self.messenger.send("Error limit of use")
            }
        }
}


// mock
#[derive(Debug)]
struct MockMessenger {
    mock_data: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger { mock_data: RefCell::new(vec![]) }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
        self.mock_data.borrow_mut().push(String::from(msg));
    }
}


// cons list
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
use List::{ Cons, Nil };

pub fn main() {
    let mock = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock, 100);

    limit_tracker.set_count(80);
    assert_eq!(limit_tracker.count, 80);

    println!("{}", mock.mock_data.borrow()[0]);


    // into_inner
    let v_rc = RefCell::new(7);
    println!("{:?}", v_rc.into_inner());

    // cons list
    // Rc, RefCell
    let value = Rc::new(RefCell::new(7));
    let v = Rc::clone(&value);

    let cons1 = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let cons2 = Rc::new(Cons(Rc::new(RefCell::new(77)), Rc::clone(&cons1)));
    let cons3 = Rc::new(Cons(Rc::new(RefCell::new(777)), Rc::clone(&cons2)));

    *value.borrow_mut() += 10;

    println!("{:?}", *cons1);
    println!("{:?}", *cons2);
    println!("{:?}", *cons3);

}


