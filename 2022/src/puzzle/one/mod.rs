use itertools::Itertools;

mod test;

pub fn solve(input: &str) {
    let answer_part_one = solve_part_one(&input);
    let answer_part_two = solve_part_two(&input);
    println!(
        "Part 1 answer:{:#?}\nPart 2 answer:{:#?}",
        answer_part_one, answer_part_two
    )
}

fn parse_elves(input: &str) -> Vec<Vec<i64>> {
    input
        .split("\n\n")
        .map(|x| x.split('\n').map(|y| y.parse::<i64>().unwrap()).collect())
        .collect::<Vec<Vec<i64>>>()
}

pub fn solve_part_one(input: &str) -> String {
    let elves = parse_elves(input);
    let calories: i64 = elves.iter().map(|x| x.iter().sum()).max().unwrap_or(0_i64);
    format!("{:?}", calories)
}

pub fn solve_part_two(input: &str) -> String {
    let elves = parse_elves(input);
    let mut calories: Vec<i64> = elves.iter().map(|x| x.iter().sum()).collect();
    calories.sort();
    let top_three: i64 = calories.iter().rev().take(3).sum();
    format!("{:?}", top_three)
}
