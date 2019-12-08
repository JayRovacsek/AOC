use super::*;


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

#[test]
fn test_part_b() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    assert_eq!(
        2_842_648,
        *execute_instructions(INPUT_VEC.to_vec()).first().unwrap()
    );
    assert_eq!(
        19_690_720,
        *execute_instructions_modify_registers(INPUT_VEC.to_vec(), 90, 74)
            .first()
            .unwrap()
    );
}