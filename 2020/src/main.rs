#![allow(unused)]

pub mod input;
pub mod puzzle;

extern crate humantime;
extern crate rand;
extern crate rayon;

use crate::input::read_contents;
use std::time::Instant;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate lazy_static;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn solve(puzzle_text: &str, day: &str) -> String {
    match day.parse::<u8>().unwrap_or(1_u8) {
        1 => puzzle::one::solve_both(puzzle_text),
        2 => puzzle::two::solve_both(puzzle_text),
        3 => puzzle::three::solve_both(puzzle_text),
        4 => puzzle::four::solve_both(puzzle_text),
        5 => puzzle::five::solve_both(puzzle_text),
        6 => puzzle::six::solve_both(puzzle_text),
        7 => puzzle::seven::solve_both(puzzle_text),
        8 => puzzle::eight::solve_both(puzzle_text),
        9 => puzzle::nine::solve_both(puzzle_text),
        10 => puzzle::ten::solve_both(puzzle_text),
        11 => puzzle::eleven::solve_both(puzzle_text),
        12 => puzzle::twelve::solve_both(puzzle_text),
        13 => puzzle::thirteen::solve_both(puzzle_text),
        14 => puzzle::fourteen::solve_both(puzzle_text),
        15 => puzzle::fifteen::solve_both(puzzle_text),
        16 => puzzle::sixteen::solve_both(puzzle_text),
        17 => puzzle::seventeen::solve_both(puzzle_text),
        18 => puzzle::eighteen::solve_both(puzzle_text),
        19 => puzzle::nineteen::solve_both(puzzle_text),
        20 => puzzle::twenty::solve_both(puzzle_text),
        21 => puzzle::twentyone::solve_both(puzzle_text),
        22 => puzzle::twentytwo::solve_both(puzzle_text),
        23 => puzzle::twentythree::solve_both(puzzle_text),
        24 => puzzle::twentyfour::solve_both(puzzle_text),
        25 => puzzle::twentyfive::solve_both(puzzle_text),
        _ => {
            let u: u8 = input::get_rand_u8(25);
            puzzle::one::solve_both(puzzle_text)
        }
    }
}

fn main() {
    loop {
        let option: u8 = input::get_option("Advent of code puzzle to complete? 1-25: ");
        match_puzzle(option);
    }
}

fn match_puzzle(option: u8) {
    let stub_input = "This is a stub";
    let start = Instant::now();
    match option {
        1 => puzzle::one::solve(&read_contents("./input/day_one.txt")),
        2 => puzzle::two::solve(&read_contents("./input/day_two.txt")),
        3 => puzzle::three::solve(&read_contents("./input/day_three.txt")),
        4 => puzzle::four::solve(&read_contents("./input/day_four.txt")),
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
