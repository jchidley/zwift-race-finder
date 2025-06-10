#!/bin/bash
# ABOUTME: Run mutation testing on OCR module with optimized settings
# Focuses on OCR-specific code to avoid long compilation times

set -euo pipefail

echo "Running mutation testing on OCR module..."
echo "This will take several minutes due to compilation."
echo ""

# Clean any previous runs
rm -f mutation_ocr_*.log

# Run mutation testing with reasonable timeout
# --in-place: Faster, modifies source directly
# --timeout 20: 20 second timeout per mutation
# -- --lib: Only run library tests, skip integration tests
cargo mutants \
    --features ocr \
    --file src/ocr_compact.rs \
    --timeout 20 \
    --in-place \
    -- --lib \
    2>&1 | tee mutation_ocr_results.log

# Extract summary
echo ""
echo "=== Summary ==="
grep -E "(MISSED|caught|ok)" mutation_ocr_results.log | tail -20

echo ""
echo "Full results saved to: mutation_ocr_results.log"