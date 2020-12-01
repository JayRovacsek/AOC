pub mod one;
pub mod two;
pub mod three;
pub mod four;
pub mod five;
pub mod six;
pub mod seven;
pub mod eight;
pub mod nine;
pub mod ten;
pub mod eleven;
pub mod twelve;
pub mod thirteen;
pub mod fourteen;
pub mod fifteen;
pub mod sixteen;
pub mod seventeen;
pub mod eighteen;
pub mod nineteen;
pub mod twenty;
pub mod twentyone;
pub mod twentytwo;
pub mod twentythree;
pub mod twentyfour;
pub mod twentyfive;

pub trait Puzzle {
    fn solve_part_one<'a>(input: &'a str) -> &'a str;
    fn solve_part_two<'a>(input: &'a str) -> &'a str;
}