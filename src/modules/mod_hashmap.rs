
use std::collections::HashMap;

pub fn main() {
    let st = String::from("apple amazon facebook google microsoft google");

    let mut map = HashMap::new();
    for i in st.split_whitespace() {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    {
        test();
        test2();
    }
}

// array & vec & hashmap
fn test() {
    // average
    let mut list: [i32; 20] = [1,5,7,8,4,4,5,7,8,5,4,3,2,2,4,5,7,8,9,102];
    let mut total = 0;
    for v in &list {
        total += v;
    }
    let ave = total / list.len() as i32;
    println!("average: {}",ave);

    // median
    list.sort();
    let median = list[list.len() / 2];
    println!("median: {}", median);

    // mode
    let mut map = HashMap::new();
    for i in &list {
        let cnt = map.entry(i).or_insert(0);
        *cnt += 1;
    };
    println!("{:?}", map);
    let mut mode = (&0, 0);
    for m in map {
        if mode.1 < m.1 {
            mode = m;
        } else {
            continue;
        }
    };
    println!("mode: {:?} times {:?}", mode.0, mode.1);
}

// string
fn test2() {
    let list: [&str;4] = ["apple", "google", "mozilla", "Facebook"];
    for st in list.iter() {
        let lower = st.to_lowercase();
        let mut stg = lower.to_string();
        let fst = stg.remove(0);
        stg.push(fst);
        match fst {
            'a' => stg.push_str("hay"),
            'i' => stg.push_str("hay"),
            'u' => stg.push_str("hay"),
            'e' => stg.push_str("hay"),
            'o' => stg.push_str("hay"),
            _ => stg.push_str("ay"),
        }
        println!("{}", stg);
    }
}

