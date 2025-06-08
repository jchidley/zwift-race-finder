#!/usr/bin/env python3
# ABOUTME: OCR prototype for extracting telemetry data from Zwift screenshots
# Compares PaddleOCR and EasyOCR for accuracy on Zwift UI elements

import cv2
import numpy as np
from pathlib import Path
import json
import time
from typing import Dict, List, Tuple, Optional
import re

# Try to import both OCR libraries
try:
    from paddleocr import PaddleOCR
    PADDLE_AVAILABLE = True
except ImportError:
    PADDLE_AVAILABLE = False
    print("PaddleOCR not available. Install with: pip install paddlepaddle paddleocr")

try:
    import easyocr
    EASY_AVAILABLE = True
except ImportError:
    EASY_AVAILABLE = False
    print("EasyOCR not available. Install with: pip install easyocr")


class ZwiftTelemetryExtractor:
    """Extract telemetry data from Zwift screenshots/video frames"""
    
    def __init__(self, ocr_engine='paddle'):
        """
        Initialize the extractor with specified OCR engine
        
        Args:
            ocr_engine: 'paddle' or 'easy'
        """
        self.ocr_engine = ocr_engine
        
        if ocr_engine == 'paddle' and PADDLE_AVAILABLE:
            # Initialize PaddleOCR with English language
            self.ocr = PaddleOCR(use_angle_cls=True, lang='en', use_gpu=False)
        elif ocr_engine == 'easy' and EASY_AVAILABLE:
            # Initialize EasyOCR with English language
            self.ocr = easyocr.Reader(['en'])
        else:
            raise ValueError(f"OCR engine '{ocr_engine}' not available")
        
        # Define regions of interest (ROI) for different telemetry data
        # These are approximate regions based on the screenshots
        self.roi_definitions = {
            'speed': {'top': 30, 'left': 510, 'width': 100, 'height': 80},
            'distance': {'top': 30, 'left': 620, 'width': 120, 'height': 80},
            'elevation': {'top': 30, 'left': 750, 'width': 120, 'height': 80},
            'time': {'top': 30, 'left': 880, 'width': 150, 'height': 80},
            'power': {'top': 50, 'left': 130, 'width': 120, 'height': 60},
            'cadence': {'top': 110, 'left': 150, 'width': 80, 'height': 40},
            'heart_rate': {'top': 110, 'left': 250, 'width': 80, 'height': 40},
            'avg_power': {'top': 150, 'left': 150, 'width': 80, 'height': 40},
            'power_per_kg': {'top': 150, 'left': 250, 'width': 80, 'height': 40},
        }
    
    def preprocess_image(self, image: np.ndarray, roi: Dict[str, int]) -> np.ndarray:
        """
        Preprocess image region for better OCR accuracy
        
        Args:
            image: Full image
            roi: Region of interest dictionary
            
        Returns:
            Preprocessed image region
        """
        # Extract ROI
        y = roi['top']
        x = roi['left']
        h = roi['height']
        w = roi['width']
        roi_img = image[y:y+h, x:x+w]
        
        # Convert to grayscale
        gray = cv2.cvtColor(roi_img, cv2.COLOR_BGR2GRAY)
        
        # Apply threshold to get white text on black background
        # Zwift UI typically has white/bright text
        _, thresh = cv2.threshold(gray, 180, 255, cv2.THRESH_BINARY)
        
        # Denoise
        denoised = cv2.fastNlDenoising(thresh)
        
        # Scale up for better OCR accuracy
        scaled = cv2.resize(denoised, None, fx=2, fy=2, interpolation=cv2.INTER_CUBIC)
        
        return scaled
    
    def extract_text_paddle(self, image: np.ndarray) -> str:
        """Extract text using PaddleOCR"""
        result = self.ocr.ocr(image, cls=True)
        if result and result[0]:
            # Combine all detected text
            texts = [line[1][0] for line in result[0]]
            return ' '.join(texts)
        return ''
    
    def extract_text_easy(self, image: np.ndarray) -> str:
        """Extract text using EasyOCR"""
        result = self.ocr.readtext(image)
        if result:
            # Combine all detected text
            texts = [item[1] for item in result]
            return ' '.join(texts)
        return ''
    
    def extract_text(self, image: np.ndarray) -> str:
        """Extract text using configured OCR engine"""
        if self.ocr_engine == 'paddle':
            return self.extract_text_paddle(image)
        else:
            return self.extract_text_easy(image)
    
    def parse_telemetry_value(self, text: str, data_type: str) -> Optional[float]:
        """
        Parse extracted text into numeric telemetry value
        
        Args:
            text: Extracted OCR text
            data_type: Type of data being parsed
            
        Returns:
            Parsed numeric value or None
        """
        # Clean up text
        text = text.strip()
        
        # Define patterns for different data types
        patterns = {
            'speed': r'(\d+\.?\d*)\s*(?:km/h|KM/H)?',
            'distance': r'(\d+\.?\d*)\s*(?:km|KM)?',
            'elevation': r'(\d+\.?\d*)\s*(?:m|M)?',
            'time': r'(\d+):(\d+)',
            'power': r'(\d+)\s*(?:w|W)?',
            'cadence': r'(\d+)\s*(?:rpm|RPM)?',
            'heart_rate': r'(\d+)\s*(?:bpm|BPM)?',
            'avg_power': r'(\d+)\s*(?:w|W)?',
            'power_per_kg': r'(\d+\.?\d*)\s*(?:w/kg|W/KG)?',
        }
        
        pattern = patterns.get(data_type)
        if not pattern:
            return None
        
        match = re.search(pattern, text)
        if match:
            if data_type == 'time':
                # Convert time to total seconds
                minutes = int(match.group(1))
                seconds = int(match.group(2))
                return minutes * 60 + seconds
            else:
                return float(match.group(1))
        
        return None
    
    def extract_telemetry(self, image_path: str) -> Dict[str, Optional[float]]:
        """
        Extract all telemetry data from a Zwift screenshot
        
        Args:
            image_path: Path to screenshot image
            
        Returns:
            Dictionary of telemetry values
        """
        # Load image
        image = cv2.imread(image_path)
        if image is None:
            raise ValueError(f"Could not load image: {image_path}")
        
        telemetry = {}
        
        for data_type, roi in self.roi_definitions.items():
            # Preprocess ROI
            roi_processed = self.preprocess_image(image, roi)
            
            # Extract text
            text = self.extract_text(roi_processed)
            
            # Parse value
            value = self.parse_telemetry_value(text, data_type)
            
            telemetry[data_type] = {
                'raw_text': text,
                'value': value,
                'roi': roi
            }
        
        return telemetry
    
    def visualize_rois(self, image_path: str, output_path: str):
        """
        Visualize regions of interest on the image
        
        Args:
            image_path: Input image path
            output_path: Output image path with ROIs drawn
        """
        image = cv2.imread(image_path)
        if image is None:
            raise ValueError(f"Could not load image: {image_path}")
        
        # Draw rectangles for each ROI
        colors = {
            'speed': (0, 255, 0),      # Green
            'distance': (255, 0, 0),    # Blue
            'elevation': (0, 0, 255),   # Red
            'time': (255, 255, 0),      # Cyan
            'power': (255, 0, 255),     # Magenta
            'cadence': (0, 255, 255),   # Yellow
            'heart_rate': (128, 0, 128), # Purple
            'avg_power': (255, 128, 0),  # Orange
            'power_per_kg': (0, 128, 255), # Light Blue
        }
        
        for data_type, roi in self.roi_definitions.items():
            color = colors.get(data_type, (255, 255, 255))
            x = roi['left']
            y = roi['top']
            w = roi['width']
            h = roi['height']
            
            cv2.rectangle(image, (x, y), (x + w, y + h), color, 2)
            cv2.putText(image, data_type, (x, y - 5), 
                       cv2.FONT_HERSHEY_SIMPLEX, 0.5, color, 1)
        
        cv2.imwrite(output_path, image)
        print(f"ROI visualization saved to: {output_path}")


def compare_ocr_engines(image_path: str):
    """Compare PaddleOCR and EasyOCR performance on the same image"""
    results = {}
    
    # Test PaddleOCR
    if PADDLE_AVAILABLE:
        print("\nTesting PaddleOCR...")
        start_time = time.time()
        paddle_extractor = ZwiftTelemetryExtractor('paddle')
        paddle_results = paddle_extractor.extract_telemetry(image_path)
        paddle_time = time.time() - start_time
        results['paddle'] = {
            'results': paddle_results,
            'time': paddle_time
        }
        print(f"PaddleOCR processing time: {paddle_time:.2f}s")
    
    # Test EasyOCR
    if EASY_AVAILABLE:
        print("\nTesting EasyOCR...")
        start_time = time.time()
        easy_extractor = ZwiftTelemetryExtractor('easy')
        easy_results = easy_extractor.extract_telemetry(image_path)
        easy_time = time.time() - start_time
        results['easy'] = {
            'results': easy_results,
            'time': easy_time
        }
        print(f"EasyOCR processing time: {easy_time:.2f}s")
    
    return results


def main():
    """Main function to test OCR extraction on Zwift screenshots"""
    # Define test images
    test_images = [
        "docs/screenshots/normal_1_01_16_02_21.jpg",
        "docs/screenshots/with_climbing_1_01_36_01_42.jpg"
    ]
    
    # Check if any OCR engine is available
    if not PADDLE_AVAILABLE and not EASY_AVAILABLE:
        print("No OCR engines available. Please install PaddleOCR or EasyOCR.")
        return
    
    # Test on each image
    for image_path in test_images:
        print(f"\n{'='*60}")
        print(f"Processing: {image_path}")
        print('='*60)
        
        # Create ROI visualization
        output_path = image_path.replace('.jpg', '_rois.jpg')
        if PADDLE_AVAILABLE:
            extractor = ZwiftTelemetryExtractor('paddle')
        else:
            extractor = ZwiftTelemetryExtractor('easy')
        
        extractor.visualize_rois(image_path, output_path)
        
        # Compare OCR engines
        results = compare_ocr_engines(image_path)
        
        # Display results
        for engine, data in results.items():
            print(f"\n{engine.upper()} Results:")
            for field, info in data['results'].items():
                print(f"  {field}:")
                print(f"    Raw text: '{info['raw_text']}'")
                print(f"    Parsed value: {info['value']}")
        
        # Save results to JSON
        json_path = image_path.replace('.jpg', '_ocr_results.json')
        with open(json_path, 'w') as f:
            json.dump(results, f, indent=2)
        print(f"\nResults saved to: {json_path}")


if __name__ == "__main__":
    main()