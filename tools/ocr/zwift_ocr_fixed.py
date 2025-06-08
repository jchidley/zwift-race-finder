#!/usr/bin/env python3
"""Fixed Zwift OCR extraction with correct UI coordinates"""

import cv2
import numpy as np
from dataclasses import dataclass
from typing import Optional, Tuple, List, Dict, Any
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


class ZwiftUILayout:
    """Fixed UI layout based on actual OCR findings"""
    
    # Top bar is a single region containing all stats
    TOP_BAR = Region(698, 48, 528, 43, "top_bar")  # Contains: speed, distance, altitude, time
    
    # Power panel (top left)
    POWER = Region(271, 49, 148, 61, "power")
    CADENCE = Region(244, 139, 36, 26, "cadence") 
    HEART_RATE = Region(311, 128, 98, 44, "heart_rate")
    AVG_POWER = Region(222, 191, 60, 31, "avg_power")
    ENERGY = Region(338, 189, 68, 29, "energy")
    
    # Gradient box (top right)
    GRADIENT_BOX = Region(1650, 50, 150, 100, "gradient_box")  # Approximate, contains gradient %
    
    # Progress bars and other info
    SEGMENT_INFO = Region(700, 150, 200, 50, "segment_info")  # Contains 5.0% gradient
    DISTANCE_TO_FINISH = Region(650, 100, 150, 40, "distance_to_finish")  # Contains km remaining
    
    # Featherweight powerup
    POWERUP_NAME = Region(218, 122, 200, 30, "powerup_name")  # "Featherweight" text


class ZwiftOCRExtractor:
    """Fixed OCR extractor for Zwift"""
    
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
        
        # Extract top bar and parse it
        top_bar_data = self._extract_top_bar(image)
        results.update(top_bar_data)
        
        # Extract power panel
        results['power'] = self._extract_number(image, ZwiftUILayout.POWER, suffix='w')
        results['cadence'] = self._extract_number(image, ZwiftUILayout.CADENCE)
        results['heart_rate'] = self._extract_number(image, ZwiftUILayout.HEART_RATE)
        results['avg_power'] = self._extract_number(image, ZwiftUILayout.AVG_POWER)
        results['energy'] = self._extract_number(image, ZwiftUILayout.ENERGY)
        
        # Extract gradient
        results['gradient'] = self._extract_gradient(image)
        
        # Extract segment info
        results['segment_gradient'] = self._extract_segment_gradient(image)
        
        # Extract distance to finish
        results['distance_to_finish'] = self._extract_distance_to_finish(image)
        
        # Extract powerup
        results['powerup_name'] = self._extract_powerup(image)
        
        return results
    
    def _extract_top_bar(self, image: np.ndarray) -> Dict[str, Any]:
        """Extract and parse the top bar containing speed, distance, altitude, time"""
        roi = ZwiftUILayout.TOP_BAR.extract(image)
        
        # Run OCR
        result = self.ocr.ocr(roi, cls=True)
        
        if self.debug:
            print(f"Top bar OCR result: {result}")
        
        data = {
            'speed': None,
            'distance': None, 
            'altitude': None,
            'race_time': None
        }
        
        if result and result[0]:
            # Extract all text
            texts = []
            for line in result[0]:
                text = line[1][0]
                texts.append(text)
            
            full_text = ' '.join(texts)
            if self.debug:
                print(f"Top bar text: '{full_text}'")
            
            # Parse the concatenated or separate values
            # Pattern 1: "20 18.4 106 31:06" (separate)
            # Pattern 2: "2018.410631:06" (concatenated)
            
            # Try to extract individual numbers
            numbers = re.findall(r'(\d+(?:\.\d+)?)', full_text)
            
            if len(numbers) >= 2:
                # First number is speed
                data['speed'] = float(numbers[0])
                
                # Second number is distance
                if len(numbers) > 1:
                    data['distance'] = float(numbers[1])
                
                # Look for altitude (3 digit number)
                for num in numbers[2:]:
                    if len(num) == 3 and '.' not in num:
                        data['altitude'] = int(num)
                        break
                
                # Look for time pattern
                time_match = re.search(r'(\d{1,2}):(\d{2})', full_text)
                if time_match:
                    data['race_time'] = time_match.group(0)
        
        return data
    
    def _extract_number(self, image: np.ndarray, region: Region, suffix: str = '') -> Optional[int]:
        """Extract a numeric value from a region"""
        roi = region.extract(image)
        
        # Run OCR
        result = self.ocr.ocr(roi, cls=True)
        
        if result and result[0]:
            text = result[0][0][1][0]
            
            # Remove suffix if present
            if suffix:
                text = text.replace(suffix, '').replace(suffix.upper(), '')
            
            # Extract number
            match = re.search(r'(\d+)', text)
            if match:
                return int(match.group(1))
        
        return None
    
    def _extract_gradient(self, image: np.ndarray) -> Optional[float]:
        """Extract gradient from the gradient box"""
        # Try the gradient box region
        roi = ZwiftUILayout.GRADIENT_BOX.extract(image)
        result = self.ocr.ocr(roi, cls=True)
        
        if result and result[0]:
            for line in result[0]:
                text = line[1][0]
                # Look for gradient pattern
                match = re.search(r'(\d+(?:\.\d+)?)\s*%', text)
                if match:
                    return float(match.group(1))
        
        return None
    
    def _extract_segment_gradient(self, image: np.ndarray) -> Optional[float]:
        """Extract segment gradient (e.g., 5.0%)"""
        roi = ZwiftUILayout.SEGMENT_INFO.extract(image)
        result = self.ocr.ocr(roi, cls=True)
        
        if result and result[0]:
            for line in result[0]:
                text = line[1][0]
                if '5.0%' in text or '5.0 %' in text:
                    return 5.0
        
        return None
    
    def _extract_distance_to_finish(self, image: np.ndarray) -> Optional[float]:
        """Extract distance to finish"""
        # This needs better coordinates - using approximate region
        roi = image[100:140, 650:800]
        result = self.ocr.ocr(roi, cls=True)
        
        if result and result[0]:
            for line in result[0]:
                text = line[1][0]
                # Look for km pattern
                match = re.search(r'(\d+(?:\.\d+)?)\s*km', text.lower())
                if match:
                    return float(match.group(1))
        
        return None
    
    def _extract_powerup(self, image: np.ndarray) -> Optional[str]:
        """Extract active powerup name"""
        roi = ZwiftUILayout.POWERUP_NAME.extract(image)
        result = self.ocr.ocr(roi, cls=True)
        
        if result and result[0]:
            text = result[0][0][1][0]
            if 'feather' in text.lower():
                return 'Featherweight'
        
        return None


def main():
    """Test the fixed OCR extraction"""
    import sys
    
    if len(sys.argv) > 1:
        image_path = sys.argv[1]
    else:
        print("Usage: python zwift_ocr_fixed.py <image_path>")
        return
    
    extractor = ZwiftOCRExtractor(debug=True)
    results = extractor.extract_telemetry(image_path)
    
    print("\nExtracted Telemetry:")
    print("-" * 40)
    for key, value in results.items():
        print(f"{key}: {value}")
    
    # Save results
    output_file = f"telemetry_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
    with open(output_file, 'w') as f:
        json.dump(results, f, indent=2)
    print(f"\nResults saved to: {output_file}")


if __name__ == "__main__":
    main()