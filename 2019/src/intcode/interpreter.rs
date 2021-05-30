use crate::intcode::operation::Operation;

#[derive(Clone)]
pub struct Interpreter {
    phase_setting_consumed: bool,
    phase_setting: Option<i64>,
    input_vec: Vec<i64>,
    head: usize,
    relative_base: i64,
}

impl Interpreter {
    pub fn new(phase_setting: Option<i64>, input_vec: Vec<i64>, head: usize) -> Interpreter {
        match phase_setting {
            Some(i) => Interpreter {
                phase_setting_consumed: false,
                phase_setting: Some(i),
                input_vec,
                head,
                relative_base: 0,
            },
            _ => Interpreter {
                phase_setting_consumed: true,
                phase_setting: None,
                input_vec,
                head,
                relative_base: 0,
            },
        }
    }

    pub fn run(&mut self, input_code: i64) -> Vec<i64> {
        self.into_iter().fold(self.input_vec.clone(), |a,operation|{
            if self.phase_setting_consumed {
                operation.execute(&self.input_vec, self.head, input_code, self.relative_base)
            } else {
                self.phase_setting_consumed = true;
                operation.execute(
                    &self.input_vec,
                    self.head,
                    self.phase_setting.unwrap_or(0),
                    self.relative_base,
                )
            };
            self.input_vec = result.0;
            self.head = result.1;
            if result.2.is_some() {
                outputs.push(result.2.unwrap());
            };
            self.relative_base = result.3;
        })

        // let mut outputs: Vec<i64> = Vec::new();
        // loop {
        //     let x = Operation::parse(&self.input_vec, self.head);
        //     if x.opcode == End {
        //         break;
        //     };
        //      let result = if self.phase_setting_consumed {
        //         x.execute(&self.input_vec, self.head, input_code, self.relative_base)
        //     } else {
        //         self.phase_setting_consumed = true;
        //         x.execute(
        //             &self.input_vec,
        //             self.head,
        //             self.phase_setting.unwrap_or(0),
        //             self.relative_base,
        //         )
        //     };
        //     self.input_vec = result.0;
        //     self.head = result.1;
        //     if result.2.is_some() {
        //         outputs.push(result.2.unwrap());
        //     };
        //     self.relative_base = result.3;
        // }
        // outputs
    }

    pub fn run_with_modified_registers(&mut self, input_code: i64, n: i64, v: i64) -> Vec<i64> {
        let mut outputs: Vec<i64> = Vec::new();
        self.input_vec[1] = n;
        self.input_vec[2] = v;
        let windows: Vec<usize> = (0..self.input_vec.len() / 4).collect();
        windows.iter().for_each(|x| {
            let z = Operation::parse(&self.input_vec, x * 4);
            let result = if self.phase_setting_consumed {
                z.execute(&self.input_vec, self.head, input_code, self.relative_base)
            } else {
                self.phase_setting_consumed = true;
                z.execute(
                    &self.input_vec,
                    self.head,
                    self.phase_setting.unwrap_or(0),
                    self.relative_base,
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
}

impl Iterator for Interpreter {
    type Item = Operation;

    fn next(&mut self) -> Option<Operation> {
        let mode = Operation::parse(&self.input_vec, self.head);
        match mode.opcode {
            End => None,
            _ => Some(mode),
        }
    }
}
