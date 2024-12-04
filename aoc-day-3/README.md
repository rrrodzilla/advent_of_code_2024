# North Pole Computer Memory Diagnostics System 🎅

A zero-heap-allocation memory analysis tool for diagnosing corrupted computer
systems at the North Pole Toboggan Rental Shop.

![Made with VHS](https://vhs.charm.sh/vhs-7jT5oJWHALmukruWU0TWeq.gif)

## ❄️ The Story

During the search for the Chief Historian, the investigation team visited the
North Pole Toboggan Rental Shop. While the shopkeeper was unable to confirm
their inventory status due to computer issues, they requested assistance in
diagnosing their malfunctioning system. This application helps analyze the
corrupted memory and validate computational instructions.

## 🔍 Technical Implementation

Built using a zero-heap-allocation architecture, this application demonstrates:

- State machine-based parsing
- Memory-efficient data processing
- Corruption-resistant validation
- A massively over-engineered solution to the Advent of Code Memory Diagnostics
  Challenge

The system uses two primary components:

1. `MemoryParser`: Scans corrupted memory for valid instructions
2. `ValidMemoryValue`: Ensures memory values are within specification by:
   - Validating 3-digit number constraints
   - Processing multiplication operations
   - Handling do()/don't() execution controls

## 💾 Running the Application

### Prerequisites

- Rust and Cargo installed
- A corrupted memory dump to analyze

### Running Memory Analysis

```bash
# Analyze corrupted memory
cargo run --quiet --release -- memory_dump.txt
```

### Input Format

The memory dump contains corrupted data with valid multiplication instructions:

```
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]
mul(11,8)do()?mul(8,5)
```

### Understanding the Output

The application will produce a diagnostic report like this:

```
🎅 North Pole Memory Analysis Complete! 🎄
==========================================
🔍 Corrupted Memory Scan Results:
✨ Total Checksum: <value>
==========================================
```

- **Total Checksum**: Verification sum of all valid multiplication operations
- **Memory Constraints**: Values limited to 3 digits (0-999)
- **Operation Control**: Tracks do()/don't() instruction enabling

## 🧪 Testing

Run the diagnostic test suite:

```bash
cargo test
```

## 🎯 Technical Details

The application uses several memory-efficient techniques:

- Zero-heap memory allocation design
- State machine instruction parsing
- Efficient numeric validation
- Contextual operation control

Memory validation criteria:

- Numbers must be 1-3 digits (0-999)
- Valid multiplication format: mul(X,Y)
- Proper operation control via do()/don't()

The operation control system represents the computer's advanced execution
management, allowing specific regions of corrupted memory to be safely ignored
while maintaining computational integrity. This innovative approach
significantly improves the system's resilience to corruption while ensuring
accurate results.

## 🎄 Integration with Chief Historian Search

While primarily focused on computer diagnostics, this system was developed
during the ongoing search for the Chief Historian. Although the shopkeeper
couldn't confirm their inventory status, fixing their computer system represents
another step forward in the investigation, potentially unlocking more clues
about the Chief Historian's whereabouts.
