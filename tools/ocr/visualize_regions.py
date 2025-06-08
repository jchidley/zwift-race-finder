#!/usr/bin/env python3
"""Visualize UI regions to debug coordinate issues"""

import cv2
import sys
from zwift_ocr_improved import ZwiftUILayout

def visualize_regions(image_path):
    """Draw rectangles on image to show where we're looking"""
    
    # Load image
    image = cv2.imread(image_path)
    if image is None:
        print(f"Failed to load image: {image_path}")
        return
    
    # Create a copy for visualization
    vis_image = image.copy()
    
    # Define colors for different types of regions
    colors = {
        'speed': (0, 255, 0),      # Green
        'power': (255, 0, 0),      # Blue
        'gradient': (0, 0, 255),   # Red
        'distance': (255, 255, 0), # Cyan
        'altitude': (255, 0, 255), # Magenta
        'race_time': (0, 255, 255), # Yellow
    }
    
    # Draw all regions
    for attr_name in dir(ZwiftUILayout):
        if not attr_name.startswith('_'):
            region = getattr(ZwiftUILayout, attr_name)
            if hasattr(region, 'x'):
                # Get color based on region name
                color = colors.get(region.name.lower(), (128, 128, 128))
                
                # Draw rectangle
                cv2.rectangle(vis_image, 
                            (region.x, region.y), 
                            (region.x + region.width, region.y + region.height),
                            color, 2)
                
                # Add label
                cv2.putText(vis_image, region.name, 
                          (region.x, region.y - 5),
                          cv2.FONT_HERSHEY_SIMPLEX, 0.5, color, 1)
    
    # Save visualization
    output_path = "debug_regions_visualization.jpg"
    cv2.imwrite(output_path, vis_image)
    print(f"Saved visualization to: {output_path}")
    
    # Also create individual region crops
    print("\nExtracting individual regions:")
    for attr_name in ['SPEED', 'POWER', 'GRADIENT', 'RACE_TIME', 'DISTANCE', 'ALTITUDE']:
        if hasattr(ZwiftUILayout, attr_name):
            region = getattr(ZwiftUILayout, attr_name)
            roi = image[region.y:region.y+region.height, region.x:region.x+region.width]
            filename = f"debug_region_{region.name}.jpg"
            cv2.imwrite(filename, roi)
            print(f"  {region.name}: ({region.x}, {region.y}, {region.width}, {region.height}) -> {filename}")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        visualize_regions(sys.argv[1])
    else:
        print("Usage: python visualize_regions.py <image_path>")