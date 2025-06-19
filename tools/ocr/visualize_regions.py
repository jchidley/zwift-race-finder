#!/usr/bin/env python3
"""
Visualize OCR regions on a screenshot for debugging
"""

import sys
import json
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

def draw_regions(image_path, config_path, output_path=None):
    """Draw bounding boxes for OCR regions on the image."""
    
    # Load image
    img = Image.open(image_path)
    draw = ImageDraw.Draw(img)
    
    # Load config
    with open(config_path, 'r') as f:
        config = json.load(f)
    
    regions = config.get('regions', {})
    
    # Colors for different region types
    colors = {
        'speed': 'red',
        'distance': 'green',
        'altitude': 'blue',
        'race_time': 'yellow',
        'power': 'orange',
        'cadence': 'purple',
        'heart_rate': 'pink',
        'gradient': 'cyan',
        'distance_to_finish': 'magenta',
        'leaderboard': 'lime',
        'rider_pose_avatar': 'white'
    }
    
    # Draw each region
    for name, region in regions.items():
        if isinstance(region, dict) and all(k in region for k in ['x', 'y', 'width', 'height']):
            x, y, w, h = region['x'], region['y'], region['width'], region['height']
            color = colors.get(name, 'red')
            
            # Draw rectangle
            draw.rectangle([x, y, x + w, y + h], outline=color, width=2)
            
            # Draw label
            draw.text((x, y - 15), name, fill=color)
    
    # Save or show
    if output_path:
        img.save(output_path)
        print(f"Saved visualization to {output_path}")
    else:
        img.show()

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python visualize_regions.py <image_path> <config_path> [output_path]")
        sys.exit(1)
    
    image_path = sys.argv[1]
    config_path = sys.argv[2]
    output_path = sys.argv[3] if len(sys.argv) > 3 else None
    
    draw_regions(image_path, config_path, output_path)