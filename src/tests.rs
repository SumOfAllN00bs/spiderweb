use super::*;
use test::Bencher;

// *** Tests for caesar cipher *** //
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
    assert_eq!(caesar("SIxTH !\"#$%&'()*+,-./0123456789", 126), "OEtPD !\"#$%&'()*+,-./0123456789");
}
#[test]
fn caesar_test_7() {
    assert_eq!(caesar("seventh", -26), "seventh");
}
#[test]
fn caesar_test_8() {
    assert_eq!(caesar("EiGtH test :;[\\^~#`", -6), "YcAnB nymn :;[\\^~#`");
}
#[test]
fn caesar_test_9() {
    assert_eq!(caesar("nine test :3 :) :-| B-i", 27), "ojof uftu :3 :) :-| C-j");
}
#[test]
fn caesar_test_10() {
    assert_eq!(caesar("ten eleveN TwElVe THIRTEEN", 13), "gra ryrirA GjRyIr GUVEGRRA");
}

// *** Benchmarks *** //

#[bench]
fn caesar_bench_1(b: &mut Bencher) {
    b.iter(|| caesar(":;[\\^~#`X4KJS58yZ8 ld7nS2ZrZp XjjV8mxLkH :3 :) :-| B-izsToP0dR6B baxo1l0jup", 13));
}