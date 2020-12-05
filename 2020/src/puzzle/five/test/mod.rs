#![allow(unused)]
use super::*;

#[test]
fn test_solve_part_one() {
    assert_eq!("This is a stub", solve_part_one("This is a stub"));
}

#[test]
fn test_solve_part_two() {
    assert_eq!("This is a stub", solve_part_two("This is a stub"));
}

#[test]
fn test_ticket_id() {
    let input_one = "BFFFBBFRRR";
    let input_two = "FFFBBBFRRR";
    let input_three = "BBFFBBFRLL";

    let ticket_one = Ticket::new(input_one);
    let ticket_two = Ticket::new(input_two);
    let ticket_three = Ticket::new(input_three);

    assert_eq!(567, ticket_one.id);
    assert_eq!(119, ticket_two.id);
    assert_eq!(820, ticket_three.id);
}

#[test]
fn test_ticket_row() {
    let input_one = "BFFFBBFRRR";
    let input_two = "FFFBBBFRRR";
    let input_three = "BBFFBBFRLL";

    let ticket_one = Ticket::new(input_one);
    let ticket_two = Ticket::new(input_two);
    let ticket_three = Ticket::new(input_three);

    assert_eq!(70, ticket_one.position.0);
    assert_eq!(14, ticket_two.position.0);
    assert_eq!(102, ticket_three.position.0);
}

#[test]
fn test_ticket_column() {
    let input_one = "BFFFBBFRRR";
    let input_two = "FFFBBBFRRR";
    let input_three = "BBFFBBFRLL";

    let ticket_one = Ticket::new(input_one);
    let ticket_two = Ticket::new(input_two);
    let ticket_three = Ticket::new(input_three);

    assert_eq!(7, ticket_one.position.1);
    assert_eq!(7, ticket_two.position.1);
    assert_eq!(4, ticket_three.position.1);
}
