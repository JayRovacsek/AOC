#![allow(unused)]
use super::*;
use crate::input::read_contents;

const TEST_INPUTS: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

#[test]
fn test_solve_part_one() {
    assert_eq!(String::from("24000"), solve_part_one(TEST_INPUTS));
}

#[test]
fn test_solve_part_two() {
    assert_eq!(String::from("45000"), solve_part_two(TEST_INPUTS));
}
