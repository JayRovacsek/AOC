#![allow(unused)]
use super::*;
use crate::input::read_contents;

#[test]
fn test_solve_part_one() {
    assert_eq!(
        "901",
        solve_part_one(&read_contents("./input/day_five.txt"))
    );
}

#[test]
fn test_solve_part_two() {
    assert_eq!("661", solve_part_two(&read_contents("./input/day_five.txt")));
}

#[test]
fn test_tickets_missing() {
    let input = &read_contents("./input/day_five.txt");
    let tickets = input
        .split_whitespace()
        .map(|x| Ticket::new(x))
        .collect::<Vec<Ticket>>();
    assert_eq!(
        false,
        tickets
            .iter()
            .any(|x| x.position.0 == 0 || x.position.0 == 127)
    );
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

#[test]
fn test_ticket_column_max() {
    let input = "BFFFBBFRRR";

    let ticket = Ticket::new(input);

    assert_eq!(7, ticket.position.1);
}

#[test]
fn test_ticket_row_max() {
    let input = "BBBBBBBRRR";

    let ticket = Ticket::new(input);

    assert_eq!(127, ticket.position.0);
}
