#[derive(Debug)]
struct Operation {
    opcode: OpCode,
    registers: Vec<usize>,
}

impl Operation {
    fn parse(input_vec: Vec<i32>, head: usize) -> Operation {
        use OpCode::*;
        let input = input_vec[head];
        let opcode = match input % 10 {
            1 => Addition,
            2 => Multiplication,
            3 => Input,
            4 => Output,
            _ => End,
        };
        let number_of_params = match &opcode {
            Input | Output => 2,
            _ => 4
        };
        let parameters = Operation::parse_parameters(opcode.clone(), input_vec[head..head+number_of_params].to_vec());
        Operation {
            opcode: opcode.clone(),
            registers: Operation::parse_registers(opcode, input_vec),
        }
    }
    fn parse_registers(opcode: OpCode, input_vec: Vec<i32>) -> Vec<usize> {
        vec!(0 as usize)
    }
    fn parse_parameters(opcode: OpCode, input_vec: Vec<i32>) -> Vec<ParameterMode> {
        use ParameterMode::*;
        use OpCode::*;
        println!("Parsing {:?} as parameters", input_vec);
        match opcode {
            Input | Output => match ((input_vec[0] / 10) % 10) % 2 {
                0 => vec!(PositionMode),
                _ => vec!(ImmediateMode),
            },
            _ => {
                let mut iter = input_vec.iter().enumerate();
                let mut parameters: Vec<ParameterMode> = Vec::new();
                loop {
                    match iter.next() {
                        Some((x,y)) => {
                            parameters.push(
                                match ((input_vec[x] / 10) % 10 * (y + 1)) % 2 {
                                    0 => PositionMode,
                                    _ => ImmediateMode,
                                }
                            )
                        },
                        _ => break
                    }
                }
                parameters
            }
        }
    }
    fn execute(&self, mut input_vec: Vec<i32>) -> Vec<i32> {
        use OpCode::*;
        match self.opcode {
            Addition => {
                input_vec[self.registers[2]] =
                    input_vec[self.registers[0]] + input_vec[self.registers[1]]
            }
            Multiplication => {
                input_vec[self.registers[2]] =
                    input_vec[self.registers[0]] * input_vec[self.registers[1]]
            }
            Input => {
                println!("Set value {} as {}, previously was: {}", self.registers[0], 1, input_vec[self.registers[0]]);
                input_vec[self.registers[0]] = 1
            }
            Output => println!("Output OpCode, Value: {}", input_vec[self.registers[0]]),
            End => panic!("Operation ending"),
        }
        input_vec
    }
}

#[derive(Debug, Clone)]
enum OpCode {
    Addition,
    Multiplication,
    Input,
    Output,
    End,
}

#[derive(Debug)]
enum ParameterMode {
    PositionMode,
    ImmediateMode,
}

pub fn solve() {
    let registers = INPUT_VEC.clone().to_vec();
    let answer_a = *execute_instructions(registers.clone()).first().unwrap();
    println!("The answer for day 5, part a is: {:?}", answer_a);
}

fn execute_instructions(mut input_vec: Vec<i32>) -> Vec<i32> {
    use OpCode::*;
    let mut head = 0;
    loop {
        let op = Operation::parse(input_vec.clone(), head);
        input_vec = op.execute(input_vec);
        head += match op.opcode {
            Input | Output => 2,
            _ => 4
        };
    }
    input_vec
}

#[test]
fn test_part_a() {
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
}

const INPUT_VEC: [i32; 678] = [
    3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1102, 59, 58, 224, 1001, 224, -3422, 224, 4,
    224, 102, 8, 223, 223, 101, 3, 224, 224, 1, 224, 223, 223, 1101, 59, 30, 225, 1101, 53, 84,
    224, 101, -137, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 3, 224, 224, 1, 223, 224, 223, 1102,
    42, 83, 225, 2, 140, 88, 224, 1001, 224, -4891, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 5,
    224, 1, 223, 224, 223, 1101, 61, 67, 225, 101, 46, 62, 224, 1001, 224, -129, 224, 4, 224, 1002,
    223, 8, 223, 101, 5, 224, 224, 1, 223, 224, 223, 1102, 53, 40, 225, 1001, 35, 35, 224, 1001,
    224, -94, 224, 4, 224, 102, 8, 223, 223, 101, 6, 224, 224, 1, 223, 224, 223, 1101, 5, 73, 225,
    1002, 191, 52, 224, 1001, 224, -1872, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 5, 224, 1,
    223, 224, 223, 102, 82, 195, 224, 101, -738, 224, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 2,
    224, 1, 224, 223, 223, 1101, 83, 52, 225, 1101, 36, 77, 225, 1101, 9, 10, 225, 1, 113, 187,
    224, 1001, 224, -136, 224, 4, 224, 1002, 223, 8, 223, 101, 2, 224, 224, 1, 224, 223, 223, 4,
    223, 99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247, 1105,
    1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106, 0, 265, 1105,
    1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225,
    225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225,
    225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999, 1007, 226, 226, 224, 1002, 223, 2, 223,
    1006, 224, 329, 1001, 223, 1, 223, 1108, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 344, 101,
    1, 223, 223, 1007, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 359, 101, 1, 223, 223, 1108,
    677, 226, 224, 1002, 223, 2, 223, 1005, 224, 374, 1001, 223, 1, 223, 7, 677, 226, 224, 102, 2,
    223, 223, 1005, 224, 389, 1001, 223, 1, 223, 1008, 677, 677, 224, 1002, 223, 2, 223, 1005, 224,
    404, 101, 1, 223, 223, 108, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 419, 101, 1, 223, 223,
    1008, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 434, 1001, 223, 1, 223, 1107, 677, 226, 224,
    1002, 223, 2, 223, 1005, 224, 449, 101, 1, 223, 223, 1008, 226, 226, 224, 102, 2, 223, 223,
    1005, 224, 464, 1001, 223, 1, 223, 8, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 479, 1001,
    223, 1, 223, 107, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 494, 1001, 223, 1, 223, 7, 226,
    226, 224, 102, 2, 223, 223, 1005, 224, 509, 1001, 223, 1, 223, 107, 226, 226, 224, 102, 2, 223,
    223, 1005, 224, 524, 101, 1, 223, 223, 107, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 539,
    101, 1, 223, 223, 8, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 554, 101, 1, 223, 223, 1107,
    677, 677, 224, 1002, 223, 2, 223, 1005, 224, 569, 101, 1, 223, 223, 108, 226, 677, 224, 1002,
    223, 2, 223, 1006, 224, 584, 101, 1, 223, 223, 7, 226, 677, 224, 1002, 223, 2, 223, 1005, 224,
    599, 1001, 223, 1, 223, 8, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 614, 1001, 223, 1, 223,
    108, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 629, 1001, 223, 1, 223, 1007, 226, 677, 224,
    1002, 223, 2, 223, 1006, 224, 644, 101, 1, 223, 223, 1108, 226, 677, 224, 102, 2, 223, 223,
    1005, 224, 659, 1001, 223, 1, 223, 1107, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 674, 1001,
    223, 1, 223, 4, 223, 99, 226,
];
