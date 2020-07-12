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
    assert_eq!(caesar(":;[\\^~#`X4KJS58yZ8 ld7nS2ZrZp XjjV8mxLkH :3 :) :-| B-izsToP0dR6B baxo1l0jup", -12), ":;[\\^~#`L4YXG58mN8 zr7bG2NfNd LxxJ8alZyV :3 :) :-| P-wngHcD0rF6P polc1z0xid");
}
#[test]
fn caesar_test_9() {
    assert_eq!(caesar("nine test :3 :) :-| B-i", 27), "ojof uftu :3 :) :-| C-j");
}
#[test]
fn caesar_test_10() {
    assert_eq!(caesar(&caesar("tenth test :3 :) :-| B-i", 13), 13), "tenth test :3 :) :-| B-i");
}

// *** Tests for upper *** //

#[test]
fn upper_test_1() {
    assert_eq!(upper("alllowercase"), "ALLLOWERCASE");
}
#[test]
fn upper_test_2() {
    assert_eq!(upper("lower case with spaces"), "LOWER CASE WITH SPACES");
}
#[test]
fn upper_test_3() {
    assert_eq!(upper("ALREADY UPPERCASE"), "ALREADY UPPERCASE");
}
#[test]
fn upper_test_4() {
    assert_eq!(upper("S5hsthehMexhths"), "S5HSTHEHMEXHTHS");
}
#[test]
fn upper_test_5() {
    assert_eq!(upper("!\"#$%&'()*+,-./0123456789with symbols!\"#$%&'()*+,-./0123456789"), "!\"#$%&'()*+,-./0123456789WITH SYMBOLS!\"#$%&'()*+,-./0123456789");
}

// *** Tests for lower *** //

#[test]
fn lower_test_1() {
    assert_eq!(lower("ALLUPPERCASE"), "alluppercase");
}
#[test]
fn lower_test_2() {
    assert_eq!(lower("UPPER CASE WITH SPACES"), "upper case with spaces");
}
#[test]
fn lower_test_3() {
    assert_eq!(lower("already lowercase"), "already lowercase");
}
#[test]
fn lower_test_4() {
    assert_eq!(lower("S5HSTHEHMEXHTHS"), "s5hsthehmexhths");
}
#[test]
fn lower_test_5() {
    assert_eq!(lower("!\"#$%&'()*+,-./0123456789WITH SYMBOLS!\"#$%&'()*+,-./0123456789"), "!\"#$%&'()*+,-./0123456789with symbols!\"#$%&'()*+,-./0123456789");
}

// *** Benchmarks *** //

#[bench]
fn caesar_bench_1(b: &mut Bencher) {
    b.iter(|| caesar(":;[\\^~#`X4KJS58yZ8 ld7nS2ZrZp XjjV8mxLkH :3 :) :-| B-izsToP0dR6B baxo1l0jup", 13));
}
#[bench]
fn upper_bench_1(b: &mut Bencher) {
    b.iter(|| upper(":;[\\^~#`X4KJS58yZ8 ld7nS2ZrZp XjjV8mxLkH :3 :) :-| B-izsToP0dR6B baxo1l0jup"));
}
#[bench]
fn lower_bench_1(b: &mut Bencher) {
    b.iter(|| lower(":;[\\^~#`X4KJS58yZ8 ld7nS2ZrZp XjjV8mxLkH :3 :) :-| B-izsToP0dR6B baxo1l0jup"));
}