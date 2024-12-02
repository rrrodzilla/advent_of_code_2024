# The Lost Historian's Coordinates 🗿

A mystical application built with the Acton reactive framework to help decode
ancient scrolls and find the lost Chief Historian through location analysis.

## 📜 The Story

Two historians have left behind scrolls containing coordinates of possible
locations where the Chief Historian was last seen. The senior and junior
historians worked independently, recording locations in their own scrolls. This
application helps decode their findings and analyze the correlation between
their recordings.

## 🔧 Technical Implementation

Built using the Acton reactive framework, this application demonstrates:

- Reactive agent-based architecture
- Asynchronous message processing
- Event-driven coordinate analysis
- A massively over-architected solution to the Advent of Code Day 1 Challenge

The system uses two main agents:

1. `ScrollDecoder`: Decodes the ancient scrolls and broadcasts coordinate pairs
2. `HistorianSage`: Analyzes the coordinates by:
   - Measuring geographical discrepancies between recordings
   - Analyzing location correlations between the two historians' findings

## 🚀 Running the Application

### Prerequisites

- Rust and Cargo installed
- Acton framework dependencies

### Running with Sample Scrolls

```bash
# Analyze the first scroll
cargo run -- part1.txt

# Analyze the second scroll
cargo run -- part2.txt
```

### Input Format

Each scroll (input file) should contain coordinate pairs, one per line:

```
3 4
4 3
2 5
```

### Understanding the Output

The application will produce an analysis like this:

```
🗿 Ancient Scroll Analysis Complete! 📜
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✨ Historical Location Correlation: <score>
📏 Geographical Discrepancy: <distance> leagues
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📌 The Chief Historian's findings have been recorded in the eternal archives!
```

- **Historical Location Correlation**: Indicates how many locations were
  independently verified by both historians
- **Geographical Discrepancy**: Shows the total distance between corresponding
  location pairs

## 🧪 Testing

Run the test suite to verify the location correlation algorithm:

```bash
cargo test
```

## 📚 Technical Details

The application uses Acton's reactive patterns:

- Message broadcasting for coordinate distribution
- Agent subscriptions for event handling
- Asynchronous processing for scroll decoding
- Immutable message passing between agents

Each coordinate pair represents a potential location sighting, with the senior
and junior historians' recordings being compared to find patterns that might
lead to the Chief Historian's whereabouts.
