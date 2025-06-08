#!/usr/bin/env python3
"""Improved Zwift OCR extraction with better UI region mapping"""

import cv2
import numpy as np
from dataclasses import dataclass
from typing import Optional, Dict, Any, List
import re
from datetime import datetime
import json

try:
    from paddleocr import PaddleOCR
    PADDLE_AVAILABLE = True
except ImportError:
    PADDLE_AVAILABLE = False
    print("PaddleOCR not available. Install with: uv add paddlepaddle paddleocr")

@dataclass
class Region:
    """Defines a region of interest in the image"""
    x: int
    y: int
    width: int
    height: int
    name: str
    
    def extract(self, image: np.ndarray) -> np.ndarray:
        """Extract this region from the image"""
        return image[self.y:self.y+self.height, self.x:self.x+self.width]


class ZwiftUILayoutV2:
    """Improved UI layout with separate regions for top bar elements"""
    
    # Split top bar into individual elements for better extraction
    SPEED = Region(700, 35, 100, 50, "speed")           # Speed in km/h
    DISTANCE = Region(820, 35, 100, 50, "distance")     # Distance in km  
    ALTITUDE = Region(940, 35, 80, 50, "altitude")      # Altitude in m
    RACE_TIME = Region(1040, 35, 120, 50, "race_time")  # Time mm:ss
    
    # Power panel (top left) - keep existing coordinates
    POWER = Region(271, 49, 148, 61, "power")
    CADENCE = Region(240, 135, 60, 40, "cadence")      # Wider and taller for better OCR
    HEART_RATE = Region(311, 128, 98, 44, "heart_rate")
    AVG_POWER = Region(222, 191, 60, 31, "avg_power")
    ENERGY = Region(338, 189, 68, 29, "energy")
    
    # Distance to finish - below top bar, right side (where 28.6km is visible)
    DISTANCE_TO_FINISH = Region(850, 60, 100, 70, "distance_to_finish")
    
    # Gradient box (smaller, more precise)
    GRADIENT_BOX = Region(1700, 75, 50, 50, "gradient_box")
    
    # Segment info (when active)
    SEGMENT_INFO = Region(700, 150, 200, 50, "segment_info")
    
    # Powerup (when active)
    POWERUP_NAME = Region(218, 122, 200, 30, "powerup_name")
    
    # Leaderboard area
    LEADERBOARD_AREA = Region(1500, 200, 420, 600, "leaderboard_area")


class ZwiftOCRExtractorV2:
    """Improved OCR extractor for Zwift"""
    
    def __init__(self, debug=False):
        if not PADDLE_AVAILABLE:
            raise ImportError("PaddleOCR not available")
        
        self.ocr = PaddleOCR(use_angle_cls=True, lang='en', show_log=False)
        self.debug = debug
    
    def extract_telemetry(self, image_path: str) -> Dict[str, Any]:
        """Extract all telemetry data from screenshot"""
        
        # Load image
        image = cv2.imread(image_path)
        if image is None:
            raise ValueError(f"Could not load image: {image_path}")
        
        results = {}
        
        # Extract each top bar element separately
        results['speed'] = self._extract_number(image, ZwiftUILayoutV2.SPEED, pattern=r'(\d+(?:\.\d+)?)')
        results['distance'] = self._extract_number(image, ZwiftUILayoutV2.DISTANCE, pattern=r'(\d+(?:\.\d+)?)', is_float=True)
        results['altitude'] = self._extract_number(image, ZwiftUILayoutV2.ALTITUDE, pattern=r'(\d+)')
        results['race_time'] = self._extract_time(image, ZwiftUILayoutV2.RACE_TIME)
        
        # Extract power panel
        results['power'] = self._extract_number(image, ZwiftUILayoutV2.POWER, suffix='w')
        results['cadence'] = self._extract_cadence(image, ZwiftUILayoutV2.CADENCE)
        results['heart_rate'] = self._extract_number(image, ZwiftUILayoutV2.HEART_RATE)
        results['avg_power'] = self._extract_number(image, ZwiftUILayoutV2.AVG_POWER)
        results['energy'] = self._extract_number(image, ZwiftUILayoutV2.ENERGY)
        
        # Extract gradient
        results['gradient'] = self._extract_gradient(image, ZwiftUILayoutV2.GRADIENT_BOX)
        
        # Extract distance to finish
        results['distance_to_finish'] = self._extract_distance_to_finish(image)
        
        # Extract segment gradient if visible
        results['segment_gradient'] = self._extract_segment_gradient(image)
        
        # Extract powerup if active
        results['powerup_name'] = self._extract_powerup(image)
        
        # Extract leaderboard
        results['leaderboard'] = self._extract_leaderboard(image)
        
        return results
    
    def _extract_number(self, image: np.ndarray, region: Region, pattern: str = r'(\d+)', 
                       suffix: str = '', is_float: bool = False) -> Optional[float]:
        """Extract a numeric value from a region"""
        roi = region.extract(image)
        
        # Preprocess for better OCR
        gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY)
        _, binary = cv2.threshold(gray, 180, 255, cv2.THRESH_BINARY)
        
        # Run OCR
        result = self.ocr.ocr(binary, cls=True)
        
        if result and result[0]:
            text = result[0][0][1][0]
            
            if self.debug:
                print(f"{region.name} OCR text: '{text}'")
            
            # Remove suffix if present
            if suffix:
                text = text.replace(suffix, '').replace(suffix.upper(), '')
            
            # Extract number
            match = re.search(pattern, text)
            if match:
                value = float(match.group(1)) if is_float else int(match.group(1))
                return value
        
        return None
    
    def _extract_cadence(self, image: np.ndarray, region: Region) -> Optional[int]:
        """Special handling for cadence extraction"""
        roi = region.extract(image)
        
        # Preprocess
        gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY)
        _, binary = cv2.threshold(gray, 180, 255, cv2.THRESH_BINARY)
        
        # Try scaling up for better OCR
        scaled = cv2.resize(binary, None, fx=2, fy=2, interpolation=cv2.INTER_CUBIC)
        
        # Run OCR
        result = self.ocr.ocr(scaled, cls=True)
        
        if result and result[0]:
            text = result[0][0][1][0]
            if self.debug:
                print(f"Cadence OCR text: '{text}'")
            
            # Look for 2-3 digit number
            match = re.search(r'(\d{2,3})', text)
            if match:
                return int(match.group(1))
        
        return None
    
    def _extract_time(self, image: np.ndarray, region: Region) -> Optional[str]:
        """Extract time in mm:ss or hh:mm:ss format"""
        roi = region.extract(image)
        
        # Run OCR
        result = self.ocr.ocr(roi, cls=True)
        
        if result and result[0]:
            text = result[0][0][1][0]
            
            # Look for time patterns
            time_patterns = [
                r'(\d{1,2}:\d{2}:\d{2})',  # hh:mm:ss
                r'(\d{1,2}:\d{2})',         # mm:ss
            ]
            
            for pattern in time_patterns:
                match = re.search(pattern, text)
                if match:
                    return match.group(1)
        
        return None
    
    def _extract_gradient(self, image: np.ndarray, region: Region) -> Optional[float]:
        """Extract gradient percentage"""
        roi = region.extract(image)
        
        # Run OCR
        result = self.ocr.ocr(roi, cls=True)
        
        if result and result[0]:
            text = result[0][0][1][0]
            
            # Look for percentage
            match = re.search(r'(\d+(?:\.\d+)?)\s*%?', text)
            if match:
                return float(match.group(1))
        
        return None
    
    def _extract_distance_to_finish(self, image: np.ndarray) -> Optional[float]:
        """Extract distance to finish below top bar"""
        roi = ZwiftUILayoutV2.DISTANCE_TO_FINISH.extract(image)
        
        result = self.ocr.ocr(roi, cls=True)
        
        if result and result[0]:
            # Concatenate all text in the region
            all_text = ' '.join([line[1][0] for line in result[0]])
            if self.debug:
                print(f"Distance to finish OCR text: '{all_text}'")
            
            # Look for distance pattern
            # Try with space between number and km
            match = re.search(r'(\d+(?:\.\d+)?)\s*km', all_text.lower())
            if match:
                return float(match.group(1))
            
            # Try concatenated (e.g., "28.6km")
            match = re.search(r'(\d+\.\d+)', all_text)
            if match and 'km' in all_text.lower():
                return float(match.group(1))
        
        return None
    
    def _extract_segment_gradient(self, image: np.ndarray) -> Optional[float]:
        """Extract segment gradient if visible"""
        roi = ZwiftUILayoutV2.SEGMENT_INFO.extract(image)
        result = self.ocr.ocr(roi, cls=True)
        
        if result and result[0]:
            for line in result[0]:
                text = line[1][0]
                match = re.search(r'(\d+(?:\.\d+)?)\s*%', text)
                if match:
                    return float(match.group(1))
        
        return None
    
    def _extract_powerup(self, image: np.ndarray) -> Optional[str]:
        """Extract active powerup name"""
        roi = ZwiftUILayoutV2.POWERUP_NAME.extract(image)
        result = self.ocr.ocr(roi, cls=True)
        
        if result and result[0]:
            text = result[0][0][1][0]
            
            # Known powerups
            powerups = {
                'feather': 'Featherweight',
                'aero': 'Aero Boost',
                'draft': 'Draft Boost',
                'lightweight': 'Lightweight',
                'steamroller': 'Steamroller',
                'anvil': 'Anvil',
                'burrito': 'Burrito'
            }
            
            text_lower = text.lower()
            for key, name in powerups.items():
                if key in text_lower:
                    return name
        
        return None
    
    def _extract_leaderboard(self, image: np.ndarray) -> List[Dict[str, Any]]:
        """Extract leaderboard entries"""
        roi = ZwiftUILayoutV2.LEADERBOARD_AREA.extract(image)
        result = self.ocr.ocr(roi, cls=True)
        
        leaderboard = []
        
        if result and result[0]:
            # Process OCR results to find rider entries
            for line in result[0]:
                text = line[1][0]
                
                # Look for patterns like "J.Name" or names with dots
                if '.' in text and any(c.isalpha() for c in text):
                    entry = {'name': text}
                    
                    # Look for associated w/kg and km values
                    # This would need more sophisticated parsing
                    # based on proximity of text elements
                    
                    leaderboard.append(entry)
        
        return leaderboard


def main():
    """Test the improved OCR extraction"""
    import sys
    
    if len(sys.argv) > 1:
        image_path = sys.argv[1]
    else:
        print("Usage: python zwift_ocr_improved_v2.py <image_path>")
        return
    
    extractor = ZwiftOCRExtractorV2(debug=True)
    results = extractor.extract_telemetry(image_path)
    
    print("\nExtracted Telemetry:")
    print("-" * 40)
    for key, value in results.items():
        if value is not None and value != []:
            print(f"✓ {key}: {value}")
        else:
            print(f"✗ {key}: Not found")
    
    # Save results
    output_file = f"telemetry_v2_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
    with open(output_file, 'w') as f:
        json.dump(results, f, indent=2)
    print(f"\nResults saved to: {output_file}")


if __name__ == "__main__":
    main()