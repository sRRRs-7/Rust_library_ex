use std::ops::Add;


type Number = String;

pub fn main() {
    type_alias();

}

fn type_alias() {
    let num: Number = String::from("7");
    let n: i32 = match num.trim().parse() {
        Ok(x) => x,
        Err(_) => panic!("panic!!"),
    };
    println!("---------------------------------------------------------------------------------------");
    println!("{}", n);

    let t = generic(7);
    println!("{}", t);
}


fn generic<T>(x: T) -> T
where T: Sized + Add<Output = T> + Copy {
    let sum = x + x;
    sum
}