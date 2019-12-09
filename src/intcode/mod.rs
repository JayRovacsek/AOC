#[derive(Clone)]
pub struct Interpreter {
    phase_setting_consumed: bool,
    phase_setting: Option<i32>,
    input_vec: Vec<i32>,
    head: usize,
}

impl Interpreter {
    pub fn new(phase_setting: Option<i32>, input_vec: Vec<i32>, head: usize) -> Interpreter {
        match phase_setting {
            Some(i) => Interpreter {
                phase_setting_consumed: false,
                phase_setting: Some(i),
                input_vec,
                head,
            },
            _ => Interpreter {
                phase_setting_consumed: true,
                phase_setting: None,
                input_vec,
                head,
            },
        }
    }

    pub fn run(&mut self, input_code: i32) -> i32 {
        use OpCode::*;
        let mut outputs: Vec<i32> = Vec::new();
        loop {
            let x = Operation::parse(self.input_vec.clone(), self.head);
            if x.opcode == End {
                break;
            };
            let result = if self.phase_setting_consumed {
                x.execute(self.input_vec.clone(), self.head, input_code)
            } else {
                self.phase_setting_consumed = true;
                x.execute(
                    self.input_vec.clone(),
                    self.head,
                    self.phase_setting.unwrap_or(0),
                )
            };
            self.input_vec = result.0;
            self.head = result.1;
            if result.2.is_some() {
                outputs.push(result.2.unwrap());
            };
        }
        *outputs.last().unwrap_or(&0_i32)
    }

    pub fn run_all_outputs(&mut self, input_code: i32) -> Vec<i32> {
        use OpCode::*;
        let mut outputs: Vec<i32> = Vec::new();
        loop {
            let x = Operation::parse(self.input_vec.clone(), self.head);
            if x.opcode == End {
                break;
            };
            let result = if self.phase_setting_consumed {
                x.execute(self.input_vec.clone(), self.head, input_code)
            } else {
                self.phase_setting_consumed = true;
                x.execute(
                    self.input_vec.clone(),
                    self.head,
                    self.phase_setting.unwrap_or(0),
                )
            };
            self.input_vec = result.0;
            self.head = result.1;
            if result.2.is_some() {
                outputs.push(result.2.unwrap());
            };
        }
        outputs
    }

    pub fn run_with_modified_registers(&mut self, input_code: i32, n: i32, v: i32) -> Vec<i32> {
        let mut outputs: Vec<i32> = Vec::new();
        self.input_vec[1] = n;
        self.input_vec[2] = v;
        let windows: Vec<usize> = (0..self.input_vec.len() / 4).collect();
        windows.iter().for_each(|x| {
            let z = Operation::parse(self.input_vec.clone(), x * 4);
            let result = if self.phase_setting_consumed {
                z.execute(self.input_vec.clone(), self.head, input_code)
            } else {
                self.phase_setting_consumed = true;
                z.execute(
                    self.input_vec.clone(),
                    self.head,
                    self.phase_setting.unwrap_or(0),
                )
            };
            self.input_vec = result.0;
            self.head = result.1;
            if result.2.is_some() {
                outputs.push(result.2.unwrap());
            }
        });
        outputs
    }

    pub fn run_one_output(&mut self, input_code: Option<i32>) -> Option<i32> {
        use OpCode::*;
        let mut output: Option<i32> = None;
        loop {
            let x = Operation::parse(self.input_vec.clone(), self.head);
            if x.opcode == End {
                break;
            };
            let result = if self.phase_setting_consumed {
                x.execute(self.input_vec.clone(), self.head, input_code.unwrap())
            } else {
                self.phase_setting_consumed = true;
                x.execute(
                    self.input_vec.clone(),
                    self.head,
                    self.phase_setting.unwrap_or(0),
                )
            };
            self.input_vec = result.0;
            self.head = result.1;
            if result.2.is_some() {
                output = Some(result.2.unwrap());
                break;
            }
        }
        output
    }
}

#[derive(Debug)]
struct Operation {
    opcode: OpCode,
    parameters: Vec<ParameterMode>,
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
            5 => JumpIfTrue,
            6 => JumpIfFalse,
            7 => LessThan,
            8 => Equals,
            _ => End,
        };
        Operation {
            opcode: opcode.clone(),
            parameters: match opcode {
                End => vec![],
                _ => Operation::parse_parameters(input_vec[head], opcode),
            },
        }
    }

    fn parse_parameters(input: i32, opcode: OpCode) -> Vec<ParameterMode> {
        use OpCode::*;
        use ParameterMode::*;
        match opcode {
            Input | Output => match ((input / 10) % 10) % 2 {
                0 => vec![PositionMode],
                _ => vec![ImmediateMode],
            },
            JumpIfTrue | JumpIfFalse => {
                let mut instruction: Vec<_> = input
                    .to_string()
                    .chars()
                    .map(|d| d.to_digit(10).unwrap())
                    .collect();
                instruction.pop();
                instruction.pop();
                while let x = instruction.len() {
                    match x {
                        n if n < 2 => instruction.insert(0, 0),
                        _ => break,
                    }
                }
                instruction
                .iter()
                .rev()
                .map(|x| match x {
                    0 => PositionMode,
                    1 => ImmediateMode,
                    _ => panic!("Attempted to parse an instruction that we otherwise don't yet know about: {}", x),
                })
                .collect()
            }
            _ => {
                let mut instruction: Vec<_> = input
                    .to_string()
                    .chars()
                    .map(|d| d.to_digit(10).unwrap())
                    .collect();
                instruction.pop();
                instruction.pop();
                while let x = instruction.len() {
                    match x {
                        n if n < 3 => instruction.insert(0, 0),
                        _ => break,
                    }
                }
                instruction
                    .iter()
                    .rev()
                    .map(|x| match x {
                        0 => PositionMode,
                        1 => ImmediateMode,
                        _ => panic!("Attempted to parse an instruction that we otherwise don't yet know about: {}", x),
                    })
                    .collect()
            }
        }
    }

    fn execute(
        &self,
        mut input_vec: Vec<i32>,
        mut head: usize,
        input_code: i32,
    ) -> (Vec<i32>, usize, Option<i32>) {
        use OpCode::*;
        use ParameterMode::*;
        let mut output: Option<i32> = None;
        let params: Vec<i32> = self
            .parameters
            .iter()
            .enumerate()
            .map(|x| match x.1 {
                ImmediateMode => (head + 1 + x.0) as i32,
                PositionMode => input_vec[head + 1 + x.0],
            })
            .collect();

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
                output = Some(input_vec[params[0] as usize]);
                head += 2;
            }
            End => output = None,
        }
        (input_vec, head, output)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum OpCode {
    Addition,
    Multiplication,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    End,
}

#[derive(Debug)]
enum ParameterMode {
    PositionMode,
    ImmediateMode,
}
