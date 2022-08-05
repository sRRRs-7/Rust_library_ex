
enum Framework {
    Vue,
    React,
    Angular,
}

enum Language {
    Javascript(Framework),
    Go,
    Rust,
}

use Framework::{Vue, React, Angular};
use Language::{Javascript, Go, Rust};

pub fn main() {
    // distinction
    let st = distinction(Javascript(React));
    println!("{}", st);

    // add
    let num = add(Some(0));
    println!("{:?}", num);
    let num2 = if_let(7);
    println!("{:?}", num2);

    // placeholder
    let n = placeholder(1);
    println!("{:?} {}", n.0, n.1);
}

// functions

fn distinction(language: Language) -> String {
    match language {
        Javascript(Vue) => String::from("Vue.js"),
        Javascript(React) => String::from("React.js"),
        Javascript(Angular) => String::from("Angular.js"),
        Go => String::from("Go"),
        Rust => String::from("Rust"),
    }
}

fn add(op: Option<i32>) -> i32 {
    match op {
        Some(x) if x > 0 => x + 3,
        Some(x) if x < 0 => x - 3,
        Some(x) => x,
        None => 0
    }
}

fn if_let(num: i32) -> i32 {
    match num {
        10 => 10,
        _ => 0,
    }
}

fn placeholder(num: u8) -> (u8, bool) {
    match num {
        1 => (1, true),
        3 => (3, true),
        5 => (5, true),
        7 => (7, true),
        _ => (0, false),
    }
}