#!/usr/bin/env python3
# ABOUTME: Improved OCR extraction for Zwift with accurate UI region mapping
# Uses precise coordinates based on actual Zwift screenshots

import cv2
import numpy as np
from pathlib import Path
import json
import time
from typing import Dict, List, Tuple, Optional, Union
import re
from dataclasses import dataclass
from enum import Enum

# Try to import both OCR libraries
try:
    from paddleocr import PaddleOCR
    PADDLE_AVAILABLE = True
except ImportError:
    PADDLE_AVAILABLE = False

try:
    import easyocr
    EASY_AVAILABLE = True
except ImportError:
    EASY_AVAILABLE = False


@dataclass
class Region:
    """Define a region of interest in the image"""
    x: int
    y: int
    width: int
    height: int
    name: str
    
    def extract(self, image: np.ndarray) -> np.ndarray:
        """Extract this region from an image"""
        return image[self.y:self.y+self.height, self.x:self.x+self.width]


class ZwiftUILayout:
    """Define the Zwift UI layout based on 1920x1080 resolution"""
    
    # Top middle HUD bar regions (corrected based on OCR)
    SPEED = Region(698, 48, 120, 43, "speed")              # km/h
    DISTANCE = Region(826, 48, 120, 43, "distance")        # km traveled
    ALTITUDE = Region(954, 48, 120, 43, "altitude")        # current altitude in meters
    RACE_TIME = Region(1082, 48, 144, 43, "race_time")     # mm:ss elapsed
    
    # Top left power panel (corrected based on OCR)
    POWER = Region(271, 49, 148, 61, "power")              # current watts
    CADENCE = Region(244, 139, 36, 26, "cadence")         # current RPM
    HEART_RATE = Region(311, 128, 98, 44, "heart_rate")   # current BPM
    AVG_POWER = Region(222, 191, 60, 31, "avg_power")     # average watts
    ENERGY = Region(338, 189, 68, 29, "energy")           # kJ expended
    
    # Progress bars
    XP_PROGRESS = Region(850, 80, 150, 30, "xp_progress")
    ROUTE_PROGRESS = Region(730, 110, 200, 30, "route_progress")
    
    # Distance to finish (visible in races/routes)
    DISTANCE_TO_FINISH = Region(900, 110, 100, 30, "distance_to_finish")
    
    # Sprint/Segment (when active)
    SEGMENT_GRADIENT = Region(290, 150, 80, 40, "segment_gradient")
    SEGMENT_TIME = Region(380, 150, 120, 40, "segment_time")
    
    # Gradient indicator (top right, visible during climbs - found at 5%)
    GRADIENT = Region(1708, 81, 26, 34, "gradient")
    
    # Power-up indicator (center-right when active)
    POWERUP_ACTIVE = Region(900, 200, 150, 150, "powerup_active")
    POWERUP_NAME = Region(880, 360, 180, 40, "powerup_name")
    
    # Mini map region
    MINIMAP = Region(1030, 30, 250, 200, "minimap")
    
    # Rider list (right side)
    RIDER_LIST = Region(1130, 250, 320, 500, "rider_list")
    
    # Rider avatar region (center of screen)
    RIDER_AVATAR = Region(860, 400, 200, 300, "rider_avatar")
    
    @classmethod
    def get_all_regions(cls) -> List[Region]:
        """Get all defined regions"""
        regions = []
        for attr_name in dir(cls):
            attr = getattr(cls, attr_name)
            if isinstance(attr, Region):
                regions.append(attr)
        return regions


class ZwiftOCRExtractor:
    """Enhanced OCR extractor specifically tuned for Zwift UI"""
    
    def __init__(self, ocr_engine='paddle', debug=False):
        self.ocr_engine = ocr_engine
        self.debug = debug
        
        if ocr_engine == 'paddle' and PADDLE_AVAILABLE:
            self.ocr = PaddleOCR(
                use_angle_cls=True, 
                lang='en',
                use_gpu=False,
                show_log=False
            )
        elif ocr_engine == 'easy' and EASY_AVAILABLE:
            self.ocr = easyocr.Reader(['en'], gpu=False, verbose=False)
        else:
            raise ValueError(f"OCR engine '{ocr_engine}' not available")
    
    def preprocess_for_ocr(self, image: np.ndarray, enhance_contrast=True) -> np.ndarray:
        """
        Preprocess image for optimal OCR results on Zwift UI
        
        Zwift UI characteristics:
        - White/bright text on dark backgrounds
        - Semi-transparent overlays
        - High contrast UI elements
        """
        # Convert to grayscale if needed
        if len(image.shape) == 3:
            gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
        else:
            gray = image.copy()
        
        # Enhance contrast for better text detection
        if enhance_contrast:
            # Apply CLAHE (Contrast Limited Adaptive Histogram Equalization)
            clahe = cv2.createCLAHE(clipLimit=2.0, tileGridSize=(8,8))
            gray = clahe.apply(gray)
        
        # Zwift text is typically white/bright, so we threshold to isolate it
        _, binary = cv2.threshold(gray, 180, 255, cv2.THRESH_BINARY)
        
        # Remove noise
        kernel = np.ones((2,2), np.uint8)
        cleaned = cv2.morphologyEx(binary, cv2.MORPH_CLOSE, kernel)
        
        # Scale up for better OCR accuracy
        scaled = cv2.resize(cleaned, None, fx=2, fy=2, interpolation=cv2.INTER_CUBIC)
        
        if self.debug:
            cv2.imwrite(f"debug_preprocessed_{time.time()}.png", scaled)
        
        return scaled
    
    def extract_text(self, image: np.ndarray) -> str:
        """Extract text from image using configured OCR engine"""
        if self.ocr_engine == 'paddle':
            result = self.ocr.ocr(image, cls=True)
            if result and result[0]:
                texts = [line[1][0] for line in result[0]]
                return ' '.join(texts)
        else:  # easyocr
            result = self.ocr.readtext(image)
            if result:
                texts = [item[1] for item in result]
                return ' '.join(texts)
        return ''
    
    def parse_zwift_value(self, text: str, value_type: str) -> Optional[Union[float, str]]:
        """
        Parse Zwift-specific text patterns into values
        
        Args:
            text: Raw OCR text
            value_type: Type of value to parse
            
        Returns:
            Parsed value or None
        """
        text = text.strip().upper()
        
        # Remove common OCR artifacts
        text = text.replace('O', '0').replace('I', '1').replace('L', '1')
        
        patterns = {
            'speed': [
                r'(\d+)\s*KM/H',
                r'(\d+)\s*KMH',
                r'(\d+)\s*K',
                r'^(\d+)$'  # Just the number
            ],
            'distance': [
                r'(\d+\.?\d*)\s*KM',
                r'(\d+\.?\d*)\s*K',
                r'^(\d+\.?\d*)$'
            ],
            'altitude': [
                r'(\d+)\s*M',
                r'^(\d+)$'
            ],
            'race_time': [
                r'(\d+):(\d+):(\d+)',  # HH:MM:SS
                r'(\d+):(\d+)',         # MM:SS
                r'^(\d+)$'              # Just seconds
            ],
            'power': [
                r'(\d+)\s*W',
                r'(\d+)\s*WATTS',
                r'^(\d+)$'
            ],
            'avg_power': [
                r'(\d+)\s*W',
                r'(\d+)\s*WATTS',
                r'AVG\s*(\d+)',
                r'^(\d+)$'
            ],
            'cadence': [
                r'(\d+)\s*RPM',
                r'^(\d+)$'
            ],
            'heart_rate': [
                r'(\d+)\s*BPM',
                r'^(\d+)$'
            ],
            'energy': [
                r'(\d+)\s*KJ',
                r'(\d+)\s*K?JOULES?',
                r'^(\d+)$'
            ],
            'gradient': [
                r'(\d+\.?\d*)\s*%',
                r'(\d+\.?\d*)%',
                r'^(\d+\.?\d*)$'
            ],
            'xp_progress': [
                r'(\d+,?\d*)/(\d+,?\d*)',  # Current/Total with possible commas
                r'(\d+)\s*/\s*(\d+)'
            ],
            'distance_to_finish': [
                r'(\d+\.?\d*)\s*KM',
                r'(\d+\.?\d*)\s*K',
                r'^(\d+\.?\d*)$'
            ],
            'powerup_name': [
                r'FEATHERWEIGHT',
                r'AERO\s*BOOST',
                r'DRAFT\s*BOOST',
                r'LIGHTWEIGHT',
                r'STEAMROLLER',
                r'ANVIL',
                r'BURRITO',
                r'^([A-Z\s]+)$'
            ]
        }
        
        pattern_list = patterns.get(value_type, [])
        
        for pattern in pattern_list:
            match = re.search(pattern, text)
            if match:
                if value_type == 'race_time':
                    if len(match.groups()) == 3:  # HH:MM:SS
                        hours = int(match.group(1))
                        minutes = int(match.group(2))
                        seconds = int(match.group(3))
                        return hours * 3600 + minutes * 60 + seconds
                    elif len(match.groups()) == 2:  # MM:SS
                        minutes = int(match.group(1))
                        seconds = int(match.group(2))
                        return minutes * 60 + seconds
                    else:  # Just seconds
                        return int(match.group(1))
                elif value_type == 'xp_progress':
                    if len(match.groups()) == 2:
                        current = match.group(1).replace(',', '')
                        total = match.group(2).replace(',', '')
                        return f"{current}/{total}"
                    return text
                else:
                    return float(match.group(1))
        
        return None
    
    def extract_telemetry(self, image_path: str) -> Dict[str, Dict]:
        """
        Extract all telemetry data from a Zwift screenshot
        
        Returns dict with structure:
        {
            'field_name': {
                'value': parsed_value,
                'raw_text': 'original OCR text',
                'confidence': 0.95
            }
        }
        """
        # Load image
        image = cv2.imread(image_path)
        if image is None:
            raise ValueError(f"Could not load image: {image_path}")
        
        results = {}
        
        # Get all regions to process
        regions = ZwiftUILayout.get_all_regions()
        
        for region in regions:
            # Skip non-telemetry regions
            if region.name in ['minimap', 'rider_list', 'powerup_active', 'rider_avatar']:
                continue
            
            # Extract region
            roi = region.extract(image)
            
            # Preprocess for OCR
            processed = self.preprocess_for_ocr(roi)
            
            # Extract text
            text = self.extract_text(processed)
            
            # Parse value
            value = self.parse_zwift_value(text, region.name)
            
            results[region.name] = {
                'value': value,
                'raw_text': text,
                'region': {
                    'x': region.x,
                    'y': region.y,
                    'width': region.width,
                    'height': region.height
                }
            }
            
            if self.debug:
                print(f"{region.name}: '{text}' -> {value}")
        
        # Special handling for power-up detection
        # Check if power-up name is detected
        if 'powerup_name' in results and results['powerup_name']['value']:
            # Try to estimate remaining time from circular timer
            powerup_remaining = self.estimate_powerup_remaining(image, ZwiftUILayout.POWERUP_ACTIVE)
            results['powerup_remaining'] = {
                'value': powerup_remaining,
                'raw_text': f"{powerup_remaining}%" if powerup_remaining else "N/A",
                'region': {
                    'x': ZwiftUILayout.POWERUP_ACTIVE.x,
                    'y': ZwiftUILayout.POWERUP_ACTIVE.y,
                    'width': ZwiftUILayout.POWERUP_ACTIVE.width,
                    'height': ZwiftUILayout.POWERUP_ACTIVE.height
                }
            }
        
        # Detect rider pose
        rider_pose = self.detect_rider_pose(image, ZwiftUILayout.RIDER_AVATAR)
        results['rider_pose'] = {
            'value': rider_pose,
            'raw_text': rider_pose,
            'region': {
                'x': ZwiftUILayout.RIDER_AVATAR.x,
                'y': ZwiftUILayout.RIDER_AVATAR.y,
                'width': ZwiftUILayout.RIDER_AVATAR.width,
                'height': ZwiftUILayout.RIDER_AVATAR.height
            }
        }
        
        return results
    
    def estimate_powerup_remaining(self, image: np.ndarray, powerup_region: Region) -> Optional[float]:
        """
        Estimate remaining power-up time by analyzing the circular timer
        
        The timer starts full at 3 o'clock and decreases anti-clockwise:
        - 3 o'clock = 100% (full) and 0% (expired)
        - 12 o'clock = 75%
        - 9 o'clock = 50%
        - 6 o'clock = 25%
        - Back to 3 = 0% (expired)
        
        Returns:
            Estimated percentage remaining (0-100)
        """
        roi = powerup_region.extract(image)
        
        # Convert to grayscale
        gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY)
        
        # Look for bright pixels in the circular timer
        # The timer is typically bright/white against darker background
        _, binary = cv2.threshold(gray, 200, 255, cv2.THRESH_BINARY)
        
        # Find center of the region
        center_x = roi.shape[1] // 2
        center_y = roi.shape[0] // 2
        
        # Sample points around the circle at different angles
        # Starting from 3 o'clock (right) and going anti-clockwise
        angles = np.linspace(0, 360, 36)  # Check every 10 degrees
        radius = min(center_x, center_y) * 0.7  # Assume timer is 70% of region size
        
        bright_angles = []
        for angle in angles:
            # Convert angle to radians (0 degrees = 3 o'clock/right)
            # In image coordinates: 0° = right, 90° = down, 180° = left, 270° = up
            rad = np.radians(angle)
            x = int(center_x + radius * np.cos(rad))
            y = int(center_y + radius * np.sin(rad))
            
            # Check if within bounds
            if 0 <= x < roi.shape[1] and 0 <= y < roi.shape[0]:
                if binary[y, x] > 128:  # Bright pixel
                    # Store angle relative to 3 o'clock start position
                    bright_angles.append(angle)
        
        if not bright_angles:
            return None
        
        # Find the maximum angle that's still bright (anti-clockwise from 3 o'clock)
        # The timer depletes anti-clockwise, so larger angles = less time remaining
        max_angle = max(bright_angles)
        
        # Convert to percentage
        # 0 degrees (3 o'clock) = 100% full
        # 360 degrees (back to 3 o'clock) = 0% (expired)
        percentage = (360 - max_angle) / 360 * 100
        
        return round(percentage, 1)
    
    def detect_rider_pose(self, image: np.ndarray, avatar_region: Region) -> str:
        """
        Detect rider pose/position from avatar appearance
        
        Zwift poses (VISUAL ONLY - no aerodynamic impact except supertuck):
        - "seated_hoods": Normal seated, hands on hoods (drafting or <33km/h)
        - "seated_drops": Normal seated, hands on drops (≥33km/h, not drafting)
        - "standing": Out of saddle (31-72 RPM on climbs ≥3%)
        - "supertuck": Descending position (ONLY position that affects speed: -25% drag)
        
        Note: Regular positions are purely visual in Zwift and don't affect speed!
        
        Returns:
            Detected pose as string
        """
        roi = avatar_region.extract(image)
        
        # Convert to grayscale for analysis
        gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY)
        
        # Use edge detection to find rider silhouette
        edges = cv2.Canny(gray, 50, 150)
        
        # Find contours to analyze rider shape
        contours, _ = cv2.findContours(edges, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
        
        if not contours:
            return "unknown"
        
        # Get the largest contour (likely the rider)
        largest_contour = max(contours, key=cv2.contourArea)
        
        # Calculate bounding box
        x, y, w, h = cv2.boundingRect(largest_contour)
        
        # Calculate aspect ratio and center of mass
        aspect_ratio = h / w if w > 0 else 0
        moments = cv2.moments(largest_contour)
        if moments['m00'] > 0:
            cx = int(moments['m10'] / moments['m00'])
            cy = int(moments['m01'] / moments['m00'])
            # Normalize center position
            cx_norm = cx / roi.shape[1]
            cy_norm = cy / roi.shape[0]
        else:
            cx_norm = cy_norm = 0.5
        
        # Analyze the upper portion of the rider silhouette
        upper_half = edges[:roi.shape[0]//2, :]
        upper_density = np.sum(upper_half > 0) / upper_half.size
        
        # Analyze the lower portion
        lower_half = edges[roi.shape[0]//2:, :]
        lower_density = np.sum(lower_half > 0) / lower_half.size
        
        # Decision logic based on shape characteristics
        # Check for supertuck first (very low and compact)
        if aspect_ratio < 0.8 and cy_norm > 0.6:
            # Very low profile = supertuck position
            return "supertuck"  # ONLY position that affects aerodynamics (-25% drag)
        elif aspect_ratio > 1.8 and cy_norm < 0.4:
            # Tall and center of mass is high = standing
            return "standing"  # Visual only, no aero impact
        elif aspect_ratio < 1.3 and upper_density < 0.02:
            # Low profile but not supertuck = drops position
            return "seated_drops"  # Visual only (≥33km/h, not drafting)
        else:
            # Default riding position
            return "seated_hoods"  # Visual only (drafting or <33km/h)
    
    def extract_rider_list(self, image_path: str) -> List[Dict[str, str]]:
        """Extract rider names and positions from the leaderboard"""
        image = cv2.imread(image_path)
        if image is None:
            raise ValueError(f"Could not load image: {image_path}")
        
        # Extract rider list region
        rider_roi = ZwiftUILayout.RIDER_LIST.extract(image)
        
        # Process with different threshold for rider list
        gray = cv2.cvtColor(rider_roi, cv2.COLOR_BGR2GRAY)
        _, binary = cv2.threshold(gray, 200, 255, cv2.THRESH_BINARY)
        
        # Extract all text
        if self.ocr_engine == 'paddle':
            result = self.ocr.ocr(binary, cls=True)
            riders = []
            if result and result[0]:
                for line in result[0]:
                    text = line[1][0]
                    # Parse rider entries - multiple possible formats:
                    # With time gap: "Name +0:00 3.1w/kg 6.3km"
                    # Current rider (no gap): "Name 3.1w/kg 6.3km"
                    # Sometimes with country flags that OCR may garble
                    
                    # Try with time gap first
                    rider_match = re.search(r'(.+?)\s+[+\-]?\d+:\d+\s+(\d+\.?\d*)\s*w/kg\s+(\d+\.?\d*)\s*km', text)
                    if not rider_match:
                        # Try without time gap (current rider)
                        rider_match = re.search(r'(.+?)\s+(\d+\.?\d*)\s*w/kg\s+(\d+\.?\d*)\s*km', text)
                    
                    if rider_match:
                        if len(rider_match.groups()) == 3:
                            riders.append({
                                'name': rider_match.group(1).strip(),
                                'watts_per_kg': float(rider_match.group(2)),
                                'distance_km': float(rider_match.group(3)),
                                'is_current_rider': '+' not in text and '-' not in text
                            })
        else:
            # EasyOCR processing
            result = self.ocr.readtext(binary)
            riders = []
            # Process similar to PaddleOCR
        
        return riders
    
    def visualize_extraction(self, image_path: str, output_path: str):
        """Create visualization showing all extraction regions and results"""
        image = cv2.imread(image_path)
        if image is None:
            raise ValueError(f"Could not load image: {image_path}")
        
        # Extract telemetry
        telemetry = self.extract_telemetry(image_path)
        
        # Define colors for different data types
        colors = {
            'speed': (0, 255, 0),
            'distance': (255, 0, 0),
            'elevation': (0, 0, 255),
            'time': (255, 255, 0),
            'power': (255, 0, 255),
            'cadence': (0, 255, 255),
            'heart_rate': (128, 0, 128),
            'avg_power': (255, 128, 0),
            'power_per_kg': (0, 128, 255),
            'xp_progress': (128, 255, 0),
            'route_progress': (255, 255, 128)
        }
        
        # Draw regions and values
        for field_name, data in telemetry.items():
            region = data['region']
            color = colors.get(field_name, (255, 255, 255))
            
            # Draw rectangle
            cv2.rectangle(image, 
                         (region['x'], region['y']), 
                         (region['x'] + region['width'], region['y'] + region['height']),
                         color, 2)
            
            # Add label with extracted value
            label = f"{field_name}: {data['value']}"
            cv2.putText(image, label, 
                       (region['x'], region['y'] - 5),
                       cv2.FONT_HERSHEY_SIMPLEX, 0.5, color, 1)
        
        cv2.imwrite(output_path, image)
        print(f"Visualization saved to: {output_path}")


def benchmark_ocr_engines(image_paths: List[str]):
    """Benchmark different OCR engines on Zwift screenshots"""
    results = {}
    
    for engine in ['paddle', 'easy']:
        if engine == 'paddle' and not PADDLE_AVAILABLE:
            continue
        if engine == 'easy' and not EASY_AVAILABLE:
            continue
        
        print(f"\nBenchmarking {engine.upper()} OCR...")
        extractor = ZwiftOCRExtractor(engine, debug=False)
        
        engine_results = []
        total_time = 0
        
        for image_path in image_paths:
            start_time = time.time()
            telemetry = extractor.extract_telemetry(image_path)
            elapsed = time.time() - start_time
            total_time += elapsed
            
            # Count successful extractions
            success_count = sum(1 for data in telemetry.values() if data['value'] is not None)
            total_fields = len(telemetry)
            
            engine_results.append({
                'image': image_path,
                'time': elapsed,
                'success_rate': success_count / total_fields if total_fields > 0 else 0,
                'telemetry': telemetry
            })
        
        results[engine] = {
            'total_time': total_time,
            'avg_time': total_time / len(image_paths),
            'results': engine_results
        }
    
    return results


def main():
    """Test the improved OCR extraction on Zwift screenshots"""
    import os
    import sys
    
    # Handle different execution contexts
    if len(sys.argv) > 1:
        # Use command line arguments
        test_images = sys.argv[1:]
    else:
        # Default test images with proper path resolution
        script_dir = os.path.dirname(os.path.abspath(__file__))
        repo_root = os.path.dirname(os.path.dirname(script_dir))
        test_images = [
            os.path.join(repo_root, "docs/screenshots/normal_1_01_16_02_21.jpg"),
            os.path.join(repo_root, "docs/screenshots/with_climbing_1_01_36_01_42.jpg")
        ]
    
    # Check if files exist
    for img in test_images:
        if not os.path.exists(img):
            print(f"Warning: Image not found: {img}")
            # Try relative to current directory
            if os.path.exists(os.path.basename(img)):
                test_images[test_images.index(img)] = os.path.basename(img)
    
    if not PADDLE_AVAILABLE and not EASY_AVAILABLE:
        print("No OCR engines available. Please install:")
        print("  uv add paddlepaddle paddleocr")
        print("  or")
        print("  uv add easyocr")
        return
    
    # Use best available engine
    engine = 'paddle' if PADDLE_AVAILABLE else 'easy'
    print(f"Using {engine.upper()} OCR engine")
    
    extractor = ZwiftOCRExtractor(engine, debug=True)
    
    for image_path in test_images:
        print(f"\n{'='*60}")
        print(f"Processing: {image_path}")
        print('='*60)
        
        # Extract telemetry
        telemetry = extractor.extract_telemetry(image_path)
        
        # Display results
        print("\nExtracted Telemetry:")
        for field, data in telemetry.items():
            if data['value'] is not None:
                print(f"  {field}: {data['value']} (raw: '{data['raw_text']}')")
            else:
                print(f"  {field}: FAILED (raw: '{data['raw_text']}')")
        
        # Create visualization
        output_path = image_path.replace('.jpg', '_extracted.jpg')
        extractor.visualize_extraction(image_path, output_path)
        
        # Save results
        json_path = image_path.replace('.jpg', '_telemetry.json')
        with open(json_path, 'w') as f:
            json.dump(telemetry, f, indent=2)
        print(f"\nResults saved to: {json_path}")
    
    # Run benchmark
    print(f"\n{'='*60}")
    print("Running OCR engine benchmark...")
    print('='*60)
    
    benchmark_results = benchmark_ocr_engines(test_images)
    
    for engine, data in benchmark_results.items():
        print(f"\n{engine.upper()} Performance:")
        print(f"  Total time: {data['total_time']:.2f}s")
        print(f"  Average time per image: {data['avg_time']:.2f}s")
        
        for result in data['results']:
            print(f"  {Path(result['image']).name}:")
            print(f"    Time: {result['time']:.2f}s")
            print(f"    Success rate: {result['success_rate']:.1%}")


if __name__ == "__main__":
    main()