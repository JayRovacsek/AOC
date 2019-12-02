pub fn solve() {
    let answer_a: f32 = INPUT_VEC.iter().map(|x| calculate_fuel(*x)).sum();
    println!("The answer for day 1, part a is: {}", answer_a);
    let answer_b: f32 = INPUT_VEC.iter().map(|x| calculate_fuel_recursive(*x)).sum();
    println!("The answer for day 1, part b is: {}", answer_b);
}

fn calculate_fuel(input: f32) -> f32 {
    (input / 3_f32).floor() - 2_f32
}

fn calculate_fuel_recursive(input: f32) -> f32 {
    let result = (input / 3_f32).floor() - 2_f32;
    match result {
        n if n <= 0_f32 => 0_f32,
        _ => calculate_fuel_recursive(result) + result,
    }
}

#[test]
fn test_solve() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    assert_eq!(2_u32, calculate_fuel(12_f32) as u32);
    assert_eq!(2_u32, calculate_fuel(14_f32) as u32);
    assert_eq!(654_u32, calculate_fuel(1969_f32) as u32);
    assert_eq!(33583_u32, calculate_fuel(100_756_f32) as u32);

    assert_eq!(2_u32, calculate_fuel_recursive(14_f32) as u32);
    assert_eq!(966_u32, calculate_fuel_recursive(1969_f32) as u32);
    assert_eq!(50346_u32, calculate_fuel_recursive(100_756_f32) as u32);
}

const INPUT_VEC: [f32; 100] = [
    51585_f32,
    137_484_f32,
    73634_f32,
    71535_f32,
    87274_f32,
    74243_f32,
    127_025_f32,
    66829_f32,
    138_729_f32,
    145_459_f32,
    118_813_f32,
    82326_f32,
    82518_f32,
    145_032_f32,
    148_699_f32,
    105_958_f32,
    103_969_f32,
    72689_f32,
    145_061_f32,
    70385_f32,
    53104_f32,
    107_851_f32,
    103_392_f32,
    107_051_f32,
    123_475_f32,
    123_918_f32,
    56709_f32,
    89284_f32,
    86208_f32,
    71943_f32,
    109_257_f32,
    108_272_f32,
    124_811_f32,
    142_709_f32,
    115_650_f32,
    53607_f32,
    142_891_f32,
    144_135_f32,
    114_277_f32,
    138_671_f32,
    111_998_f32,
    70838_f32,
    69802_f32,
    107_210_f32,
    103_319_f32,
    60377_f32,
    58639_f32,
    131_863_f32,
    100_807_f32,
    118_360_f32,
    52573_f32,
    108_207_f32,
    128_009_f32,
    96180_f32,
    148_492_f32,
    112_914_f32,
    72867_f32,
    140_991_f32,
    131_267_f32,
    125_123_f32,
    58393_f32,
    129_615_f32,
    87239_f32,
    63085_f32,
    59231_f32,
    95007_f32,
    147_712_f32,
    109_838_f32,
    89829_f32,
    55634_f32,
    96163_f32,
    52323_f32,
    106_701_f32,
    141_511_f32,
    125_349_f32,
    137_267_f32,
    50694_f32,
    53692_f32,
    57466_f32,
    117_769_f32,
    63535_f32,
    101_708_f32,
    113_593_f32,
    79163_f32,
    112_327_f32,
    91994_f32,
    129_674_f32,
    58076_f32,
    145_062_f32,
    122_730_f32,
    102_481_f32,
    109_994_f32,
    136_271_f32,
    111_178_f32,
    117_920_f32,
    107_933_f32,
    104_305_f32,
    99613_f32,
    68482_f32,
    126_543_f32,
];
