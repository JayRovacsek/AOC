mod test;

use std::collections::HashSet;

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
    format!("{}", toboggan_hill(&input, 3, 1))
}

pub fn solve_part_two(input: &str) -> String {
    let first_slope = toboggan_hill(&input, 1, 1);
    let second_slope = toboggan_hill(&input, 3, 1);
    let third_slope = toboggan_hill(&input, 5, 1);
    let forth_slope = toboggan_hill(&input, 7, 1);
    let fifth_slope = toboggan_hill(&input, 1, 2);
    format!(
        "{}",
        first_slope * second_slope * third_slope * forth_slope * fifth_slope
    )
}

fn toboggan_hill(input: &str, x_step: usize, y_step: usize) -> usize {
    let map: HashSet<(usize, usize, bool)> = input
        .split("\n")
        .enumerate()
        .map(|x| {
            x.1.chars()
                .enumerate()
                .map(|y| (y.0, x.0, y.1 == '#'))
                .collect::<Vec<(usize, usize, bool)>>()
        })
        .flatten()
        .collect();

    let end = map.iter().max_by_key(|x| x.1).unwrap_or(&(0, 0, false)).1;
    let width = map.iter().max_by_key(|x| x.0).unwrap_or(&(0, 0, false)).0;

    let path: Vec<(usize, usize)> = (1..=(end / y_step))
        .fold((vec![(0, 0)], 0), |acc, y_pos| {
            let x_pos = (acc.1 + x_step) % (width + 1);
            let mut v = acc.0;
            v.push((x_pos, y_pos * y_step));
            (v, x_pos)
        })
        .0;

    path.iter()
        .fold(0, |acc, x| match map.contains(&(x.0, x.1, true)) {
            true => acc + 1,
            _ => acc,
        })
}
