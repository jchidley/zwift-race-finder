#!/usr/bin/env python3
"""Analyze Zwift UI structure to find telemetry regions"""

import argparse
import json
from pathlib import Path
from typing import Dict, List

import cv2
import matplotlib.patches as patches
import matplotlib.pyplot as plt
import numpy as np


def analyze_ui_layout(image_path: str, debug: bool = False) -> Dict:
    """Analyze the UI layout to find likely telemetry regions"""
    img = cv2.imread(image_path)
    if img is None:
        raise ValueError(f'Could not load image: {image_path}')

    height, width = img.shape[:2]

    hsv = cv2.cvtColor(img, cv2.COLOR_BGR2HSV)

    regions = {
        'top_bar': analyze_top_bar(img, hsv),
        'left_panel': analyze_left_panel(img, hsv),
        'right_panel': analyze_right_panel(img, hsv),
        'center_area': analyze_center_area(img, hsv),
    }

    dark_regions = find_dark_ui_panels(img, hsv)

    text_regions = find_text_regions(img)

    analysis = {
        'resolution': f'{width}x{height}',
        'ui_regions': regions,
        'dark_panels': dark_regions,
        'text_candidates': text_regions,
    }

    if debug:
        visualize_analysis(img, analysis, image_path)

    return analysis


def analyze_top_bar(img: np.ndarray, hsv: np.ndarray) -> Dict:
    """Analyze the top bar area (speed, distance, altitude, time)"""
    height, width = img.shape[:2]

    top_region = img[0 : int(height * 0.12), :]

    gray = cv2.cvtColor(top_region, cv2.COLOR_BGR2GRAY)

    _, thresh = cv2.threshold(gray, 180, 255, cv2.THRESH_BINARY)
    contours, _ = cv2.findContours(thresh, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)

    text_boxes = []
    for contour in contours:
        x, y, w, h = cv2.boundingRect(contour)
        if 20 < w < 200 and 20 < h < 100:
            text_boxes.append({'x': x, 'y': y, 'width': w, 'height': h})

    text_boxes.sort(key=lambda b: b['x'])

    fields = []
    if len(text_boxes) >= 4:
        field_names = ['speed', 'distance', 'altitude', 'race_time']
        for i, (name, box) in enumerate(zip(field_names, text_boxes[:4])):
            fields.append(
                {
                    'name': name,
                    'x': box['x'],
                    'y': box['y'],
                    'width': box['width'],
                    'height': box['height'],
                    'region': 'top_bar',
                }
            )

    return {'fields': fields, 'text_boxes': text_boxes}


def analyze_left_panel(img: np.ndarray, hsv: np.ndarray) -> Dict:
    """Analyze the left panel (power, cadence, heart rate)"""
    height, width = img.shape[:2]

    left_region = img[int(height * 0.12) : int(height * 0.4), 0 : int(width * 0.25)]

    gray = cv2.cvtColor(left_region, cv2.COLOR_BGR2GRAY)

    _, thresh = cv2.threshold(gray, 180, 255, cv2.THRESH_BINARY)
    contours, _ = cv2.findContours(thresh, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)

    text_boxes = []
    for contour in contours:
        x, y, w, h = cv2.boundingRect(contour)
        if 30 < w < 250 and 20 < h < 100:
            text_boxes.append({'x': x, 'y': y + int(height * 0.12), 'width': w, 'height': h})

    fields = []
    if text_boxes:
        text_boxes.sort(key=lambda b: b['y'])

        if text_boxes:
            fields.append(
                {
                    'name': 'power',
                    'x': text_boxes[0]['x'],
                    'y': text_boxes[0]['y'],
                    'width': text_boxes[0]['width'],
                    'height': text_boxes[0]['height'],
                    'region': 'left_panel',
                }
            )

        if len(text_boxes) > 2:
            remaining = sorted(text_boxes[1:], key=lambda b: b['x'])
            if len(remaining) >= 2:
                fields.extend(
                    [
                        {
                            'name': 'cadence',
                            'x': remaining[0]['x'],
                            'y': remaining[0]['y'],
                            'width': remaining[0]['width'],
                            'height': remaining[0]['height'],
                            'region': 'left_panel',
                        },
                        {
                            'name': 'heart_rate',
                            'x': remaining[1]['x'],
                            'y': remaining[1]['y'],
                            'width': remaining[1]['width'],
                            'height': remaining[1]['height'],
                            'region': 'left_panel',
                        },
                    ]
                )

    return {'fields': fields, 'text_boxes': text_boxes}


def analyze_right_panel(img: np.ndarray, hsv: np.ndarray) -> Dict:
    """Analyze the right panel (leaderboard area)"""
    height, width = img.shape[:2]

    right_region = img[:, int(width * 0.75) :]

    gray = cv2.cvtColor(right_region, cv2.COLOR_BGR2GRAY)

    _, thresh = cv2.threshold(gray, 150, 255, cv2.THRESH_BINARY)

    contours, _ = cv2.findContours(thresh, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)

    if contours:
        largest = max(contours, key=cv2.contourArea)
        x, y, w, h = cv2.boundingRect(largest)

        leaderboard = {
            'name': 'leaderboard',
            'x': x + int(width * 0.75),
            'y': y,
            'width': w,
            'height': h,
            'region': 'right_panel',
        }
    else:
        leaderboard = {
            'name': 'leaderboard',
            'x': int(width * 0.75),
            'y': int(height * 0.2),
            'width': int(width * 0.23),
            'height': int(height * 0.7),
            'region': 'right_panel',
        }

    return {'fields': [leaderboard]}


def analyze_center_area(img: np.ndarray, hsv: np.ndarray) -> Dict:
    """Analyze center area (gradient, distance to finish, avatar)"""
    height, width = img.shape[:2]

    fields = []

    gradient_region = {
        'name': 'gradient',
        'x': int(width * 0.85),
        'y': int(height * 0.2),
        'width': 120,
        'height': 60,
        'region': 'center_area',
    }
    fields.append(gradient_region)

    dtf_region = {
        'name': 'distance_to_finish',
        'x': int(width * 0.58),
        'y': int(height * 0.12),
        'width': 150,
        'height': 50,
        'region': 'center_area',
    }
    fields.append(dtf_region)

    avatar_region = {
        'name': 'rider_pose_avatar',
        'x': int(width * 0.4),
        'y': int(height * 0.35),
        'width': int(width * 0.2),
        'height': int(height * 0.3),
        'region': 'center_area',
    }
    fields.append(avatar_region)

    return {'fields': fields}


def find_dark_ui_panels(img: np.ndarray, hsv: np.ndarray) -> List[Dict]:
    """Find dark UI panels that likely contain telemetry"""
    lower_dark = np.array([0, 0, 0])
    upper_dark = np.array([180, 255, 50])

    dark_mask = cv2.inRange(hsv, lower_dark, upper_dark)

    contours, _ = cv2.findContours(dark_mask, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)

    panels = []
    for contour in contours:
        x, y, w, h = cv2.boundingRect(contour)
        area = w * h

        if area > 1000 and w > 50 and h > 30:
            panels.append({'x': x, 'y': y, 'width': w, 'height': h, 'area': area})

    return panels


def find_text_regions(img: np.ndarray) -> List[Dict]:
    """Find regions likely to contain text using edge detection"""
    gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)

    edges = cv2.Canny(gray, 50, 150)

    kernel = cv2.getStructuringElement(cv2.MORPH_RECT, (5, 5))
    dilated = cv2.dilate(edges, kernel, iterations=2)

    contours, _ = cv2.findContours(dilated, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)

    text_regions = []
    for contour in contours:
        x, y, w, h = cv2.boundingRect(contour)

        aspect = w / h if h > 0 else 0
        if 0.5 < aspect < 10 and 20 < w < 300 and 15 < h < 100:
            text_regions.append({'x': x, 'y': y, 'width': w, 'height': h, 'aspect_ratio': aspect})

    return text_regions


def visualize_analysis(img: np.ndarray, analysis: Dict, image_path: str):
    """Visualize the UI analysis"""
    fig, ax = plt.subplots(1, figsize=(16, 9))

    img_rgb = cv2.cvtColor(img, cv2.COLOR_BGR2RGB)
    ax.imshow(img_rgb)

    colors = {
        'top_bar': 'yellow',
        'left_panel': 'red',
        'right_panel': 'blue',
        'center_area': 'green',
    }

    for region_name, region_data in analysis['ui_regions'].items():
        if 'fields' in region_data:
            for field in region_data['fields']:
                rect = patches.Rectangle(
                    (field['x'], field['y']),
                    field['width'],
                    field['height'],
                    linewidth=2,
                    edgecolor=colors.get(region_name, 'white'),
                    facecolor='none',
                )
                ax.add_patch(rect)
                ax.text(
                    field['x'],
                    field['y'] - 5,
                    field['name'],
                    color=colors.get(region_name, 'white'),
                    fontsize=10,
                    weight='bold',
                    bbox=dict(boxstyle='round,pad=0.3', facecolor='black', alpha=0.7),
                )

    for panel in analysis['dark_panels'][:10]:
        rect = patches.Rectangle(
            (panel['x'], panel['y']),
            panel['width'],
            panel['height'],
            linewidth=1,
            edgecolor='cyan',
            facecolor='none',
            linestyle='--',
        )
        ax.add_patch(rect)

    ax.set_title('Zwift UI Structure Analysis')
    ax.axis('off')

    output_path = Path(image_path).stem + '_ui_analysis.png'
    plt.savefig(output_path, dpi=150, bbox_inches='tight')
    print(f'Saved UI analysis visualization to: {output_path}')
    plt.close()


def suggest_regions_from_analysis(analysis: Dict) -> Dict[str, Dict]:
    """Suggest OCR regions based on UI analysis"""
    suggested_regions = {}

    for region_data in analysis['ui_regions'].values():
        if 'fields' in region_data:
            for field in region_data['fields']:
                name = field['name']
                suggested_regions[name] = {
                    'x': field['x'],
                    'y': field['y'],
                    'width': field['width'],
                    'height': field['height'],
                    'source': field['region'],
                }

    padding = 10
    for name, region in suggested_regions.items():
        region['x'] = max(0, region['x'] - padding)
        region['y'] = max(0, region['y'] - padding)
        region['width'] += 2 * padding
        region['height'] += 2 * padding

    return suggested_regions


def main():
    parser = argparse.ArgumentParser(description='Analyze Zwift UI structure')
    parser.add_argument('image', help='Path to Zwift screenshot')
    parser.add_argument('--debug', action='store_true', help='Save debug visualization')
    parser.add_argument('-o', '--output', help='Output JSON file for suggested regions')

    args = parser.parse_args()

    print('Analyzing UI structure...')
    analysis = analyze_ui_layout(args.image, debug=args.debug)

    suggested_regions = suggest_regions_from_analysis(analysis)

    print('\nSuggested OCR regions:')
    for name, region in suggested_regions.items():
        print(
            f'  {name}: ({region["x"]}, {region["y"]}) {region["width"]}x{region["height"]} - from {region["source"]}'
        )

    if args.output:
        output = {'analysis': analysis, 'suggested_regions': suggested_regions}
        with open(args.output, 'w') as f:
            json.dump(output, f, indent=2)
        print(f'\nSaved analysis to: {args.output}')


if __name__ == '__main__':
    main()
