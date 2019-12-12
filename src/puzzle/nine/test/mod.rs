use crate::intcode::Interpreter;

#[test]
fn test_solve() {
    assert_eq!(true, true);
    assert_ne!(true, false);

    let mut interpreter =
        Interpreter::new(None, vec![1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0], 0);
    let a = *interpreter.run(0).last().unwrap_or(&0_i64);
}

#[test]
fn test_day_9_examples() {
    assert_eq!(true, true);
    assert_ne!(true, false);

    let mut interpreter_a =
        Interpreter::new(None, vec![1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0], 0);

    assert_eq!(
        1_219_070_632_396_864_i64,
        *interpreter_a.run(0).last().unwrap_or(&0_i64)
    );

    let mut interpreter_b = Interpreter::new(None, vec![104, 1_125_899_906_842_624, 99], 0);

    assert_eq!(
        1_125_899_906_842_624_i64,
        *interpreter_b.run(0).last().unwrap_or(&0_i64)
    );
}
