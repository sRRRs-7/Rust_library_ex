

pub fn main() {
    let num = plus_plus(plus, 3);
    println!("{}", num);

    let num2 = closure();
    println!("{:?}", num2);
}

fn plus(x: i32) -> i32 {
    x + 7
}

fn plus_plus(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x) + f(x)
}

// closure function
fn closure() -> Box<fn(i32) -> i32>  {
    Box::new(|x| x + 1)
}
