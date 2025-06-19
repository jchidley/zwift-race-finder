#!/usr/bin/env python3
"""Multi-pass calibration to find OCR regions with optimal preprocessing settings"""

import cv2
import numpy as np
import json
import argparse
from pathlib import Path
from datetime import datetime
from paddleocr import PaddleOCR
from typing import List, Dict, Tuple, Optional
import sys

# Add parent directory to path to import the original OCR
sys.path.append(str(Path(__file__).parent))
from zwift_ocr_compact import ZwiftOCR

# Preprocessing configurations to try
PREPROCESSING_CONFIGS = [
    # Standard bright text
    {"name": "bright_text", "threshold": 200, "scale": 3, "invert": False},
    # Dimmer text (distance to finish)
    {"name": "dim_text", "threshold": 150, "scale": 3, "invert": False},
    # Stylized gradient font
    {"name": "gradient_style", "threshold": 100, "scale": 4, "invert": True},
    # Alternative gradient
    {"name": "gradient_alt", "threshold": 150, "scale": 4, "invert": False},
    # Very bright text
    {"name": "very_bright", "threshold": 220, "scale": 3, "invert": False},
    # Low contrast
    {"name": "low_contrast", "threshold": 120, "scale": 3, "invert": False},
    # No threshold (for CLAHE regions)
    {"name": "clahe_only", "threshold": None, "scale": 2, "invert": False, "clahe": True},
]

# Known text patterns for field identification
FIELD_PATTERNS = {
    "speed": {"pattern": r"^\d+$", "range": (15, 60), "unit": "KMHA"},
    "power": {"pattern": r"^\d+w?$", "range": (50, 500), "unit": "w"},
    "cadence": {"pattern": r"^\d+$", "range": (40, 120), "unit": "RPM"},
    "heart_rate": {"pattern": r"^\d+$", "range": (80, 200), "unit": "BPM"},
    "distance": {"pattern": r"^\d+\.\d+$", "range": (0, 100), "unit": "M"},
    "altitude": {"pattern": r"^\d+M?$", "range": (0, 2000), "unit": "M"},
    "race_time": {"pattern": r"^\d+:\d+$", "range": None, "unit": None},
    "gradient": {"pattern": r"^\d+$", "range": (-20, 20), "unit": "%"},
    "distance_to_finish": {"pattern": r"^\d+\.\d+km$", "range": (0, 100), "unit": "km"},
}

def preprocess_image(img, config: Dict) -> np.ndarray:
    """Apply preprocessing based on config"""
    # Convert to grayscale
    if len(img.shape) == 3:
        gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
    else:
        gray = img.copy()
    
    # Apply CLAHE if requested
    if config.get("clahe", False):
        clahe = cv2.createCLAHE(clipLimit=2.0, tileGridSize=(8, 8))
        gray = clahe.apply(gray)
    
    # Invert if requested
    if config.get("invert", False):
        gray = cv2.bitwise_not(gray)
    
    # Apply threshold if specified
    if config.get("threshold") is not None:
        _, binary = cv2.threshold(gray, config["threshold"], 255, cv2.THRESH_BINARY)
    else:
        binary = gray
    
    # Scale up
    scale = config.get("scale", 3)
    scaled = cv2.resize(binary, None, fx=scale, fy=scale, interpolation=cv2.INTER_CUBIC)
    
    return scaled

def detect_with_preprocessing(ocr: PaddleOCR, img: np.ndarray, config: Dict) -> List[Dict]:
    """Run OCR with specific preprocessing"""
    processed = preprocess_image(img, config)
    
    # Run OCR
    result = ocr.ocr(processed, cls=True)
    if not result or not result[0]:
        return []
    
    # Adjust coordinates back to original scale
    scale = config.get("scale", 3)
    detections = []
    for detection in result[0]:
        bbox, (text, confidence) = detection
        
        # Scale bbox back to original coordinates
        scaled_bbox = []
        for point in bbox:
            scaled_bbox.append([point[0] / scale, point[1] / scale])
        
        # Calculate center and size
        x_coords = [p[0] for p in scaled_bbox]
        y_coords = [p[1] for p in scaled_bbox]
        
        detection_info = {
            "text": text.strip(),
            "confidence": confidence,
            "x": int(min(x_coords)),
            "y": int(min(y_coords)),
            "width": int(max(x_coords) - min(x_coords)),
            "height": int(max(y_coords) - min(y_coords)),
            "preprocessing": config["name"],
            "config": config
        }
        
        detections.append(detection_info)
    
    return detections

def multi_pass_detection(image_path: str) -> Dict[str, List[Dict]]:
    """Run multiple preprocessing passes to find all text"""
    img = cv2.imread(image_path)
    if img is None:
        raise ValueError(f"Could not load image: {image_path}")
    
    ocr = PaddleOCR(use_angle_cls=True, lang='en', show_log=False)
    
    # Collect all detections from all preprocessing methods
    all_detections = []
    
    print("Running multi-pass detection...")
    for config in PREPROCESSING_CONFIGS:
        print(f"  - Trying {config['name']}...")
        detections = detect_with_preprocessing(ocr, img, config)
        all_detections.extend(detections)
        print(f"    Found {len(detections)} text regions")
    
    # Group detections by location to find best preprocessing for each region
    # Use spatial clustering to group detections of the same text
    grouped = group_detections_by_location(all_detections)
    
    return grouped

def group_detections_by_location(detections: List[Dict], iou_threshold: float = 0.5) -> Dict[str, List[Dict]]:
    """Group detections that overlap significantly"""
    grouped = {}
    used = set()
    
    for i, det1 in enumerate(detections):
        if i in used:
            continue
        
        # Start a new group
        group = [det1]
        used.add(i)
        
        # Find all overlapping detections
        for j, det2 in enumerate(detections):
            if j in used or i == j:
                continue
            
            if calculate_iou(det1, det2) > iou_threshold:
                group.append(det2)
                used.add(j)
        
        # Choose the best detection from the group (highest confidence)
        best = max(group, key=lambda d: d["confidence"])
        
        # Try to identify the field type
        field_type = identify_field_type(best["text"])
        if field_type:
            if field_type not in grouped:
                grouped[field_type] = []
            grouped[field_type].append(best)
    
    return grouped

def calculate_iou(det1: Dict, det2: Dict) -> float:
    """Calculate intersection over union for two detections"""
    x1 = max(det1["x"], det2["x"])
    y1 = max(det1["y"], det2["y"])
    x2 = min(det1["x"] + det1["width"], det2["x"] + det2["width"])
    y2 = min(det1["y"] + det1["height"], det2["y"] + det2["height"])
    
    if x2 < x1 or y2 < y1:
        return 0.0
    
    intersection = (x2 - x1) * (y2 - y1)
    area1 = det1["width"] * det1["height"]
    area2 = det2["width"] * det2["height"]
    union = area1 + area2 - intersection
    
    return intersection / union if union > 0 else 0

def identify_field_type(text: str) -> Optional[str]:
    """Try to identify what field this text represents"""
    import re
    
    # Clean the text
    clean_text = text.strip()
    
    # Check each field pattern
    for field, config in FIELD_PATTERNS.items():
        pattern = config["pattern"]
        if pattern and re.match(pattern, clean_text):
            # Check numeric range if applicable
            if config["range"]:
                try:
                    value = float(re.sub(r"[^\d.]", "", clean_text))
                    if config["range"][0] <= value <= config["range"][1]:
                        return field
                except:
                    pass
    
    # Check for units
    if "KMHA" in clean_text:
        return "speed"
    elif "RPM" in clean_text:
        return "cadence"
    elif "BPM" in clean_text:
        return "heart_rate"
    elif clean_text.endswith("w"):
        return "power"
    elif clean_text.endswith("km") and "." in clean_text:
        return "distance_to_finish"
    elif clean_text.endswith("M") and "." not in clean_text:
        return "altitude"
    
    return None

def compare_with_original(image_path: str, detected_regions: Dict[str, List[Dict]]):
    """Compare detected regions with original OCR implementation"""
    print("\nComparing with original implementation...")
    
    # Use original OCR
    original_ocr = ZwiftOCR()
    original_results = original_ocr.extract(image_path)
    
    print("\nOriginal OCR results:")
    for field, value in original_results.items():
        if value is not None and field != "leaderboard":
            print(f"  {field}: {value}")
    
    print("\nMulti-pass detection results:")
    for field, detections in detected_regions.items():
        if detections:
            best = detections[0]  # Already sorted by confidence
            print(f"  {field}: {best['text']} (preprocessing: {best['preprocessing']})")

def save_calibration_config(detected_regions: Dict[str, List[Dict]], image_path: str, output_path: str):
    """Save the calibration configuration"""
    img = cv2.imread(image_path)
    height, width = img.shape[:2]
    
    config = {
        "version": "2.0.0",  # Version 2 includes preprocessing info
        "resolution": f"{width}x{height}",
        "created": datetime.utcnow().isoformat(),
        "calibration_image": Path(image_path).name,
        "regions": {},
        "preprocessing": {}
    }
    
    # Add regions with optimal preprocessing
    for field, detections in detected_regions.items():
        if detections:
            best = detections[0]
            
            # Add padding to regions
            padding = 10
            config["regions"][field] = {
                "x": max(0, best["x"] - padding),
                "y": max(0, best["y"] - padding),
                "width": best["width"] + 2 * padding,
                "height": best["height"] + 2 * padding,
                "detected_text": best["text"],
                "confidence": best["confidence"]
            }
            
            # Save preprocessing config
            config["preprocessing"][field] = best["config"]
    
    # Add hardcoded regions for fields we didn't detect
    if "leaderboard" not in config["regions"]:
        config["regions"]["leaderboard"] = {
            "x": 1478,
            "y": 288,
            "width": 422,
            "height": 778,
            "note": "hardcoded - use CLAHE enhancement"
        }
    
    if "rider_pose_avatar" not in config["regions"]:
        config["regions"]["rider_pose_avatar"] = {
            "x": 768,
            "y": 378,
            "width": 384,
            "height": 324,
            "note": "hardcoded - edge detection"
        }
    
    # Save config
    with open(output_path, 'w') as f:
        json.dump(config, f, indent=2)
    
    print(f"\nSaved enhanced calibration config to: {output_path}")

def main():
    parser = argparse.ArgumentParser(description="Multi-pass OCR calibration")
    parser.add_argument("image", help="Path to calibration image")
    parser.add_argument("-o", "--output", help="Output config file path")
    parser.add_argument("--compare", action="store_true", help="Compare with original OCR")
    
    args = parser.parse_args()
    
    # Run multi-pass detection
    detected_regions = multi_pass_detection(args.image)
    
    # Show results
    print("\nDetected regions by field type:")
    for field, detections in detected_regions.items():
        if detections:
            best = detections[0]
            print(f"\n{field}:")
            print(f"  Text: {best['text']}")
            print(f"  Position: ({best['x']}, {best['y']}) size: {best['width']}x{best['height']}")
            print(f"  Best preprocessing: {best['preprocessing']}")
            print(f"  Config: threshold={best['config']['threshold']}, scale={best['config']['scale']}, invert={best['config']['invert']}")
    
    # Compare if requested
    if args.compare:
        compare_with_original(args.image, detected_regions)
    
    # Save config
    if args.output:
        output_path = args.output
    else:
        output_path = f"ocr-configs/{Path(args.image).stem}_multipass.json"
    
    save_calibration_config(detected_regions, args.image, output_path)

if __name__ == "__main__":
    main()