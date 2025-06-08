#!/usr/bin/env python3
"""Find correct UI coordinates by manual inspection"""

import cv2
import sys
from paddleocr import PaddleOCR

def find_text_regions(image_path):
    """Use OCR to find all text in the image and their locations"""
    
    # Initialize OCR
    ocr = PaddleOCR(use_angle_cls=True, lang='en', show_log=False)
    
    # Load image
    image = cv2.imread(image_path)
    if image is None:
        print(f"Failed to load image: {image_path}")
        return
    
    # Run OCR on full image
    result = ocr.ocr(image, cls=True)
    
    # Create visualization
    vis_image = image.copy()
    
    print("Found text regions:")
    print("-" * 80)
    
    if result and result[0]:
        for line in result[0]:
            coords = line[0]
            text = line[1][0]
            confidence = line[1][1]
            
            # Get bounding box
            x_coords = [p[0] for p in coords]
            y_coords = [p[1] for p in coords]
            x_min, x_max = int(min(x_coords)), int(max(x_coords))
            y_min, y_max = int(min(y_coords)), int(max(y_coords))
            
            # Draw rectangle
            cv2.rectangle(vis_image, (x_min, y_min), (x_max, y_max), (0, 255, 0), 2)
            
            # Print info
            print(f"Text: '{text}' | Confidence: {confidence:.2f}")
            print(f"  Location: ({x_min}, {y_min}) -> ({x_max}, {y_max})")
            print(f"  Size: {x_max - x_min} x {y_max - y_min}")
            print()
    
    # Save visualization
    cv2.imwrite("debug_all_text_regions.jpg", vis_image)
    print(f"\nSaved visualization to: debug_all_text_regions.jpg")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        find_text_regions(sys.argv[1])
    else:
        print("Usage: python find_text_regions.py <image_path>")