#!/usr/bin/env python3
"""Calibrate OCR using the same approach as the original but finding correct positions"""

import argparse
import json
import sys
from datetime import datetime
from pathlib import Path
from typing import Dict, Optional

import cv2
import numpy as np
from paddleocr import PaddleOCR

# Add parent directory to path
sys.path.append(str(Path(__file__).parent))

# The original preprocessing approaches
FIELD_PREPROCESSING = {
    'speed': {'threshold': 200, 'scale': 3, 'invert': False},
    'distance': {'threshold': 200, 'scale': 3, 'invert': False},
    'altitude': {'threshold': 200, 'scale': 3, 'invert': False},
    'race_time': {'threshold': 200, 'scale': 3, 'invert': False},
    'power': {'threshold': 200, 'scale': 3, 'invert': False},
    'cadence': {'threshold': 200, 'scale': 3, 'invert': False},
    'heart_rate': {'threshold': 200, 'scale': 3, 'invert': False},
    'gradient': {'threshold': 100, 'scale': 4, 'invert': True},  # Special case
    'distance_to_finish': {'threshold': 150, 'scale': 3, 'invert': False},  # Dimmer
}

# Search areas based on typical Zwift UI layout (adjusted based on first pass)
SEARCH_AREAS = {
    # Top bar elements (left to right)
    'speed': {'x': 600, 'y': 20, 'width': 250, 'height': 100},
    'distance': {'x': 750, 'y': 20, 'width': 250, 'height': 100},
    'altitude': {'x': 950, 'y': 20, 'width': 200, 'height': 100},
    'race_time': {'x': 1050, 'y': 20, 'width': 250, 'height': 100},
    # Left panel elements
    'power': {'x': 50, 'y': 20, 'width': 300, 'height': 150},
    'cadence': {'x': 50, 'y': 100, 'width': 150, 'height': 100},
    'heart_rate': {'x': 170, 'y': 110, 'width': 150, 'height': 80},  # Expanded to get full number
    # Right side elements - based on user feedback
    'gradient': {'x': 1680, 'y': 250, 'width': 150, 'height': 100},  # Above leaderboard position
    'distance_to_finish': {'x': 1100, 'y': 120, 'width': 200, 'height': 60},  # Below race time
}


def preprocess_for_field(img: np.ndarray, field: str) -> np.ndarray:
    """Apply field-specific preprocessing like the original"""
    config = FIELD_PREPROCESSING[field]

    # Convert to grayscale
    if len(img.shape) == 3:
        gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
    else:
        gray = img.copy()

    # Invert if needed (gradient field)
    if config['invert']:
        gray = cv2.bitwise_not(gray)

    # Apply threshold
    _, binary = cv2.threshold(gray, config['threshold'], 255, cv2.THRESH_BINARY)

    # Scale up
    scaled = cv2.resize(
        binary, None, fx=config['scale'], fy=config['scale'], interpolation=cv2.INTER_CUBIC
    )

    return scaled


def find_text_in_area(ocr: PaddleOCR, img: np.ndarray, area: Dict, field: str) -> Optional[Dict]:
    """Search for text in a specific area using field-specific preprocessing"""
    # Extract region of interest
    x, y, w, h = area['x'], area['y'], area['width'], area['height']
    roi = img[y : y + h, x : x + w]

    # Preprocess for this field type
    processed = preprocess_for_field(roi, field)

    # Run OCR
    result = ocr.ocr(processed, cls=True)
    if not result or not result[0]:
        return None

    # Find the most likely match based on field type
    candidates = []
    scale = FIELD_PREPROCESSING[field]['scale']

    for detection in result[0]:
        bbox, (text, confidence) = detection

        # Calculate position in original image
        roi_x = min(bbox[0][0], bbox[2][0]) / scale
        roi_y = min(bbox[0][1], bbox[2][1]) / scale
        roi_w = abs(bbox[2][0] - bbox[0][0]) / scale
        roi_h = abs(bbox[2][1] - bbox[0][1]) / scale

        # Convert to full image coordinates
        full_x = x + roi_x
        full_y = y + roi_y

        candidates.append(
            {
                'text': text,
                'confidence': confidence,
                'x': int(full_x),
                'y': int(full_y),
                'width': int(roi_w),
                'height': int(roi_h),
                'preprocessing': f'{field}_specific',
            }
        )

    # Return best candidate (highest confidence)
    if candidates:
        return max(candidates, key=lambda c: c['confidence'])
    return None


def refine_search_area(img: np.ndarray, field: str, initial_area: Dict) -> Optional[Dict]:
    """Refine search area by looking for UI elements"""
    x, y, w, h = initial_area['x'], initial_area['y'], initial_area['width'], initial_area['height']
    roi = img[y : y + h, x : x + w]

    # Look for bright text on dark background (typical Zwift telemetry)
    gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY)

    # Find bright regions
    _, thresh = cv2.threshold(gray, 180, 255, cv2.THRESH_BINARY)

    # Find contours
    contours, _ = cv2.findContours(thresh, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)

    if not contours:
        return None

    # Find the largest contour that could be text
    valid_contours = []
    for contour in contours:
        cx, cy, cw, ch = cv2.boundingRect(contour)
        # Basic size filtering
        if 20 < cw < 250 and 15 < ch < 100:
            valid_contours.append((cx, cy, cw, ch))

    if not valid_contours:
        return None

    # Get bounding box of all valid contours
    min_x = min(c[0] for c in valid_contours)
    min_y = min(c[1] for c in valid_contours)
    max_x = max(c[0] + c[2] for c in valid_contours)
    max_y = max(c[1] + c[3] for c in valid_contours)

    # Add some padding
    padding = 5
    return {
        'x': x + max(0, min_x - padding),
        'y': y + max(0, min_y - padding),
        'width': min(max_x - min_x + 2 * padding, w),
        'height': min(max_y - min_y + 2 * padding, h),
    }


def calibrate_fields(image_path: str, debug: bool = False) -> Dict[str, Dict]:
    """Calibrate all fields using targeted search"""
    img = cv2.imread(image_path)
    if img is None:
        raise ValueError(f'Could not load image: {image_path}')

    ocr = PaddleOCR(use_angle_cls=True, lang='en', show_log=False)

    calibrated_regions = {}

    print('Calibrating fields...')
    for field, search_area in SEARCH_AREAS.items():
        print(f'  Searching for {field}...')

        # First try to refine the search area
        refined_area = refine_search_area(img, field, search_area)
        if refined_area:
            search_area = refined_area
            if debug:
                print(f'    Refined search area to: {refined_area}')

        # Search for text in this area
        result = find_text_in_area(ocr, img, search_area, field)

        if result:
            print(f"    Found: '{result['text']}' at ({result['x']}, {result['y']})")
            calibrated_regions[field] = result
        else:
            print('    Not found in expected area')

    return calibrated_regions


def validate_with_known_values(regions: Dict[str, Dict], known_values: Dict) -> Dict[str, bool]:
    """Validate detected regions against known values"""
    validation = {}

    for field, known_value in known_values.items():
        if field in regions:
            detected = regions[field]['text']
            # Clean up detected text
            detected_clean = detected.replace(' ', '').upper()
            known_clean = str(known_value).replace(' ', '').upper()

            # Check if the known value is contained in detected text
            is_valid = known_clean in detected_clean
            validation[field] = is_valid

            if not is_valid:
                print(f"  {field}: Expected '{known_value}', got '{detected}'")

    return validation


def main():
    parser = argparse.ArgumentParser(description='Calibrate OCR like original implementation')
    parser.add_argument('image', help='Path to calibration image')
    parser.add_argument('--debug', action='store_true', help='Enable debug output')
    parser.add_argument('-o', '--output', help='Output config file path')

    # Known values from user feedback
    parser.add_argument('--validate', action='store_true', help='Validate against known values')

    args = parser.parse_args()

    # Known values for frame_000100.png
    known_values = {
        'speed': '34',
        'power': '195',
        'heart_rate': '129',
        'cadence': '55',
        'distance': '0.3',
        'altitude': '1',
        'distance_to_finish': '52.5',
        'gradient': '1',
    }

    # Calibrate fields
    regions = calibrate_fields(args.image, debug=args.debug)

    # Validate if requested
    if args.validate:
        print('\nValidating against known values...')
        validation = validate_with_known_values(regions, known_values)
        valid_count = sum(1 for v in validation.values() if v)
        print(f'Validation: {valid_count}/{len(validation)} fields correct')

    # Create configuration
    img = cv2.imread(args.image)
    height, width = img.shape[:2]

    config = {
        'version': '3.0.0',  # Version 3 uses original preprocessing approach
        'resolution': f'{width}x{height}',
        'created': datetime.utcnow().isoformat(),
        'calibration_image': Path(args.image).name,
        'regions': {},
        'preprocessing': FIELD_PREPROCESSING,
        'method': 'targeted_search_with_field_preprocessing',
    }

    # Add detected regions
    for field, region in regions.items():
        config['regions'][field] = {
            'x': region['x'],
            'y': region['y'],
            'width': region['width'],
            'height': region['height'],
            'detected_text': region['text'],
            'confidence': region['confidence'],
        }

    # Add hardcoded regions for complex areas
    if 'leaderboard' not in config['regions']:
        config['regions']['leaderboard'] = {
            'x': 1478,
            'y': 288,
            'width': 422,
            'height': 778,
            'note': 'Uses CLAHE enhancement',
        }

    if 'rider_pose_avatar' not in config['regions']:
        config['regions']['rider_pose_avatar'] = {
            'x': 768,
            'y': 378,
            'width': 384,
            'height': 324,
            'note': 'Uses edge detection',
        }

    # Save configuration
    output_path = args.output or f'ocr-configs/{Path(args.image).stem}_calibrated.json'
    Path(output_path).parent.mkdir(exist_ok=True)

    with open(output_path, 'w') as f:
        json.dump(config, f, indent=2)

    print(f'\nSaved calibration to: {output_path}')
    print(f'Detected {len(regions)}/{len(SEARCH_AREAS)} fields')


if __name__ == '__main__':
    main()
