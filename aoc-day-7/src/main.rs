use aoc_day_7::BridgeCalibrator;
use std::io::Read;

/// 🌴 Bridge Repair Calibration System 🌉
/// Helps engineers fix the frequently-breaking rope bridge
/// while accounting for mischievous elephants who keep
/// stealing operators from the calibration equations!
fn main() {
    // Check if we need to redact sensitive engineering data
    let redact = std::env::args().any(|arg| arg == "--redact");

    // 🐘 Collect the scattered engineering notes left behind
    // after the elephants' playful operator-stealing spree
    println!("🌿 Starting bridge calibration sequence...");
    println!("🔍 Searching for scattered engineering notes...");

    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("📜 Critical failure: Engineering notes lost in the jungle!");

    // 🛠️ Initialize the bridge calibration system
    match BridgeCalibrator::new(&input) {
        Ok(calibrator) => {
            // 📊 Phase 1: Basic stress analysis (only + and *)
            println!("\n🌴 Phase 1: Basic Structural Analysis 🌴");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("📝 Analyzing equations with standard operators...");
            let basic_stress = if redact {
                "**REDACTED**".to_string()
            } else {
                calibrator.calculate_basic_stress().to_string()
            };
            println!("🔢 Basic Stress Coefficient: {}\n", basic_stress);

            // 🎯 Phase 2: Advanced analysis (including || operator)
            println!("🌴 Phase 2: Advanced Structural Analysis 🌴");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("🐘 Accounting for sneaky elephant operators...");
            let advanced_stress = if redact {
                "**REDACTED**".to_string()
            } else {
                calibrator.calculate_advanced_stress().to_string()
            };
            println!("🔢 Advanced Stress Coefficient: {}\n", advanced_stress);

            if redact {
                println!("🔒 Security Notice: Values redacted for jungle safety!");
            }
            println!("✨ Bridge calibration complete! ✨");
            println!("🌉 The rope bridge should now be safe to cross.");
            println!("   (At least until the elephants return...)");
        }
        Err(e) => {
            eprintln!("\n🚫 CALIBRATION ERROR! 🚫");
            eprintln!("━━━━━━━━━━━━━━━━━━━");
            eprintln!("🐘 The elephants may have caused more chaos than expected!");
            eprintln!("❌ Engineering Error: {}", e);
            eprintln!("📋 Recommendation: Double-check all equations and");
            eprintln!("   search the jungle for missing operators.");
            std::process::exit(1);
        }
    }
}
