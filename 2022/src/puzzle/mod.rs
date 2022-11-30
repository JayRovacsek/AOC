pub mod eight;
pub mod eighteen;
pub mod eleven;
pub mod fifteen;
pub mod five;
pub mod four;
pub mod fourteen;
pub mod nine;
pub mod nineteen;
pub mod one;
pub mod seven;
pub mod seventeen;
pub mod six;
pub mod sixteen;
pub mod ten;
pub mod thirteen;
pub mod three;
pub mod twelve;
pub mod twenty;
pub mod twentyfive;
pub mod twentyfour;
pub mod twentyone;
pub mod twentythree;
pub mod twentytwo;
pub mod two;

pub trait Puzzle {
    fn solve_part_one(input: &str) -> String;
    fn solve_part_two(input: &str) -> String;
    fn solve(input: &str) -> String;
}
