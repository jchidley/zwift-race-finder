#!/usr/bin/env python3
"""Find leaderboard location in Zwift screenshot"""

import cv2
from paddleocr import PaddleOCR
import sys

def find_leaderboard(image_path):
    """Find all text that looks like leaderboard entries"""
    
    ocr = PaddleOCR(use_angle_cls=True, lang='en', show_log=False)
    img = cv2.imread(image_path)
    
    # Run OCR on full image
    result = ocr.ocr(img, cls=True)
    
    leaderboard_entries = []
    
    if result and result[0]:
        for line in result[0]:
            text = line[1][0]
            coords = line[0]
            
            # Get bounding box
            x_coords = [p[0] for p in coords]
            y_coords = [p[1] for p in coords]
            x_min, x_max = int(min(x_coords)), int(max(x_coords))
            y_min, y_max = int(min(y_coords)), int(max(y_coords))
            
            # Look for patterns that match leaderboard entries
            # Pattern: name, w/kg, distance
            if 'w/kg' in text or 'wkg' in text.lower():
                leaderboard_entries.append({
                    'text': text,
                    'x': x_min,
                    'y': y_min,
                    'width': x_max - x_min,
                    'height': y_max - y_min
                })
                print(f"Leaderboard entry at ({x_min}, {y_min}): {text}")
            
            # Also look for rider names with dots (like J.Chidley)
            elif '.' in text and any(c.isupper() for c in text):
                print(f"Possible name at ({x_min}, {y_min}): {text}")
            
            # Look for time gaps
            elif text.startswith('+') and ':' in text:
                print(f"Time gap at ({x_min}, {y_min}): {text}")
            
            # Look for distances
            elif 'km' in text.lower():
                print(f"Distance at ({x_min}, {y_min}): {text}")
    
    # Create visualization
    vis_img = img.copy()
    for entry in leaderboard_entries:
        cv2.rectangle(vis_img, 
                     (entry['x'], entry['y']),
                     (entry['x'] + entry['width'], entry['y'] + entry['height']),
                     (0, 255, 0), 2)
    
    cv2.imwrite("debug_leaderboard_regions.jpg", vis_img)
    print(f"\nVisualization saved to: debug_leaderboard_regions.jpg")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        find_leaderboard(sys.argv[1])
    else:
        print("Usage: python find_leaderboard.py <image_path>")