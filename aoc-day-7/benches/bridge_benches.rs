use aoc_day_7::{BridgeCalibrator, BridgeEquation};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use memory_stats::memory_stats;
use std::time::Duration;

// 📊 Load actual engineering specifications from file
fn load_engineering_specs() -> String {
    std::fs::read_to_string("input.txt").expect("Failed to read input file")
}

// 🔬 Individual component benchmarks
fn bench_basic_calibration(c: &mut Criterion) {
    let input = load_engineering_specs();
    let calibrator = BridgeCalibrator::new(&input).unwrap();
    c.bench_function("basic stress analysis", |b| {
        b.iter(|| black_box(calibrator.calculate_basic_stress()))
    });
}

fn bench_advanced_calibration(c: &mut Criterion) {
    let input = load_engineering_specs();
    let calibrator = BridgeCalibrator::new(&input).unwrap();
    c.bench_function("advanced stress analysis", |b| {
        b.iter(|| black_box(calibrator.calculate_advanced_stress()))
    });
}

// 🧪 Detailed equation parsing benchmarks
fn bench_equation_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("equation parsing");
    let input = load_engineering_specs();
    let sample_lines: Vec<_> = input.lines().take(2).collect();

    // Test simple equation (first line from input)
    if let Some(first_line) = sample_lines.first() {
        group.bench_function("simple equation", |b| {
            b.iter(|| black_box(first_line.parse::<BridgeEquation>()))
        });
    }

    // Test complex equation (second line from input)
    if let Some(second_line) = sample_lines.get(1) {
        group.bench_function("complex equation", |b| {
            b.iter(|| black_box(second_line.parse::<BridgeEquation>()))
        });
    }

    group.finish();
}

// 🎯 Calibration possibility checking benchmarks
fn bench_calibration_checks(c: &mut Criterion) {
    let mut group = c.benchmark_group("calibration checks");
    let input = load_engineering_specs();
    let sample_lines: Vec<_> = input.lines().take(2).collect();

    // Simple equation checking
    if let Some(first_line) = sample_lines.first() {
        let simple_eq = first_line.parse::<BridgeEquation>().unwrap();
        group.bench_function("basic operators (simple eq)", |b| {
            b.iter(|| black_box(simple_eq.is_calibratable(false)))
        });
        group.bench_function("all operators (simple eq)", |b| {
            b.iter(|| black_box(simple_eq.is_calibratable(true)))
        });
    }

    // Complex equation checking
    if let Some(second_line) = sample_lines.get(1) {
        let complex_eq = second_line.parse::<BridgeEquation>().unwrap();
        group.bench_function("basic operators (complex eq)", |b| {
            b.iter(|| black_box(complex_eq.is_calibratable(false)))
        });
        group.bench_function("all operators (complex eq)", |b| {
            b.iter(|| black_box(complex_eq.is_calibratable(true)))
        });
    }

    group.finish();
}

// 🏗️ Full system benchmarks
fn bench_full_system(c: &mut Criterion) {
    let input = load_engineering_specs();
    c.bench_function("full system initialization", |b| {
        b.iter(|| BridgeCalibrator::new(black_box(&input)))
    });
}

// 📊 Memory usage benchmarks
fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory usage");
    let input = load_engineering_specs();

    group.bench_function("system initialization", |b| {
        let mut total_physical_mem: usize = 0;
        let mut iterations: usize = 0;
        b.iter_custom(|iters| {
            let mut duration = Duration::ZERO;
            for _ in 0..iters {
                if let Some(usage) = memory_stats() {
                    let before = usage.physical_mem;
                    let start = std::time::Instant::now();
                    black_box(BridgeCalibrator::new(&input).unwrap());
                    duration += start.elapsed();
                    let after = memory_stats().unwrap().physical_mem;
                    total_physical_mem += after - before;
                    iterations += 1;
                }
            }
            //          println!(
            //              "Average memory increase: {} bytes",
            //              total_physical_mem / iterations
            //          );
            duration / iters as u32
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_basic_calibration,
    bench_advanced_calibration,
    bench_equation_parsing,
    bench_calibration_checks,
    bench_full_system,
    bench_memory_usage
);
criterion_main!(benches);
