
use std::slice;

pub fn main() {
    let mut num = 5;

    // raw pointer
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // unsafe rust zone
    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }

    {
        let mut s = vec![1,2,3,4,5,6];
        let (x, y) = split(&mut s, 2);
        println!("{:?} {:?}", x, y);
    }

    {
        by_the_way();
        global_var();
    }

}

fn split(slice: &mut [i32], idx: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(idx <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, idx),
        slice::from_raw_parts_mut(ptr.offset(idx as isize), len - idx))
    }

}

static mut COUNTER: i32 = 0;

fn global_var() {
    unsafe {
        COUNTER +=3
    }
    unsafe {
        COUNTER += 1;
        println!("global variable is {}", COUNTER);
    }
}



fn by_the_way() {
    let mut a = 7;
    let b = &a;
    let c = &a;
    println!("{} {}", b, c);

    let d = &mut a;
    *d = 77;
    println!("{}", d);
}