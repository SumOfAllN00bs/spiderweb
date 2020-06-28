use super::*;

#[test]
fn caesar_test_1() {
    assert_eq!(caesar("first test phrase", 1), "gjstu uftu qisbtf");
}
#[test]
fn caesar_test_2() {
    assert_eq!(caesar("Second test phrase!", 26), "Second test phrase!");
}
#[test]
fn caesar_test_3() {
    assert_eq!(caesar("Third test phrase", -1), "Sghqc sdrs ogqzrd");
}
#[test]
fn caesar_test_4() {
    assert_eq!(caesar("fOuRtH test phrase", 0), "fOuRtH test phrase");
}
#[test]
fn caesar_test_5() {
    assert_eq!(caesar("FIFTH test phrase", -0), "FIFTH test phrase");
}
#[test]
fn caesar_test_6() {
    assert_eq!(caesar("SIxTH !\"#$%&'()*+,-./0123456789", 1000), "EUjFT !\"#$%&'()*+,-./0123456789");
}
#[test]
fn caesar_test_7() {
    assert_eq!(caesar("seventh", -26), "seventh");
}
#[test]
fn caesar_test_8() {
    assert_eq!(caesar("EiGtH test :;[\\^~#`", -6), "YcAnB nymn :;[\\^~#`");
}