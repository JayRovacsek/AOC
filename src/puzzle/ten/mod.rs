mod test;

use rayon::prelude::*;

pub fn solve() {
    let point_map = generate_point_map(INPUT, 34);
    println!("{:?} points", point_map.len());

    let answer_a = most_visible_point(point_map);

    println!("The answer for day 10, part a is: {:?}", answer_a);
}

fn most_visible_point(point_map: Vec<(f32, f32)>) -> usize {
    point_map
        .par_iter()
        .map(|x| {
            point_map
                .iter()
                .map(|y| {
                    point_map
                        .iter()
                        .filter(|z| x != y && x != *z && y != *z)
                        .map(|z| is_visible(*x, *y, *z))
                        .filter(|a| *a)
                        .count()
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

// fn is_visible(x: (f32, f32), y: (f32, f32), z: (f32, f32)) -> bool {
//     let cross = (z.1 - x.1) * (y.0 - x.0) - (z.0 - x.0) * (y.1 - x.1);
//     let dot_product = (z.0 - x.0) * (y.0 - x.0) + (z.1 - x.1) * (y.1 - x.1);
//     let squaredlengthba = (y.0 - x.0) * (y.0 - x.0) + (y.1 - x.1) * (y.1 - x.1);

//     if cross.abs() > std::f32::EPSILON {
//         true
//     } else {
//         if dot_product > 0_f32 {
//             true
//         } else {
//             if dot_product > squaredlengthba {
//                 true
//             } else {
//                 false
//             }
//         }
//     }
// }

fn is_visible(x: (f32, f32), y: (f32, f32), z: (f32, f32)) -> bool {
    let dxc = z.0 - x.0;
    let dyc = z.1 - x.1;

    let dxl = y.0 - x.0;
    let dyl = y.1 - x.1;

    let cross = dxc * dyl - dyc * dxl;
    // cross != 0_f32

    if cross == 0_f32 {
        if dxl.abs() >= dyl.abs() {
            if dxl > 0_f32 {
                true
            } else {
                x.0 <= z.0 && z.0 <= y.0
            }
        } else if dyl > 0_f32 {
            true
        } else {
            x.1 <= z.1 && z.1 <= y.1
        }
    } else {
        true
    }
}

fn distance(x: (f32, f32), y: (f32, f32)) -> f32 {
    (((x.0 - y.0).powf(2.0)) + ((x.1 - y.1).powf(2.0))).sqrt()
}

fn generate_point_map(input: &str, width: usize) -> Vec<(f32, f32)> {
    if input.chars().filter(|x| *x != '\n').count() % width != 0 {
        panic!("Odd number of inputs to process!");
    };

    input
        .chars()
        .filter(|x| *x != '\n')
        .map(|x| x == '#')
        .collect::<Vec<bool>>()
        .chunks(width)
        .enumerate()
        .map(|x| {
            x.1.iter()
                .enumerate()
                .filter(|y| *y.1)
                .map(|y| (x.0 as f32, y.0 as f32))
                .collect::<Vec<(f32, f32)>>()
        })
        .flatten()
        .collect::<Vec<(f32, f32)>>()
}

const INPUT: &str = "#.#....#.#......#.....#......####.
#....#....##...#..#..##....#.##..#
#.#..#....#..#....##...###......##
...........##..##..##.####.#......
...##..##....##.#.....#.##....#..#
..##.....#..#.......#.#.........##
...###..##.###.#..................
.##...###.#.#.......#.#...##..#.#.
...#...##....#....##.#.....#...#.#
..##........#.#...#..#...##...##..
..#.##.......#..#......#.....##..#
....###..#..#...###...#.###...#.##
..#........#....#.....##.....#.#.#
...#....#.....#..#...###........#.
.##...#........#.#...#...##.......
.#....#.#.#.#.....#...........#...
.......###.##...#..#.#....#..##..#
#..#..###.#.......##....##.#..#...
..##...#.#.#........##..#..#.#..#.
.#.##..#.......#.#.#.........##.##
...#.#.....#.#....###.#.........#.
.#..#.##...#......#......#..##....
.##....#.#......##...#....#.##..#.
#..#..#..#...........#......##...#
#....##...#......#.###.#..#.#...#.
#......#.#.#.#....###..##.##...##.
......#.......#.#.#.#...#...##....
....##..#.....#.......#....#...#..
.#........#....#...#.#..#....#....
.#.##.##..##.#.#####..........##..
..####...##.#.....##.............#
....##......#.#..#....###....##...
......#..#.#####.#................
.#....#.#..#.###....##.......##.#.";
