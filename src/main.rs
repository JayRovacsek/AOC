mod input;
mod one;
mod two;
mod three;
mod four;
mod five;
mod six;
mod seven;
mod eight;
mod nine;
mod ten;
mod eleven;
mod twelve;
mod thirteen;
mod fourteen;
mod fifteen;
mod sixteen;
mod seventeen;
mod eighteen;
mod nineteen;
mod twenty;
mod twentyone;
mod twentytwo;
mod twentythree;
mod twentyfour;
mod twentyfive;

extern crate rand;

fn main() {
    loop {
        let option: u8 = input::get_option("Advent of code puzzle to complete? 1-25: ");
        match_puzzle(option);
    }
}

fn match_puzzle(option: u8) {
    match option {
        1 => one::puzzle(),
        2 => two::puzzle(),
        3 => three::puzzle(),
        4 => four::puzzle(),
        5 => five::puzzle(),
        6 => six::puzzle(),
        7 => seven::puzzle(),
        8 => eight::puzzle(),
        9 => nine::puzzle(),
        10 => ten::puzzle(),
        11 => eleven::puzzle(),
        12 => twelve::puzzle(),
        13 => thirteen::puzzle(),
        14 => fourteen::puzzle(),
        15 => fifteen::puzzle(),
        16 => sixteen::puzzle(),
        17 => seventeen::puzzle(),
        18 => eighteen::puzzle(),
        19 => nineteen::puzzle(),
        20 => twenty::puzzle(),
        21 => twentyone::puzzle(),
        22 => twentytwo::puzzle(),
        23 => twentythree::puzzle(),
        24 => twentyfour::puzzle(),
        25 => twentyfive::puzzle(),
        _ => {
            let u: u8 = input::get_rand_u8(25);
            println!("Failed to parse a suitable number from input, let's enjoy some chaos and choose a random one...");
            println!("Looks like we chose {}!", u);
            match_puzzle(u)
        }
    }
}
