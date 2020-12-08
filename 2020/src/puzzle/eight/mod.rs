mod test;

use std::collections::HashSet;

struct Memeboy {
    program: Vec<Command>,
    head: i32,
    acc: i32,
    executed: HashSet<i32>,
}

impl<'a> Memeboy {
    fn new(input: &str) -> Self {
        Memeboy {
            program: input
                .split("\n")
                .map(|x| Command::new(x))
                .collect::<Vec<Command>>(),
            head: 0,
            acc: 0,
            executed: HashSet::new(),
        }
    }

    fn step(&mut self) -> bool {
        let command = self.program.iter().nth(self.head as usize).unwrap();
        let x = self.head;
        if self.executed.contains(&x) {
            false
        } else {
            self.executed.insert(x);
            match command.operation {
                Operation::acc => {
                    self.acc += command.argument;
                    self.head += 1;
                }
                Operation::jmp => self.head += command.argument,
                _ => {
                    self.head += 1;
                }
            };
            true
        }
    }
}

impl Iterator for Memeboy {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        // Increment our count. This is why we started at zero.
        match self.step() {
            true => Some(self.acc),
            _ => None,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Command {
    operation: Operation,
    argument: i32,
}

impl Command {
    fn new(input: &str) -> Self {
        let parts = input.split_ascii_whitespace().collect::<Vec<&str>>();
        Command {
            operation: match *parts.iter().nth(0).unwrap() {
                "acc" => Operation::acc,
                "jmp" => Operation::jmp,
                "nop" => Operation::nop,
                _ => unreachable!("Something went wrong!"),
            },
            argument: parts.iter().last().unwrap().parse::<i32>().unwrap(),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Operation {
    acc,
    jmp,
    nop,
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
    let memboy = Memeboy::new(input);

    format!("{}", memboy.into_iter().last().unwrap())
}

pub fn solve_part_two(input: &str) -> String {
    String::from("input")
}
