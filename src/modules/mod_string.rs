use std::str::from_utf8;


pub fn main() {
    let mut s = String::from("starbucks");
    s.push_str(" coffee");

    // chars
    for ch in s.chars() {
        println!("{}", ch);
    }

    // bytes + 10
    let mut arr = Vec::new();
    for c in s.bytes() {
        let by = c + 10 as u8;
        arr.push(by);
    }
    let st = from_utf8(&arr).unwrap();
    println!("{}", st);

    // bytes -10 = original
    let mut arr2 = Vec::new();
    for i in st.as_bytes() {
        let by = i - 10 as u8;
        arr2.push(by);
    }
    let st2 = from_utf8(&arr2).unwrap();
    println!("{:?}", st2);
}
