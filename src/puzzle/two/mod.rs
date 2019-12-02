#[derive(Debug)]
enum Operation {
    Addition,
    Multiplication,
    End
}

pub fn solve() {
    // let instructions: Vec<Instruction> = parse_instructions(INPUT_VEC.to_vec());
    // println!("{:?}", instructions);
    let numbers = INPUT_VEC.clone();
    let results = execute_instructions_new(numbers.to_vec());
    let results2 = execute_instructions(numbers);
    println!("Results: {:?}", results);
    println!("Results: {:?}", results2);
    // for x in 0..100 {
    //     for y in 0..100 {
    //         solve_p2(derp_vec, x, y);
    //     }
    // }

    // let mut collection = instructions.iter_mut();
    // let mut done = false;
    // let mut i = 0;
    // while !done {
    //     let instruction = collection.next().unwrap();
    //     done = instruction.done();
    //     let operation = instruction.operations(i);
    //     i += 1;
    // }
}

fn parse_operation(input: Vec<i32>) -> (Operation, usize, usize, usize) {
    match input[0] {
        1 => {
            (Operation::Addition, input[1] as usize,input[2] as usize,input[3] as usize)
        },
        2 => {
            (Operation::Multiplication, input[1] as usize,input[2] as usize,input[3] as usize)
        },
        _ => {
            (Operation::End, 0,0,0)
        }
    }
}

fn execute_instructions_new(mut input_vec: Vec<i32>) -> Vec<i32> {
    let windows: Vec<usize> = (0..input_vec.len() / 4).collect();
    windows.iter().for_each(|x|{
        let y = x * 4;
        let z = y + 4;
        let op = parse_operation(input_vec[y..z].to_vec());
        match op.0 {
            Operation::Addition => {
                input_vec[op.3] = input_vec[op.1] + input_vec[op.2]
            },
            Operation::Multiplication => {
                input_vec[op.3] = input_vec[op.1] * input_vec[op.2]
            },
            _ => {
                println!("Looks like we have a winner: {:?}", input_vec[y]);
            }
        }
    });
    input_vec
}

fn execute_instructions(mut derp_vec: [i32; 117]) -> i32 {
    let mut i = 0;
    let mut done = false;
    let mut result = 0;
    while !done {
        let z = i * 4;
        let part: Vec<i32> = derp_vec[z..z + 4].to_vec();
        println!("{:?}", part);
        if part[0] == 1 {
            let x = derp_vec[part[1] as usize] + derp_vec[part[2] as usize];
            derp_vec[part[3] as usize] = x;
        } else if part[0] == 2 {
            let x = derp_vec[part[1] as usize] * derp_vec[part[2] as usize];
            derp_vec[part[3] as usize] = x;
        } else {
            result = *derp_vec.iter().collect::<Vec<_>>()[0];
            done = true;
        }
        i += 1;
    }
    result
}

fn solve_p2(mut derp_vec: [i32; 117], n: i32, v: i32) {
    derp_vec[1] = n;
    derp_vec[2] = v;
    let mut i = 0;
    let mut done = false;
    while !done {
        let z = i * 4;
        let part: Vec<i32> = derp_vec[z..z + 4].to_vec();
        if part[0] == 1 {
            let x = derp_vec[part[1] as usize] + derp_vec[part[2] as usize];
            derp_vec[part[3] as usize] = x;
        } else if part[0] == 2 {
            let x = derp_vec[part[1] as usize] * derp_vec[part[2] as usize];
            derp_vec[part[3] as usize] = x;
        } else {
            let a = derp_vec.iter().collect::<Vec<_>>()[0];
            println!("{:?}", a);
            if a == &19690720 {
                println!("Answer: {:?}", (n * 100) + v);
                panic!("Found it")
            }
            done = true;
        }
        i += 1;
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    opcode: i32,
    variables: Vec<i32>,
}

// fn regenerate(input: Vec<Instruction>, current_index: usize) -> Vec<Instruction> {
//     let i = &input[current_index];
//     let point = i.point(&input);
//     let mut result = input.clone();
//     result[point.0].update((point.1, point.2));
//     result
// }

// fn update(mut input: Vec<Instruction>, operation: (bool, usize, usize, usize)) {
//     let x = input[operation.1];
//     let y = input[operation.2];
//     let z = if operation.0 {
//         input[x / 4].
//     } else {

//     }

//     match operation.0 {
//         true => {
//             input[operation.3].variables[]
//         }
//         false => {
//             let mut v = self.variables.to_owned();
//             v[point.0] = point.1 as i32;
//             self.variables = v.to_owned();
//         }
//     }
// }

impl Instruction {
    // fn update(&mut self, operation: (usize, usize)) {
    //     match operation.0 {
    //         true => {
    //             let mut v = self.variables.to_owned();
    //             v[operation.0] = operation.1 as i32;
    //             self.variables = v.to_owned();
    //         }
    //         false => {
    //             let mut v = self.variables.to_owned();
    //             v[point.0] = point.1 as i32;
    //             self.variables = v.to_owned();
    //         }
    //     }
    // }

    fn done(&self) -> bool {
        self.opcode == 99
    }

    fn operations(&self, index: usize) -> (bool, usize, usize, usize) {
        let variables = &self.variables;
        let operation = self.opcode == 1;
        let x = match variables.first() {
            Some(v) => v,
            _ => panic!("No value!"),
        };
        let index_a: usize = variables[1] as usize;
        let index_b: usize = variables[2] as usize;
        (operation, index_a, index_b, index)
    }
}

fn parse_instructions(mut input: Vec<i32>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    while !input.is_empty() {
        match input.first() {
            Some(v) => match v {
                99 => instructions.push(Instruction {
                    opcode: 99,
                    variables: Vec::new(),
                }),
                _ => {
                    let l = if input.len() < 4 { input.len() } else { 4 };
                    instructions.push(Instruction {
                        opcode: input[0],
                        variables: input[1..l].to_vec(),
                    })
                }
            },
            _ => continue,
        }
        input.drain(0..3);
    }

    instructions
}

#[test]
fn test_solve() {
    assert_eq!(true, true);
    assert_ne!(true, false);
}

const INPUT_VEC: [i32; 117] = [
    1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 1, 19, 6, 23, 2, 13, 23, 27, 1,
    27, 13, 31, 1, 9, 31, 35, 1, 35, 9, 39, 1, 39, 5, 43, 2, 6, 43, 47, 1, 47, 6, 51, 2, 51, 9, 55,
    2, 55, 13, 59, 1, 59, 6, 63, 1, 10, 63, 67, 2, 67, 9, 71, 2, 6, 71, 75, 1, 75, 5, 79, 2, 79,
    10, 83, 1, 5, 83, 87, 2, 9, 87, 91, 1, 5, 91, 95, 2, 13, 95, 99, 1, 99, 10, 103, 1, 103, 2,
    107, 1, 107, 6, 0, 99, 2, 14, 0, 0,
];
