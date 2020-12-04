mod test;

use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct Passport {
    properties: HashMap<String, String>,
}

lazy_static! {
    static ref BYR_RE: Regex = Regex::new(r"^(19[2-9][\d])$|^(200[0-2])$").unwrap();
    static ref IYR_RE: Regex = Regex::new(r"^(201[\d])$|^(2020)$").unwrap();
    static ref EYR_RE: Regex = Regex::new(r"^(202[\d])$|^(2030)$").unwrap();
    static ref HGT_RE: Regex = Regex::new(r"^(1[5-8][\d]cm)$|^(19[0-3]cm)$|^(59in)$|^(6[0-9]in)$|^(7[0-6]in)$").unwrap();
    static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
    static ref ECL_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PID_RE: Regex = Regex::new(r"^(\d){9}$").unwrap();
}

impl Passport {
    fn new(input: &str) -> Self {
        Passport {
            properties: input
                .split_whitespace()
                .map(|x| {
                    (
                        x.split(":").nth(0).unwrap().to_string(),
                        x.split(":").nth(1).unwrap().to_string(),
                    )
                })
                .collect::<HashMap<String, String>>(),
        }
    }

    fn is_valid_part_one(&self) -> bool {
        self.properties.len() == 8
            || (self.properties.len() == 7 && !self.properties.keys().any(|x| *x == "cid"))
    }

    fn is_valid_part_two(&self) -> bool {
        if self.is_valid_part_one() {
            BYR_RE.is_match(self.properties.get("byr").unwrap())
                && IYR_RE.is_match(self.properties.get("iyr").unwrap())
                && EYR_RE.is_match(self.properties.get("eyr").unwrap())
                && HGT_RE.is_match(self.properties.get("hgt").unwrap())
                && HCL_RE.is_match(self.properties.get("hcl").unwrap())
                && ECL_RE.is_match(self.properties.get("ecl").unwrap())
                && PID_RE.is_match(self.properties.get("pid").unwrap())
        } else {
            false
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
    let passports: Vec<Passport> = input.split("\n\n").map(|x| Passport::new(x)).collect();
    format!(
        "{:?}",
        passports.iter().filter(|x| x.is_valid_part_one()).count()
    )
}

pub fn solve_part_two(input: &str) -> String {
    let passports: Vec<Passport> = input.split("\n\n").map(|x| Passport::new(x)).collect();
    format!(
        "{:?}",
        passports.iter().filter(|x| x.is_valid_part_two()).count()
    )
}
