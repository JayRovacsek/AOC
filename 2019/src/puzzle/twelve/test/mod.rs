use super::*;

#[test]
fn test_solve() {
    assert_eq!(true, true);
    assert_ne!(true, false);
}

#[test]
fn test_calculate_total_energy_a1() {
    let bodies: Vec<Body> = vec![
        "<x=-1, y=0, z=2>",
        "<x=2, y=-10, z=-7>",
        "<x=4, y=-8, z=8>",
        "<x=3, y=5, z=-1>",
    ]
    .iter()
    .map(|x| Body::from_str(x))
    .collect();

    assert_eq!(true, true);
    assert_ne!(true, false);

    assert_eq!(
        179,
        run_steps(bodies, 10)
            .par_iter()
            .map(|x| x.total_energy())
            .sum::<usize>()
    )
}

#[test]
fn test_calculate_total_energy_a2() {
    assert_eq!(true, true);
    assert_ne!(true, false);

    let bodies: Vec<Body> = vec![
        "<x=-8, y=-10, z=0>",
        "<x=5, y=5, z=10>",
        "<x=2, y=-7, z=3>",
        "<x=9, y=-8, z=-3>",
    ]
    .iter()
    .map(|x| Body::from_str(x))
    .collect();

    assert_eq!(
        1940,
        run_steps(bodies, 100)
            .par_iter()
            .map(|x| x.total_energy())
            .sum::<usize>()
    )
}

// #[test]
// fn test_repeated_state() {
//     assert_eq!(true, true);
//     assert_ne!(true, false);

//     let bodies: Vec<Body> = vec![
//         "<x=-1, y=0, z=2>",
//         "<x=2, y=-10, z=-7>",
//         "<x=4, y=-8, z=8>",
//         "<x=3, y=5, z=-1>",
//     ]
//     .iter()
//     .map(|x| Body::from_str(x))
//     .collect();

//     assert_eq!(2772, find_repeated_state(bodies, HashSet::new(), 0))
// }
