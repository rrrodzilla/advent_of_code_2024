use aoc_day_7::BridgeCalibrator;
use memory_stats::memory_stats;
use std::time::Instant;

fn measure_memory<F>(name: &str, f: F) -> u64
where
    F: FnOnce(),
{
    if let Some(usage_before) = memory_stats() {
        let start = Instant::now();
        f();
        let duration = start.elapsed();

        if let Some(usage_after) = memory_stats() {
            let mem_diff = usage_after
                .physical_mem
                .saturating_sub(usage_before.physical_mem);
            println!("{}: ", name);
            println!("  Time: {:?}", duration);
            println!("  Memory delta: {} KB", mem_diff / 1024);
            println!(
                "  Final memory: {} MB",
                usage_after.physical_mem / 1024 / 1024
            );
            return mem_diff.try_into().unwrap();
        }
    }
    0
}

fn main() {
    println!("Memory Profile Analysis");
    println!("======================\n");

    // Read input
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input");
    println!("Input size: {} bytes\n", input.len());

    // Measure initialization
    let mut calibrator = None;
    measure_memory("Initialization", || {
        calibrator = Some(BridgeCalibrator::new(&input).expect("Failed to create calibrator"));
    });

    let calibrator = calibrator.unwrap();

    // Measure basic stress calculation
    measure_memory("Basic stress calculation", || {
        let result = calibrator.calculate_basic_stress();
        println!("  Result: {}", result);
    });

    // Measure advanced stress calculation
    measure_memory("Advanced stress calculation", || {
        let result = calibrator.calculate_advanced_stress();
        println!("  Result: {}", result);
    });

    // Force a garbage collection if possible
    drop(calibrator);

    if let Some(final_usage) = memory_stats() {
        println!(
            "\nFinal memory state: {} MB",
            final_usage.physical_mem / 1024 / 1024
        );
    }
}
