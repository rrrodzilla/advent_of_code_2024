use aoc_day_7::BridgeCalibrator;
use std::io::Read;

fn main() {
    // 📥 Read engineering specifications
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read engineering notes");

    match BridgeCalibrator::new(&input) {
        Ok(calibrator) => {
            println!(
                "Basic Stress Analysis: {}",
                calibrator.calculate_basic_stress()
            );
            println!(
                "Advanced Stress Analysis: {}",
                calibrator.calculate_advanced_stress()
            );
        }
        Err(e) => {
            eprintln!("🚨 Engineering Error: {}", e);
            std::process::exit(1);
        }
    }
}
