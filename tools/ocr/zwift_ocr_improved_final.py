#!/usr/bin/env python3
"""Final improved Zwift OCR extraction with debug and validation features

This version uses zwift_ocr_compact.py for core OCR functionality and adds:
- Debug mode with visual output
- Validation and accuracy testing
- Powerup detection
- Multiple preprocessing fallbacks
"""

import cv2
import numpy as np
from typing import Optional, Dict, Any, List
import re
from datetime import datetime
import json
import sys

# Import the compact OCR as our core library
from zwift_ocr_compact import ZwiftOCR


class ZwiftOCRExtractorFinal:
    """Extended OCR extractor with debug and validation features"""

    def __init__(self, debug=False):
        # Use the compact OCR for core functionality
        self.ocr = ZwiftOCR()
        self.debug = debug

        # Additional region for powerup detection
        self.powerup_region = (444, 211, 225, 48)  # x, y, width, height

    def extract_telemetry(self, image_path: str) -> Dict[str, Any]:
        """Extract all telemetry data with debug support"""

        # Use the compact OCR for core extraction
        results = self.ocr.extract(image_path)

        # Load image for additional processing
        image = cv2.imread(image_path)
        if image is None:
            raise ValueError(f"Could not load image: {image_path}")

        # Add powerup detection (not in compact version)
        results["powerup_name"] = self._extract_powerup(image)

        # Add debug visualization if enabled
        if self.debug:
            self._create_debug_visualization(image, results)

        return results

    def _extract_powerup(self, image: np.ndarray) -> Optional[str]:
        """Extract active powerup name with letter-only constraint"""
        x, y, w, h = self.powerup_region
        roi = image[y : y + h, x : x + w]

        # Preprocess for colored text
        gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY)
        clahe = cv2.createCLAHE(clipLimit=2.0, tileGridSize=(8, 8))
        enhanced = clahe.apply(gray)
        _, binary = cv2.threshold(enhanced, 150, 255, cv2.THRESH_BINARY)
        processed = cv2.resize(binary, None, fx=2, fy=2, interpolation=cv2.INTER_CUBIC)

        result = self.ocr.ocr.ocr(processed, cls=True)

        if result and result[0]:
            text = result[0][0][1][0]
            # Extract only letters
            letters_only = re.sub(r"[^A-Za-z]", "", text)

            if self.debug:
                print(f"Powerup OCR text: '{text}' -> '{letters_only}'")

            # Known powerups
            powerups = {
                "feather": "Featherweight",
                "aero": "Aero Boost",
                "draft": "Draft Boost",
                "lightweight": "Lightweight",
                "steamroller": "Steamroller",
                "anvil": "Anvil",
                "burrito": "Burrito",
            }

            text_lower = letters_only.lower()
            for key, name in powerups.items():
                if key in text_lower:
                    return name

        return None

    def _create_debug_visualization(self, image: np.ndarray, results: Dict[str, Any]):
        """Create debug visualization showing all regions and extracted values"""
        debug_img = image.copy()

        # Draw rectangles for all regions
        for name, (x, y, w, h) in self.ocr.regions.items():
            cv2.rectangle(debug_img, (x, y), (x + w, y + h), (0, 255, 0), 2)
            cv2.putText(
                debug_img,
                name,
                (x, y - 5),
                cv2.FONT_HERSHEY_SIMPLEX,
                0.5,
                (0, 255, 0),
                1,
            )

            # Show extracted value
            value = results.get(name)
            if value is not None:
                cv2.putText(
                    debug_img,
                    str(value),
                    (x, y + h + 15),
                    cv2.FONT_HERSHEY_SIMPLEX,
                    0.5,
                    (255, 255, 0),
                    1,
                )

        # Draw powerup region
        x, y, w, h = self.powerup_region
        cv2.rectangle(debug_img, (x, y), (x + w, y + h), (255, 0, 255), 2)
        cv2.putText(
            debug_img,
            "powerup",
            (x, y - 5),
            cv2.FONT_HERSHEY_SIMPLEX,
            0.5,
            (255, 0, 255),
            1,
        )

        # Save debug image
        debug_path = "debug_ocr_output.jpg"
        cv2.imwrite(debug_path, debug_img)
        print(f"Debug visualization saved to: {debug_path}")

    def validate_extraction(self, results: Dict[str, Any]) -> Dict[str, Any]:
        """Validate extracted data against expected ranges"""
        validation = {"valid": True, "warnings": [], "field_status": {}}

        # Check each field
        checks = {
            "speed": (0, 100, "km/h"),
            "power": (0, 2000, "W"),
            "heart_rate": (40, 220, "bpm"),
            "cadence": (0, 150, "rpm"),
            "gradient": (-20, 20, "%"),
        }

        for field, (min_val, max_val, unit) in checks.items():
            value = results.get(field)
            if value is not None:
                if min_val <= value <= max_val:
                    validation["field_status"][field] = "valid"
                else:
                    validation["field_status"][field] = "out_of_range"
                    validation["warnings"].append(
                        f"{field}: {value}{unit} is outside normal range ({min_val}-{max_val})"
                    )
                    validation["valid"] = False
            else:
                validation["field_status"][field] = "missing"

        # Check data consistency
        if results.get("power") and results.get("speed"):
            if results["power"] > 300 and results["speed"] < 20:
                validation["warnings"].append(
                    "High power with low speed - possible climb or calibration issue"
                )

        return validation

    def compare_with_expected(
        self, results: Dict[str, Any], expected: Dict[str, Any]
    ) -> Dict[str, Any]:
        """Compare extracted values with expected values for accuracy testing"""
        comparison = {
            "total_fields": len(expected),
            "correct_fields": 0,
            "accuracy_by_field": {},
            "overall_accuracy": 0.0,
        }

        for field, expected_value in expected.items():
            actual_value = results.get(field)

            if actual_value is None:
                comparison["accuracy_by_field"][field] = {
                    "status": "missing",
                    "actual": None,
                    "expected": expected_value,
                }
            elif actual_value == expected_value:
                comparison["correct_fields"] += 1
                comparison["accuracy_by_field"][field] = {
                    "status": "correct",
                    "actual": actual_value,
                    "expected": expected_value,
                }
            else:
                comparison["accuracy_by_field"][field] = {
                    "status": "incorrect",
                    "actual": actual_value,
                    "expected": expected_value,
                }

        comparison["overall_accuracy"] = (
            comparison["correct_fields"] / comparison["total_fields"]
        ) * 100

        return comparison


def main():
    """Test the final OCR extraction with validation"""
    if len(sys.argv) > 1:
        image_path = sys.argv[1]
    else:
        print("Usage: python zwift_ocr_improved_final.py <image_path> [--validate]")
        return

    # Check for validation mode
    validate_mode = "--validate" in sys.argv

    extractor = ZwiftOCRExtractorFinal(debug=True)
    results = extractor.extract_telemetry(image_path)

    print("\nExtracted Telemetry:")
    print("-" * 40)

    # Display results
    for key, value in results.items():
        if value is not None:
            print(f"{key}: {value}")

    # Validation mode
    if validate_mode:
        print("\n\nValidation Results:")
        print("-" * 40)
        validation = extractor.validate_extraction(results)

        print(f"Overall Valid: {validation['valid']}")
        print("\nField Status:")
        for field, status in validation["field_status"].items():
            print(f"  {field}: {status}")

        if validation["warnings"]:
            print("\nWarnings:")
            for warning in validation["warnings"]:
                print(f"  - {warning}")

    # Test against known values if available
    if "normal_1_01_16_02_21" in image_path:
        # Known values from the test screenshot
        expected = {
            "speed": 34,
            "distance": 6.4,
            "altitude": 28,
            "race_time": "11:07",
            "power": 268,
            "heart_rate": 160,
            "cadence": 72,
            "gradient": 3.0,
            "distance_to_finish": 28.6,
        }

        print("\n\nAccuracy Test Results:")
        print("-" * 40)
        comparison = extractor.compare_with_expected(results, expected)

        print(f"Overall Accuracy: {comparison['overall_accuracy']:.1f}%")
        print(
            f"Correct Fields: {comparison['correct_fields']}/{comparison['total_fields']}"
        )

        print("\nField-by-Field Results:")
        for field, details in comparison["accuracy_by_field"].items():
            status_symbol = "✓" if details["status"] == "correct" else "✗"
            print(
                f"  {status_symbol} {field}: {details['actual']} (expected: {details['expected']})"
            )

    # Save results
    output_file = f"ocr_results_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
    with open(output_file, "w") as f:
        json.dump(
            {
                "image": image_path,
                "timestamp": datetime.now().isoformat(),
                "results": results,
                "validation": (
                    extractor.validate_extraction(results) if validate_mode else None
                ),
            },
            f,
            indent=2,
        )
    print(f"\nResults saved to: {output_file}")


if __name__ == "__main__":
    main()
