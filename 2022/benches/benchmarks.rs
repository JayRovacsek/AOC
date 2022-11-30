use aoc_2022::input::read_contents;
use aoc_2022::puzzle::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 1, part 1", |b| {
        b.iter(|| one::solve_part_one(black_box(&read_contents("./input/day_one.txt"))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
