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
    assert_eq!(
        43210,
        vec![4, 3, 2, 1, 0].iter().fold(0, |y, z| {
            let mut interpreter = Interpreter::new(Some(*z));
            interpreter.run(
                [
                    3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
                ]
                .to_vec(),
                y,
            )
        })
    );
    assert_eq!(
        54321,
        vec![0, 1, 2, 3, 4].iter().fold(0, |y, z| {
            let mut interpreter = Interpreter::new(Some(*z));
            interpreter.run(
                [
                    3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0, 3, 23, 3, 24,
                    1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
                    0, 0,
                ]
                .to_vec(),
                y,
            )
        })
    );
    assert_eq!(
        65210,
        vec![1, 0, 4, 3, 2].iter().fold(0, |y, z| {
            let mut interpreter = Interpreter::new(Some(*z));
            interpreter.run(
                [
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
                ]
                .to_vec(),
                y,
            )
        })
    );
}

#[test]
fn test_part_b_phase_settings() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    assert_eq!(
        139629729,
        vec![9, 8, 7, 6, 5].iter().fold(0, |y, z| {
            let mut interpreter = Interpreter::new(Some(*z));
            interpreter.run(
                [
                    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001,
                    28, -1, 28, 1005, 28, 6, 99, 0, 0, 5,
                ]
                .to_vec(),
                y,
            )
        })
    );
}
