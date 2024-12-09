use aoc_day_7::BridgeCalibrator;
use memory_stats::memory_stats;
use std::time::Instant;

fn main() {
    // Read input
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input");

    // Measure initial memory
    let start_mem = memory_stats()
        .expect("Failed to get memory stats")
        .physical_mem;

    // Time initialization
    let start = Instant::now();
    let calibrator = BridgeCalibrator::new(&input).expect("Failed to create calibrator");
    let init_time = start.elapsed();

    // Measure post-init memory
    let post_init_mem = memory_stats()
        .expect("Failed to get memory stats")
        .physical_mem;

    // Time and measure basic stress calculation
    let start = Instant::now();
    let basic_result = calibrator.calculate_basic_stress();
    let basic_time = start.elapsed();
    let post_basic_mem = memory_stats()
        .expect("Failed to get memory stats")
        .physical_mem;

    // Time and measure advanced stress calculation
    let start = Instant::now();
    let advanced_result = calibrator.calculate_advanced_stress();
    let advanced_time = start.elapsed();
    let post_advanced_mem = memory_stats()
        .expect("Failed to get memory stats")
        .physical_mem;

    // Print results
    println!("Memory Usage Analysis:");
    println!("---------------------");
    println!("Initial memory: {} bytes", start_mem);
    println!(
        "Memory after init: {} bytes (+ {} bytes)",
        post_init_mem,
        post_init_mem - start_mem
    );
    println!(
        "Memory after basic: {} bytes (+ {} bytes)",
        post_basic_mem,
        post_basic_mem - post_init_mem
    );
    println!(
        "Memory after advanced: {} bytes (+ {} bytes)",
        post_advanced_mem,
        post_advanced_mem - post_basic_mem
    );

    println!("\nTiming Analysis:");
    println!("----------------");
    println!("Initialization: {:?}", init_time);
    println!("Basic stress calculation: {:?}", basic_time);
    println!("Advanced stress calculation: {:?}", advanced_time);

    println!("\nResults:");
    println!("--------");
    println!("Basic stress result: {}", basic_result);
    println!("Advanced stress result: {}", advanced_result);
}
