mod test;

use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref DIGIT: Regex = Regex::new(r"(\d)+").unwrap();
    static ref BAG_DESCRIPTION: Regex = Regex::new(r"([a-z]+\s[a-z]+)").unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bag {
    description: String,
    contents: Vec<(String, usize)>,
}

impl Bag {
    fn new(input: &str) -> Self {
        let parts = input
            .split(" bags contain ")
            .map(|x| x.split(", ").collect::<Vec<&str>>())
            .flatten()
            .collect::<Vec<&str>>();

        let description = String::from(*parts.first().unwrap());

        let contents = parts
            .iter()
            .filter_map(|x| {
                if DIGIT.is_match(x) {
                    Some((
                        String::from(BAG_DESCRIPTION.find(x).unwrap().as_str()),
                        DIGIT.find(x).unwrap().as_str().parse::<usize>().unwrap(),
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<(String, usize)>>();

        Bag {
            description,
            contents,
        }
    }

    fn has_children(&self) -> bool {
        self.contents.is_empty()
    }

    fn children<'a>(&self, bags: &'a HashSet<Bag>) -> HashSet<&'a Bag> {
        bags.iter()
            .filter(|x| self.contents.iter().any(|y| y.0 == x.description))
            .collect::<HashSet<&'a Bag>>()
    }

    fn child_contains(&self, bag: &str, bags: &HashSet<Bag>) -> bool {
        self.children(bags).iter().any(|x| x.description == bag)
    }

    fn descendent_contains(&self, bag: &str, bags: &HashSet<Bag>) -> bool {
        if !self.children(bags).iter().any(|x| x.description == bag) {
            self.children(bags)
                .iter()
                .any(|x| x.descendent_contains(bag, bags))
        } else {
            true
        }
    }

    fn parents(&self) -> Vec<&str> {
        self.contents
            .iter()
            .map(|x| x.0.as_ref())
            .collect::<Vec<&str>>()
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

fn find_parents(bag: &str, bags: HashSet<Bag>) -> HashSet<Bag> {
    let direct_parents = bags
        .iter()
        .filter(|x| x.child_contains(bag, &bags))
        .cloned()
        .collect::<HashSet<Bag>>();

    vec![
        direct_parents.clone(),
        direct_parents
            .iter()
            .map(|x| find_parents(&x.description, bags.clone()))
            .flatten()
            .collect::<HashSet<Bag>>(),
    ]
    .iter()
    .flatten()
    .cloned()
    .collect::<HashSet<Bag>>()
}

fn map_children(bag: &str, bags: &HashSet<Bag>) -> usize {
    let b1 = bags
        .iter()
        .filter(|x| x.description == bag)
        .collect::<Vec<&Bag>>();

    let b2 = b1.first().unwrap();
    b2.contents.iter().fold(0, |acc, x| acc + x.1)
        + b2.children(&bags)
            .iter()
            .map(|x| map_children(x.description.as_ref(), &bags))
            .sum::<usize>()
}

pub fn solve_part_one(input: &str) -> String {
    let bags = input
        .split("\n")
        .map(|x| Bag::new(x))
        .collect::<HashSet<Bag>>();

    let parent_bags = find_parents("shiny gold", bags);

    format!(
        "{}",
        parent_bags.iter().unique_by(|x| &x.description).count()
    )
}

pub fn solve_part_two(input: &str) -> String {
    let bags = input
        .split("\n")
        .map(|x| Bag::new(x))
        .collect::<HashSet<Bag>>();

    format!("{}", map_children("shiny gold", &bags))
}
