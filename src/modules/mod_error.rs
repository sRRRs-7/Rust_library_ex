

// backtrace

// [profile.release]
//panic = 'abort'

use std::{fs::File, io::ErrorKind};

pub fn main() {
    let file = File::open("text.txt");
    let file = match file {
        Ok(f) => f,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            match File::create("text.txt") {
                Ok(fc) => fc,
                Err(ec) => panic!("Error creating file: {}", ec),
            }
        }
        Err(e) => panic!("Error open file e.g privilege : {}", e),
    };
    println!("{:?}", file);
}

// shortcut methods
// unwrap(), expect(), ?, Ok()