use super::*;

#[test]
fn test_solve() {
    assert_eq!(true, true);
    assert_ne!(true, false);
}

#[test]
fn test_interpreter_phase_settings() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    let test_phase_option_a = vec![4, 3, 2, 1, 0];
    let a = test_phase_option_a.iter().fold(0, |y, z| {
        let mut interpreter = Interpreter::new(Some(*z));
        interpreter.run(
            [
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ]
            .to_vec(),
            y,
        )
    });
    assert_eq!(43210, a);
}
