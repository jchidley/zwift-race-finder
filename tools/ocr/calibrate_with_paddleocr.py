#!/usr/bin/env python3
"""Auto-calibrate OCR regions by detecting all text in a Zwift screenshot using PaddleOCR"""

import json
import warnings
from datetime import datetime
from pathlib import Path
from typing import Dict, List

import cv2
from paddleocr import PaddleOCR

warnings.filterwarnings('ignore', category=UserWarning, module='paddle')


def detect_all_text_regions(image_path: str, debug: bool = False) -> List[Dict]:
    """Run PaddleOCR on full image to detect all text with bounding boxes"""

    ocr = PaddleOCR(use_angle_cls=True, lang='en', show_log=False)

    img = cv2.imread(image_path)
    if img is None:
        raise ValueError(f'Could not load image: {image_path}')

    height, width = img.shape[:2]
    print(f'Image resolution: {width}x{height}')

    print('Running PaddleOCR on full image...')
    result = ocr.ocr(img, cls=True)

    if not result or not result[0]:
        print('No text detected!')
        return []

    regions = []
    for detection in result[0]:
        bbox, (text, confidence) = detection

        x1, y1 = int(bbox[0][0]), int(bbox[0][1])
        x2, y2 = int(bbox[1][0]), int(bbox[1][1])
        x3, y3 = int(bbox[2][0]), int(bbox[2][1])
        x4, y4 = int(bbox[3][0]), int(bbox[3][1])

        x_min = min(x1, x2, x3, x4)
        y_min = min(y1, y2, y3, y4)
        x_max = max(x1, x2, x3, x4)
        y_max = max(y1, y2, y3, y4)

        region = {
            'text': text,
            'confidence': confidence,
            'x': x_min,
            'y': y_min,
            'width': x_max - x_min,
            'height': y_max - y_min,
            'center_x': (x_min + x_max) // 2,
            'center_y': (y_min + y_max) // 2,
        }
        regions.append(region)

        if debug:
            print(
                f"Detected: '{text}' at ({x_min}, {y_min}) size {region['width']}x{region['height']} conf={confidence:.2f}"
            )

    print(f'Detected {len(regions)} text regions')
    return regions, width, height


def add_padding_to_region(region: Dict, padding: int = 10) -> Dict:
    """Add padding around detected text region for better OCR"""
    padded = region.copy()
    padded['x'] = max(0, region['x'] - padding)
    padded['y'] = max(0, region['y'] - padding)
    padded['width'] = region['width'] + 2 * padding
    padded['height'] = region['height'] + 2 * padding
    return padded


def classify_regions(regions: List[Dict], width: int, height: int) -> Dict[str, Dict]:
    """Classify detected regions based on text content and position"""

    classified = {}

    for region in regions:
        text = region['text'].strip()
        x, y = region['x'], region['y']
        cx, cy = region['center_x'], region['center_y']

        if region['confidence'] < 0.5:
            continue

        if ('km/h' in text.lower() or 'mph' in text.lower()) and x < width * 0.3:
            classified['speed'] = add_padding_to_region(region, 15)
        elif (
            x < width * 0.15
            and height * 0.1 < y < height * 0.2
            and text.replace('.', '').isdigit()
            and 'speed' not in classified
        ):
            try:
                val = float(text.replace(',', '.'))
                if 0 < val < 100:
                    classified['speed'] = add_padding_to_region(region, 15)
            except:
                pass

        elif 'w' in text.lower() and x < width * 0.3 and y > height * 0.1:
            classified['power'] = add_padding_to_region(region, 20)
        elif x < width * 0.2 and height * 0.15 < y < height * 0.3 and text.isdigit():
            try:
                val = int(text)
                if 50 < val < 2000:
                    classified['power'] = add_padding_to_region(region, 20)
            except:
                pass

        elif y < height * 0.1 and ('.' in text or 'km' in text.lower()):
            if x > width * 0.2:
                classified['distance'] = add_padding_to_region(region, 15)

        elif y < height * 0.1 and ('m' in text.lower() or text.replace(',', '').isdigit()):
            if x > width * 0.4 and 'altitude' not in classified:
                classified['altitude'] = add_padding_to_region(region, 15)

        elif ':' in text and text.count(':') >= 1 and y < height * 0.15:
            parts = text.split(':')
            if all(p.isdigit() for p in parts):
                classified['race_time'] = add_padding_to_region(region, 15)

        elif x < width * 0.2 and text.isdigit():
            try:
                val = int(text)
                if 40 < val < 220 and 'heart_rate' not in classified:
                    if y > height * 0.3:
                        classified['heart_rate'] = add_padding_to_region(region, 15)
            except:
                pass

        elif 'rpm' in text.lower() and x < width * 0.3:
            classified['cadence'] = add_padding_to_region(region, 15)
        elif x < width * 0.2 and text.isdigit():
            try:
                val = int(text)
                if 30 < val < 150 and 'cadence' not in classified:
                    if height * 0.25 < y < height * 0.35:
                        classified['cadence'] = add_padding_to_region(region, 15)
            except:
                pass

        elif '%' in text and x < width * 0.4:
            classified['gradient'] = add_padding_to_region(region, 15)

        elif 'km' in text.lower() and x > width * 0.6 and y < height * 0.2:
            classified['distance_to_finish'] = add_padding_to_region(region, 15)

    leaderboard_regions = [r for r in regions if r['x'] > width * 0.7]
    if len(leaderboard_regions) > 3:
        min_x = min(r['x'] for r in leaderboard_regions)
        min_y = min(r['y'] for r in leaderboard_regions)
        max_x = max(r['x'] + r['width'] for r in leaderboard_regions)
        max_y = max(r['y'] + r['height'] for r in leaderboard_regions)

        classified['leaderboard'] = {
            'x': min_x,
            'y': min_y,
            'width': max_x - min_x,
            'height': max_y - min_y,
            'text': 'leaderboard_area',
            'confidence': 1.0,
        }

    classified['rider_pose_avatar'] = {
        'x': int(width * 0.4),
        'y': int(height * 0.35),
        'width': int(width * 0.2),
        'height': int(height * 0.3),
        'text': 'avatar_area',
        'confidence': 0.5,
    }

    return classified


def visualize_regions(image_path: str, classified: Dict[str, Dict], output_path: str = None):
    """Create visualization of detected regions"""

    img = cv2.imread(image_path)

    colors = {
        'speed': (0, 255, 0),
        'power': (255, 0, 0),
        'distance': (0, 255, 255),
        'altitude': (255, 255, 0),
        'race_time': (255, 0, 255),
        'heart_rate': (0, 0, 255),
        'cadence': (128, 255, 0),
        'gradient': (255, 128, 0),
        'distance_to_finish': (128, 0, 255),
        'leaderboard': (255, 255, 255),
        'rider_pose_avatar': (128, 128, 128),
    }

    for name, region in classified.items():
        x, y = region['x'], region['y']
        w, h = region['width'], region['height']
        color = colors.get(name, (0, 255, 0))

        cv2.rectangle(img, (x, y), (x + w, y + h), color, 2)

        label = f'{name}: {region.get("text", "")[:20]}'
        cv2.putText(img, label, (x, y - 5), cv2.FONT_HERSHEY_SIMPLEX, 0.5, color, 2)

    if output_path:
        cv2.imwrite(output_path, img)
        print(f'Visualization saved to: {output_path}')
    else:
        cv2.imshow('Detected OCR Regions', img)
        cv2.waitKey(0)
        cv2.destroyAllWindows()


def main():
    """Main calibration function"""
    import sys

    if len(sys.argv) < 2:
        print('Usage: python calibrate_with_paddleocr.py <screenshot.png> [--debug] [--visualize]')
        sys.exit(1)

    image_path = sys.argv[1]
    debug = '--debug' in sys.argv
    visualize = '--visualize' in sys.argv

    regions, width, height = detect_all_text_regions(image_path, debug)

    classified = classify_regions(regions, width, height)

    config = {
        'version': '1.0.0',
        'resolution': f'{width}x{height}',
        'zwift_version': '1.67.0',
        'created': datetime.now().isoformat(),
        'regions': {},
    }

    for name, region in classified.items():
        config['regions'][name] = {
            'x': region['x'],
            'y': region['y'],
            'width': region['width'],
            'height': region['height'],
            'note': region.get('text', '')[:50] if region.get('text') else '',
        }

    config['notes'] = 'Auto-detected using PaddleOCR full-image scan'
    config['calibration_image'] = Path(image_path).name

    output_file = f'ocr-configs/{width}x{height}_v1.67.0_auto.json'
    Path('ocr-configs').mkdir(exist_ok=True)

    with open(output_file, 'w') as f:
        json.dump(config, f, indent=2)

    raw_output_file = f'ocr-configs/{width}x{height}_v1.67.0_all_detections.json'
    raw_data = {
        'resolution': f'{width}x{height}',
        'total_detections': len(regions),
        'detections': [
            {
                'text': r['text'],
                'x': r['x'],
                'y': r['y'],
                'width': r['width'],
                'height': r['height'],
                'confidence': r['confidence'],
            }
            for r in regions
        ],
    }
    with open(raw_output_file, 'w') as f:
        json.dump(raw_data, f, indent=2)
    print(f'\nAll detections saved to: {raw_output_file}')

    print(f'\nConfiguration saved to: {output_file}')

    print('\nDetection Summary:')
    print('-' * 50)
    expected = [
        'speed',
        'power',
        'distance',
        'altitude',
        'race_time',
        'heart_rate',
        'cadence',
        'gradient',
        'distance_to_finish',
        'leaderboard',
    ]

    for field in expected:
        if field in classified:
            region = classified[field]
            print(
                f"✓ {field:<20} at ({region['x']:4}, {region['y']:4}) - '{region.get('text', 'N/A')}'"
            )
        else:
            print(f'✗ {field:<20} NOT DETECTED')

    print('\nNOTE: Please review and adjust the generated config.')
    print('Some regions may need manual adjustment for accuracy.')

    if visualize:
        viz_path = output_file.replace('.json', '_visualization.png')
        visualize_regions(image_path, classified, viz_path)


if __name__ == '__main__':
    main()
