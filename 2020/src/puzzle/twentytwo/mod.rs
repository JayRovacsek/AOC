mod test;

pub fn solve(input: String) {
    let answer_part_one = solve_part_one(input.clone());
    let answer_part_two = solve_part_two(input);
    println!(
        "Part 1 answer:{:#?}\nPart 2 answer:{:#?}",
        answer_part_one, answer_part_two
    )
}

pub fn solve_part_one(input: String) -> String {
    String::from(input)
}

pub fn solve_part_two(input: String) -> String {
    String::from(input)
}
