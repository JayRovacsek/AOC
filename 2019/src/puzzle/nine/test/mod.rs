use crate::intcode::interpreter::Interpreter;

#[test]
fn test_solve() {
    assert_eq!(true, true);
    assert_ne!(true, false);

    let mut interpreter =
        Interpreter::new(None, vec![1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0], 0);
    let a = *interpreter.run(0).last().unwrap_or(&0_i64);
}

#[test]
fn test_day_9_ouroboros() {
    assert_eq!(true, true);
    assert_ne!(true, false);

    let mut interpreter = Interpreter::new(
        None,
        vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ],
        0,
    );

    assert_eq!(
        vec!(109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99),
        interpreter.run(0)
    );
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
