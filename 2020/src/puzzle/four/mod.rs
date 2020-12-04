mod test;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Passport {
    iyr: Option<u32>,
    cid: Option<u32>,
    pid: Option<u32>,
    eyr: Option<u32>,
    hcl: Option<String>,
    ecl: Option<String>,
    byr: Option<u32>,
    hgt: Option<u32>,
}

impl Passport {
    fn new(input: &str) -> Self {
        let parts: Vec<&str> = input.split_whitespace().collect();
        Passport {
            iyr: parts
                .iter()
                .find(|x| x.contains("iyr"))
                .map(|x| x.split(":").last().unwrap().parse::<u32>().unwrap()),
            cid: parts
                .iter()
                .find(|x| x.contains("cid"))
                .map(|x| x.split(":").last().unwrap().parse::<u32>().unwrap()),
            pid: parts
                .iter()
                .find(|x| x.contains("pid"))
                .map(|x| x.split(":").last().unwrap().parse::<u32>().unwrap()),
            eyr: parts
                .iter()
                .find(|x| x.contains("eyr"))
                .map(|x| x.split(":").last().unwrap().parse::<u32>().unwrap()),
            hcl: parts
                .iter()
                .find(|x| x.contains("hcl"))
                .map(|x| String::from(x.split(":").last().unwrap())),
            byr: parts
                .iter()
                .find(|x| x.contains("byr"))
                .map(|x| x.split(":").last().unwrap().parse::<u32>().unwrap()),
            ecl: parts
                .iter()
                .find(|x| x.contains("ecl"))
                .map(|x| String::from(x.split(":").last().unwrap())),
            hgt: parts.iter().find(|x| x.contains("hgt")).map(|x| {
                x.split(":")
                    .last()
                    .unwrap()
                    .replace("cm", "")
                    .replace("in", "")
                    .parse::<u32>()
                    .unwrap()
            }),
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
    println!("{:#?}", passports);
    println!("HI");
    println!("HI");
    String::from(input)
}

pub fn solve_part_two(input: &str) -> String {
    String::from(input)
}
