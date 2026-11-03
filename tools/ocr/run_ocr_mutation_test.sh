#!/bin/bash

set -euo pipefail

echo "Running mutation testing on OCR module..."
echo "This will take several minutes due to compilation."
echo ""

rm -f mutation_ocr_*.log

cargo mutants \
    --features ocr \
    --file src/ocr_compact.rs \
    --timeout 20 \
    --in-place \
    -- --lib \
    2>&1 | tee mutation_ocr_results.log

echo ""
echo "=== Summary ==="
grep -E "(MISSED|caught|ok)" mutation_ocr_results.log | tail -20

echo ""
echo "Full results saved to: mutation_ocr_results.log"