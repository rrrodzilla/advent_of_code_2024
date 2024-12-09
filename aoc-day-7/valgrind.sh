#!/bin/bash

# Build release binary
cargo build --release

# Run massif (heap profiler)
valgrind --tool=massif --massif-out-file=massif.out ./target/release/bridge_calibrator <input.txt

# Run with cachegrind (cache profiler)
valgrind --tool=cachegrind --cachegrind-out-file=cachegrind.out ./target/release/bridge_calibrator <input.txt

# Run basic memory profiling
valgrind --leak-check=full --show-leak-kinds=all ./target/release/bridge_calibrator <input.txt

# Generate report
ms_print massif.out >massif_report.txt
cg_annotate cachegrind.out >cachegrind_report.txt
