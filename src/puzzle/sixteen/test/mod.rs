use super::*;

#[test]
fn test_solve() {
    assert_eq!(true, true);
    assert_ne!(true, false);
}

#[test]
fn test_execute_phase() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    let input_vec = build_vec_from_str("12345678");
    assert_eq!(build_vec_from_str("48226158"), execute_phase(&input_vec));
}

#[test]
fn test_execute_phase_x_times() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    let input_vec = build_vec_from_str("12345678");
    assert_eq!(
        build_vec_from_str("34040438"),
        execute_phase_x_times(&input_vec, 2)
    );
    assert_eq!(
        build_vec_from_str("03415518"),
        execute_phase_x_times(&input_vec, 3)
    );
    assert_eq!(
        build_vec_from_str("01029498"),
        execute_phase_x_times(&input_vec, 4)
    );
}

#[test]
fn test_execute_phase_x_times_with_offset() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    let input_vec = build_vec_from_str("03036732577212944063491565474664");
    assert_eq!(
        84462026,
        flatten_int_vec(execute_phase_x_times_with_offset(&input_vec, 100))
    );
}
