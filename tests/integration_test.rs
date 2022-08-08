
// command
// cargo test --test integration_test

mod  common;

#[cfg(test)]
#[test]
fn test1() {
    common::setup();

    let mut num = 0;
    for i in 0..10 {
        num += i;
    }
    assert_eq!(num, 45);
}