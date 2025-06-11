#!/bin/bash
# ABOUTME: Run mutation testing specifically on OCR modules
# Usage: ./run_ocr_mutation_testing.sh

set -euo pipefail
IFS=$'\n\t'

# Script directory for relative paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Error handling
die() { echo "ERROR: $*" >&2; exit 1; }
warn() { echo "WARNING: $*" >&2; }

# Create timestamped output directory
OUTPUT_BASE="${SCRIPT_DIR}/mutation_results"
mkdir -p "$OUTPUT_BASE"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
OUTPUT_DIR="${OUTPUT_BASE}/ocr_run_${TIMESTAMP}"

echo "Creating OCR mutation output directory: $OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

# Get number of available cores
AVAILABLE_CORES=$(nproc)
# Use 8 threads as tested to work well
THREADS=8

echo "System has $AVAILABLE_CORES cores available"
echo "Running OCR mutation testing with $THREADS parallel jobs..."

# Check if nextest is available
if command -v cargo-nextest &> /dev/null; then
    echo "✓ Using cargo-nextest for faster test execution"
    NEXTEST_FLAG="--test-tool=nextest"
else
    echo "⚠ cargo-nextest not found, using standard cargo test"
    echo "  Install with: cargo install cargo-nextest"
    NEXTEST_FLAG=""
fi

# Check if mold is available
if command -v mold &> /dev/null; then
    export RUSTFLAGS="-Clink-arg=-fuse-ld=mold"
    echo "✓ Using mold linker for faster builds"
else
    warn "mold linker not found, using default linker"
fi

echo
echo "Starting OCR mutation testing in background..."
echo "Targeting OCR modules only (estimated 30-60 minutes)"
echo

# Run mutation testing ONLY on OCR files
# Note: Using explicit file list to avoid testing binary files
nohup cargo mutants \
    --file src/ocr_compact.rs \
    --file src/ocr_constants.rs \
    --file src/ocr_image_processing.rs \
    --file src/ocr_ocrs.rs \
    --file src/ocr_parallel.rs \
    --file src/ocr_regex.rs \
    --output "$OUTPUT_DIR" \
    --jobs $THREADS \
    --timeout 180 \
    $NEXTEST_FLAG \
    > "${OUTPUT_DIR}/mutation_run.log" 2>&1 &

PID=$!
echo "OCR Mutation testing started with PID: $PID"
echo "$PID" > "${OUTPUT_DIR}/mutation.pid"

# Create a symlink to the current OCR run
ln -sfn "$OUTPUT_DIR" "${OUTPUT_BASE}/ocr_current"

echo
echo "Configuration:"
echo "  • Output directory: $OUTPUT_DIR"
echo "  • Target files: OCR modules only (6 files, ~1065 lines)"
echo "  • Parallel jobs: $THREADS"
echo "  • Test tool: ${NEXTEST_FLAG:-cargo test}"
echo "  • Timeout: 180 seconds per mutant"
echo "  • Estimated time: 30-60 minutes"
echo
echo "Monitor progress with:"
echo "  tail -f ${OUTPUT_DIR}/mutation_run.log"
echo "  ./check_mutation_progress.sh"
echo "  watch -n 30 'grep -c \"MISSED\\|CAUGHT\" ${OUTPUT_DIR}/mutation_run.log || echo 0'"
echo
echo "View results when complete:"
echo "  cat ${OUTPUT_BASE}/ocr_current/mutants.out/missed.txt"
echo "  cat ${OUTPUT_BASE}/ocr_current/mutants.out/caught.txt"
echo "  jq '.summary' ${OUTPUT_BASE}/ocr_current/mutants.out/outcomes.json"
echo
echo "To stop: kill $PID"
echo
echo "Starting monitoring loop..."
echo "Initial check in 30 seconds..."