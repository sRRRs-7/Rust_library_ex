
struct Average {
    list: Vec<i32>,
    average: i32,
}

trait Calculate {
    fn add(&mut self, value: i32) -> i32;
    fn list_pop(&mut self) -> Option<i32>;
    fn get_ave(&self) -> i32;
    fn get_sum(&self) -> i32;
    fn update_ave(&self) -> i32;
}

impl Average {
    fn new(a: Vec<i32>) -> Self {
        let ave = (&a).iter().sum::<i32>() / (&a).len() as i32;
        Self {
            list: a,
            average: ave,
        }
    }
}

impl Calculate for Average {
    fn add(&mut self, value: i32) -> i32 {
        self.list.push(value);
        let ave = self.update_ave();
        ave
    }
    fn list_pop(&mut self) -> Option<i32> {
        let p = self.list.pop();
        match p {
            Some(x) => {
                self.update_ave();
                Some(x)
            },
            None => None,
        }
    }
    fn get_ave(&self) -> i32 {
        self.average
    }
    fn get_sum(&self) -> i32 {
        self.list.iter().sum::<i32>()
    }
    fn update_ave(&self) -> i32 {
        let ave = self.list.iter().sum::<i32>() / self.list.len() as i32;
        ave
    }
}


pub fn main() {
    let mut ave_list = Average::new(vec![1, 2, 3, 4, 5]);
    let a = ave_list.add(6);
    println!("--------------------------------------------------------------------------");
    println!("average: {}", a);

    let b = ave_list.list_pop().unwrap();
    println!("average: {}", b);

    let c = ave_list.get_ave();
    println!("{}", c);

    let d = ave_list.get_sum();
    println!("{}", d);

    let e = ave_list.update_ave();
    println!("{}", e);
}