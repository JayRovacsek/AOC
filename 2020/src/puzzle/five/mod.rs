mod test;

use std::collections::HashSet;

lazy_static! {
    static ref BIN: Vec<i32> = vec!(64, 32, 16, 8, 4, 2, 1, 4, 2, 1);
}

struct Ticket {
    position: (i32, i32),
    id: i32,
}

impl Ticket {
    fn new(input: &str) -> Self {
        let position = input.chars().enumerate().fold((0, 0), |acc, x| match x.0 {
            0..=6 => {
                if (x.1 == 'B') {
                    (acc.0 + BIN[x.0], acc.1)
                } else {
                    acc
                }
            }
            n if n > 6 => {
                if (x.1 == 'R') {
                    (acc.0, acc.1 + BIN[x.0])
                } else {
                    acc
                }
            }
            _ => unreachable!("Something went badly wrong!"),
        });

        Ticket {
            position: (position.0, position.1),
            id: position.0 * 8 + position.1,
        }
    }
}

pub fn solve(input: &str) {
    let answer_part_one = solve_part_one(&input);
    let answer_part_two = solve_part_two(&input);
    println!(
        "Part 1 answer:{:#?}\nPart 2 answer:{:#?}",
        answer_part_one, answer_part_two
    )
}

pub fn solve_both(input: &str) -> String {
    let answer_part_one = solve_part_one(input);
    let answer_part_two = solve_part_two(input);
    format!(
        "Part 1 answer:{:#?}\nPart 2 answer:{:#?}",
        answer_part_one, answer_part_two
    )
}

pub fn solve_part_one(input: &str) -> String {
    let tickets = input
        .split_whitespace()
        .map(|x| Ticket::new(x))
        .collect::<Vec<Ticket>>();

    format!("{}", tickets.iter().max_by_key(|x| x.id).unwrap().id)
}

pub fn solve_part_two(input: &str) -> String {
    let ids = input
        .split_whitespace()
        .map(|x| Ticket::new(x).id)
        .collect::<HashSet<i32>>();

    let max = ids.iter().max().unwrap_or(&0);
    let min = ids.iter().min().unwrap_or(&0);

    format!("{}", (*min..=*max).find(|x| !ids.contains(x)).unwrap_or(0))
}
