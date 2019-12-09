mod input;
mod puzzle;

extern crate bmp;
extern crate humantime;
extern crate rand;
extern crate rayon;

use std::time::Instant;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-rust-example!");
}

fn main() {
    loop {
        let option: u8 = input::get_option("Advent of code puzzle to complete? 1-25: ");
        match_puzzle(option);
    }
}

fn match_puzzle(option: u8) {
    let start = Instant::now();
    match option {
        1 => puzzle::one::solve(),
        2 => puzzle::two::solve(),
        3 => puzzle::three::solve(),
        4 => puzzle::four::solve(),
        5 => puzzle::five::solve(),
        6 => puzzle::six::solve(),
        7 => puzzle::seven::solve(),
        8 => puzzle::eight::solve(),
        9 => puzzle::nine::solve(),
        10 => puzzle::ten::solve(),
        11 => puzzle::eleven::solve(),
        12 => puzzle::twelve::solve(),
        13 => puzzle::thirteen::solve(),
        14 => puzzle::fourteen::solve(),
        15 => puzzle::fifteen::solve(),
        16 => puzzle::sixteen::solve(),
        17 => puzzle::seventeen::solve(),
        18 => puzzle::eighteen::solve(),
        19 => puzzle::nineteen::solve(),
        20 => puzzle::twenty::solve(),
        21 => puzzle::twentyone::solve(),
        22 => puzzle::twentytwo::solve(),
        23 => puzzle::twentythree::solve(),
        24 => puzzle::twentyfour::solve(),
        25 => puzzle::twentyfive::solve(),
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
