#![allow(unused)]
use super::*;
use crate::input::read_contents;

#[test]
fn test_solve_part_one() {
    assert_eq!("6911", solve_part_one(&read_contents("./input/day_six.txt")));
}

#[test]
fn test_solve_part_two() {
    assert_eq!("3473", solve_part_two(&read_contents("./input/day_six.txt")));
}