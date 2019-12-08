use super::*;


#[test]
fn test_solve() {
    assert_eq!(true, true);
    assert_ne!(true, false);
}

#[test]
fn test_has_double() {
    assert_eq!(true, has_double(&1223));
    assert_eq!(false, has_double(&1234));
}

#[test]
fn test_has_exact_double() {
    assert_eq!(false, has_exact_double(&1112));
    assert_eq!(false, has_exact_double(&1234));
    assert_eq!(true, has_exact_double(&112_233));
    assert_eq!(false, has_exact_double(&123_444));
    assert_eq!(true, has_exact_double(&111_122));
    assert_eq!(true, has_exact_double(&111_233));
}

#[test]
fn test_digit_sequential_count_max() {
    assert_eq!(3, digit_sequential_count_max(&1112));
    assert_eq!(0, digit_sequential_count_max(&1234));
    assert_eq!(2, digit_sequential_count_max(&112_233));
    assert_eq!(3, digit_sequential_count_max(&123_444));
    assert_eq!(4, digit_sequential_count_max(&111_122));
    assert_eq!(4, digit_sequential_count_max(&112_222));
}

#[test]
fn test_has_decrease() {
    assert_eq!(false, has_decrease(&1223));
    assert_eq!(false, has_decrease(&1234));
    assert_eq!(true, has_decrease(&1232));
}