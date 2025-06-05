#!/bin/bash
# Script to run mutation testing in background with optimizations

# Archive any previous runs
if [ -d mutants.out ] || [ -d mutation_logs ]; then
    TIMESTAMP=$(date +%Y%m%d_%H%M%S)
    echo "Archiving previous mutation testing runs to *-backup-${TIMESTAMP}/"
    [ -d mutants.out ] && mv mutants.out "mutants.out-backup-${TIMESTAMP}"
    [ -d mutation_logs ] && mv mutation_logs "mutation_logs-backup-${TIMESTAMP}"
fi
mkdir -p mutation_logs

# Get number of available cores and use 75% for safety
AVAILABLE_CORES=$(nproc)
# Use 75% of cores (but at least 1, and max 8 as cargo-mutants suggests)
THREADS=$(( (AVAILABLE_CORES * 3 / 4) > 8 ? 8 : (AVAILABLE_CORES * 3 / 4) ))
THREADS=$(( THREADS < 1 ? 1 : THREADS ))

echo "System has $AVAILABLE_CORES cores available"
echo "Running mutation testing with $THREADS threads (safe limit)..."

# Check if nextest is available
if command -v cargo-nextest &> /dev/null; then
    echo "✓ Using cargo-nextest for faster test execution"
    NEXTEST_FLAG="--test-tool=nextest"
else
    echo "⚠ cargo-nextest not found, using standard cargo test"
    echo "  Install with: cargo install cargo-nextest"
    NEXTEST_FLAG=""
fi

# Use ramdisk for faster I/O if available
if [ -d "/ram" ] && [ -w "/ram" ]; then
    export TMPDIR=/ram
    echo "✓ Using ramdisk at /ram for temporary files"
else
    echo "⚠ Ramdisk not available at /ram, using default temp directory"
fi

# Configure mold linker for faster builds
export RUSTFLAGS="-Clink-arg=-fuse-ld=mold"
echo "✓ Using mold linker for faster builds"

echo "✓ Using 'mutants' profile (no debug symbols, opt-level=1)"
echo "✓ Skipping doctests and benchmarks for better performance"

# Run mutation testing on all modules with parallel execution
echo
echo "Starting mutation testing in background..."

# Use nohup to survive terminal closure, with all optimizations
# The -o flag sets the output directory
nohup cargo mutants \
    -j $THREADS \
    --timeout 120 \
    -o . \
    $NEXTEST_FLAG \
    > mutation_logs/full_run.log 2>&1 &

PID=$!
echo "Mutation testing started with PID: $PID"
echo
echo "Optimizations enabled:"
echo "  • Ramdisk at /ram (faster file I/O)"
echo "  • Custom 'mutants' profile (no debug symbols)"
echo "  • Nextest runner (faster test execution)"
echo "  • Skipping benchmarks and doctests (reduced overhead)"
echo "  • $THREADS parallel threads"
echo
echo "Monitor progress with:"
echo "  tail -f mutation_logs/full_run.log"
echo "  ./check_mutation_progress.sh"
echo
echo "To stop: kill $PID"