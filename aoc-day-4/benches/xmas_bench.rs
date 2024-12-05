use aoc_day_4::Grid;
use criterion::{criterion_group, criterion_main, Criterion};
use std::{fs, str::FromStr, time::Duration};

fn bench_xmas_search(c: &mut Criterion) {
    let input = fs::read_to_string("part_one.input").expect("Failed to read input file");
    let grid = Grid::from_str(&input).unwrap();

    let mut group = c.benchmark_group("grid_patterns");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("find_word_xmas", |b| {
        b.iter(|| criterion::black_box(grid.find_word_xmas()))
    });

    group.bench_function("find_crossed_mas", |b| {
        b.iter(|| criterion::black_box(grid.find_crossed_mas()))
    });

    group.finish();
}

criterion_group!(benches, bench_xmas_search);
criterion_main!(benches);
