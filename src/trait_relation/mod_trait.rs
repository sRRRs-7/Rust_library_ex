use std::{fmt::{self, Display}, ops::Add, ptr::addr_of};



// iterator
struct Counter {
    counter: i32
}

impl Iterator for Counter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.counter)
    }
}


// empty struct
struct Cafe;

trait Starbucks {
    fn call(&self) {
        println!("Starbucks")
    }
}

trait Tullys {
    fn call(&self) {
        println!("TULLYS")
    }
}

impl Starbucks for Cafe {
    fn call(&self) {
        println!("Starbucks")
    }
}

impl Tullys for Cafe {
    fn call(&self) {
        println!("TULLYS")
    }
}

impl Cafe {
    fn call(&self){
        println!("Cafe!!")
    }
}


struct Point {
    x: i32,
    y: i32,
}

// Display trait
 trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        print!("* ");
        print!("{}", output);
        println!(" *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
 }

 impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
 }

 impl OutlinePrint for Point {}

 // orphan rule
 struct Wrapper(Vec<String>);

 impl Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join(" "))
    }
 }


 // struct add calculation
 #[derive(Debug)]
 struct Coordinates {
    x: i32,
    y: i32,
 }

 impl Add for Coordinates {
    type Output = Coordinates;
    fn add(self, other: Coordinates) -> Coordinates {
        Coordinates {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
 }

 pub fn main() {
    let cafe = Cafe;
    cafe.call();
    <Cafe as Starbucks>::call(&cafe);   // full path
    Tullys::call(&cafe);

    // Display
    let p = Point{
        x: 7,
        y: 11,
    };
    p.outline_print();

    //orphan rule
    let wrapper = Wrapper(
        vec![
            String::from("star"),
            String::from("bucks"),
            ]
        );
    println!("{}", wrapper);

    // add trait
    let plus = Coordinates{x: 1, y: 1} + Coordinates{x: 1, y: 1};
    println!("{:?}", plus);
}
