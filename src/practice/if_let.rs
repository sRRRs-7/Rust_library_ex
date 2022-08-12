
// if let, else if, else if let
pub fn main() {
    if_let_func();
    while_let();
    range_match();
    divide_substitute();
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


struct Rotate {
    roll: f64,
    pitch: f64,
    yaw: f64,
}

fn divide_substitute() {
    let rotate = Rotate {
        roll: 0.5,
        pitch: 0.5,
        yaw: 0.5,
    };

    let Rotate{roll: z, pitch: x, yaw: y} = rotate;
    println!("{} {} {}", z,x,y);
}