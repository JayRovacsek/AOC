#[derive(Debug)]
enum Operation {
    Addition,
    Multiplication,
    End,
}

pub fn solve() {
    let registers = INPUT_VEC.clone().to_vec();
    let answer_a = *execute_instructions(registers.clone()).first().unwrap();
    println!("The answer for day 2, part a is: {:?}", answer_a);
    for x in 0..100 {
        for y in 0..100 {
            if *execute_instructions_modify_registers(registers.clone(), x, y)
                .first()
                .unwrap()
                == 19_690_720
            {
                let answer_b = (x, y);
                println!("The answer for day 2, part b is: {:?}", answer_b);
            }
        }
    }
}

fn parse_operation(input: Vec<i32>) -> (Operation, usize, usize, usize) {
    match input[0] {
        1 => (
            Operation::Addition,
            input[1] as usize,
            input[2] as usize,
            input[3] as usize,
        ),
        2 => (
            Operation::Multiplication,
            input[1] as usize,
            input[2] as usize,
            input[3] as usize,
        ),
        _ => (Operation::End, 0, 0, 0),
    }
}

fn execute_instructions(mut input_vec: Vec<i32>) -> Vec<i32> {
    let windows: Vec<usize> = (0..input_vec.len() / 4).collect();
    windows.iter().for_each(|x| {
        let y = x * 4;
        let z = y + 4;
        let op = parse_operation(input_vec[y..z].to_vec());
        match op.0 {
            Operation::Addition => input_vec[op.3] = input_vec[op.1] + input_vec[op.2],
            Operation::Multiplication => input_vec[op.3] = input_vec[op.1] * input_vec[op.2],
            _ => {}
        };
    });
    input_vec
}

fn execute_instructions_modify_registers(mut input_vec: Vec<i32>, n: i32, v: i32) -> Vec<i32> {
    input_vec[1] = n;
    input_vec[2] = v;
    let windows: Vec<usize> = (0..input_vec.len() / 4).collect();
    windows.iter().for_each(|x| {
        let y = x * 4;
        let z = y + 4;
        let op = parse_operation(input_vec[y..z].to_vec());
        match op.0 {
            Operation::Addition => input_vec[op.3] = input_vec[op.1] + input_vec[op.2],
            Operation::Multiplication => input_vec[op.3] = input_vec[op.1] * input_vec[op.2],
            _ => {}
        };
    });
    input_vec
}

#[test]
fn test_solve() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    assert_eq!(
        70,
        execute_instructions(vec!(1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50))[3]
    );
    assert_eq!(
        3500,
        execute_instructions(vec!(1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50))[0]
    );
    assert_eq!(2, execute_instructions(vec!(1, 0, 0, 0, 99))[0]);
    assert_eq!(6, execute_instructions(vec!(2, 3, 0, 3, 99))[3]);
    assert_eq!(9801, execute_instructions(vec!(2, 4, 4, 5, 99, 0))[5]);
    assert_eq!(
        30,
        execute_instructions(vec!(1, 1, 1, 4, 99, 5, 6, 0, 99))[0]
    );
    assert_eq!(
        2_842_648,
        *execute_instructions(INPUT_VEC.to_vec()).first().unwrap()
    );
    assert_eq!(
        19_690_720,
        *execute_instructions_modify_registers(INPUT_VEC.to_vec(), 90, 74)
            .first()
            .unwrap()
    );
}

const INPUT_VEC: [i32; 117] = [
    1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 1, 19, 6, 23, 2, 13, 23, 27, 1,
    27, 13, 31, 1, 9, 31, 35, 1, 35, 9, 39, 1, 39, 5, 43, 2, 6, 43, 47, 1, 47, 6, 51, 2, 51, 9, 55,
    2, 55, 13, 59, 1, 59, 6, 63, 1, 10, 63, 67, 2, 67, 9, 71, 2, 6, 71, 75, 1, 75, 5, 79, 2, 79,
    10, 83, 1, 5, 83, 87, 2, 9, 87, 91, 1, 5, 91, 95, 2, 13, 95, 99, 1, 99, 10, 103, 1, 103, 2,
    107, 1, 107, 6, 0, 99, 2, 14, 0, 0,
];
