use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn puzzle() {
    let inputs: Vec<f32> = read_input()
        .iter()
        .map(|x| x.parse::<f32>().unwrap())
        .collect();
    let answer_a: f32 = inputs.iter().map(|x| calculate_fuel(*x)).sum();
    println!("The answer for day 1, part a is: {}", answer_a);
    let answer_b: f32 = inputs.iter().map(|x| calculate_fuel_recusive(*x)).sum();
    println!("The answer for day 1, part b is: {}", answer_b);
}

fn read_input() -> Vec<String> {
    let file = BufReader::new(File::open("src/one/input.txt").unwrap());
    file.lines().map(|x| x.unwrap()).collect()
}

fn calculate_fuel(input: f32) -> f32 {
    (input / 3 as f32).floor() - 2 as f32
}

fn calculate_fuel_recusive(input: f32) -> f32 {
    let result = (input / 3 as f32).floor() - 2 as f32;
    match result {
        n if n <= 0 as f32 => 0 as f32,
        _ => calculate_fuel_recusive(result) + result,
    }
}

#[test]
fn test_puzzle() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    assert_eq!(2 as f32, calculate_fuel(12 as f32));
    assert_eq!(2 as f32, calculate_fuel(14 as f32));
    assert_eq!(654 as f32, calculate_fuel(1969 as f32));
    assert_eq!(33583 as f32, calculate_fuel(100756 as f32));

    assert_eq!(2 as f32, calculate_fuel_recusive(14 as f32));
    assert_eq!(966 as f32, calculate_fuel_recusive(1969 as f32));
    assert_eq!(50346 as f32, calculate_fuel_recusive(100756 as f32));
}
