use super::*;

#[test]
fn test_part_a() {
    assert_eq!(true, true);
    assert_ne!(true, false);

    let test_wire_a: Vec<Instruction> =
        vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"]
            .par_iter()
            .map(|x| Instruction::from_str(x))
            .collect();
    let test_wire_b: Vec<Instruction> =
        vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"]
            .par_iter()
            .map(|x| Instruction::from_str(x))
            .collect();

    let test_wire_c: Vec<Instruction> = vec![
        "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
    ]
    .par_iter()
    .map(|x| Instruction::from_str(x))
    .collect();

    let test_wire_d: Vec<Instruction> = vec![
        "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
    ]
    .par_iter()
    .map(|x| Instruction::from_str(x))
    .collect();

    assert_eq!(
        159,
        find_closest_manhattan_distance_size(&test_wire_a, &test_wire_b)
    );
    assert_eq!(
        135,
        find_closest_manhattan_distance_size(&test_wire_c, &test_wire_d)
    );
}

#[test]
fn test_generate_wire_map() {
    assert_eq!(true, true);
    assert_ne!(true, false);

    let test_wire_a: Vec<Instruction> = vec!["U1"]
        .par_iter()
        .map(|x| Instruction::from_str(x))
        .collect();
    let test_wire_a_map = generate_wire_map(&test_wire_a);

    let test_wire_b: Vec<Instruction> = vec!["D1"]
        .par_iter()
        .map(|x| Instruction::from_str(x))
        .collect();
    let test_wire_b_map = generate_wire_map(&test_wire_b);

    let test_wire_c: Vec<Instruction> = vec!["L1"]
        .par_iter()
        .map(|x| Instruction::from_str(x))
        .collect();
    let test_wire_c_map = generate_wire_map(&test_wire_c);

    let test_wire_d: Vec<Instruction> = vec!["R1"]
        .par_iter()
        .map(|x| Instruction::from_str(x))
        .collect();
    let test_wire_d_map = generate_wire_map(&test_wire_d);

    assert_eq!(
        vec!((0, 0), (0, 1))
            .into_iter()
            .collect::<HashSet<(i64, i64)>>(),
        test_wire_a_map
    );

    assert_eq!(
        vec!((0, 0), (0, -1))
            .into_iter()
            .collect::<HashSet<(i64, i64)>>(),
        test_wire_b_map
    );

    assert_eq!(
        vec!((0, 0), (-1, 0))
            .into_iter()
            .collect::<HashSet<(i64, i64)>>(),
        test_wire_c_map
    );

    assert_eq!(
        vec!((0, 0), (1, 0))
            .into_iter()
            .collect::<HashSet<(i64, i64)>>(),
        test_wire_d_map
    );
}

#[test]
fn test_part_b() {
    assert_eq!(true, true);
    assert_ne!(true, false);
}

#[test]
fn test_manhattan_distance() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    assert_eq!(
        1 as usize,
        manhattan_distance((10000, 10001), (10000, 10000))
    );
    assert_eq!(
        1 as usize,
        manhattan_distance((10001, 10000), (10000, 10000))
    );
    assert_eq!(
        1 as usize,
        manhattan_distance((9999, 10000), (10000, 10000))
    );
    assert_eq!(
        1 as usize,
        manhattan_distance((10000, 9999), (10000, 10000))
    );
    assert_eq!(
        100 as usize,
        manhattan_distance((10000, 10100), (10000, 10000))
    );
    assert_eq!(
        100 as usize,
        manhattan_distance((10100, 10000), (10000, 10000))
    );
    assert_eq!(
        100 as usize,
        manhattan_distance((9900, 10000), (10000, 10000))
    );
    assert_eq!(
        100 as usize,
        manhattan_distance((10000, 9900), (10000, 10000))
    );
}

#[test]
fn test_instruction_init() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    assert_eq!(100, Instruction::from_str("U100").distance);
    assert_eq!(100, Instruction::from_str("D100").distance);
    assert_eq!(100, Instruction::from_str("L100").distance);
    assert_eq!(100, Instruction::from_str("R100").distance);

    assert_eq!(Direction::Up, Instruction::from_str("U100").direction);
    assert_eq!(Direction::Down, Instruction::from_str("D100").direction);
    assert_eq!(Direction::Left, Instruction::from_str("L100").direction);
    assert_eq!(Direction::Right, Instruction::from_str("R100").direction);
}

#[test]
fn test_calculate_intersection_lowest_distance() {
    assert_eq!(true, true);
    assert_ne!(true, false);

    let test_wire_a: Vec<Instruction> =
        vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"]
            .par_iter()
            .map(|x| Instruction::from_str(x))
            .collect();
    let test_wire_b: Vec<Instruction> =
        vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"]
            .par_iter()
            .map(|x| Instruction::from_str(x))
            .collect();

    let test_wire_c: Vec<Instruction> = vec![
        "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
    ]
    .par_iter()
    .map(|x| Instruction::from_str(x))
    .collect();

    let test_wire_d: Vec<Instruction> = vec![
        "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
    ]
    .par_iter()
    .map(|x| Instruction::from_str(x))
    .collect();

    assert_eq!(
        610,
        calculate_intersection_lowest_distance(test_wire_a, test_wire_b)
    );
    assert_eq!(
        410,
        calculate_intersection_lowest_distance(test_wire_c, test_wire_d)
    );
}
