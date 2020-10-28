mod test;

use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Body {
    x: i64,
    y: i64,
    z: i64,
    velocity: Velocity,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Velocity {
    x: i64,
    y: i64,
    z: i64,
}

impl Body {
    fn from_str(input: &str) -> Body {
        let point: Vec<i64> = input
            .split('=')
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.split(',').collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>()
            .iter()
            .flatten()
            .collect::<Vec<&&str>>()
            .iter()
            .map(|x| x.replace("<", "").replace(">", ""))
            .filter(|x| x.parse::<i64>().is_ok())
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        Body {
            x: *point.first().unwrap_or(&0),
            y: *point.get(1).unwrap_or(&0),
            z: *point.last().unwrap_or(&0),
            velocity: Default::default(),
        }
    }

    fn update_velocity(self, bodies: Vec<Body>) -> Body {
        let x: i64 = bodies.iter().map(|x|{
            match x.x {
                n if n > self.x => 1,
                n if n == self.x => 0,
                n if n < self.x => -1,
                _ => panic!("The Rust complier thinks the above does not match all patterns. Alas, I may be wrong but I think it may")
            }
        }).sum();
        let y: i64 = bodies.iter().map(|x|{
            match x.y {
                n if n > self.y => 1,
                n if n == self.y => 0,
                n if n < self.y => -1,
                _ => panic!("The Rust complier thinks the above does not match all patterns. Alas, I may be wrong but I think it may")
            }
        }).sum();
        let z: i64 = bodies.iter().map(|x|{
            match x.z {
                n if n > self.z => 1,
                n if n == self.z => 0,
                n if n < self.z => -1,
                _ => panic!("The Rust complier thinks the above does not match all patterns. Alas, I may be wrong but I think it may")
            }
        }).sum();
        Body {
            x: self.x,
            y: self.y,
            z: self.z,
            velocity: Velocity {
                x: x + self.velocity.x,
                y: y + self.velocity.y,
                z: z + self.velocity.z,
            },
        }
    }

    fn apply_velocity(self) -> Body {
        Body {
            x: self.x + self.velocity.x,
            y: self.y + self.velocity.y,
            z: self.z + self.velocity.z,
            velocity: self.velocity,
        }
    }

    fn potential_energy(self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }

    fn kinetic_energy(self) -> usize {
        (self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()) as usize
    }

    fn total_energy(self) -> usize {
        self.potential_energy() * self.kinetic_energy()
    }
}

fn run_steps(bodies: Vec<Body>, steps: usize) -> Vec<Body> {
    (0..steps)
        .collect::<Vec<_>>()
        .iter()
        .fold(bodies, |bodies_vec, _| {
            bodies_vec
                .par_iter()
                .map(|x| {
                    let alternate_bodies: Vec<Body> = bodies_vec
                        .clone()
                        .into_iter()
                        .filter(|y| *y != *x)
                        .collect();
                    x.update_velocity(alternate_bodies)
                })
                .map(|x| x.apply_velocity())
                .collect()
        })
}

fn find_repeated_state(bodies: Vec<Body>, seen: HashSet<Vec<Body>>, c: usize) -> usize {
    let duplicate_found: bool = bodies
        .iter()
        .map(|_| seen.iter().map(|y| *y == bodies).any(|y| y))
        .filter(|x| *x)
        .any(|x| x);
    if duplicate_found {
        c
    } else {
        let s: HashSet<Vec<Body>> = seen
            .into_iter()
            .chain(vec![bodies.clone()].into_iter())
            .collect();
        let b = run_steps(bodies, 1);
        find_repeated_state(b, s, c + 1)
    }
}

pub fn solve() {
    let bodies = INPUT_VEC
        .iter()
        .map(|x| Body::from_str(x))
        .collect::<Vec<Body>>();

    let answer_a: usize = run_steps(bodies.clone(), 1000)
        .iter()
        .map(|x| x.total_energy())
        .sum();

    println!("Answer a: {:?}", answer_a);

    let answer_b = find_repeated_state(bodies, HashSet::new(), 0);
    println!("Answer b: {:?}", answer_b);
}

const INPUT_VEC: [&str; 4] = [
    "<x=-4, y=-14, z=8>",
    "<x=1, y=-8, z=10>",
    "<x=-15, y=2, z=1>",
    "<x=-17, y=-17, z=16>",
];
