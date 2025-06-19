#!/usr/bin/env python3
"""Analyze Zwift UI structure to find telemetry regions"""

import cv2
import numpy as np
import json
import argparse
from pathlib import Path
from typing import List, Dict, Tuple
import matplotlib.pyplot as plt
import matplotlib.patches as patches

def analyze_ui_layout(image_path: str, debug: bool = False) -> Dict:
    """Analyze the UI layout to find likely telemetry regions"""
    img = cv2.imread(image_path)
    if img is None:
        raise ValueError(f"Could not load image: {image_path}")
    
    height, width = img.shape[:2]
    
    # Convert to HSV for better color segmentation
    hsv = cv2.cvtColor(img, cv2.COLOR_BGR2HSV)
    
    # Analyze different regions based on Zwift UI patterns
    regions = {
        "top_bar": analyze_top_bar(img, hsv),
        "left_panel": analyze_left_panel(img, hsv),
        "right_panel": analyze_right_panel(img, hsv),
        "center_area": analyze_center_area(img, hsv)
    }
    
    # Identify dark UI panels (common in Zwift)
    dark_regions = find_dark_ui_panels(img, hsv)
    
    # Find text-like regions
    text_regions = find_text_regions(img)
    
    # Combine analysis
    analysis = {
        "resolution": f"{width}x{height}",
        "ui_regions": regions,
        "dark_panels": dark_regions,
        "text_candidates": text_regions
    }
    
    if debug:
        visualize_analysis(img, analysis, image_path)
    
    return analysis

def analyze_top_bar(img: np.ndarray, hsv: np.ndarray) -> Dict:
    """Analyze the top bar area (speed, distance, altitude, time)"""
    height, width = img.shape[:2]
    
    # Top bar is typically in the top 10% of the screen
    top_region = img[0:int(height * 0.12), :]
    
    # Look for dark background with white text
    gray = cv2.cvtColor(top_region, cv2.COLOR_BGR2GRAY)
    
    # Find contours of bright areas (text)
    _, thresh = cv2.threshold(gray, 180, 255, cv2.THRESH_BINARY)
    contours, _ = cv2.findContours(thresh, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
    
    # Group contours into likely text regions
    text_boxes = []
    for contour in contours:
        x, y, w, h = cv2.boundingRect(contour)
        # Filter by size (text should be reasonable size)
        if 20 < w < 200 and 20 < h < 100:
            text_boxes.append({"x": x, "y": y, "width": w, "height": h})
    
    # Zwift typically has 4 main values in top bar
    # Sort by X coordinate to get left-to-right order
    text_boxes.sort(key=lambda b: b["x"])
    
    fields = []
    if len(text_boxes) >= 4:
        # Likely: speed, distance, altitude, time
        field_names = ["speed", "distance", "altitude", "race_time"]
        for i, (name, box) in enumerate(zip(field_names, text_boxes[:4])):
            fields.append({
                "name": name,
                "x": box["x"],
                "y": box["y"],
                "width": box["width"],
                "height": box["height"],
                "region": "top_bar"
            })
    
    return {"fields": fields, "text_boxes": text_boxes}

def analyze_left_panel(img: np.ndarray, hsv: np.ndarray) -> Dict:
    """Analyze the left panel (power, cadence, heart rate)"""
    height, width = img.shape[:2]
    
    # Left panel is typically in the left 20% of screen, below top bar
    left_region = img[int(height * 0.12):int(height * 0.4), 0:int(width * 0.25)]
    
    # Look for dark background panels
    gray = cv2.cvtColor(left_region, cv2.COLOR_BGR2GRAY)
    
    # Find bright text regions
    _, thresh = cv2.threshold(gray, 180, 255, cv2.THRESH_BINARY)
    contours, _ = cv2.findContours(thresh, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
    
    text_boxes = []
    for contour in contours:
        x, y, w, h = cv2.boundingRect(contour)
        if 30 < w < 250 and 20 < h < 100:
            # Adjust coordinates to full image
            text_boxes.append({
                "x": x,
                "y": y + int(height * 0.12),
                "width": w,
                "height": h
            })
    
    # Power is usually the largest number at top
    # Cadence and HR are below
    fields = []
    if text_boxes:
        # Sort by Y coordinate
        text_boxes.sort(key=lambda b: b["y"])
        
        # First large number is likely power
        if text_boxes:
            fields.append({
                "name": "power",
                "x": text_boxes[0]["x"],
                "y": text_boxes[0]["y"],
                "width": text_boxes[0]["width"],
                "height": text_boxes[0]["height"],
                "region": "left_panel"
            })
        
        # Look for smaller numbers below (cadence, HR)
        if len(text_boxes) > 2:
            # Sort remaining by X coordinate
            remaining = sorted(text_boxes[1:], key=lambda b: b["x"])
            if len(remaining) >= 2:
                fields.extend([
                    {
                        "name": "cadence",
                        "x": remaining[0]["x"],
                        "y": remaining[0]["y"],
                        "width": remaining[0]["width"],
                        "height": remaining[0]["height"],
                        "region": "left_panel"
                    },
                    {
                        "name": "heart_rate",
                        "x": remaining[1]["x"],
                        "y": remaining[1]["y"],
                        "width": remaining[1]["width"],
                        "height": remaining[1]["height"],
                        "region": "left_panel"
                    }
                ])
    
    return {"fields": fields, "text_boxes": text_boxes}

def analyze_right_panel(img: np.ndarray, hsv: np.ndarray) -> Dict:
    """Analyze the right panel (leaderboard area)"""
    height, width = img.shape[:2]
    
    # Right panel is typically in the right 25% of screen
    right_region = img[:, int(width * 0.75):]
    
    # Leaderboard has a different background
    gray = cv2.cvtColor(right_region, cv2.COLOR_BGR2GRAY)
    
    # Look for consistent text patterns
    _, thresh = cv2.threshold(gray, 150, 255, cv2.THRESH_BINARY)
    
    # Find the main leaderboard area
    # It's usually a large continuous region with text
    contours, _ = cv2.findContours(thresh, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
    
    # Find largest contour area (likely leaderboard)
    if contours:
        largest = max(contours, key=cv2.contourArea)
        x, y, w, h = cv2.boundingRect(largest)
        
        # Adjust to full image coordinates
        leaderboard = {
            "name": "leaderboard",
            "x": x + int(width * 0.75),
            "y": y,
            "width": w,
            "height": h,
            "region": "right_panel"
        }
    else:
        # Fallback to expected area
        leaderboard = {
            "name": "leaderboard",
            "x": int(width * 0.75),
            "y": int(height * 0.2),
            "width": int(width * 0.23),
            "height": int(height * 0.7),
            "region": "right_panel"
        }
    
    return {"fields": [leaderboard]}

def analyze_center_area(img: np.ndarray, hsv: np.ndarray) -> Dict:
    """Analyze center area (gradient, distance to finish, avatar)"""
    height, width = img.shape[:2]
    
    fields = []
    
    # Gradient is typically in top-right of center area
    gradient_region = {
        "name": "gradient",
        "x": int(width * 0.85),
        "y": int(height * 0.2),
        "width": 120,
        "height": 60,
        "region": "center_area"
    }
    fields.append(gradient_region)
    
    # Distance to finish is usually below top bar, right side
    dtf_region = {
        "name": "distance_to_finish",
        "x": int(width * 0.58),
        "y": int(height * 0.12),
        "width": 150,
        "height": 50,
        "region": "center_area"
    }
    fields.append(dtf_region)
    
    # Avatar area is center
    avatar_region = {
        "name": "rider_pose_avatar",
        "x": int(width * 0.4),
        "y": int(height * 0.35),
        "width": int(width * 0.2),
        "height": int(height * 0.3),
        "region": "center_area"
    }
    fields.append(avatar_region)
    
    return {"fields": fields}

def find_dark_ui_panels(img: np.ndarray, hsv: np.ndarray) -> List[Dict]:
    """Find dark UI panels that likely contain telemetry"""
    # Look for dark regions with consistent color
    lower_dark = np.array([0, 0, 0])
    upper_dark = np.array([180, 255, 50])
    
    dark_mask = cv2.inRange(hsv, lower_dark, upper_dark)
    
    # Find contours
    contours, _ = cv2.findContours(dark_mask, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
    
    panels = []
    for contour in contours:
        x, y, w, h = cv2.boundingRect(contour)
        area = w * h
        
        # Filter by size - panels should be reasonably sized
        if area > 1000 and w > 50 and h > 30:
            panels.append({
                "x": x,
                "y": y,
                "width": w,
                "height": h,
                "area": area
            })
    
    return panels

def find_text_regions(img: np.ndarray) -> List[Dict]:
    """Find regions likely to contain text using edge detection"""
    gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
    
    # Use Canny edge detection
    edges = cv2.Canny(gray, 50, 150)
    
    # Dilate to connect text components
    kernel = cv2.getStructuringElement(cv2.MORPH_RECT, (5, 5))
    dilated = cv2.dilate(edges, kernel, iterations=2)
    
    # Find contours
    contours, _ = cv2.findContours(dilated, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
    
    text_regions = []
    for contour in contours:
        x, y, w, h = cv2.boundingRect(contour)
        
        # Filter by aspect ratio and size
        aspect = w / h if h > 0 else 0
        if 0.5 < aspect < 10 and 20 < w < 300 and 15 < h < 100:
            text_regions.append({
                "x": x,
                "y": y,
                "width": w,
                "height": h,
                "aspect_ratio": aspect
            })
    
    return text_regions

def visualize_analysis(img: np.ndarray, analysis: Dict, image_path: str):
    """Visualize the UI analysis"""
    fig, ax = plt.subplots(1, figsize=(16, 9))
    
    # Display image
    img_rgb = cv2.cvtColor(img, cv2.COLOR_BGR2RGB)
    ax.imshow(img_rgb)
    
    # Draw UI regions
    colors = {
        "top_bar": "yellow",
        "left_panel": "red",
        "right_panel": "blue",
        "center_area": "green"
    }
    
    for region_name, region_data in analysis["ui_regions"].items():
        if "fields" in region_data:
            for field in region_data["fields"]:
                rect = patches.Rectangle(
                    (field["x"], field["y"]),
                    field["width"],
                    field["height"],
                    linewidth=2,
                    edgecolor=colors.get(region_name, "white"),
                    facecolor='none'
                )
                ax.add_patch(rect)
                ax.text(
                    field["x"], 
                    field["y"] - 5, 
                    field["name"],
                    color=colors.get(region_name, "white"),
                    fontsize=10,
                    weight='bold',
                    bbox=dict(boxstyle="round,pad=0.3", facecolor='black', alpha=0.7)
                )
    
    # Draw dark panels
    for panel in analysis["dark_panels"][:10]:  # Limit to avoid clutter
        rect = patches.Rectangle(
            (panel["x"], panel["y"]),
            panel["width"],
            panel["height"],
            linewidth=1,
            edgecolor='cyan',
            facecolor='none',
            linestyle='--'
        )
        ax.add_patch(rect)
    
    ax.set_title("Zwift UI Structure Analysis")
    ax.axis('off')
    
    # Save visualization
    output_path = Path(image_path).stem + "_ui_analysis.png"
    plt.savefig(output_path, dpi=150, bbox_inches='tight')
    print(f"Saved UI analysis visualization to: {output_path}")
    plt.close()

def suggest_regions_from_analysis(analysis: Dict) -> Dict[str, Dict]:
    """Suggest OCR regions based on UI analysis"""
    suggested_regions = {}
    
    # Collect all detected fields
    for region_data in analysis["ui_regions"].values():
        if "fields" in region_data:
            for field in region_data["fields"]:
                name = field["name"]
                suggested_regions[name] = {
                    "x": field["x"],
                    "y": field["y"],
                    "width": field["width"],
                    "height": field["height"],
                    "source": field["region"]
                }
    
    # Add padding to regions
    padding = 10
    for name, region in suggested_regions.items():
        region["x"] = max(0, region["x"] - padding)
        region["y"] = max(0, region["y"] - padding)
        region["width"] += 2 * padding
        region["height"] += 2 * padding
    
    return suggested_regions

def main():
    parser = argparse.ArgumentParser(description="Analyze Zwift UI structure")
    parser.add_argument("image", help="Path to Zwift screenshot")
    parser.add_argument("--debug", action="store_true", help="Save debug visualization")
    parser.add_argument("-o", "--output", help="Output JSON file for suggested regions")
    
    args = parser.parse_args()
    
    # Analyze UI
    print("Analyzing UI structure...")
    analysis = analyze_ui_layout(args.image, debug=args.debug)
    
    # Get suggested regions
    suggested_regions = suggest_regions_from_analysis(analysis)
    
    print("\nSuggested OCR regions:")
    for name, region in suggested_regions.items():
        print(f"  {name}: ({region['x']}, {region['y']}) {region['width']}x{region['height']} - from {region['source']}")
    
    # Save if requested
    if args.output:
        output = {
            "analysis": analysis,
            "suggested_regions": suggested_regions
        }
        with open(args.output, 'w') as f:
            json.dump(output, f, indent=2)
        print(f"\nSaved analysis to: {args.output}")

if __name__ == "__main__":
    main()