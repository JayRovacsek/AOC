use aoc_2020::input::read_contents;
use aoc_2020::puzzle::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 1, part 1", |b| {
        b.iter(|| one::solve_part_one(black_box(&read_contents("./input/day_one.txt"))))
    });
    c.bench_function("day 1, part 2", |b| {
        b.iter(|| one::solve_part_two(black_box(&read_contents("./input/day_one.txt"))))
    });
    c.bench_function("day 2, part 1", |b| {
        b.iter(|| two::solve_part_one(black_box(&read_contents("./input/day_two.txt"))))
    });
    c.bench_function("day 2, part 2", |b| {
        b.iter(|| two::solve_part_two(black_box(&read_contents("./input/day_two.txt"))))
    });
    c.bench_function("day 3, part 1", |b| {
        b.iter(|| three::solve_part_one(black_box(&read_contents("./input/day_three.txt"))))
    });
    c.bench_function("day 3, part 2", |b| {
        b.iter(|| three::solve_part_two(black_box(&read_contents("./input/day_three.txt"))))
    });
    c.bench_function("day 4, part 1", |b| {
        b.iter(|| four::solve_part_one(black_box(&read_contents("./input/day_four.txt"))))
    });
    c.bench_function("day 4, part 2", |b| {
        b.iter(|| four::solve_part_two(black_box(&read_contents("./input/day_four.txt"))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
