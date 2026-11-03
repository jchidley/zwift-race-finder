#!/usr/bin/env bash

set -euo pipefail

BINARY="target/release/zwift_ocr_compact"
if [[ ! -f "$BINARY" ]]; then
    echo "Building release binary..."
    cargo build --features ocr --bin zwift_ocr_compact --release
fi

IMAGE="docs/screenshots/normal_1_01_16_02_21.jpg"
if [[ ! -f "$IMAGE" ]]; then
    echo "Error: Test image not found at $IMAGE"
    exit 1
fi

echo "=== OCR Performance Diagnosis ==="
echo "Binary: $BINARY"
echo "Image: $IMAGE"
echo ""

echo "1. Cold Start Performance (single run)"
echo "   Sequential:"
time "$BINARY" "$IMAGE" > /dev/null 2>&1
echo ""
echo "   Parallel:"
time "$BINARY" "$IMAGE" --parallel > /dev/null 2>&1
echo ""

echo "2. Warm Performance (5 runs average)"

echo "   Sequential:"
SEQ_TOTAL=0
for i in {1..5}; do
    START=$(date +%s.%N)
    "$BINARY" "$IMAGE" > /dev/null 2>&1
    END=$(date +%s.%N)
    DURATION=$(echo "$END - $START" | bc)
    SEQ_TOTAL=$(echo "$SEQ_TOTAL + $DURATION" | bc)
    echo "     Run $i: ${DURATION}s"
done
SEQ_AVG=$(echo "scale=3; $SEQ_TOTAL / 5" | bc)
echo "     Average: ${SEQ_AVG}s"
echo ""

echo "   Parallel:"
PAR_TOTAL=0
for i in {1..5}; do
    START=$(date +%s.%N)
    "$BINARY" "$IMAGE" --parallel > /dev/null 2>&1
    END=$(date +%s.%N)
    DURATION=$(echo "$END - $START" | bc)
    PAR_TOTAL=$(echo "$PAR_TOTAL + $DURATION" | bc)
    echo "     Run $i: ${DURATION}s"
done
PAR_AVG=$(echo "scale=3; $PAR_TOTAL / 5" | bc)
echo "     Average: ${PAR_AVG}s"
echo ""

SPEEDUP=$(echo "scale=2; $SEQ_AVG / $PAR_AVG" | bc)
echo "3. Parallel Speedup: ${SPEEDUP}x"

echo ""
echo "4. Accuracy Check"
echo "   Sequential output:"
"$BINARY" "$IMAGE" 2>/dev/null | jq -r 'to_entries | map(select(.value != null)) | length' | xargs echo "     Fields extracted:"
echo "   Parallel output:"
"$BINARY" "$IMAGE" --parallel 2>/dev/null | jq -r 'to_entries | map(select(.value != null)) | length' | xargs echo "     Fields extracted:"

echo ""
echo "5. Detailed Time Profile"
echo "   Sequential:"
/usr/bin/time -v "$BINARY" "$IMAGE" > /dev/null 2>&1
echo ""
echo "   Parallel:"
/usr/bin/time -v "$BINARY" "$IMAGE" --parallel > /dev/null 2>&1