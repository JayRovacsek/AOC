#![allow(unused)]
use super::*;
use crate::input::read_contents;

#[test]
fn test_solve_part_one() {
    assert_eq!("32064", solve_part_one(read_contents("./input/day1.txt")));
}

#[test]
fn test_solve_part_two() {
    // assert_eq!(String::from("This is a stub"), solve_part_two(String::from("This is a stub")));
}

#[test]
fn test_find_sum_values() {
    assert_eq!(Some((1, 2)), find_sum_values(&vec![1, 2, 3], 3));
}
