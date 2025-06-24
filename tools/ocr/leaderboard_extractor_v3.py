#!/usr/bin/env python3
"""Improved leaderboard extraction with better name detection"""

import re
import warnings
from dataclasses import dataclass
from typing import List, Optional

import cv2
from paddleocr import PaddleOCR

warnings.filterwarnings('ignore', category=UserWarning, module='paddle')


@dataclass
class LeaderboardEntry:
    """Represents a single leaderboard entry"""

    position: int
    name: str
    time_delta: Optional[str]  # None for current rider
    watts_per_kg: float
    distance_km: float
    is_current_rider: bool = False


def is_likely_name(text: str) -> bool:
    """Determine if text is likely a rider name"""
    # Filter out obvious non-names
    if 'KM' in text.upper():
        return False
    if 'w/kg' in text.lower():
        return False
    if text.replace('.', '').replace(',', '').isdigit():
        return False

    # Positive indicators for names
    # Has dots between letters (J.Chidley)
    if re.match(r'^[A-Z]\.[A-Za-z]', text):
        return True
    # Multiple dots (C.J.Y.S)
    if text.count('.') >= 2 and any(c.isalpha() for c in text):
        return True
    # Starts with uppercase and has lowercase (Laindre)
    if re.match(r'^[A-Z][a-z]', text):
        return True
    # Contains parenthesis (J.T.Noxen))
    if '(' in text or ')' in text:
        return True
    # Single letter with dot
    if re.match(r'^[A-Z]\.$', text):
        return True

    return False


def extract_leaderboard_improved(image_path: str, debug: bool = False) -> List[LeaderboardEntry]:
    """Extract leaderboard with improved logic"""

    img = cv2.imread(image_path)
    ocr = PaddleOCR(use_angle_cls=True, lang='en', show_log=False)

    # Leaderboard area - you might need to adjust these coordinates
    x, y, w, h = 1500, 200, 420, 600
    roi = img[y : y + h, x : x + w]

    # Preprocess for better OCR
    gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY)
    # Enhance contrast
    clahe = cv2.createCLAHE(clipLimit=2.0, tileGridSize=(8, 8))
    enhanced = clahe.apply(gray)

    result = ocr.ocr(enhanced, cls=True)
    if not result or not result[0]:
        return []

    # Collect all detections with positions
    detections = []
    for detection in result[0]:
        bbox, (text, conf) = detection
        y_top = bbox[0][1]
        y_bottom = bbox[2][1]
        y_center = (y_top + y_bottom) / 2
        x_left = bbox[0][0]

        detections.append(
            {
                'text': text,
                'y_center': y_center,
                'y_top': y_top,
                'y_bottom': y_bottom,
                'x': x_left,
                'height': y_bottom - y_top,
                'conf': conf,
            }
        )

    # Sort by Y position
    detections.sort(key=lambda d: d['y_center'])

    if debug:
        print('Filtered detections:')
        for d in detections:
            print(
                f"Y={d['y_center']:.1f}, X={d['x']:.1f}, Text='{d['text']}', IsName={is_likely_name(d['text'])}"
            )

    # Find rider names
    names = [d for d in detections if is_likely_name(d['text'])]

    # Skip the event title if it exists
    if names and names[0]['y_center'] < 180:  # Adjust threshold as needed
        event_title = names.pop(0)
        if debug:
            print(f'Skipping event title: {event_title["text"]}')

    entries = []

    for i, name_det in enumerate(names):
        # Initialize entry
        entry = LeaderboardEntry(
            position=i + 1,
            name=name_det['text'].strip(),
            time_delta=None,
            watts_per_kg=0.0,
            distance_km=0.0,
        )

        # Define the data row region (typically 15-35 pixels below name)
        name_y = name_det['y_center']
        data_row_min_y = name_y + 10
        data_row_max_y = name_y + 40

        # Find all detections in the data row
        data_row = [d for d in detections if data_row_min_y <= d['y_center'] <= data_row_max_y]

        if debug and data_row:
            print(f'\nData row for {entry.name}:')
            for d in data_row:
                print(f'  {d["text"]}')

        # Process each detection in the data row
        for det in data_row:
            text = det['text'].strip()

            # Time delta (contains : and +/-)
            if ':' in text and any(c in text for c in ['+', '-']):
                entry.time_delta = text
            # Distance (contains KM)
            elif 'KM' in text.upper():
                num_match = re.search(r'(\d+\.?\d*)', text)
                if num_match:
                    entry.distance_km = float(num_match.group(1))
            # W/kg (contains w/kg or is a decimal in the middle region)
            elif 'w/kg' in text.lower() or 'wkg' in text.lower():
                num_match = re.search(r'(\d+\.?\d*)', text)
                if num_match:
                    entry.watts_per_kg = float(num_match.group(1))
            # Check position to determine if it's w/kg (middle column)
            elif 80 < det['x'] < 180 and '.' in text:
                # Likely w/kg value based on position
                num_match = re.search(r'(\d+\.?\d*)', text)
                if num_match:
                    value = float(num_match.group(1))
                    if 0.5 <= value <= 7.0:  # Reasonable w/kg range
                        entry.watts_per_kg = value

        # Determine if this is the current rider
        # Current rider has no time delta but has other data
        if entry.time_delta is None and (entry.watts_per_kg > 0 or entry.distance_km > 0):
            entry.is_current_rider = True

        # Only add entries with at least some data
        if entry.watts_per_kg > 0 or entry.distance_km > 0 or entry.time_delta:
            entries.append(entry)

    return entries


def update_leaderboard_extraction_in_main_script():
    """Generate the updated leaderboard extraction method"""
    print('\n\nUpdated _extract_leaderboard_structured method:')
    print('=' * 60)
    print(
        '''
    def _extract_leaderboard_structured(self, image: np.ndarray) -> List[Dict[str, Any]]:
        """Extract leaderboard with two-row structure per entry"""
        roi = ZwiftUILayoutFinal.LEADERBOARD_AREA.extract(image)
        
        # Preprocess for better OCR
        gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY)
        clahe = cv2.createCLAHE(clipLimit=2.0, tileGridSize=(8,8))
        enhanced = clahe.apply(gray)
        
        result = self.ocr.ocr(enhanced, cls=True)
        if not result or not result[0]:
            return []
        
        # Helper function to identify names
        def is_likely_name(text: str) -> bool:
            if 'KM' in text.upper() or 'w/kg' in text.lower():
                return False
            if text.replace('.', '').isdigit():
                return False
            # Positive indicators
            if re.match(r'^[A-Z]\.[A-Za-z]', text) or text.count('.') >= 2:
                return True
            if re.match(r'^[A-Z][a-z]', text) or '(' in text or ')' in text:
                return True
            return False
        
        # Collect detections
        detections = []
        for detection in result[0]:
            bbox, (text, conf) = detection
            y_center = (bbox[0][1] + bbox[2][1]) / 2
            x_left = bbox[0][0]
            detections.append({
                'text': text.strip(),
                'y': y_center,
                'x': x_left
            })
        
        # Sort by Y position
        detections.sort(key=lambda d: d['y'])
        
        # Find names and build entries
        names = [d for d in detections if is_likely_name(d['text'])]
        entries = []
        
        for i, name_det in enumerate(names):
            if name_det['y'] < 180:  # Skip event title
                continue
                
            entry = {
                'position': len(entries) + 1,
                'name': name_det['text'],
                'time_delta': None,
                'watts_per_kg': None,
                'distance_km': None,
                'is_current_rider': False
            }
            
            # Find data in row below name (15-40 pixels)
            data_row = [d for d in detections 
                       if name_det['y'] + 10 <= d['y'] <= name_det['y'] + 40]
            
            for det in data_row:
                text = det['text']
                # Time delta
                if ':' in text and any(c in text for c in ['+', '-']):
                    entry['time_delta'] = text
                # Distance
                elif 'KM' in text.upper():
                    match = re.search(r'(\d+\.?\d*)', text)
                    if match:
                        entry['distance_km'] = float(match.group(1))
                # W/kg (middle column position)
                elif 80 < det['x'] < 180 and '.' in text:
                    match = re.search(r'(\d+\.?\d*)', text)
                    if match:
                        value = float(match.group(1))
                        if 0.5 <= value <= 7.0:
                            entry['watts_per_kg'] = value
            
            # Current rider has no time delta
            if entry['time_delta'] is None and (entry['watts_per_kg'] or entry['distance_km']):
                entry['is_current_rider'] = True
            
            if entry['watts_per_kg'] or entry['distance_km'] or entry['time_delta']:
                entries.append(entry)
        
        return entries
    '''
    )


if __name__ == '__main__':
    import sys

    if len(sys.argv) > 1:
        image_path = sys.argv[1]
    else:
        image_path = '../../docs/screenshots/normal_1_01_16_02_21.jpg'

    print('Extracting leaderboard with improved logic...')
    entries = extract_leaderboard_improved(image_path, debug=True)

    print('\n\nExtracted entries:')
    print('-' * 60)
    for entry in entries:
        marker = ' ⬅️  YOU' if entry.is_current_rider else ''
        time_str = entry.time_delta if entry.time_delta else '---'
        print(
            f'{entry.position}. {entry.name:<15} {time_str:>6}  '
            f'{entry.watts_per_kg:>3.1f} w/kg  '
            f'{entry.distance_km:>4.1f} km{marker}'
        )

    # Show the code to update
    update_leaderboard_extraction_in_main_script()
