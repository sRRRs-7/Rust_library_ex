
use std::fmt;

struct Password {
    password: i32
}
impl Password {
    fn new(password: i32) -> Password {
        if password < 0 {
            panic!("Invalid password, Please enter plus number: {}", password);
        }
        Password { password }
    }
    fn show(&self) -> i32 {
        self.password
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.password.to_string();
        write!(f, "{}", s.chars().map(|_| '*').collect::<String>())
    }
}

#[derive(Debug)]
// vec struct
struct List {
    list: Vec<i32>,
}
impl List {
    fn new(v: i32) -> Self {
        self::List {
            list: vec![v]
        }
    }
    fn add(&mut self, v: i32) -> Self {
        let mut arr = self.list.clone();
        arr.push(v);
        self::List {
            list: arr
        }
    }
}

pub fn main() {
    let password = Password::new(78654);
    let num = password.show();
    println!("{}", num);

    let s = Password{password: 146643};
    println!("{}", s);

    // arr
    let mut l = List::new(0);
    l.list.push(3);
    println!("{:?}", l.list);

    let ll = l.add(7);
    println!("{:?}", ll.list);
}
