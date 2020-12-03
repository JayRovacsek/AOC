#![allow(unused)]
use super::*;
use crate::input::read_contents;

#[test]
fn test_solve_part_one() {
    assert_eq!(
        "286",
        solve_part_one(&read_contents("./input/day_three.txt"))
    );
}

#[test]
fn test_solve_part_two() {
    assert_eq!(
        String::from("3638606400"),
        solve_part_two(&read_contents("./input/day_three.txt"))
    );
}

#[test]
fn test_toboggan_hill() {
    let input = String::from(
        "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
    );
    assert_eq!(2, toboggan_hill(&input, 1, 1));
    assert_eq!(7, toboggan_hill(&input, 3, 1));
    assert_eq!(3, toboggan_hill(&input, 5, 1));
    assert_eq!(4, toboggan_hill(&input, 7, 1));
    assert_eq!(2, toboggan_hill(&input, 1, 2));
}
