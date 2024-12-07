# The Time-Traveling Guard Patrol Simulator 🕰️

A highly optimized temporal analysis system for predicting historical guard
patrol patterns in the North Pole's 1518 prototype suit manufacturing lab,
powered by quantum-efficient path prediction algorithms.

![Made with VHS](https://vhs.charm.sh/vhs-2klXZ8q0E3qDmIzMBPBJMw.gif)

## 📜 The Story

Using The Historians' temporal displacement device, you find yourself in the
North Pole's prototype suit manufacturing lab circa 1518. The mission: locate
the missing Chief Historian while avoiding detection by the lab's guard. This
application helps analyze the guard's patrol patterns and identify potential
temporal paradox points that could be used to safely search the facility.

## 🔧 Technical Implementation

Built using Rust's zero-cost abstractions, this application demonstrates:

- Bit-packed state representation for maximum efficiency
- Parallel processing of temporal calculations using Rayon
- Custom Display trait implementation with security redaction
- A massively over-engineered solution to the Advent of Code Day 6 Challenge

The system uses three main components:

1. `TemporalLab`: Quantum-efficient representation of the historical lab space
2. `ParadoxTracker`: Tracks guard movements through spacetime using bit
   operations
3. `TimeSlice`: Represents discrete moments in the historical timeline

## 🚀 Running the Application

### Prerequisites

- Rust and Cargo installed
- A firm grasp on temporal mechanics

### Running with Sample Data

```bash
# Standard temporal analysis
cargo run < input.txt

# Classified analysis (for temporal security)
cargo run --redact < input.txt
```

### Input Format

The input represents a temporal map of the 1518 lab, where:

- `^` marks the guard's starting position and direction
- `#` indicates historical obstacles (crates, desks, etc.)
- `.` represents traversable space

Example input:

```
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
```

### Understanding the Output

The application produces a temporal analysis report like this:

```
🕰️ Temporal Analysis Complete!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
👮‍♂️ Guard's Patrol Coverage: 41 unique temporal coordinates

🌀 Paradox Analysis Complete!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
⚡️ Viable Paradox Points: 6 coordinates
🏷️ Time Stream Analysis Concluded
```

- **Patrol Coverage**: Number of distinct positions the guard will visit
- **Paradox Points**: Number of locations where a new obstacle would create a
  time loop

With the `--redact` flag, sensitive numbers are replaced with asterisks to
prevent temporal knowledge contamination.

## 🧪 Testing

Verify the temporal calculations with the test suite:

```bash
cargo test
```

## 📚 Technical Details

The application leverages several advanced optimization techniques:

- Bit-packed grid representation for minimal memory usage
- SIMD-friendly data structures
- Cache-optimized coordinate handling
- Parallel processing of paradox calculations
- Custom Display implementation with redaction support

Each analysis carefully maintains temporal consistency while identifying safe
search locations for The Historians to locate their missing Chief without
creating dangerous temporal paradoxes.
