#!/usr/bin/env python3
"""Debug visualizer v3 for Zwift OCR extraction - shows corrected regions"""

import cv2
import numpy as np
from zwift_ocr_improved_v3 import ZwiftOCRExtractorV3, ZwiftUILayoutV3
import sys
from datetime import datetime


class DebugVisualizerV3:
    """Creates debug images showing extraction regions and results"""

    def __init__(self):
        self.colors = {
            "power": (0, 0, 255),  # Red
            "speed": (0, 255, 0),  # Green
            "heart_rate": (255, 0, 0),  # Blue
            "cadence": (255, 255, 0),  # Cyan
            "distance": (255, 0, 255),  # Magenta
            "altitude": (0, 255, 255),  # Yellow
            "race_time": (128, 255, 128),  # Light green
            "avg_power": (255, 128, 128),  # Light red
            "energy": (128, 128, 255),  # Light blue
            "gradient": (255, 165, 0),  # Orange
            "distance_to_finish": (100, 200, 255),  # Sky blue
            "leaderboard": (200, 200, 200),  # Light gray
            "default": (128, 128, 128),  # Gray
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
        extractor = ZwiftOCRExtractorV3(debug=False)
        results = extractor.extract_telemetry(image_path)

        # Draw regions and results
        font = cv2.FONT_HERSHEY_SIMPLEX
        font_scale = 0.7
        thickness = 2

        # Power panel
        self._draw_region(
            vis_image,
            ZwiftUILayoutV3.POWER,
            "power",
            f"Power: {results.get('power', '?')}W",
            self.colors["power"],
        )

        self._draw_region(
            vis_image,
            ZwiftUILayoutV3.HEART_RATE,
            "heart_rate",
            f"HR: {results.get('heart_rate', '?')} BPM",
            self.colors["heart_rate"],
        )

        self._draw_region(
            vis_image,
            ZwiftUILayoutV3.CADENCE,
            "cadence",
            f"Cadence: {results.get('cadence', '?')} RPM",
            self.colors["cadence"],
        )

        self._draw_region(
            vis_image,
            ZwiftUILayoutV3.AVG_POWER,
            "avg_power",
            f"Avg Power: {results.get('avg_power', '?')}W",
            self.colors["avg_power"],
        )

        self._draw_region(
            vis_image,
            ZwiftUILayoutV3.ENERGY,
            "energy",
            f"Energy: {results.get('energy', '?')} kJ",
            self.colors["energy"],
        )

        # Top bar elements (split into 4)
        self._draw_region(
            vis_image,
            ZwiftUILayoutV3.SPEED,
            "speed",
            f"Speed: {results.get('speed', '?')} km/h",
            self.colors["speed"],
        )

        self._draw_region(
            vis_image,
            ZwiftUILayoutV3.DISTANCE,
            "distance",
            f"Dist: {results.get('distance', '?')} km",
            self.colors["distance"],
        )

        self._draw_region(
            vis_image,
            ZwiftUILayoutV3.ALTITUDE,
            "altitude",
            f"Alt: {results.get('altitude', '?')}m",
            self.colors["altitude"],
        )

        self._draw_region(
            vis_image,
            ZwiftUILayoutV3.RACE_TIME,
            "race_time",
            f"Time: {results.get('race_time', '?')}",
            self.colors["race_time"],
        )

        # Distance to finish
        self._draw_region(
            vis_image,
            ZwiftUILayoutV3.DISTANCE_TO_FINISH,
            "distance_to_finish",
            f"To Finish: {results.get('distance_to_finish', '?')} km",
            self.colors["distance_to_finish"],
        )

        # Gradient
        self._draw_region(
            vis_image,
            ZwiftUILayoutV3.GRADIENT_BOX,
            "gradient",
            f"Gradient: {results.get('gradient', '?')}%",
            self.colors["gradient"],
        )

        # Leaderboard area
        self._draw_region(
            vis_image,
            ZwiftUILayoutV3.LEADERBOARD_AREA,
            "leaderboard",
            f"Leaderboard: {len(results.get('leaderboard', []))} entries",
            self.colors["leaderboard"],
        )

        # Add leaderboard text if present
        if results.get("leaderboard"):
            y_offset = ZwiftUILayoutV3.LEADERBOARD_AREA.y - 10
            for entry in results["leaderboard"][:5]:  # Show top 5 entries
                if isinstance(entry, dict):
                    name = entry.get("name", "Unknown")
                    w_kg = entry.get("watts_per_kg", "?")
                    km = entry.get("distance_km", "?")
                    gap = entry.get("gap", "")
                    text = f"{gap} {name}: {w_kg} w/kg, {km} km".strip()
                else:
                    text = str(entry)
                cv2.putText(
                    vis_image,
                    text,
                    (ZwiftUILayoutV3.LEADERBOARD_AREA.x, y_offset),
                    font,
                    0.6,
                    self.colors["leaderboard"],
                    1,
                )
                y_offset -= 20

        # Add legend
        self._add_legend(vis_image, results)

        # Add timestamp
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        cv2.putText(
            vis_image,
            f"OCR Debug V3 - {timestamp}",
            (10, image.shape[0] - 10),
            font,
            0.5,
            (255, 255, 255),
            1,
        )

        # Save result
        if save_path is None:
            save_path = f"debug_ocr_v3_{datetime.now().strftime('%Y%m%d_%H%M%S')}.jpg"

        cv2.imwrite(save_path, vis_image)
        print(f"\nDebug visualization saved to: {save_path}")

        # Print extraction summary
        self._print_summary(results)

    def _draw_region(self, image, region, name, text, color):
        """Draw a region box with label"""
        # Draw rectangle
        cv2.rectangle(
            image,
            (region.x, region.y),
            (region.x + region.width, region.y + region.height),
            color,
            2,
        )

        # Add label background
        label_size = cv2.getTextSize(text, cv2.FONT_HERSHEY_SIMPLEX, 0.6, 1)[0]
        label_y = region.y - 10 if region.y > 30 else region.y + region.height + 20

        # Make sure label stays within image bounds
        if label_y - label_size[1] - 4 < 0:
            label_y = region.y + region.height + 20

        cv2.rectangle(
            image,
            (region.x, label_y - label_size[1] - 4),
            (region.x + label_size[0] + 4, label_y + 4),
            color,
            -1,
        )

        # Add label text
        cv2.putText(
            image,
            text,
            (region.x + 2, label_y),
            cv2.FONT_HERSHEY_SIMPLEX,
            0.6,
            (255, 255, 255),
            1,
        )

    def _add_legend(self, image, results):
        """Add a legend showing extraction status"""
        legend_x = 10
        legend_y = 200
        line_height = 25

        # Calculate expected vs actual values
        expected = {
            "speed": 20,
            "distance": 18.4,
            "altitude": 28,
            "race_time": "31:06",
            "distance_to_finish": 28.6,
            "gradient": 3,
        }

        # Background
        cv2.rectangle(
            image,
            (legend_x - 5, legend_y - 20),
            (legend_x + 400, legend_y + 15 * line_height + 10),
            (0, 0, 0),
            -1,
        )
        cv2.rectangle(
            image,
            (legend_x - 5, legend_y - 20),
            (legend_x + 400, legend_y + 15 * line_height + 10),
            (255, 255, 255),
            1,
        )

        # Title
        cv2.putText(
            image,
            "OCR EXTRACTION STATUS",
            (legend_x, legend_y),
            cv2.FONT_HERSHEY_SIMPLEX,
            0.7,
            (255, 255, 255),
            2,
        )
        legend_y += line_height

        # Status for each field
        fields = [
            ("Speed", results.get("speed"), "speed"),
            ("Distance", results.get("distance"), "distance"),
            ("Altitude", results.get("altitude"), "altitude"),
            ("Race Time", results.get("race_time"), "race_time"),
            ("Power", results.get("power"), "power"),
            ("Heart Rate", results.get("heart_rate"), "heart_rate"),
            ("Cadence", results.get("cadence"), "cadence"),
            ("Avg Power", results.get("avg_power"), "avg_power"),
            ("Energy", results.get("energy"), "energy"),
            ("Gradient", results.get("gradient"), "gradient"),
            ("Dist to Finish", results.get("distance_to_finish"), "distance_to_finish"),
        ]

        for field_name, value, color_key in fields:
            # Color box
            cv2.rectangle(
                image,
                (legend_x, legend_y - 12),
                (legend_x + 15, legend_y + 3),
                self.colors.get(color_key, self.colors["default"]),
                -1,
            )

            # Status text
            status = "✓" if value is not None else "✗"
            text = (
                f"{status} {field_name}: {value if value is not None else 'Not found'}"
            )

            # Add expected value if different
            if color_key in expected and value != expected[color_key]:
                text += f" (expected: {expected[color_key]})"

            cv2.putText(
                image,
                text,
                (legend_x + 25, legend_y),
                cv2.FONT_HERSHEY_SIMPLEX,
                0.5,
                (255, 255, 255),
                1,
            )
            legend_y += line_height

    def _print_summary(self, results):
        """Print extraction summary to console"""
        print("\n" + "=" * 60)
        print("OCR EXTRACTION SUMMARY")
        print("=" * 60)

        # Expected values
        expected = {
            "speed": 20,
            "distance": 18.4,
            "altitude": 28,
            "race_time": "31:06",
            "power": 268,
            "heart_rate": 160,
            "cadence": 72,
            "avg_power": 222,
            "energy": 142,
            "gradient": 3.0,
            "distance_to_finish": 28.6,
        }

        successful = sum(1 for v in results.values() if v is not None and v != [])
        total = len(results)

        print(f"Success rate: {successful}/{total} ({successful/total*100:.1f}%)")
        print("\nExtracted values:")

        for key, value in results.items():
            if value is not None and value != []:
                status = "✓"
                # Check if matches expected
                if key in expected:
                    if value == expected[key]:
                        status += " (correct)"
                    else:
                        status += f" (expected: {expected[key]})"
                print(f"  {status} {key}: {value}")
            else:
                status = "✗"
                if key in expected:
                    status += f" (expected: {expected[key]})"
                print(f"  {status} {key}: Not found")

        print("=" * 60)


def main():
    """Run debug visualization"""
    if len(sys.argv) > 1:
        image_path = sys.argv[1]
        save_path = sys.argv[2] if len(sys.argv) > 2 else None

        visualizer = DebugVisualizerV3()
        visualizer.create_debug_image(image_path, save_path)
    else:
        print("Usage: python debug_visualizer_v3.py <image_path> [output_path]")
        print("Example: python debug_visualizer_v3.py screenshot.jpg debug_output.jpg")


if __name__ == "__main__":
    main()
