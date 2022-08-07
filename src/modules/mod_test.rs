

#[cfg(test)]
mod tests {
    #[test]
    fn works() {
        assert_eq!(add_two(2), 4);
    }
    fn add_two(n: i32) -> i32 {
        n + 2
    }
}
