#![allow(unused)]
use super::*;
use crate::input::read_contents;

#[test]
fn test_solve_part_one() {
    assert_eq!(
        String::from("396"),
        solve_part_one(&read_contents("./input/day_two.txt"))
    );
}

#[test]
fn test_solve_part_two() {
    assert_eq!(
        String::from("428"),
        solve_part_two(&read_contents("./input/day_two.txt"))
    );
}

#[test]
fn test_rule_defaults() {
    let r1 = Rule::new("bad rule doesn't_panic");
    let r2 = Rule::new("also_bad rule doesn't_panic");

    // Two bad rules should result in indentical default rules
    assert_eq!(r1, r2);
}

#[test]
fn test_rule_new() {
    let r1 = Rule::new("1-3 a: abcde");
    let r2 = Rule::new("1-3 b: cdefg");
    let r3 = Rule::new("2-9 c: ccccccccc");

    // Test the struct is initialised correctly
    assert_eq!(r1.min_occurrences, 1);
    assert_eq!(r1.max_occurrences, 3);
    assert_eq!(r1.value, String::from("a"));
    assert_eq!(r1.password, String::from("abcde"));

    assert_eq!(r2.min_occurrences, 1);
    assert_eq!(r2.max_occurrences, 3);
    assert_eq!(r2.value, String::from("b"));
    assert_eq!(r2.password, String::from("cdefg"));

    assert_eq!(r3.min_occurrences, 2);
    assert_eq!(r3.max_occurrences, 9);
    assert_eq!(r3.value, String::from("c"));
    assert_eq!(r3.password, String::from("ccccccccc"));
}

#[test]
fn test_rule_functions() {
    let r1 = Rule::new("1-3 a: abcde");
    let r2 = Rule::new("1-3 b: cdefg");
    let r3 = Rule::new("2-9 c: ccccccccc");

    // Test the struct functions work as expected
    assert_eq!(r1.is_valid_sled_password(), true);
    assert_eq!(r1.is_valid_toboggan_password(), true);

    assert_eq!(r2.is_valid_sled_password(), false);
    assert_eq!(r2.is_valid_toboggan_password(), false);

    assert_eq!(r3.is_valid_sled_password(), true);
    assert_eq!(r3.is_valid_toboggan_password(), false);
}
