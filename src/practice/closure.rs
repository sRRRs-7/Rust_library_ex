
use rand::{self, Rng};
use std::{thread, time};

struct Casher<T> {
    calculation: T,
    value: Option<u32>,
}

impl<T> Casher<T>
where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Casher<T> {
        Casher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(x) if Some(x) != Some(arg) => {
                let v = (self.calculation) (arg);
                self.value = Some(v);
                v
            }
            Some(x) => x,
            None => {
                let v = (self.calculation) (arg);
                self.value = Some(v);
                v
            },
        }
    }
}

pub fn main() {
    let rng: u32 = rand::thread_rng().gen();
    println!("{:?}", rng);

    gen_work(27, rng);

    let mut w = Casher::new(|x| x);
    w.value(1);
    w.value(2);
    println!("{:?}", w.value);

    // capture variable
    closure_move();
}


pub fn gen_work(intensity: u32, random: u32) {
    let mut gen_work_calculate = Casher::new(|num| {
            println!("calculating...");
            thread::sleep(time::Duration::from_millis(1));
            num
        });
    if intensity < 25 {
        println!(" do {} push ups!!", gen_work_calculate.value(intensity));
        println!(" next do {} sit ups!!", gen_work_calculate.value(intensity));
    } else {
        if random > 21 * 10000000 {
            println!("today is rest");
        } else {
            println!("today is {} minutes running", gen_work_calculate.value(intensity));
        }
    }
}


fn closure_move() {
    let x = vec![1,2,3,4,5];

    let closure = move |z| {z == x};

    let arr2 = vec![1,2,3,4,5];

    assert!(closure(arr2));
}

