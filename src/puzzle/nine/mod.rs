mod test;

use crate::intcode::Interpreter;

pub fn solve() {
    println!("Currently a stub");
    let mut interpreter =
        Interpreter::new(None, vec![1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0], 0);
    let a = *interpreter.run(0).last().unwrap_or(&0_i64);
    println!("A: {:?}", a);

    let mut interpreter_b = Interpreter::new(None, vec![104, 1_125_899_906_842_624, 99], 0);
    let b = *interpreter_b.run(0).last().unwrap_or(&0_i64);
}
