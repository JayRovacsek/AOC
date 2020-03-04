use super::*;
use crate::intcode::Interpreter;

#[test]
fn test_part_a() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    let mut interpreter_a =
        Interpreter::new(None, vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], 0);
    assert_eq!(70, *interpreter_a.clone().run(0).get(3).unwrap_or(&0));
    assert_eq!(3500, *interpreter_a.run(0).first().unwrap_or(&0));

    let mut interpreter_b = Interpreter::new(None, vec![1, 0, 0, 0, 99], 0);
    let mut interpreter_c = Interpreter::new(None, vec![2, 3, 0, 3, 99], 0);
    let mut interpreter_d = Interpreter::new(None, vec![2, 4, 4, 5, 99, 0], 0);
    let mut interpreter_e = Interpreter::new(None, vec![1, 1, 1, 4, 99, 5, 6, 0, 99], 0);

    assert_eq!(&2_i64, interpreter_b.run(0).first().unwrap_or(&0_i64));
    assert_eq!(&6_i64, interpreter_c.run(0).get(3).unwrap_or(&0_i64));
    assert_eq!(&9_801_i64, interpreter_d.run(0).get(5).unwrap_or(&0_i64));
    assert_eq!(&30_i64, interpreter_e.run(0).first().unwrap_or(&0_i64));
}

#[test]
fn test_part_b() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    let mut interpreter = Interpreter::new(None, INPUT_VEC.to_vec(), 0);
    assert_eq!(&2_842_648_i64, interpreter.run(0).first().unwrap_or(&0_i64));
    assert_eq!(
        &19_690_720_i64,
        interpreter
            .run_with_modified_registers(0, 90, 74)
            .first()
            .unwrap_or(&0_i64)
    );
}
