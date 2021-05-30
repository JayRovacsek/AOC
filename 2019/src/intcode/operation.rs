use crate::intcode::opcode::OpCode;
use crate::intcode::parametermode::ParameterMode;

#[derive(Debug)]
pub struct Operation {
    pub opcode: OpCode,
    pub parameters: Vec<ParameterMode>,
}

impl Operation {
    pub fn parse(input_vec: &[i64], head: usize) -> Operation {
        use OpCode::*;
        let input = input_vec[head];
        let o = match input % 100 {
            n if n < 9 => match n {
                1 => Addition,
                2 => Multiplication,
                3 => Input,
                4 => Output,
                5 => JumpIfTrue,
                6 => JumpIfFalse,
                7 => LessThan,
                8 => Equals,
                _ => End,
            },
            n if n >= 9 => match n {
                9 => RelativeAdjust,
                _ => End,
            },
            _ => End,
        };
        Operation {
            parameters: match &o {
                End => vec![],
                _ => Operation::parse_parameters(input_vec[head], &o),
            },
            opcode: o,
        }
    }

    fn parse_parameters(input: i64, opcode: &OpCode) -> Vec<ParameterMode> {
        use OpCode::*;
        use ParameterMode::*;
        match opcode {
            Input | Output | RelativeAdjust => match input / 100 {
                0 => vec![Position],
                1 => vec![Immediate],
                2 => vec![Relative],
                _ => panic!(
                    "Attempted to parse an instruction that we otherwise don't yet know about: {}",
                    input
                ),
            },
            JumpIfTrue | JumpIfFalse => {
                let mut instruction: Vec<_> = (input / 100)
                    .to_string()
                    .chars()
                    .map(|d| d.to_digit(10).unwrap())
                    .collect();
                while instruction.len() < 2 {
                    match instruction.len() {
                        n if n < 2 => instruction.insert(0, 0),
                        _ => break,
                    }
                }
                instruction
                .iter()
                .rev()
                .map(|x| match x {
                    0 => Position,
                    1 => Immediate,
                    2 => Relative,
                    _ => panic!("Attempted to parse an instruction that we otherwise don't yet know about: {}", x),
                })
                .collect()
            }
            _ => {
                let mut instruction: Vec<_> = (input / 100)
                    .to_string()
                    .chars()
                    .map(|d| d.to_digit(10).unwrap())
                    .collect();
                while instruction.len() < 3 {
                    match instruction.len() {
                        n if n < 3 => instruction.insert(0, 0),
                        _ => break,
                    }
                }
                instruction
                    .iter()
                    .rev()
                    .map(|x| match x {
                        0 => Position,
                        1 => Immediate,
                        2 => Relative,
                        _ => panic!("Attempted to parse an instruction that we otherwise don't yet know about: {}", x),
                    })
                    .collect()
            }
        }
    }

    pub fn execute(
        &self,
        input_vec: &Vec<i64>,
        head: usize,
        input_code: i64,
        relative_base: i64,
    ) -> (Vec<i64>, usize, Option<i64>, i64) {
        use OpCode::*;
        use ParameterMode::*;
        let mut output: Option<i64> = None;
        let mut input_vec = input_vec.clone();
        let params: Vec<i64> = self
            .parameters
            .iter()
            .enumerate()
            .map(|x| match x.1 {
                Immediate => (head + 1 + x.0) as i64,
                Position => {
                    if head + 1 + x.0 > input_vec.len() {
                        input_vec.resize((head + 1 + x.0) as usize, 0);
                    }
                    input_vec[head + 1 + x.0]
                }
                Relative => {
                    if (input_vec[(relative_base + input_vec[head + 1]) as usize]) as usize > input_vec.len() {
                        input_vec.resize(
                            (input_vec[(relative_base + input_vec[head + 1]) as usize] + 1) as usize,
                            0,
                        );
                    }
                    input_vec[(relative_base + input_vec[head + 1]) as usize]
                }
            })
            .collect();

        // let expand_memory = params
        //     .iter()
        //     .map(|x| *x as usize > input_vec.len())
        //     .filter(|x| *x)
        //     .any(|x| x);

        // input_vec = if expand_memory {
        //     input_vec.resize((*params.iter().max().unwrap_or(&0_i64) + 1) as usize, 0);
        //     input_vec
        // } else {
        //     input_vec
        // };

        match self.opcode {
            Addition => {
                input_vec[params[2] as usize] =
                    input_vec[params[0] as usize] + input_vec[params[1] as usize];
                head += 4;
            }
            Multiplication => {
                input_vec[params[2] as usize] =
                    input_vec[params[0] as usize] * input_vec[params[1] as usize];
                head += 4;
            }
            Input => {
                input_vec[params[0] as usize] = input_code;
                head += 2;
            }
            JumpIfTrue => {
                if input_vec[params[0] as usize] != 0 {
                    head = input_vec[params[1] as usize] as usize;
                } else {
                    head += 3;
                }
            }
            JumpIfFalse => {
                if input_vec[params[0] as usize] == 0 {
                    head = input_vec[params[1] as usize] as usize;
                } else {
                    head += 3;
                }
            }
            LessThan => {
                input_vec[params[2] as usize] =
                    if input_vec[params[0] as usize] < input_vec[params[1] as usize] {
                        1
                    } else {
                        0
                    };
                head += 4
            }
            Equals => {
                input_vec[params[2] as usize] =
                    if input_vec[params[0] as usize] == input_vec[params[1] as usize] {
                        1
                    } else {
                        0
                    };
                head += 4
            }
            Output => {
                output = Some(params[0]);
                head += 2;
            }
            RelativeAdjust => {
                relative_base += params[0];
                head += 2
            }
            End => output = None,
        }
        (input_vec, head, output, relative_base)
    }
}
