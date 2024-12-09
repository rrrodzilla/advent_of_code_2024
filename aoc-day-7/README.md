# The Jungle Bridge Repair Calibrator 🌉

A highly optimized bridge stress analysis system for validating equations after
mischievous elephants stole operators from the engineers' calibration formulas,
powered by advanced mathematical pattern recognition algorithms.

![Made with VHS](https://vhs.charm.sh/vhs-4Giu798WnhLqBELifuUd0T.gif)

## 📜 The Story

Deep in a jungle, you encounter a group of engineers attempting to repair a
frequently-breaking rope bridge. The final calibrations are stalled because some
playful young elephants have stolen all the operators from their equations! This
application helps analyze which combinations of operators could possibly produce
valid calibration results, ensuring the bridge can be safely repaired.

## 🔧 Technical Implementation

Built using Rust's robust type system, this application demonstrates:

- Multi-phase calibration analysis for different operator sets
- Comprehensive equation validation system
- Custom error handling for invalid calibration sequences
- Security-conscious output redaction for sensitive engineering data
- A massively over-engineered solution to the Advent of Code Day 7 Challenge

The system uses two main components:

1. `BridgeCalibrator`: Validates equations under different operator rulesets
2. `StressAnalysis`: Computes structural integrity coefficients based on valid
   equations

## 🚀 Running the Application

### Prerequisites

- Rust and Cargo installed
- A good sense of humor about elephants

### Running with Sample Data

```bash
# Standard calibration analysis
cargo run < input.txt

# Classified analysis (for engineering security)
cargo run -- --redact < input.txt
```

### Input Format

The input represents calibration equations, where each line contains:

- A test value followed by a colon
- A sequence of numbers that need operators between them

Example input:

```
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
```

### Understanding the Output

The application produces a calibration report like this:

```
🌴 Phase 1: Basic Structural Analysis 🌴
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📝 Analyzing equations with standard operators...
🔢 Basic Stress Coefficient: 21572148763543

🌴 Phase 2: Advanced Structural Analysis 🌴
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🐘 Accounting for sneaky elephant operators...
🔢 Advanced Stress Coefficient: 581941094529163

✨ Bridge calibration complete! ✨
```

- **Basic Stress Coefficient**: Sum of valid test values using only + and *
  operators
- **Advanced Stress Coefficient**: Updated sum including the || concatenation
  operator

With the `--redact` flag, sensitive numbers are replaced with asterisks to
protect proprietary engineering data.

## 🧪 Testing

Verify the calibration calculations with the test suite:

```bash
cargo test
```

## 📚 Technical Details

The application employs several sophisticated techniques:

- Left-to-right operator evaluation
- Comprehensive operator combination validation
- Multi-phase stress analysis
- Advanced error detection for invalid equations
- Custom output formatting with redaction support

Each analysis carefully validates possible operator combinations while
accounting for both standard mathematical operators and the special
concatenation operator that those clever elephants tried to hide in the jungle.
The system ensures accurate bridge calibration results while maintaining a
playful narrative about the mischievous elephants who started it all.
