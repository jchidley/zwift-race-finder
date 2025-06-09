#!/usr/bin/env python3
"""Compare Python (PaddleOCR) vs Rust Compact (Tesseract) OCR"""
import json
import subprocess
import time
import sys
from pathlib import Path

# Add tools/ocr to path
sys.path.insert(0, str(Path(__file__).parent))
from zwift_ocr_compact import ZwiftOCR

def main():
    # Test image
    image_path = Path(__file__).parent.parent.parent / "docs" / "screenshots" / "normal_1_01_16_02_21.jpg"
    
    if not image_path.exists():
        print(f"Error: Image not found at {image_path}")
        sys.exit(1)
    
    print("=== Compact OCR Comparison ===")
    print(f"Image: {image_path.name}\n")
    
    # Python (PaddleOCR)
    print("Running Python OCR...")
    start = time.time()
    ocr = ZwiftOCR()
    py_result = ocr.extract(str(image_path))
    py_time = time.time() - start
    
    # Rust Compact (Tesseract)
    print("Running Rust Compact OCR...")
    binary = Path(__file__).parent.parent.parent / "target" / "debug" / "zwift_ocr_compact"
    
    start = time.time()
    result = subprocess.run([str(binary), str(image_path)], capture_output=True, text=True)
    rs_time = time.time() - start
    
    if result.returncode == 0:
        rs_result = json.loads(result.stdout)
    else:
        print(f"Rust error: {result.stderr}")
        rs_result = {}
    
    # Results
    print(f"\nPerformance: Python={py_time:.2f}s, Rust={rs_time:.2f}s ({py_time/rs_time:.1f}x faster)")
    
    # Compare fields
    fields = ['speed', 'distance', 'altitude', 'race_time', 'power', 'cadence', 'heart_rate']
    matches = sum(1 for f in fields if py_result.get(f) == rs_result.get(f))
    
    print(f"Accuracy: {matches}/{len(fields)} fields match ({matches/len(fields)*100:.0f}%)")
    
    if matches < len(fields):
        print("\nDifferences:")
        for field in fields:
            py_val = py_result.get(field)
            rs_val = rs_result.get(field)
            if py_val != rs_val:
                print(f"  {field}: Python={py_val}, Rust={rs_val}")

if __name__ == "__main__":
    main()