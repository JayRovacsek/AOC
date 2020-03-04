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
fn test_offset_vec() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    let input_vec = build_vec_from_str("03036732577212944063491565474664");
    assert_eq!(84_462_026, offset_vec(&input_vec));
}

#[test]
fn test_flatten_int_vec() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    assert_eq!(123_456, flatten_int_vec(vec!(1, 2, 3, 4, 5, 6)));
}
