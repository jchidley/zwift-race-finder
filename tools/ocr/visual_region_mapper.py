#!/usr/bin/env python3
"""Visual tool to map UI regions and test OCR extraction in real-time"""

import json
from dataclasses import dataclass
from datetime import datetime
from typing import Dict, Tuple

import cv2

try:
    from paddleocr import PaddleOCR

    PADDLE_AVAILABLE = True
except ImportError:
    PADDLE_AVAILABLE = False
    print('PaddleOCR not available. Install with: uv add paddlepaddle paddleocr')


@dataclass
class Region:
    name: str
    x: int
    y: int
    width: int
    height: int
    char_type: str  # 'number', 'decimal', 'time', 'letters', 'mixed'
    color: Tuple[int, int, int] = (0, 255, 0)

    def contains_point(self, x: int, y: int) -> bool:
        """Check if point is inside region"""
        return self.x <= x <= self.x + self.width and self.y <= y <= self.y + self.height

    def get_center(self) -> Tuple[int, int]:
        """Get center point of region"""
        return (self.x + self.width // 2, self.y + self.height // 2)


class VisualRegionMapper:
    """Interactive visual tool for mapping Zwift UI regions"""

    def __init__(self, image_path: str):
        self.image = cv2.imread(image_path)
        if self.image is None:
            raise ValueError(f'Could not load image: {image_path}')

        self.image_path = image_path
        self.display_image = self.image.copy()
        self.regions = self._init_regions()
        self.selected_region = None
        self.dragging = False
        self.resize_mode = False
        self.drag_start = None

        # OCR engine
        if PADDLE_AVAILABLE:
            self.ocr = PaddleOCR(use_angle_cls=True, lang='en', show_log=False)
        else:
            self.ocr = None

        # Window setup
        self.window_name = 'Zwift Region Mapper'
        cv2.namedWindow(self.window_name, cv2.WINDOW_NORMAL)
        cv2.resizeWindow(self.window_name, 1400, 800)
        cv2.setMouseCallback(self.window_name, self.mouse_callback)

    def _init_regions(self) -> Dict[str, Region]:
        """Initialize default regions based on known UI layout"""
        return {
            # Top bar
            'speed': Region('speed', 690, 40, 100, 40, 'number', (255, 0, 0)),
            'distance': Region('distance', 810, 40, 100, 40, 'decimal', (255, 0, 0)),
            'altitude': Region('altitude', 930, 40, 80, 40, 'number', (255, 0, 0)),
            'race_time': Region('race_time', 1040, 40, 120, 40, 'time', (255, 0, 0)),
            # Power panel
            'power': Region('power', 271, 49, 148, 61, 'number', (0, 255, 0)),
            'cadence': Region('cadence', 240, 135, 60, 40, 'number', (0, 255, 0)),
            'heart_rate': Region('heart_rate', 311, 128, 98, 44, 'number', (0, 255, 0)),
            'avg_power': Region('avg_power', 222, 191, 60, 31, 'number', (0, 255, 0)),
            'energy': Region('energy', 338, 189, 68, 29, 'number', (0, 255, 0)),
            # Optional elements
            'gradient': Region('gradient', 1700, 75, 50, 50, 'number', (0, 0, 255)),
            'distance_to_finish': Region(
                'distance_to_finish', 1720, 90, 150, 50, 'decimal', (0, 0, 255)
            ),
            'powerup': Region('powerup', 218, 122, 200, 30, 'letters', (255, 255, 0)),
        }

    def mouse_callback(self, event, x, y, flags, param):
        """Handle mouse events"""

        if event == cv2.EVENT_LBUTTONDOWN:
            # Check if clicking on a region
            for name, region in self.regions.items():
                if region.contains_point(x, y):
                    self.selected_region = name
                    self.drag_start = (x - region.x, y - region.y)
                    self.dragging = True
                    self.resize_mode = flags & cv2.EVENT_FLAG_SHIFTKEY
                    break

        elif event == cv2.EVENT_MOUSEMOVE:
            if self.dragging and self.selected_region:
                region = self.regions[self.selected_region]

                if self.resize_mode:
                    # Resize mode (Shift + drag)
                    region.width = max(20, x - region.x)
                    region.height = max(20, y - region.y)
                else:
                    # Move mode
                    region.x = x - self.drag_start[0]
                    region.y = y - self.drag_start[1]

                self._update_display()

        elif event == cv2.EVENT_LBUTTONUP:
            if self.dragging:
                self.dragging = False
                # Test OCR on the selected region
                if self.selected_region and self.ocr:
                    self._test_region(self.selected_region)

        elif event == cv2.EVENT_RBUTTONDOWN:
            # Right click to test all regions
            self._test_all_regions()

    def _update_display(self):
        """Update the display with current regions"""
        self.display_image = self.image.copy()

        # Draw all regions
        for name, region in self.regions.items():
            # Draw rectangle
            thickness = 3 if name == self.selected_region else 2
            cv2.rectangle(
                self.display_image,
                (region.x, region.y),
                (region.x + region.width, region.y + region.height),
                region.color,
                thickness,
            )

            # Draw label with character type
            label = f'{name} [{region.char_type[0].upper()}]'
            label_pos = (region.x, region.y - 5)
            cv2.putText(
                self.display_image,
                label,
                label_pos,
                cv2.FONT_HERSHEY_SIMPLEX,
                0.5,
                region.color,
                1,
            )

    def _test_region(self, region_name: str):
        """Test OCR on a specific region"""
        if not self.ocr:
            print('OCR not available')
            return

        region = self.regions[region_name]
        roi = self.image[region.y : region.y + region.height, region.x : region.x + region.width]

        # Preprocess
        gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY)
        _, binary = cv2.threshold(gray, 150, 255, cv2.THRESH_BINARY)

        # Run OCR
        result = self.ocr.ocr(binary, cls=True)

        if result and result[0]:
            text = result[0][0][1][0]

            # Apply character constraints
            filtered = self._apply_constraints(text, region.char_type)

            print(f'\n{region_name}:')
            print(f"  Raw: '{text}'")
            if filtered != text:
                print(f"  Filtered: '{filtered}'")
            print(f'  Position: ({region.x}, {region.y}) Size: {region.width}x{region.height}')
        else:
            print(f'\n{region_name}: No text detected')

    def _apply_constraints(self, text: str, char_type: str) -> str:
        """Apply character constraints based on type"""
        if char_type == 'number':
            return ''.join(c for c in text if c.isdigit())
        elif char_type == 'decimal':
            return ''.join(c for c in text if c in '0123456789.')
        elif char_type == 'time':
            return ''.join(c for c in text if c in '0123456789:')
        elif char_type == 'letters':
            return ''.join(c for c in text if c.isalpha())
        else:
            return text

    def _test_all_regions(self):
        """Test OCR on all regions"""
        print('\n' + '=' * 60)
        print('Testing all regions:')
        print('=' * 60)

        results = {}

        for name in sorted(self.regions.keys()):
            if self.ocr:
                region = self.regions[name]
                roi = self.image[
                    region.y : region.y + region.height,
                    region.x : region.x + region.width,
                ]

                # Preprocess
                gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY)
                _, binary = cv2.threshold(gray, 150, 255, cv2.THRESH_BINARY)

                # Run OCR
                result = self.ocr.ocr(binary, cls=True)

                if result and result[0]:
                    text = result[0][0][1][0]
                    filtered = self._apply_constraints(text, region.char_type)

                    results[name] = {
                        'raw': text,
                        'filtered': filtered,
                        'value': self._convert_value(filtered, region.char_type),
                    }

                    print(f'✓ {name}: {filtered}')
                else:
                    results[name] = None
                    print(f'✗ {name}: Not detected')

        # Save results
        self._save_results(results)

    def _convert_value(self, text: str, char_type: str):
        """Convert filtered text to appropriate type"""
        if not text:
            return None

        if char_type == 'number':
            try:
                return int(text)
            except ValueError:
                return None
        elif char_type == 'decimal':
            try:
                return float(text)
            except ValueError:
                return None
        elif char_type == 'time' and ':' in text:
            return text
        else:
            return text

    def _save_results(self, results: Dict):
        """Save test results and region definitions"""
        timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')

        # Save results
        results_file = f'ocr_results_{timestamp}.json'
        with open(results_file, 'w') as f:
            json.dump(results, f, indent=2)
        print(f'\nResults saved to: {results_file}')

        # Save region definitions
        regions_data = {
            name: {
                'x': r.x,
                'y': r.y,
                'width': r.width,
                'height': r.height,
                'char_type': r.char_type,
            }
            for name, r in self.regions.items()
        }

        regions_file = f'region_definitions_{timestamp}.json'
        with open(regions_file, 'w') as f:
            json.dump(regions_data, f, indent=2)
        print(f'Regions saved to: {regions_file}')

        # Generate Python code
        print('\nPython code for regions:')
        print('-' * 40)
        for name, r in self.regions.items():
            print(
                f"'{name}': Region('{name}', {r.x}, {r.y}, {r.width}, {r.height}, '{r.char_type}'),"
            )

    def run(self):
        """Run the interactive mapper"""
        print('=' * 60)
        print('Zwift Visual Region Mapper')
        print('=' * 60)
        print('\nControls:')
        print('- Click and drag to move regions')
        print('- Shift + drag to resize regions')
        print('- Right click to test all regions')
        print("- Press 's' to save current configuration")
        print("- Press 'r' to reset to defaults")
        print("- Press 'q' to quit")
        print('\nRegion types:')
        print('- [N] Number (0-9)')
        print('- [D] Decimal (0-9.)')
        print('- [T] Time (0-9:)')
        print('- [L] Letters (A-Z)')
        print('- [M] Mixed (any)')

        self._update_display()

        while True:
            cv2.imshow(self.window_name, self.display_image)
            key = cv2.waitKey(1) & 0xFF

            if key == ord('q'):
                break
            elif key == ord('s'):
                # Save current configuration
                self._test_all_regions()
            elif key == ord('r'):
                # Reset to defaults
                self.regions = self._init_regions()
                self._update_display()
            elif key == ord('1') and self.selected_region:
                # Change selected region to number type
                self.regions[self.selected_region].char_type = 'number'
                self._update_display()
            elif key == ord('2') and self.selected_region:
                # Change to decimal type
                self.regions[self.selected_region].char_type = 'decimal'
                self._update_display()
            elif key == ord('3') and self.selected_region:
                # Change to time type
                self.regions[self.selected_region].char_type = 'time'
                self._update_display()
            elif key == ord('4') and self.selected_region:
                # Change to letters type
                self.regions[self.selected_region].char_type = 'letters'
                self._update_display()
            elif key == ord('5') and self.selected_region:
                # Change to mixed type
                self.regions[self.selected_region].char_type = 'mixed'
                self._update_display()

        cv2.destroyAllWindows()


def main():
    """Run the visual mapper"""
    import sys

    if len(sys.argv) > 1:
        image_path = sys.argv[1]
    else:
        image_path = '../../docs/screenshots/normal_1_01_16_02_21.jpg'

    mapper = VisualRegionMapper(image_path)
    mapper.run()


if __name__ == '__main__':
    main()
