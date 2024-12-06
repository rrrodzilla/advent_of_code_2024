# The Enchanted North Pole Printer 🖨️

A mystical application for decoding and validating the sacred printing order of
the North Pole's sleigh safety manual updates, powered by magical topological
sorting algorithms.

![Made with VHS](https://vhs.charm.sh/vhs-6QuWmu4QC4Ixda8w0dnEvr.gif)

## 📜 The Story

Deep in sub-basement 17 of the North Pole printing department, an enchanted
printer must produce safety manual updates in a very specific order. The printer
follows sacred rules, where each page must be materialized in accordance with
ancient printing lore. This application helps the Elves decode these mystical
rules and ensure the safety manuals are printed correctly.

## 🔧 Technical Implementation

Built using Rust's powerful type system, this application demonstrates:

- Custom Display trait implementation with formatter flags
- Topological sorting for page order validation
- Dependency graph management
- A massively over-architected solution to the Advent of Code Day 5 Challenge

The system uses two main components:

1. `PrinterLore`: Decodes and maintains the sacred rules of page ordering
2. `EnchantedOrderKeeper`: Ensures proper page sequencing by:
   - Validating existing update orders
   - Reordering pages according to the sacred rules
   - Computing mystical middle page numbers

## 🚀 Running the Application

### Prerequisites

- Rust and Cargo installed
- A sense of holiday spirit

### Running with Sample Updates

```bash
# Normal analysis
cargo run < input.txt

# Redacted analysis (for security)
cargo run --redact < input.txt
```

### Input Format

Input files should contain two sections separated by a blank line:

1. Sacred printing rules (one per line):

```
47|53
97|13
75|29
```

2. Update sequences (one per line):

```
75,47,61,53,29
97,61,53,29,13
```

### Understanding the Output

The application will produce a magical diagnosis like this:

```
🖨️  Ancient Printer Diagnosis Complete! 📜
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✨ Properly Ordered Updates Sum: 143
📝 Reordered Updates Sum: 123
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

- **Properly Ordered Updates Sum**: The sum of middle page numbers from
  correctly sequenced updates
- **Reordered Updates Sum**: The sum of middle page numbers after fixing
  incorrectly sequenced updates

With the `--redact` flag, sensitive numbers are replaced with asterisks for
security.

## 🧪 Testing

Run the test suite to verify the magical ordering algorithms:

```bash
cargo test
```

## 📚 Technical Details

The application uses several advanced Rust features:

- Custom Display implementation with formatter flags for redaction
- Graph-based dependency management
- Efficient topological sorting
- HashMap and HashSet for fast lookup and validation

Each update sequence is validated against the sacred printing rules, ensuring
that the safety manual pages materialize in the correct order to maintain the
integrity of North Pole sleigh operations.
