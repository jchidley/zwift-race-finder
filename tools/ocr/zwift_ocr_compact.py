#!/usr/bin/env python3
"""Compact Zwift OCR extractor - essential telemetry only"""

import json
import re
import warnings

import cv2

warnings.filterwarnings('ignore', category=UserWarning, module='paddle')
from paddleocr import PaddleOCR


class ZwiftOCR:
    def __init__(self):
        self.ocr = PaddleOCR(use_angle_cls=True, lang='en', show_log=False)
        # Optimized regions from visual mapper
        self.regions = {
            'speed': (693, 44, 71, 61),
            'distance': (833, 44, 84, 55),
            'altitude': (975, 45, 75, 50),
            'race_time': (1070, 45, 134, 49),
            'power': (268, 49, 117, 61),
            'cadence': (240, 135, 45, 31),
            'heart_rate': (341, 129, 69, 38),
            'gradient': (1695, 71, 50, 50),
            'distance_to_finish': (1143, 138, 50, 27),
            'leaderboard': (1500, 200, 420, 600),
        }

    def extract(self, image_path: str) -> dict:
        img = cv2.imread(image_path)
        if img is None:
            return {}

        results = {}

        # Extract numeric fields
        for field in ['speed', 'altitude', 'power', 'cadence', 'heart_rate']:
            results[field] = self._extract_number(img, field)

        # Extract decimal fields
        for field in ['distance', 'distance_to_finish']:
            results[field] = self._extract_decimal(img, field)

        # Special extractions
        results['race_time'] = self._extract_time(img)
        results['gradient'] = self._extract_gradient(img)
        results['leaderboard'] = self._extract_leaderboard(img)

        return results

    def _get_roi(self, img, field):
        x, y, w, h = self.regions[field]
        return img[y : y + h, x : x + w]

    def _preprocess_top_bar(self, roi):
        gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY)
        _, binary = cv2.threshold(gray, 200, 255, cv2.THRESH_BINARY)
        return cv2.resize(binary, None, fx=3, fy=3, interpolation=cv2.INTER_CUBIC)

    def _extract_number(self, img, field):
        roi = self._get_roi(img, field)
        processed = self._preprocess_top_bar(roi)
        result = self.ocr.ocr(processed, cls=True)
        if result and result[0]:
            text = re.sub(r'[^0-9]', '', result[0][0][1][0])
            return int(text) if text else None
        return None

    def _extract_decimal(self, img, field):
        roi = self._get_roi(img, field)
        processed = self._preprocess_top_bar(roi)
        result = self.ocr.ocr(processed, cls=True)
        if result and result[0]:
            text = re.sub(r'[^0-9.]', '', result[0][0][1][0])
            try:
                return float(text)
            except:
                return None
        return None

    def _extract_time(self, img):
        roi = self._get_roi(img, 'race_time')
        processed = self._preprocess_top_bar(roi)
        result = self.ocr.ocr(processed, cls=True)
        if result and result[0]:
            text = result[0][0][1][0]
            match = re.search(r'(\d{1,2}:\d{2})', text)
            if match:
                return match.group(1)
            # Insert colon if missing
            digits = re.sub(r'[^0-9]', '', text)
            if len(digits) == 4:
                return f'{digits[:2]}:{digits[2:]}'
            elif len(digits) == 3:
                return f'{digits[0]}:{digits[1:]}'
        return None

    def _extract_gradient(self, img):
        roi = self._get_roi(img, 'gradient')
        gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY)
        # Special processing for stylized gradient font
        inverted = cv2.bitwise_not(gray)
        _, binary = cv2.threshold(inverted, 100, 255, cv2.THRESH_BINARY)
        scaled = cv2.resize(binary, None, fx=4, fy=4, interpolation=cv2.INTER_CUBIC)
        result = self.ocr.ocr(scaled, cls=True)
        if result and result[0]:
            text = re.sub(r'[^0-9.]', '', result[0][0][1][0])
            try:
                return float(text)
            except:
                return None
        return None

    def _extract_leaderboard(self, img):
        roi = self._get_roi(img, 'leaderboard')
        gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY)
        clahe = cv2.createCLAHE(clipLimit=2.0, tileGridSize=(8, 8))
        enhanced = clahe.apply(gray)

        result = self.ocr.ocr(enhanced, cls=True)
        if not result or not result[0]:
            return []

        # Collect all detections
        detections = []
        for det in result[0]:
            bbox, (text, _) = det
            y = (bbox[0][1] + bbox[2][1]) / 2
            x = bbox[0][0]
            detections.append({'text': text.strip(), 'y': y, 'x': x})

        detections.sort(key=lambda d: d['y'])

        # Find names (contain dots or start with uppercase)
        def is_name(text):
            if 'KM' in text.upper() or 'w/kg' in text.lower():
                return False
            return bool(re.match(r'^[A-Z]\.[A-Za-z]|^[A-Z][a-z]|.*\..*[A-Za-z]', text))

        names = [d for d in detections if is_name(d['text']) and d['y'] > 180]
        entries = []

        for name_det in names:
            # Get data from row below (10-40 pixels)
            data_row = [d for d in detections if name_det['y'] + 10 <= d['y'] <= name_det['y'] + 40]

            entry = {'name': name_det['text'], 'current': False}

            for d in data_row:
                text = d['text']
                # Time delta
                if ':' in text and any(c in text for c in ['+', '-']):
                    entry['delta'] = text
                # Distance
                elif 'KM' in text.upper():
                    m = re.search(r'(\d+\.?\d*)', text)
                    if m:
                        entry['km'] = float(m.group(1))
                # W/kg (middle column)
                elif 80 < d['x'] < 180:
                    m = re.search(r'(\d+\.?\d*)', text)
                    if m:
                        val = float(m.group(1))
                        if 0.5 <= val <= 7.0:
                            entry['wkg'] = val

            # Current rider has no time delta
            if 'delta' not in entry and ('wkg' in entry or 'km' in entry):
                entry['current'] = True

            if len(entry) > 2:  # Has name + at least one data field
                entries.append(entry)

        return entries


# Usage
if __name__ == '__main__':
    import sys

    if len(sys.argv) > 1:
        ocr = ZwiftOCR()
        results = ocr.extract(sys.argv[1])

        # Display results
        print(f'Speed: {results.get("speed")} km/h')
        print(f'Distance: {results.get("distance")} km')
        print(f'Altitude: {results.get("altitude")} m')
        print(f'Time: {results.get("race_time")}')
        print(f'Power: {results.get("power")} W')
        print(f'Cadence: {results.get("cadence")} rpm')
        print(f'HR: {results.get("heart_rate")} bpm')
        print(f'Gradient: {results.get("gradient")}%')
        print(f'To finish: {results.get("distance_to_finish")} km')

        if results.get('leaderboard'):
            print('\nLeaderboard:')
            for i, e in enumerate(results['leaderboard'], 1):
                you = ' <-- YOU' if e.get('current') else ''
                delta = e.get('delta', '---')
                print(
                    f'{i}. {e["name"]:<15} {delta:>6} {e.get("wkg", 0):>3.1f}w/kg {e.get("km", 0):>4.1f}km{you}'
                )

        # Save JSON
        with open('telemetry.json', 'w') as f:
            json.dump(results, f, indent=2)
