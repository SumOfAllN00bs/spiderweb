use super::*;
#[test]
fn caesar_test_1() {
    assert_eq!(caesar("first test phrase", 1), "gjstu uftu qisbtf");
}

#[test]
#[ignore]
fn caesar_test_2() {
    assert_eq!(caesar("Example", 10), "Hello, World!");
}

#[test]
#[ignore]
fn caesar_test_3() {
    assert_eq!(caesar("Example", 10), "Hello, World!");
}