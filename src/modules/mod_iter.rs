
#[derive(Debug, Clone)]
struct Cloth {
    size: i32,
    kind: String,
}

impl Cloth {
    fn new(size: i32, kind: String) -> Cloth {
        Cloth { size, kind }
    }
}


pub fn main() {
    {
        iterator();
        iter_sum();
        iter_map();
    }

    // filter
    let cloth = vec![
        Cloth::new(170, String::from("pants")),
        Cloth::new(160, String::from("shirts")),
        Cloth::new(180, String::from("sox")),
    ];
    let cloth2 = cloth.to_vec();

    let arr = size_filter_size(cloth, 165);
    assert_eq!(arr.len(), 2);

    let arr2 = size_filter_kind(cloth2, String::from("pants"));
    assert_eq!(arr2.len(), 1);

    // iterator trait counter
    count_iter();
}


fn size_filter_size(cloth: Vec<Cloth>, size: i32) -> Vec<Cloth> {
    let arr = cloth.into_iter()
        .filter(|x| { x.size > size })
        .collect();
    arr
}

fn size_filter_kind(cloth: Vec<Cloth>, kind: String) -> Vec<Cloth> {
    let arr = cloth.into_iter()
        .filter(|x| { x.kind == kind })
        .collect();
    arr
}


fn iterator() {
    let arr = vec![1, 2, 3, 4, 5];

    let mut i = arr.iter();

    assert_eq!(i.next(), Some(&1));
    assert_eq!(i.next(), Some(&2));
    assert_eq!(i.next(), Some(&3));
    assert_eq!(i.next(), Some(&4));
    assert_eq!(i.next(), Some(&5));
    assert_eq!(i.next(), None);
}


fn iter_sum() {
    let arr = vec![1, 2, 3, 4, 5];
    let i = arr.iter();
    let s: i32 = i.sum();
    assert_eq!(s, 15)
}


fn iter_map() {
    let arr = vec![1, 2, 3, 4, 5];
    let m: Vec<i32> = arr.iter().map(|x| {x * 2}).collect();

    let sum: i32 = m.iter().sum();
    assert_eq!(30, sum);
}


struct Counter {
    num: i32
}

impl Counter {
    fn new(num: i32) -> Counter {
        Counter { num }
    }
}

impl Iterator for Counter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.num += 1;

        if self.num < 10 {
            Some(self.num)
        } else {
            Some(0)
        }
    }
}

// main
fn count_iter() {
    let mut cnt = Counter::new(0);
    let mut num = cnt.next().unwrap();
    assert_eq!(num, 1);
    num = cnt.next().unwrap();
    assert_eq!(num, 2);

   // methods
    methods();
}

fn methods() {
    let arr = vec![1, 2, 3, 4, 5, 6];
    let arr2 = vec![1, 2, 3, 4, 5, 6];
    let cnt: i32 = arr.into_iter().zip(arr2.into_iter().skip(1)).map(|(a, b)| {a + b}).filter(|x| x % 3 == 0).sum();
    println!("{}", cnt);
    println!("----------------------")
}

