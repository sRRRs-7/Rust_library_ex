
// if let, else if, else if let
pub fn main() {
    if_let_func();
    while_let();
    range_match();
    divide_substitute();
    omit_description();
    vec_struct_sum();
    ref_match();
    match_guard();
    enum_match();
}


fn if_let_func() {
    let favorite_color: Option<&str> = None;
    let is_thursday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("{}", color);
    } else if is_thursday {
        println!("Thursday");
    } else if let Ok(age) = age {
        println!("{}", age);
    } else {
        println!("default value");
    }
}


fn while_let() {
    let mut arr = Vec::new();
    arr.push(3);
    arr.push(5);
    arr.push(7);

    while let Some(n) = &arr.pop() {
        println!("while {}", n);
    }
}


fn range_match() {
    let x = 4;
    match x {
        1..=5 => println!("{}", x),
        _ => println!("number is out of 1..5"),
    }

    let c = 'c';
    match c {
        'a'..='m' => println!("{}", x),
        'o'..='z' => println!("{}", x),
        _ => println!("not match"),
    }
}


#[derive(Debug)]
struct Rotate {
    roll: i32,
    pitch: i32,
    yaw: i32,
}

fn divide_substitute() {
    let rotate = Rotate {
        roll: 1,
        pitch: 1,
        yaw: 1,
    };

    match rotate {
        Rotate { roll: 0, pitch: 1, yaw} => println!("1 : {}", yaw),
        Rotate { roll: 1, pitch, yaw} => println!("2 : {} {}", pitch, yaw),
        Rotate { roll: 0, pitch, yaw} => println!("3 : {} {}", pitch, yaw),
        _ => panic!("not match")
    }

    let Rotate{roll: z, pitch: x, yaw: y} = rotate;
    println!("{} {} {}", z,x,y);
}


fn omit_description() {
    let tuple = (1,2,3,4,5);

    match tuple {
        (first, .., last) => println!("{} {}", first, last),
    }
}


struct Number {
    num1: i32,
    num2: i32,
    num3: i32,
}

impl Number {
    fn new(num1: i32, num2: i32, num3: i32) -> Self {
        Self { num1, num2, num3 }
    }
}

fn vec_struct_sum() {
    let numbers = vec![
        Number::new(1,2,3),
        Number::new(4,5,6),
        Number::new(7,8,9),
    ];

    let sum: i32 = numbers.into_iter().map(|Number { num1, num2, num3 }| {
        num1 + num2 + num3
    }).sum();

    println!("{}", sum);
}


fn ref_match() {
    let s = Some(String::from("Apple"));
    match s {
        Some(ref x) => println!("{}", x),
        None => (),
    }
    println!("{:?}", s);


    let mut s2 = Some(String::from("Google"));
    match s2 {
        Some(ref mut x) => *x = String::from("Linux"),
        None => (),
    }
    println!("{:?}", s2);
}


fn match_guard() {
    let x = Some(5);
    let y = 7;

    let num = match x {
        Some(50) => 50,
        Some(n) if n == y => y,
        Some(n) if n < 6 => n,
        _ => 0
    };
    println!("{}", num);

    let s = 7;
    let b = true;

    let m = match s {
        5|6|7 if b => s,
        _ => 0
    };
    println!("{}", m);
}


enum Message {
    Letter {subject: i32}
}
use Message::Letter;

fn enum_match() {
    let letter = Letter { subject: 3};
    let l = match letter {
        Letter { subject: num @ 6..=10} => num,
        Letter { subject: num @ 1..=5} => num,
        _ => 0,
    };

    println!("{}", l);
}