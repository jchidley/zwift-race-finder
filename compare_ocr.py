#!/usr/bin/env python3
"""Compare OCR performance between ocrs and Tesseract"""

import subprocess
import time
import json
import statistics

def run_ocr_command(cmd, runs=5):
    """Run OCR command multiple times and measure performance"""
    times = []
    outputs = []
    
    for i in range(runs):
        start = time.time()
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
        elapsed = time.time() - start
        times.append(elapsed)
        outputs.append(result.stdout)
        print(f"  Run {i+1}: {elapsed:.3f}s")
    
    return {
        'times': times,
        'mean': statistics.mean(times),
        'median': statistics.median(times),
        'stdev': statistics.stdev(times) if len(times) > 1 else 0,
        'output': outputs[0] if outputs else ""
    }

def main():
    test_image = "docs/screenshots/normal_1_01_16_02_21.jpg"
    
    print("Performance Comparison: ocrs vs Tesseract\n")
    print("=" * 50)
    
    # Test ocrs CLI
    print("\nocrs CLI:")
    ocrs_cli_results = run_ocr_command(f"ocrs {test_image}")
    
    # Test ocrs library (Rust)
    print("\nocrs library (Rust):")
    ocrs_lib_results = run_ocr_command(f"./target/release/zwift_ocr_ocrs {test_image}")
    
    # Test Tesseract (Rust)
    print("\nTesseract (Rust):")
    tesseract_results = run_ocr_command(f"./target/release/zwift_ocr_compact {test_image}")
    
    # Summary
    print("\n" + "=" * 50)
    print("SUMMARY (Release Build)\n")
    print(f"ocrs CLI:        {ocrs_cli_results['mean']:.3f}s ± {ocrs_cli_results['stdev']:.3f}s")
    print(f"ocrs library:    {ocrs_lib_results['mean']:.3f}s ± {ocrs_lib_results['stdev']:.3f}s")
    print(f"Tesseract:       {tesseract_results['mean']:.3f}s ± {tesseract_results['stdev']:.3f}s")
    print(f"\nSpeedup factor:")
    print(f"ocrs vs Tesseract: {tesseract_results['mean'] / ocrs_lib_results['mean']:.1f}x faster")
    
    # Output quality check
    print("\n" + "=" * 50)
    print("OUTPUT QUALITY CHECK\n")
    
    print("Tesseract output (structured):")
    try:
        tess_data = json.loads(tesseract_results['output'])
        print(json.dumps(tess_data, indent=2))
    except:
        print("Failed to parse Tesseract output")
    
    print("\nocrs output (first 200 chars):")
    print(ocrs_lib_results['output'][:200] + "..." if len(ocrs_lib_results['output']) > 200 else ocrs_lib_results['output'])

if __name__ == "__main__":
    main()