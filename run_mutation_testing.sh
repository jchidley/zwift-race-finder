#!/bin/bash
# ABOUTME: Run mutation testing with optimizations and proper output management
# Usage: ./run_mutation_testing.sh

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
OUTPUT_DIR="${OUTPUT_BASE}/run_${TIMESTAMP}"

echo "Creating output directory: $OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

# Archive the most recent mutants.out if it exists
if [ -d mutants.out ]; then
    echo "Archiving existing mutants.out to mutants.out.old"
    rm -rf mutants.out.old
    mv mutants.out mutants.out.old
fi

# Get number of available cores
AVAILABLE_CORES=$(nproc)
# Use 8 threads as this has been tested to work well
THREADS=8

echo "System has $AVAILABLE_CORES cores available"
echo "Running mutation testing with $THREADS parallel jobs..."

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
    echo "  Install with: sudo apt install mold"
fi

# Create mutants profile if it doesn't exist
if ! grep -q '\[profile.mutants\]' Cargo.toml; then
    echo "✓ Adding 'mutants' profile to Cargo.toml for optimized builds"
    cat >> Cargo.toml << 'EOF'

[profile.mutants]
inherits = "test"
debug = "none"
EOF
fi

# Create config file for cargo-mutants
CONFIG_FILE="${SCRIPT_DIR}/.cargo/mutants.toml"
mkdir -p "${SCRIPT_DIR}/.cargo"
cat > "$CONFIG_FILE" << EOF
# cargo-mutants configuration
test_tool = "nextest"
profile = "mutants"
minimum_test_timeout = 300
# Note: jobs parameter is not supported in config file, must use command line
EOF

echo "✓ Created cargo-mutants config at $CONFIG_FILE"
echo "✓ Using 'mutants' profile (no debug symbols for faster builds)"
echo "✓ Timeout set to 300 seconds per mutant"

# Run mutation testing in background
echo
echo "Starting mutation testing in background..."
echo "Output directory: $OUTPUT_DIR"
echo

# Use nohup to survive terminal closure
# Redirect all output to the timestamped directory
nohup cargo mutants \
    --output "$OUTPUT_DIR" \
    --jobs $THREADS \
    > "${OUTPUT_DIR}/mutation_run.log" 2>&1 &

PID=$!
echo "Mutation testing started with PID: $PID"
echo "$PID" > "${OUTPUT_DIR}/mutation.pid"

# Create a symlink to the current run
ln -sfn "$OUTPUT_DIR" "${OUTPUT_BASE}/current"

echo
echo "Configuration:"
echo "  • Output directory: $OUTPUT_DIR"
echo "  • Parallel jobs: $THREADS"
echo "  • Test tool: ${NEXTEST_FLAG:-cargo test}"
echo "  • Profile: mutants (no debug symbols)"
echo "  • Timeout: 300 seconds per mutant"
echo
echo "Monitor progress with:"
echo "  tail -f ${OUTPUT_DIR}/mutation_run.log"
echo "  ./check_mutation_progress.sh"
echo "  watch -n 10 'cat ${OUTPUT_BASE}/current/mutants.out/caught.txt | wc -l'"
echo
echo "View results:"
echo "  less ${OUTPUT_BASE}/current/mutants.out/outcomes.json"
echo "  cat ${OUTPUT_BASE}/current/mutants.out/missed.txt"
echo
echo "To stop: kill $PID"