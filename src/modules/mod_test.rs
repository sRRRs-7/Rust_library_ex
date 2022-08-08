

#[cfg(test)]
mod tests {

    #[test]
    fn works() {
        assert_eq!(add_two(2), 4);
    }

    fn add_two(n: i32) -> i32 {
        n + 2
    }

    #[test]
    #[should_panic(expected = "panic error function")]
    fn error_func() {
        panic!("panic error function");
    }

    #[test]
    #[ignore]
    fn if_test() -> Result<(), String> {
        if 2 * 2 == 4 {
            Ok(())
        } else {
            Err(String::from("error function"))
        }
    }
}
