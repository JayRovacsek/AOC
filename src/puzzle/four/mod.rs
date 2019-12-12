mod test;

use rayon::prelude::*;

pub fn solve() {
    let potential_keys: Vec<usize> = (INPUT[0]..=INPUT[1]).collect::<Vec<usize>>();
    let answer_a = calculate_keyspace_part_a(&potential_keys);
    println!("The answer for day 4, part a is: {:?}", answer_a);
    let answer_b = calculate_keyspace_part_b(&potential_keys);
    println!("The answer for day 4, part b is: {:?}", answer_b);
}

fn calculate_keyspace_part_a(potential_keys: &[usize]) -> usize {
    potential_keys
        .par_iter()
        .filter(|x| has_double(**x) && !has_decrease(**x))
        .count()
}

fn calculate_keyspace_part_b(potential_keys: &[usize]) -> usize {
    potential_keys
        .par_iter()
        .filter(|x| has_exact_double(**x) && !has_decrease(**x))
        .count()
}

fn has_double(input: usize) -> bool {
    input
        .to_string()
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .map(|v| v[0] == v[1])
        .any(|v| v)
}

fn has_exact_double(input: usize) -> bool {
    "0123456789"
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .map(|x| {
            input
                .to_string()
                .chars()
                .collect::<Vec<char>>()
                .windows(2)
                .map(|y| y[0] == y[1] && y[0] == *x)
                .filter(|y| *y)
                .count()
                == 1
        })
        .filter(|x| *x)
        .any(|x| x)
}

#[allow(dead_code)]
fn digit_sequential_count_max(input: usize) -> usize {
    "0123456789"
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .map(|x| {
            input
                .to_string()
                .chars()
                .collect::<Vec<char>>()
                .windows(2)
                .map(|y| y[0] == y[1] && y[0] == *x)
                .filter(|y| *y)
                .count()
        })
        .max()
        .unwrap_or(0)
}

fn has_decrease(input: usize) -> bool {
    let mut result = false;
    let digits: Vec<_> = input.to_string().chars().collect();
    let mut iter = digits.windows(2);

    while !result {
        match iter.next() {
            Some(v) => result = v[0] > v[1],
            _ => break,
        }
    }
    result
}

const INPUT: [usize; 2] = [193_651, 649_729];
