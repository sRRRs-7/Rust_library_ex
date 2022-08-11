
//!crate, container description
//! # My Crate
//! 'my_crate' is a collection of utility calculation
// cargo doc --open
#[cfg(test)]
#[test]
pub fn main() {
    // comments
    let num1 = add(3);
    assert_eq!(num1, 10);
    // documentation comments
    let num2 = add(3);
    assert_eq!(num2, 11);
}

 /// documentation comments
fn add(num: i32) -> i32 {
    num + 7
}

pub mod kind {
    /// main color list
    pub enum PrimaryColor {
        Red(u32, u32, u32),
        Yellow(u32, u32, u32),
        Blue(u32, u32, u32),
    }
    /// sub color list
    pub enum SecondaryColor {
        Orange(u32, u32, u32),
        Green(u32, u32, u32),
        Purple(u32, u32, u32),
    }
}

pub mod utils {
    use super::kind::*;
    use PrimaryColor::{Red, Yellow, Blue};
    use SecondaryColor::{Orange, Green, Purple};

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        let green = Green(0, 0xf, 0);
        let orange = Orange(0xf, 0xf, 0);
        let purple = Purple(0xf, 0, 0xf);
        green
    }
}

