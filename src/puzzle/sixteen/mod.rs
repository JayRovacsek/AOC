mod test;

pub fn solve() {
    let input_vec = build_vec_from_str(INPUT);
    let answer_a = execute_phase_x_times(&input_vec, 100);
    println!("The answer for day 16, part a is: {:?}", &answer_a[0..8]);

    // let answer_a = execute_phase_x_times_with_offset(&input_vec, 10000);
    // println!("The answer for day 16, part b is: {:?}", &answer_a[0..8]);
}

fn build_vec_from_str(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|d| d.to_digit(10).unwrap_or(0) as i32)
        .collect::<Vec<i32>>()
}

fn flatten_int_vec(input: Vec<i32>) -> usize {
    input
        .iter()
        .rev()
        .enumerate()
        .fold(0, |a, b| a + (*b.1 * (10_i32.pow(b.0 as u32))) as usize)
}

fn generate_pattern(input: usize, size: usize) -> Vec<i32> {
    let mut r: Vec<i32> = Vec::new();
    while r.len() <= size {
        r.append(
            &mut PATTERN
                .iter()
                .map(|i| vec![*i; input + 1])
                .flatten()
                .collect::<Vec<i32>>(),
        );
    }
    r.rotate_left(1);
    let d = r.drain(0..size);
    d.collect::<Vec<i32>>()
}

fn execute_phase_x_times(input: &Vec<i32>, times: u32) -> Vec<i32> {
    (1..times)
        .collect::<Vec<u32>>()
        .iter()
        .fold(execute_phase(&input), |a, _| execute_phase(&a))
}

fn execute_phase_x_times_with_offset(input: &Vec<i32>, times: u32) -> Vec<i32> {
    let offset = flatten_int_vec(input.clone().drain(0..8).collect()) % input.len();
    let result = (1..times)
        .collect::<Vec<u32>>()
        .iter()
        .fold(execute_phase(&input), |a, _| execute_phase(&a));

    result[offset..offset + 8].to_vec()
}

fn offset_vec(input: &Vec<i32>) -> usize {
    let offset = flatten_int_vec(input.clone().drain(0..8).collect()) % input.len();
    flatten_int_vec(input[offset..offset + 8].to_vec())
}

fn execute_phase(input: &[i32]) -> Vec<i32> {
    (0..input.len())
        .collect::<Vec<usize>>()
        .iter()
        .enumerate()
        .map(|x| {
            println!("Phases: {:?}", x.0);
            let phases = generate_pattern(x.0, input.len());
            let x = input
                .iter()
                .zip(phases.iter())
                .map(|y| (*y.0 as i32) * y.1)
                .sum::<i32>();
            (x.abs()) % 10
        })
        .collect::<Vec<i32>>()
}

const INPUT: &str = "59702216318401831752516109671812909117759516365269440231257788008453756734827826476239905226493589006960132456488870290862893703535753691507244120156137802864317330938106688973624124594371608170692569855778498105517439068022388323566624069202753437742801981883473729701426171077277920013824894757938493999640593305172570727136129712787668811072014245905885251704882055908305407719142264325661477825898619802777868961439647723408833957843810111456367464611239017733042717293598871566304020426484700071315257217011872240492395451028872856605576492864646118292500813545747868096046577484535223887886476125746077660705155595199557168004672030769602168262";
const PATTERN: [i32; 4] = [0, 1, 0, -1];
