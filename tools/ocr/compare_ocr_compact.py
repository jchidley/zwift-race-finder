#!/usr/bin/env python3
"""Compare Python (PaddleOCR) vs Rust Compact (Tesseract) OCR"""

import json
import subprocess
import sys
import time
from pathlib import Path

# Add tools/ocr to path
sys.path.insert(0, str(Path(__file__).parent))
from zwift_ocr_compact import ZwiftOCR


def main():
    # Test image
    image_path = (
        Path(__file__).parent.parent.parent / 'docs' / 'screenshots' / 'normal_1_01_16_02_21.jpg'
    )

    if not image_path.exists():
        print(f'Error: Image not found at {image_path}')
        sys.exit(1)

    print('=== Compact OCR Comparison ===')
    print(f'Image: {image_path.name}\n')

    # Python (PaddleOCR)
    print('Running Python OCR...')
    start = time.time()
    ocr = ZwiftOCR()
    py_result = ocr.extract(str(image_path))
    py_time = time.time() - start

    # Rust Compact (Tesseract)
    print('Running Rust Compact OCR...')
    binary = Path(__file__).parent.parent.parent / 'target' / 'release' / 'zwift_ocr_compact'

    if not binary.exists():
        print(f'Error: Release binary not found at {binary}')
        print('Build it with: cargo build --features ocr --bin zwift_ocr_compact --release')
        sys.exit(1)

    # Test both sequential and parallel
    print('  - Sequential mode...')
    start = time.time()
    result = subprocess.run([str(binary), str(image_path)], capture_output=True, text=True)
    rs_seq_time = time.time() - start

    if result.returncode == 0:
        rs_seq_result = json.loads(result.stdout)
    else:
        print(f'Rust sequential error: {result.stderr}')
        rs_seq_result = {}

    print('  - Parallel mode...')
    start = time.time()
    result = subprocess.run(
        [str(binary), str(image_path), '--parallel'], capture_output=True, text=True
    )
    rs_par_time = time.time() - start

    if result.returncode == 0:
        rs_result = json.loads(result.stdout)
    else:
        print(f'Rust error: {result.stderr}')
        rs_result = {}

    # Results
    print('\nPerformance:')
    print(f'  Python:         {py_time:.2f}s')
    print(f'  Rust Sequential: {rs_seq_time:.2f}s ({py_time / rs_seq_time:.1f}x faster)')
    print(f'  Rust Parallel:   {rs_par_time:.2f}s ({py_time / rs_par_time:.1f}x faster)')
    print(f'  Parallel speedup: {rs_seq_time / rs_par_time:.2f}x')

    # Compare all fields
    core_fields = ['speed', 'distance', 'altitude', 'race_time', 'power', 'cadence', 'heart_rate']
    extra_fields = ['gradient', 'distance_to_finish']
    all_fields = core_fields + extra_fields

    # Compare core and extra fields separately
    core_matches = sum(1 for f in core_fields if py_result.get(f) == rs_result.get(f))
    extra_matches = sum(1 for f in extra_fields if py_result.get(f) == rs_result.get(f))
    total_matches = core_matches + extra_matches

    print(
        f'\nCore telemetry: {core_matches}/{len(core_fields)} fields match ({core_matches / len(core_fields) * 100:.0f}%)'
    )
    print(
        f'Extra fields: {extra_matches}/{len(extra_fields)} fields match ({extra_matches / len(extra_fields) * 100:.0f}%)'
    )
    print(
        f'Total accuracy: {total_matches}/{len(all_fields)} fields match ({total_matches / len(all_fields) * 100:.0f}%)'
    )

    # Compare leaderboard
    py_lb = py_result.get('leaderboard', [])
    rs_lb = rs_result.get('leaderboard', [])
    print(
        f'\nLeaderboard: Python extracted {len(py_lb)} entries, Rust extracted {len(rs_lb)} entries'
    )

    if total_matches < len(all_fields):
        print('\nDifferences:')
        for field in all_fields:
            py_val = py_result.get(field)
            rs_val = rs_result.get(field)
            if py_val != rs_val:
                print(f'  {field}: Python={py_val}, Rust={rs_val}')

    # Show extracted data only if requested
    if '--verbose' in sys.argv:
        print('\n=== Extracted Data ===')
        print('Python:', json.dumps(py_result, indent=2))
        print('\nRust:', json.dumps(rs_result, indent=2))


if __name__ == '__main__':
    main()
