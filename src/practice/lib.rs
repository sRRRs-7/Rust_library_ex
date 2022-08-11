
use crate::modules::mod_comments::kind;

#[cfg(test)]
mod tests {
    use rand::Rng;

    #[test]
    pub fn test1() {
        use super::*;

        let rng: u8 = rand::thread_rng().gen();
    }
}

