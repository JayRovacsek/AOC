use super::*;

#[test]
fn test_solve() {
    assert_eq!(true, true);
    assert_ne!(true, false);
}

#[test]
fn test_has_double() {
    assert_eq!(true, has_double(1223));
    assert_eq!(false, has_double(1234));
}

#[test]
fn test_has_exact_double() {
    assert_eq!(false, has_exact_double(1112));
    assert_eq!(false, has_exact_double(1234));
    assert_eq!(true, has_exact_double(112_233));
    assert_eq!(false, has_exact_double(123_444));
    assert_eq!(true, has_exact_double(111_122));
    assert_eq!(true, has_exact_double(111_233));
}

#[test]
fn test_has_decrease() {
    assert_eq!(false, has_decrease(1223));
    assert_eq!(false, has_decrease(1234));
    assert_eq!(true, has_decrease(1232));
}

#[test]
fn test_calculate_keyspace_part_a() {
    let keys = (INPUT[0]..=INPUT[1]).collect::<Vec<usize>>();
    assert_eq!(1605, calculate_keyspace_part_a(&keys));
    assert_ne!(1606, calculate_keyspace_part_a(&keys));
}
