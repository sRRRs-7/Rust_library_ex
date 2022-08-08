
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
struct Season {
    spring: i32,
    summer: i32,
    autumn: i32,
    winter: i32,
}

impl Season {
    fn new(spring: i32, summer: i32, autumn: i32, winter: i32) -> Self {
        Self { spring, summer, autumn, winter }
    }
}

impl Display for Season {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn main() {
    let season = Season::new(3,2,1,4,);
    println!("{}", season);
    assert_eq!(season.spring, 3);
    assert_eq!(season.summer, 2);
    assert_eq!(season.autumn, 1);
    assert_eq!(season.winter, 4);
}