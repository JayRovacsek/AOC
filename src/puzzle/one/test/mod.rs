use super::*;

#[test]
fn test_part_a() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    assert_eq!(2_u32, calculate_fuel(12_f32) as u32);
    assert_eq!(2_u32, calculate_fuel(14_f32) as u32);
    assert_eq!(654_u32, calculate_fuel(1969_f32) as u32);
    assert_eq!(33583_u32, calculate_fuel(100_756_f32) as u32);
}

#[test]
fn test_part_b() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    assert_eq!(2_u32, calculate_fuel_recursive(14_f32) as u32);
    assert_eq!(966_u32, calculate_fuel_recursive(1969_f32) as u32);
    assert_eq!(50346_u32, calculate_fuel_recursive(100_756_f32) as u32);
}
