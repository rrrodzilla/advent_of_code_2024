# X-MAS Grid Cross Pattern Analyzer 🎄

An efficient solution for analyzing grid patterns to identify instances of
"XMAS" and their cross formations in a structured grid. This program is built to
efficiently search and analyze recurring patterns, providing clear diagnostic
output to better understand complex grid arrangements.

![Made with VHS](https://vhs.charm.sh/vhs-4kukaYgn7VHYXKSb3jd4VC.gif)

## ❄️ The Story

The Elf has been working hard at the North Pole, but some mysterious patterns
have appeared across the holiday grid. To help with festive analysis, the Elf
requested a way to find and count occurrences of the word "XMAS" and its more
intricate cross patterns. This tool provides the necessary insights to uncover
and count these magical formations.

## 🔍 Technical Implementation

The X-MAS Grid Cross Pattern Analyzer demonstrates:

- Optimized grid parsing for detecting target words.
- Pattern matching in multiple directions.
- Efficient cross-analysis for intricate formations involving the word "XMAS".
- Use of SIMD (Single Instruction, Multiple Data) to accelerate grid searches,
  leveraging parallel processing capabilities for increased performance.

The system uses two main analytical components:

1. **`find_word_xmas`**: Detects occurrences of the word "XMAS" in all possible
   directions in the grid.
2. **`find_crossed_mas`**: Searches for more complex cross formations of the
   word "XMAS", providing insight into overlapping and intersecting patterns.

## 💾 Running the Application

### Prerequisites

- Rust and Cargo installed.
- A text representation of the grid to be analyzed.

### Running Pattern Analysis

```bash
# Analyze the grid for X-MAS patterns
cargo run --quiet --release -- grid_input.txt
```

### Input Format

The grid should be a text-based input with rows representing grid lines. For
example:

```
..X...
.SAMX.
.A..A.
XMAS.S
.X....
```

### Understanding the Output

The application will produce an analysis report like this:

```
✨ Magical Word Analysis: 'XMAS' Found!
🔍 Total 'XMAS' Instances Found: 4
--------------------------------------------------
🧩 Cross Pattern Analysis: Searching for 'X-MAS' Crosses...
📌 Total 'X-MAS' Cross Patterns Found: 2
```

- **Total 'XMAS' Instances Found**: Count of direct matches for the word "XMAS"
  in the grid.
- **Total 'X-MAS' Cross Patterns Found**: Count of complex cross formations
  involving overlapping or intersecting instances of "XMAS".

## 🧪 Testing

Run the test suite to verify functionality:

```bash
cargo test
```

## 🎯 Technical Details

The application uses efficient pattern matching and grid navigation:

- **Grid-based Search**: Checks all possible directions (up, down, left, right,
  and diagonals).
- **Cross Pattern Detection**: Analyzes for overlapping occurrences to find
  intricate cross shapes.

This approach ensures that the Elf can accurately locate both straightforward
and more complex festive patterns, contributing to the orderly celebrations at
the North Pole.

## 🎄 Integration with Elf Operations

This grid analysis tool is a part of the broader festive preparation efforts. By
accurately identifying "XMAS" patterns, the Elf can maintain the decorative grid
and avoid any unexpected surprises that might disrupt the holiday spirit.
