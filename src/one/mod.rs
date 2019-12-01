pub fn puzzle() {
    let inputs: Vec<f32> = INPUT_VEC.iter().map(|x|**x).collect::<Vec<f32>>();
    let answer_a: f32 = inputs.iter().map(|x| calculate_fuel(*x)).sum();
    println!("The answer for day 1, part a is: {}", answer_a);
    let answer_b: f32 = inputs.iter().map(|x| calculate_fuel_recursive(*x)).sum();
    println!("The answer for day 1, part b is: {}", answer_b);
}

fn calculate_fuel(input: f32) -> f32 {
    (input / 3 as f32).floor() - 2 as f32
}

fn calculate_fuel_recursive(input: f32) -> f32 {
    let result = (input / 3 as f32).floor() - 2 as f32;
    match result {
        n if n <= 0 as f32 => 0 as f32,
        _ => calculate_fuel_recursive(result) + result,
    }
}

#[test]
fn test_puzzle() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    assert_eq!(2 as f32, calculate_fuel(12 as f32));
    assert_eq!(2 as f32, calculate_fuel(14 as f32));
    assert_eq!(654 as f32, calculate_fuel(1969 as f32));
    assert_eq!(33583 as f32, calculate_fuel(100756 as f32));

    assert_eq!(2 as f32, calculate_fuel_recursive(14 as f32));
    assert_eq!(966 as f32, calculate_fuel_recursive(1969 as f32));
    assert_eq!(50346 as f32, calculate_fuel_recursive(100756 as f32));
}

const INPUT_VEC: [&'static f32; 100] = [
    &51585_f32,
    &137484_f32,
    &73634_f32,
    &71535_f32,
    &87274_f32,
    &74243_f32,
    &127025_f32,
    &66829_f32,
    &138729_f32,
    &145459_f32,
    &118813_f32,
    &82326_f32,
    &82518_f32,
    &145032_f32,
    &148699_f32,
    &105958_f32,
    &103969_f32,
    &72689_f32,
    &145061_f32,
    &70385_f32,
    &53104_f32,
    &107851_f32,
    &103392_f32,
    &107051_f32,
    &123475_f32,
    &123918_f32,
    &56709_f32,
    &89284_f32,
    &86208_f32,
    &71943_f32,
    &109257_f32,
    &108272_f32,
    &124811_f32,
    &142709_f32,
    &115650_f32,
    &53607_f32,
    &142891_f32,
    &144135_f32,
    &114277_f32,
    &138671_f32,
    &111998_f32,
    &70838_f32,
    &69802_f32,
    &107210_f32,
    &103319_f32,
    &60377_f32,
    &58639_f32,
    &131863_f32,
    &100807_f32,
    &118360_f32,
    &52573_f32,
    &108207_f32,
    &128009_f32,
    &96180_f32,
    &148492_f32,
    &112914_f32,
    &72867_f32,
    &140991_f32,
    &131267_f32,
    &125123_f32,
    &58393_f32,
    &129615_f32,
    &87239_f32,
    &63085_f32,
    &59231_f32,
    &95007_f32,
    &147712_f32,
    &109838_f32,
    &89829_f32,
    &55634_f32,
    &96163_f32,
    &52323_f32,
    &106701_f32,
    &141511_f32,
    &125349_f32,
    &137267_f32,
    &50694_f32,
    &53692_f32,
    &57466_f32,
    &117769_f32,
    &63535_f32,
    &101708_f32,
    &113593_f32,
    &79163_f32,
    &112327_f32,
    &91994_f32,
    &129674_f32,
    &58076_f32,
    &145062_f32,
    &122730_f32,
    &102481_f32,
    &109994_f32,
    &136271_f32,
    &111178_f32,
    &117920_f32,
    &107933_f32,
    &104305_f32,
    &99613_f32,
    &68482_f32,
    &126543_f32,
];
