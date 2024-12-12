# The Plutonian Stone Transformation Simulator ✨🌌

A highly sophisticated analysis system for observing and predicting the behavior
of ancient Plutonian artifacts, powered by recursively optimized transformation
algorithms.

![Made with VHS](https://vhs.charm.sh/vhs-3mivZoiZh4bxybNfKCFAhz.gif)

## 🪨 The Story

Deep in the infinite corridors of Pluto's ancient civilization, you've
discovered a mesmerizing line of numbered stones that transform every time you
blink. This application helps track and predict these cosmic transformations,
revealing the mathematical patterns hidden within these mysterious artifacts.

## 🛸 Technical Implementation

This straightforward implementation demonstrates:

- Simple recursive pattern matching for transformation rules
- Basic memoization using a HashMap to avoid redundant calculations
- String manipulation for splitting and combining stone inscriptions
- Multiplication by the Plutonian constant (2024)

The core logic consists of:

1. A recursive function that applies the transformation rules
2. A cache to remember previously calculated results
3. Helper functions for string manipulation and arithmetic

## 🌠 Running the Application

### Prerequisites

- Rust and Cargo installed
- A steady blinking rhythm

### Running with Sample Data

```bash
# Standard stone observation
cargo run < stones.txt
```

### Input Format

The input represents a sequence of numbered stones, where each number is a
string of digits representing the inscription on each stone. Example input:

```
0 1 10 99 999
```

### Understanding the Output

The application produces a cosmic analysis report like this:

```
✨ 🌌 Cosmic Phase 1 (after 25 blinks): 220722
✨ 🌌 Cosmic Phase 2 (after 75 blinks): 261952051690787
```

- **Phase 1**: Number of stones after 25 blinks
- **Phase 2**: Number of stones after 75 blinks

## 💫 Testing

Verify the stone transformations with the test suite:

```bash
cargo test
```

## 🔮 Technical Details

The application follows three fundamental transformation rules:

1. Stones marked '0' transform to '1'
2. Stones with even-length inscriptions split into two new stones
3. Other stones multiply their inscription by the Plutonian constant (2024)

The implementation features:

- Memoization to cache repeated calculations
- Simple string operations for splitting numbers
- Basic handling of leading zeros
- Multiplication by 2024 for odd-length numbers
- Maintenance of stone order through transformations

Each transformation carefully maintains the order of stones while tracking their
evolution through multiple blinks, revealing the mathematical beauty of these
ancient Plutonian artifacts.
