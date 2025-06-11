#!/usr/bin/env python3
"""Test PaddleOCR import performance"""
import time
import sys

print("Testing PaddleOCR performance...")

# Test 1: Import time
start = time.time()
from paddleocr import PaddleOCR
import_time = time.time() - start
print(f"Import time: {import_time:.2f}s")

# Test 2: Instance creation time
start = time.time()
ocr = PaddleOCR(use_angle_cls=True, lang="en", show_log=False)
creation_time = time.time() - start
print(f"Instance creation time: {creation_time:.2f}s")

# Test 3: Actual OCR time
img_path = "../../docs/screenshots/normal_1_01_16_02_21.jpg"
start = time.time()
result = ocr.ocr(img_path, cls=True)
ocr_time = time.time() - start
print(f"OCR processing time: {ocr_time:.2f}s")

print(f"\nTotal time: {import_time + creation_time + ocr_time:.2f}s")
print(f"Overhead (import + creation): {import_time + creation_time:.2f}s")