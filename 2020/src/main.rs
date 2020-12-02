pub mod input;
pub mod puzzle;

extern crate humantime;
extern crate rand;
extern crate rayon;

use crate::input::read_contents;
use std::time::Instant;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn solve(puzzle_text: &str, day: &str) -> String {
    let stub_input = String::from("This is a stub");
    let start = Instant::now();
    match day.parse::<u8>().unwrap_or(1_u8) {
        1 => puzzle::one::solve(String::from(puzzle_text)),
        2 => puzzle::two::solve(String::from(puzzle_text)),
        3 => puzzle::three::solve(String::from(puzzle_text)),
        4 => puzzle::four::solve(String::from(puzzle_text)),
        5 => puzzle::five::solve(String::from(puzzle_text)),
        6 => puzzle::six::solve(String::from(puzzle_text)),
        7 => puzzle::seven::solve(String::from(puzzle_text)),
        8 => puzzle::eight::solve(String::from(puzzle_text)),
        9 => puzzle::nine::solve(String::from(puzzle_text)),
        10 => puzzle::ten::solve(String::from(puzzle_text)),
        11 => puzzle::eleven::solve(String::from(puzzle_text)),
        12 => puzzle::twelve::solve(String::from(puzzle_text)),
        13 => puzzle::thirteen::solve(String::from(puzzle_text)),
        14 => puzzle::fourteen::solve(String::from(puzzle_text)),
        15 => puzzle::fifteen::solve(String::from(puzzle_text)),
        16 => puzzle::sixteen::solve(String::from(puzzle_text)),
        17 => puzzle::seventeen::solve(String::from(puzzle_text)),
        18 => puzzle::eighteen::solve(String::from(puzzle_text)),
        19 => puzzle::nineteen::solve(String::from(puzzle_text)),
        20 => puzzle::twenty::solve(String::from(puzzle_text)),
        21 => puzzle::twentyone::solve(String::from(puzzle_text)),
        22 => puzzle::twentytwo::solve(String::from(puzzle_text)),
        23 => puzzle::twentythree::solve(String::from(puzzle_text)),
        24 => puzzle::twentyfour::solve(String::from(puzzle_text)),
        25 => puzzle::twentyfive::solve(String::from(puzzle_text)),
        _ => {
            let u: u8 = input::get_rand_u8(25);
            println!("Failed to parse a suitable number from input, let's enjoy some chaos and choose a random one...");
            println!("Looks like we chose {}!", u);
            match_puzzle(u)
        }
    };
    let elapsed = start.elapsed();
    println!("Time elapsed: {}", humantime::format_duration(elapsed));
    String::from("")
}

fn main() {
    loop {
        let option: u8 = input::get_option("Advent of code puzzle to complete? 1-25: ");
        match_puzzle(option);
    }
}

fn match_puzzle(option: u8) {
    let stub_input = String::from("This is a stub");
    let start = Instant::now();
    match option {
        1 => puzzle::one::solve(read_contents("./input/day_one.txt")),
        2 => puzzle::two::solve(read_contents("./input/day_two.txt")),
        3 => puzzle::three::solve(stub_input),
        4 => puzzle::four::solve(stub_input),
        5 => puzzle::five::solve(stub_input),
        6 => puzzle::six::solve(stub_input),
        7 => puzzle::seven::solve(stub_input),
        8 => puzzle::eight::solve(stub_input),
        9 => puzzle::nine::solve(stub_input),
        10 => puzzle::ten::solve(stub_input),
        11 => puzzle::eleven::solve(stub_input),
        12 => puzzle::twelve::solve(stub_input),
        13 => puzzle::thirteen::solve(stub_input),
        14 => puzzle::fourteen::solve(stub_input),
        15 => puzzle::fifteen::solve(stub_input),
        16 => puzzle::sixteen::solve(stub_input),
        17 => puzzle::seventeen::solve(stub_input),
        18 => puzzle::eighteen::solve(stub_input),
        19 => puzzle::nineteen::solve(stub_input),
        20 => puzzle::twenty::solve(stub_input),
        21 => puzzle::twentyone::solve(stub_input),
        22 => puzzle::twentytwo::solve(stub_input),
        23 => puzzle::twentythree::solve(stub_input),
        24 => puzzle::twentyfour::solve(stub_input),
        25 => puzzle::twentyfive::solve(stub_input),
        _ => {
            let u: u8 = input::get_rand_u8(25);
            println!("Failed to parse a suitable number from input, let's enjoy some chaos and choose a random one...");
            println!("Looks like we chose {}!", u);
            match_puzzle(u)
        }
    };
    let elapsed = start.elapsed();
    println!("Time elapsed: {}", humantime::format_duration(elapsed));
}
