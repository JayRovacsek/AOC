mod test;

struct Ticket {
    position: (i32, i32),
    id: i32,
    input: String,
}

impl Ticket {
    fn new(input: &str) -> Self {
        let pos = input
            .chars()
            .enumerate()
            .fold((0, 127, 0, 7, 0, 0), |acc, x| match x {
                n if n.0 < 6 && n.1 == 'F' => {
                    (acc.0, ((acc.1 + acc.0) >> 1), acc.2, acc.3, acc.4, acc.5)
                }
                n if n.0 < 6 && n.1 == 'B' => {
                    ((acc.0 + acc.1 + 1) / 2, acc.1, acc.2, acc.3, acc.4, acc.5)
                }
                n if n.0 == 6 && n.1 == 'F' => (acc.0, acc.1, acc.2, acc.3, acc.0, acc.5),
                n if n.0 == 6 && n.1 == 'B' => (acc.0, acc.1, acc.2, acc.3, acc.1, acc.5),
                n if n.0 > 6 && n.0 < 9 && n.1 == 'L' => {
                    (acc.0, acc.1, acc.2, ((acc.3 + acc.2) >> 1), acc.4, acc.5)
                }
                n if n.0 > 6 && n.0 < 9 && n.1 == 'R' => {
                    (acc.0, acc.1, (acc.3 + 1) / 2, acc.3, acc.4, acc.5)
                }
                n if n.0 == 9 && n.1 == 'L' => (acc.0, acc.1, acc.2, acc.3, acc.4, acc.2),
                n if n.0 == 9 && n.1 == 'R' => (acc.0, acc.1, acc.2, acc.3, acc.4, acc.3),
                _ => acc,
            });

        Ticket {
            position: (pos.4, pos.5),
            id: pos.4 * 8 + pos.5,
            input: String::from(input),
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
    let tickets = input
        .split_whitespace()
        .map(|x| Ticket::new(x))
        .collect::<Vec<Ticket>>();

    let spots = (0..=127)
        .map(|x| (0..=7).map(|y| (x, y)).collect::<Vec<(i32, i32)>>())
        .flatten()
        .collect::<Vec<(i32, i32)>>();

    let empty_spots = spots.iter().filter(|x| {
        tickets
            .iter()
            .find(|y| y.position.0 == x.0 && y.position.1 == x.1)
            .is_some()
    }).collect::<Vec<&(i32,i32)>>();

    println!("{:?}", empty_spots);

    // format!("{}", tickets.iter().filter(|x| x.position).unwrap().id)
    format!("")
}
