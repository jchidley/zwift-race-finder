#!/usr/bin/env python3
"""Debug visualizer for Zwift OCR extraction - shows what was extracted from where"""

import cv2
import numpy as np
from zwift_ocr_fixed import ZwiftOCRExtractor, ZwiftUILayout
import sys
from datetime import datetime

class DebugVisualizer:
    """Creates debug images showing extraction regions and results"""
    
    def __init__(self):
        self.colors = {
            'power': (0, 0, 255),        # Red
            'speed': (0, 255, 0),        # Green
            'heart_rate': (255, 0, 0),   # Blue
            'cadence': (255, 255, 0),    # Cyan
            'distance': (255, 0, 255),   # Magenta
            'altitude': (0, 255, 255),   # Yellow
            'race_time': (128, 255, 128),# Light green
            'avg_power': (255, 128, 128),# Light red
            'energy': (128, 128, 255),   # Light blue
            'gradient': (255, 165, 0),   # Orange
            'leaderboard': (200, 200, 200), # Light gray
            'default': (128, 128, 128)   # Gray
        }
    
    def create_debug_image(self, image_path: str, save_path: str = None):
        """Create debug visualization of OCR extraction"""
        
        # Load image
        image = cv2.imread(image_path)
        if image is None:
            print(f"Could not load image: {image_path}")
            return
        
        # Create copy for visualization
        vis_image = image.copy()
        
        # Run OCR extraction
        print("Running OCR extraction...")
        extractor = ZwiftOCRExtractor(debug=False)
        results = extractor.extract_telemetry(image_path)
        
        # Draw regions and results
        font = cv2.FONT_HERSHEY_SIMPLEX
        font_scale = 0.7
        thickness = 2
        
        # Power panel
        self._draw_region(vis_image, ZwiftUILayout.POWER, 'power', 
                         f"Power: {results.get('power', '?')}W", self.colors['power'])
        
        self._draw_region(vis_image, ZwiftUILayout.HEART_RATE, 'heart_rate',
                         f"HR: {results.get('heart_rate', '?')} BPM", self.colors['heart_rate'])
        
        self._draw_region(vis_image, ZwiftUILayout.CADENCE, 'cadence',
                         f"Cadence: {results.get('cadence', '?')} RPM", self.colors['cadence'])
        
        self._draw_region(vis_image, ZwiftUILayout.AVG_POWER, 'avg_power',
                         f"Avg Power: {results.get('avg_power', '?')}W", self.colors['avg_power'])
        
        self._draw_region(vis_image, ZwiftUILayout.ENERGY, 'energy',
                         f"Energy: {results.get('energy', '?')} kJ", self.colors['energy'])
        
        # Top bar
        self._draw_region(vis_image, ZwiftUILayout.TOP_BAR, 'top_bar',
                         f"Speed: {results.get('speed', '?')} km/h | "
                         f"Dist: {results.get('distance', '?')} km | "
                         f"Alt: {results.get('altitude', '?')}m | "
                         f"Time: {results.get('race_time', '?')}", 
                         self.colors['speed'])
        
        # Gradient
        self._draw_region(vis_image, ZwiftUILayout.GRADIENT_BOX, 'gradient',
                         f"Gradient: {results.get('gradient', '?')}%", self.colors['gradient'])
        
        # Leaderboard area
        leaderboard_region = (1500, 650, 420, 150)  # Approximate
        cv2.rectangle(vis_image, 
                     (leaderboard_region[0], leaderboard_region[1]),
                     (leaderboard_region[0] + leaderboard_region[2], 
                      leaderboard_region[1] + leaderboard_region[3]),
                     self.colors['leaderboard'], 2)
        
        # Add leaderboard text
        if results.get('leaderboard'):
            y_offset = leaderboard_region[1] - 10
            for entry in results['leaderboard']:
                text = f"{entry['name']}: {entry['watts_per_kg']} w/kg, {entry['distance_km']} km"
                cv2.putText(vis_image, text, 
                           (leaderboard_region[0], y_offset),
                           font, 0.6, self.colors['leaderboard'], 1)
                y_offset -= 20
        
        # Add legend
        self._add_legend(vis_image, results)
        
        # Add timestamp
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        cv2.putText(vis_image, f"OCR Debug - {timestamp}", 
                   (10, image.shape[0] - 10),
                   font, 0.5, (255, 255, 255), 1)
        
        # Save result
        if save_path is None:
            save_path = f"debug_ocr_visualization_{datetime.now().strftime('%Y%m%d_%H%M%S')}.jpg"
        
        cv2.imwrite(save_path, vis_image)
        print(f"\nDebug visualization saved to: {save_path}")
        
        # Print extraction summary
        self._print_summary(results)
    
    def _draw_region(self, image, region, name, text, color):
        """Draw a region box with label"""
        # Draw rectangle
        cv2.rectangle(image, 
                     (region.x, region.y),
                     (region.x + region.width, region.y + region.height),
                     color, 2)
        
        # Add label background
        label_size = cv2.getTextSize(text, cv2.FONT_HERSHEY_SIMPLEX, 0.6, 1)[0]
        label_y = region.y - 10 if region.y > 30 else region.y + region.height + 20
        
        cv2.rectangle(image,
                     (region.x, label_y - label_size[1] - 4),
                     (region.x + label_size[0] + 4, label_y + 4),
                     color, -1)
        
        # Add label text
        cv2.putText(image, text,
                   (region.x + 2, label_y),
                   cv2.FONT_HERSHEY_SIMPLEX, 0.6, (255, 255, 255), 1)
    
    def _add_legend(self, image, results):
        """Add a legend showing extraction status"""
        legend_x = 10
        legend_y = 200
        line_height = 25
        
        # Background
        cv2.rectangle(image, (legend_x - 5, legend_y - 20), 
                     (legend_x + 300, legend_y + len(self.colors) * line_height + 10),
                     (0, 0, 0), -1)
        cv2.rectangle(image, (legend_x - 5, legend_y - 20), 
                     (legend_x + 300, legend_y + len(self.colors) * line_height + 10),
                     (255, 255, 255), 1)
        
        # Title
        cv2.putText(image, "OCR EXTRACTION STATUS", 
                   (legend_x, legend_y), 
                   cv2.FONT_HERSHEY_SIMPLEX, 0.7, (255, 255, 255), 2)
        legend_y += line_height
        
        # Status for each field
        fields = [
            ('Power', results.get('power'), 'power'),
            ('Heart Rate', results.get('heart_rate'), 'heart_rate'),
            ('Cadence', results.get('cadence'), 'cadence'),
            ('Avg Power', results.get('avg_power'), 'avg_power'),
            ('Energy', results.get('energy'), 'energy'),
            ('Speed', results.get('speed'), 'speed'),
            ('Distance', results.get('distance'), 'distance'),
            ('Altitude', results.get('altitude'), 'altitude'),
            ('Race Time', results.get('race_time'), 'race_time'),
            ('Gradient', results.get('gradient'), 'gradient'),
        ]
        
        for field_name, value, color_key in fields:
            # Color box
            cv2.rectangle(image, 
                         (legend_x, legend_y - 12),
                         (legend_x + 15, legend_y + 3),
                         self.colors.get(color_key, self.colors['default']), -1)
            
            # Status text
            status = "✓" if value is not None else "✗"
            text = f"{status} {field_name}: {value if value is not None else 'Not found'}"
            cv2.putText(image, text,
                       (legend_x + 25, legend_y),
                       cv2.FONT_HERSHEY_SIMPLEX, 0.5, (255, 255, 255), 1)
            legend_y += line_height
    
    def _print_summary(self, results):
        """Print extraction summary to console"""
        print("\n" + "="*60)
        print("OCR EXTRACTION SUMMARY")
        print("="*60)
        
        successful = sum(1 for v in results.values() if v is not None and v != [])
        total = len(results)
        
        print(f"Success rate: {successful}/{total} ({successful/total*100:.1f}%)")
        print("\nExtracted values:")
        
        for key, value in results.items():
            if value is not None and value != []:
                print(f"  ✓ {key}: {value}")
            else:
                print(f"  ✗ {key}: Not found")
        
        print("="*60)


def main():
    """Run debug visualization"""
    if len(sys.argv) > 1:
        image_path = sys.argv[1]
        save_path = sys.argv[2] if len(sys.argv) > 2 else None
        
        visualizer = DebugVisualizer()
        visualizer.create_debug_image(image_path, save_path)
    else:
        print("Usage: python debug_visualizer.py <image_path> [output_path]")
        print("Example: python debug_visualizer.py screenshot.jpg debug_output.jpg")


if __name__ == "__main__":
    main()