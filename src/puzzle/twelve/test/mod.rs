use super::*;

#[test]
fn test_solve() {
    assert_eq!(true, true);
    assert_ne!(true, false);
}

#[test]
fn test_calculate_total_energy() {
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
        run_steps(bodies, 10).iter().map(|x| x.total_energy()).sum::<usize>()
    )
}
