#!/usr/bin/env python3
"""Debug OCR extraction to see what's happening"""

import cv2
import numpy as np
from paddleocr import PaddleOCR
import sys

def debug_extract(image_path):
    """Debug OCR extraction step by step"""
    
    # Initialize OCR
    ocr = PaddleOCR(use_angle_cls=True, lang='en', show_log=False)
    
    # Load image
    image = cv2.imread(image_path)
    if image is None:
        print(f"Failed to load image: {image_path}")
        return
    
    print(f"Image shape: {image.shape}")
    
    # Define some key regions to test
    regions = {
        'speed': (520, 35, 90, 50),
        'power': (200, 40, 90, 50),
        'gradient': (1300, 30, 80, 60),
        'race_time': (820, 35, 100, 50),
    }
    
    for name, (x, y, w, h) in regions.items():
        print(f"\n--- Extracting {name} from ({x}, {y}, {w}, {h}) ---")
        
        # Extract region
        roi = image[y:y+h, x:x+w]
        
        # Preprocess
        gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY)
        _, binary = cv2.threshold(gray, 180, 255, cv2.THRESH_BINARY)
        
        # Save for inspection
        cv2.imwrite(f"debug_{name}_raw.jpg", roi)
        cv2.imwrite(f"debug_{name}_binary.jpg", binary)
        
        # OCR on different versions
        result_raw = ocr.ocr(roi, cls=True)
        result_binary = ocr.ocr(binary, cls=True)
        
        print(f"Raw OCR: {result_raw}")
        print(f"Binary OCR: {result_binary}")
        
        # Extract text
        if result_raw and result_raw[0]:
            texts = [line[1][0] for line in result_raw[0]]
            print(f"Raw text: {' '.join(texts)}")
        
        if result_binary and result_binary[0]:
            texts = [line[1][0] for line in result_binary[0]]
            print(f"Binary text: {' '.join(texts)}")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        debug_extract(sys.argv[1])
    else:
        print("Usage: python debug_ocr.py <image_path>")