pub fn solve() {
    let answer_a = calculate_keyspace_part_a(INPUT[0], INPUT[1]);
    println!("The answer for day 4, part a is: {:?}", answer_a);
    let answer_b = calculate_keyspace_part_b(INPUT[0], INPUT[1]);
    println!("The answer for day 4, part b is: {:?}", answer_b);
}

fn calculate_keyspace_part_a(start: usize, end: usize) -> usize {
    (start..=end)
        .collect::<Vec<usize>>()
        .iter()
        .filter(|x| has_double(*x) && !has_decrease(*x))
        .count()
}

fn calculate_keyspace_part_b(start: usize, end: usize) -> usize {
    (start..=end)
        .collect::<Vec<usize>>()
        .iter()
        .filter(|x| {
            has_exact_double(*x) && !has_decrease(*x)
        })
        .count()
}

fn has_double(input: &usize) -> bool {
    let mut result = false;
    let digits: Vec<_> = input.to_string().chars().collect();
    let mut iter = digits.windows(2);

    while !result {
        match iter.next() {
            Some(v) => result = v[0] == v[1],
            _ => break,
        }
    }
    result
}

fn has_exact_double(input: &usize) -> bool {
    let mut result: Vec<usize> = Vec::new();
    let digits: Vec<_> = input.to_string().chars().collect();
    let mut iter = digits.windows(2);
    let mut current_count: usize = 1;

    loop {
        match iter.next() {
            Some(v) => {
                if v[0] == v[1] {
                    current_count += 1;
                } else {
                    if current_count > 1 {
                        result.push(current_count);
                    }
                    current_count = 1;
                }
            }
            _ => {
                if current_count > 1 {
                    result.push(current_count);
                };
                break
            }
        }
    }

    !result.iter().filter(|x|*x == &2).collect::<Vec<_>>().is_empty()
}

#[allow(dead_code)]
fn digit_sequential_count_max(input: &usize) -> usize {
    let mut result: Vec<usize> = Vec::new();
    let digits: Vec<_> = input.to_string().chars().collect();
    let mut iter = digits.windows(2);
    let mut current_count: usize = 1;

    loop {
        match iter.next() {
            Some(v) => {
                if v[0] == v[1] {
                    current_count += 1;
                    result.push(current_count);
                } else {
                    if current_count > 1 {
                        result.push(current_count);
                    }
                    current_count = 1;
                }
            }
            _ => break,
        }
    }

    match result.iter().max() {
        Some(v) => *v,
        None => 0
    }
}

fn has_decrease(input: &usize) -> bool {
    let mut result = false;
    let digits: Vec<_> = input.to_string().chars().collect();
    let mut iter = digits.windows(2);

    while !result {
        match iter.next() {
            Some(v) => result = v[0] > v[1],
            _ => break,
        }
    }
    result
}

#[test]
fn test_solve() {
    assert_eq!(true, true);
    assert_ne!(true, false);
}

#[test]
fn test_has_double() {
    assert_eq!(true, has_double(&1223));
    assert_eq!(false, has_double(&1234));
}

#[test]
fn test_has_exact_double() {
    assert_eq!(false, has_exact_double(&1112));
    assert_eq!(false, has_exact_double(&1234));
    assert_eq!(true, has_exact_double(&112_233));
    assert_eq!(false, has_exact_double(&123_444));
    assert_eq!(true, has_exact_double(&111_122));
}

#[test]
fn test_digit_sequential_count_max() {
    assert_eq!(3, digit_sequential_count_max(&1112));
    assert_eq!(0, digit_sequential_count_max(&1234));
    assert_eq!(2, digit_sequential_count_max(&112_233));
    assert_eq!(3, digit_sequential_count_max(&123_444));
    assert_eq!(4, digit_sequential_count_max(&111_122));
    assert_eq!(4, digit_sequential_count_max(&112_222));
}

#[test]
fn test_has_decrease() {
    assert_eq!(false, has_decrease(&1223));
    assert_eq!(false, has_decrease(&1234));
    assert_eq!(true, has_decrease(&1232));
}

const INPUT: [usize; 2] = [193_651, 649_729];
