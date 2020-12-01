mod test;
use std::collections::HashSet;

pub fn solve(input: String) {
    let answer_part_one = solve_part_one(input.clone());
    let answer_part_two = solve_part_two(input);
    println!(
        "Part 1 answer:{:#?}\nPart 2 answer:{:#?}",
        answer_part_one, answer_part_two
    )
}

pub fn solve_part_one(input: String) -> String {
    let input_vec = input
        .split("\n")
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    let values = find_sum_values(&input_vec, 2020);

    match values {
        Some(v) => format!("{}", v.0 * v.1),
        _ => panic!("Failed to determine suitable values to meet the predicate"),
    }
}

pub fn solve_part_two(input: String) -> String {
    let input_vec: Vec<i32> = input
        .split("\n")
        .into_iter()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    let values = input_vec.iter().find_map(|x| {
        let r = find_sum_values(&input_vec, 2020 - x);
        match r {
            Some(v) => Some((*x, v.0, v.1)),
            _ => None,
        }
    });

    match values {
        Some(v) => format!("{}", v.0 * v.1 * v.2),
        _ => panic!("Failed to determine suitable values to meet the predicate"),
    }
}

fn find_sum_values(input: &Vec<i32>, target: i32) -> Option<(i32, i32)> {
    let hs: HashSet<&i32> = input.iter().collect();
    let lowest = input.iter().min().unwrap_or(&0);

    match input.iter().find(|x| {
        if *x + lowest <= target {
            let diff = target - **x;
            hs.contains(&diff)
        } else {
            false
        }
    }) {
        Some(v) => Some((*v, target - v)),
        _ => None,
    }
}
