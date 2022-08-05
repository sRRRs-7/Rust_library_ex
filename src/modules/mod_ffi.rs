
extern "C" {
    fn abs(input: i32) -> i32;
}

// FFI (foreign function interface)
pub fn main() {
    // call from rust to C
    unsafe {
        let num = abs(-7);
        println!("C function called: {}", num);
    }

    {
        call_from_c();
    }
}

// call from C to rust
#[no_mangle]
pub fn call_from_c() {
    println!("this function is called from C lang");
}


