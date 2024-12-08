use aoc_day_7::{BridgeCalibrator, BridgeEquation};
use criterion::{black_box, criterion_group, criterion_main, Criterion}; // Replace with your crate name

// 📊 Test data for bridge calibration measurements
const ENGINEERING_SPECS: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

// 🔬 Individual component benchmarks
fn bench_basic_calibration(c: &mut Criterion) {
    let calibrator = BridgeCalibrator::new(ENGINEERING_SPECS).unwrap();
    c.bench_function("basic stress analysis", |b| {
        b.iter(|| black_box(calibrator.calculate_basic_stress()))
    });
}

fn bench_advanced_calibration(c: &mut Criterion) {
    let calibrator = BridgeCalibrator::new(ENGINEERING_SPECS).unwrap();
    c.bench_function("advanced stress analysis", |b| {
        b.iter(|| black_box(calibrator.calculate_advanced_stress()))
    });
}

// 🧪 Detailed equation parsing benchmarks
fn bench_equation_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("equation parsing");

    // Test simple equation parsing
    group.bench_function("simple equation", |b| {
        b.iter(|| black_box("190: 10 19".parse::<BridgeEquation>()))
    });

    // Test complex equation parsing
    group.bench_function("complex equation", |b| {
        b.iter(|| black_box("21037: 9 7 18 13".parse::<BridgeEquation>()))
    });

    group.finish();
}

// 🎯 Calibration possibility checking benchmarks
fn bench_calibration_checks(c: &mut Criterion) {
    let mut group = c.benchmark_group("calibration checks");

    // Simple two-number equation
    let simple_eq = "190: 10 19".parse::<BridgeEquation>().unwrap();
    group.bench_function("basic operators (2 numbers)", |b| {
        b.iter(|| black_box(simple_eq.is_calibratable(false)))
    });
    group.bench_function("all operators (2 numbers)", |b| {
        b.iter(|| black_box(simple_eq.is_calibratable(true)))
    });

    // Complex four-number equation
    let complex_eq = "21037: 9 7 18 13".parse::<BridgeEquation>().unwrap();
    group.bench_function("basic operators (4 numbers)", |b| {
        b.iter(|| black_box(complex_eq.is_calibratable(false)))
    });
    group.bench_function("all operators (4 numbers)", |b| {
        b.iter(|| black_box(complex_eq.is_calibratable(true)))
    });

    group.finish();
}

// 🏗️ Full system benchmarks
fn bench_full_system(c: &mut Criterion) {
    c.bench_function("full system initialization", |b| {
        b.iter(|| BridgeCalibrator::new(black_box(ENGINEERING_SPECS)))
    });
}

criterion_group!(
    benches,
    bench_basic_calibration,
    bench_advanced_calibration,
    bench_equation_parsing,
    bench_calibration_checks,
    bench_full_system
);
criterion_main!(benches);
