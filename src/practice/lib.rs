

#[cfg(test)]
mod tests {
    use rand::Rng;

    #[test]
    pub fn test1() {
        use super::*;

        let rng: u8 = rand::thread_rng().gen();
        assert_eq!(rng, 7)
    }
}

