# The Red-Nosed Reactor Safety System 🦌

A highly sophisticated application built with the Acton reactive framework to
analyze safety levels in the Red-Nosed Reindeer nuclear fusion/fission plant.

![Made with VHS](https://vhs.charm.sh/vhs-5sFweLkJxWGHS8O0NoTkoY.gif)

## 🎄 The Story

While searching for the Chief Historian, the investigation team stumbled upon
the Red-Nosed Reindeer nuclear facility. The engineers there, still reminiscing
about Rudolph's molecular reconstruction, requested help analyzing unusual
reactor data. This application helps process their safety reports and determine
which readings indicate safe operating conditions.

## 🔬 Technical Implementation

Built using the Acton reactive framework, this application demonstrates:

- Reactive agent-based architecture
- Asynchronous message processing
- Event-driven safety analysis
- A massively over-architected solution to the Advent of Code Day 2 Challenge

The system uses two main agents:

1. `ReactorScanner`: Processes raw reactor readings and broadcasts level reports
2. `ReactorSafetyAnalyzer`: Analyzes the safety of each report by:
   - Verifying consistent level progression (increasing or decreasing)
   - Validating adjacent level differences (1-3 units)
   - Applying the Problem Dampener to handle single anomalies

## ⚛️ Running the Application

### Prerequisites

- Rust and Cargo installed
- Acton framework dependencies

### Running with Sample Data

```bash
# Analyze reactor readings
cargo run -- part2.txt
```

### Input Format

Each reading should contain space-separated reactor levels, one report per line:

```
7 6 4 2 1
1 2 7 8 9
8 6 4 4 1
```

### Understanding the Output

The application will produce an analysis like this:

```
🦌 Red-Nosed Reactor Safety Analysis Complete! 🎄
================================================
🔬 Total Reports Analyzed: <count>
✅ Safe Reports Detected: <safe_count>
🛠️ Problem Dampener Active
================================================
```

- **Total Reports Analyzed**: Number of reactor level reports processed
- **Safe Reports Detected**: Number of reports meeting safety criteria
- **Problem Dampener**: Indicates active single-anomaly correction

## 🧪 Testing

Run the test suite to verify the safety analysis algorithms:

```bash
cargo test
```

## 🎯 Technical Details

The application uses Acton's reactive patterns:

- Message broadcasting for reactor readings
- Agent subscriptions for event handling
- Asynchronous processing for safety analysis
- Immutable message passing between agents

Safety criteria for reactor levels:

- Must be consistently increasing or decreasing
- Adjacent levels must differ by 1-3 units
- Problem Dampener can correct one anomalous reading

The Problem Dampener module represents cutting-edge reactor safety technology,
allowing the system to tolerate a single aberrant reading in an otherwise safe
sequence. This innovative approach significantly improves the reactor's
operational flexibility while maintaining strict safety standards.

## 🎅 Integration with Chief Historian Search

While primarily focused on reactor safety, this system was discovered during the
ongoing search for the Chief Historian. Despite not finding any direct evidence
of the Chief Historian's presence at the facility, the engineering team's
cooperation has provided valuable assistance to the search effort.
