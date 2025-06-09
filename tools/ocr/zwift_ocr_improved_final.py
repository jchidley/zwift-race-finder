#!/usr/bin/env python3
"""Final improved Zwift OCR extraction with all fixes applied"""

import cv2
import numpy as np
from dataclasses import dataclass
from typing import Optional, Dict, Any, List, Tuple
import re
from datetime import datetime
import json

import warnings

# Suppress paddle warnings
warnings.filterwarnings("ignore", category=UserWarning, module="paddle")

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
        return image[self.y : self.y + self.height, self.x : self.x + self.width]


class ZwiftUILayoutFinal:
    """Final layout with optimized coordinates from visual mapper"""

    # Top bar elements (from visual mapper results)
    SPEED = Region(693, 44, 71, 61, "speed")
    DISTANCE = Region(833, 44, 84, 55, "distance")
    ALTITUDE = Region(975, 45, 75, 50, "altitude")
    RACE_TIME = Region(1070, 45, 134, 49, "race_time")

    # Power panel (optimized coordinates)
    POWER = Region(268, 49, 117, 61, "power")
    CADENCE = Region(240, 135, 45, 31, "cadence")
    HEART_RATE = Region(341, 129, 69, 38, "heart_rate")

    # Gradient box
    GRADIENT_BOX = Region(1695, 71, 50, 50, "gradient_box")

    # Distance to finish
    DISTANCE_TO_FINISH = Region(1143, 138, 50, 27, "distance_to_finish")

    # Powerup (when active)
    POWERUP_NAME = Region(444, 211, 225, 48, "powerup_name")

    # Leaderboard area
    LEADERBOARD_AREA = Region(1500, 200, 420, 600, "leaderboard_area")


class ZwiftOCRExtractorFinal:
    """Final OCR extractor for Zwift with all improvements"""

    def __init__(self, debug=False):
        if not PADDLE_AVAILABLE:
            raise ImportError("PaddleOCR not available")

        self.ocr = PaddleOCR(use_angle_cls=True, lang="en", show_log=False)
        self.debug = debug

    def extract_telemetry(self, image_path: str) -> Dict[str, Any]:
        """Extract all telemetry data from screenshot"""

        # Load image
        image = cv2.imread(image_path)
        if image is None:
            raise ValueError(f"Could not load image: {image_path}")

        results = {}

        # Extract top bar elements using optimized regions
        results["speed"] = self._extract_simple_number(image, ZwiftUILayoutFinal.SPEED)
        results["distance"] = self._extract_simple_float(
            image, ZwiftUILayoutFinal.DISTANCE
        )
        results["altitude"] = self._extract_simple_number(
            image, ZwiftUILayoutFinal.ALTITUDE
        )
        results["race_time"] = self._extract_race_time(
            image, ZwiftUILayoutFinal.RACE_TIME
        )

        # Extract power panel data
        results["power"] = self._extract_number(
            image, ZwiftUILayoutFinal.POWER, suffix="w"
        )
        results["cadence"] = self._extract_cadence(image, ZwiftUILayoutFinal.CADENCE)
        results["heart_rate"] = self._extract_number(
            image, ZwiftUILayoutFinal.HEART_RATE
        )

        # Extract gradient
        results["gradient"] = self._extract_gradient(
            image, ZwiftUILayoutFinal.GRADIENT_BOX
        )

        # Extract distance to finish
        results["distance_to_finish"] = self._extract_simple_float(
            image, ZwiftUILayoutFinal.DISTANCE_TO_FINISH
        )

        # Extract powerup if active
        results["powerup_name"] = self._extract_powerup(image)

        # Extract leaderboard with structured parsing
        results["leaderboard"] = self._extract_leaderboard_structured(image)

        return results

    def _extract_simple_number(
        self, image: np.ndarray, region: Region
    ) -> Optional[int]:
        """Extract a simple integer from a region with preprocessing"""
        roi = region.extract(image)

        # Preprocess based on region type
        if region.name in ["speed", "distance", "altitude", "race_time"]:
            # Top bar - white text on dark background
            gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY)
            _, binary = cv2.threshold(gray, 200, 255, cv2.THRESH_BINARY)
            processed = cv2.resize(
                binary, None, fx=3, fy=3, interpolation=cv2.INTER_CUBIC
            )
        else:
            # Power panel - may have varying backgrounds
            gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY)
            binary = cv2.adaptiveThreshold(
                gray, 255, cv2.ADAPTIVE_THRESH_GAUSSIAN_C, cv2.THRESH_BINARY, 11, 2
            )
            if np.mean(binary) > 127:
                binary = cv2.bitwise_not(binary)
            processed = cv2.resize(
                binary, None, fx=3, fy=3, interpolation=cv2.INTER_CUBIC
            )

        result = self.ocr.ocr(processed, cls=True)

        if result and result[0]:
            text = result[0][0][1][0]
            if self.debug:
                print(f"{region.name} OCR text: '{text}'")

            # Extract only numbers
            numbers_only = re.sub(r"[^0-9]", "", text)
            if numbers_only:
                return int(numbers_only)

        return None

    def _extract_simple_float(
        self, image: np.ndarray, region: Region
    ) -> Optional[float]:
        """Extract a simple float from a region with preprocessing"""
        roi = region.extract(image)

        # Preprocess
        gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY)
        _, binary = cv2.threshold(gray, 200, 255, cv2.THRESH_BINARY)
        processed = cv2.resize(binary, None, fx=3, fy=3, interpolation=cv2.INTER_CUBIC)

        result = self.ocr.ocr(processed, cls=True)

        if result and result[0]:
            text = result[0][0][1][0]
            if self.debug:
                print(f"{region.name} OCR text: '{text}'")

            # Extract only numbers and decimal points
            decimal_only = re.sub(r"[^0-9.]", "", text)
            if decimal_only:
                try:
                    return float(decimal_only)
                except ValueError:
                    pass

        return None

    def _extract_race_time(self, image: np.ndarray, region: Region) -> Optional[str]:
        """Extract race time from region"""
        roi = region.extract(image)
        result = self.ocr.ocr(roi, cls=True)

        if result and result[0]:
            text = result[0][0][1][0]
            if self.debug:
                print(f"Race time OCR text: '{text}'")

            # Look for time pattern
            time_match = re.search(r"(\d{1,2}:\d{2})", text)
            if time_match:
                return time_match.group(1)

            # If no colon found but we have 3-4 digits, insert colon
            digit_match = re.search(r"(\d{3,4})", text)
            if digit_match:
                digits = digit_match.group(1)
                if len(digits) == 4:
                    return f"{digits[:2]}:{digits[2:]}"
                elif len(digits) == 3:
                    return f"{digits[0]}:{digits[1:]}"

        return None

    def _extract_number(
        self,
        image: np.ndarray,
        region: Region,
        pattern: str = r"(\d+)",
        suffix: str = "",
        is_float: bool = False,
    ) -> Optional[float]:
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
                text = text.replace(suffix, "").replace(suffix.upper(), "")

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
            match = re.search(r"(\d{2,3})", text)
            if match:
                return int(match.group(1))

        return None

    def _extract_gradient(self, image: np.ndarray, region: Region) -> Optional[float]:
        """Extract gradient percentage with special handling for stylized font"""
        roi = region.extract(image)

        # The gradient uses a different font that needs special preprocessing
        gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY)

        # Method 1: Inverted threshold (works best for the stylized gradient font)
        inverted = cv2.bitwise_not(gray)
        _, binary = cv2.threshold(inverted, 100, 255, cv2.THRESH_BINARY)
        scaled = cv2.resize(binary, None, fx=4, fy=4, interpolation=cv2.INTER_CUBIC)

        result = self.ocr.ocr(scaled, cls=True)

        if result and result[0]:
            # Concatenate all text found
            all_text = " ".join([line[1][0] for line in result[0]])
            if self.debug:
                print(f"Gradient OCR text: '{all_text}'")

            # Extract only numbers (ignore % symbol)
            numbers_only = re.sub(r"[^0-9.]", "", all_text)
            if numbers_only:
                try:
                    return float(numbers_only)
                except ValueError:
                    pass

        # Fallback: Try HSV method
        hsv = cv2.cvtColor(roi, cv2.COLOR_BGR2HSV)
        _, _, v = cv2.split(hsv)
        _, binary2 = cv2.threshold(v, 200, 255, cv2.THRESH_BINARY)
        scaled2 = cv2.resize(binary2, None, fx=3, fy=3, interpolation=cv2.INTER_CUBIC)

        result2 = self.ocr.ocr(scaled2, cls=True)
        if result2 and result2[0]:
            text = result2[0][0][1][0]
            numbers_only = re.sub(r"[^0-9.]", "", text)
            if numbers_only:
                try:
                    return float(numbers_only)
                except ValueError:
                    pass

        return None

    def _extract_powerup(self, image: np.ndarray) -> Optional[str]:
        """Extract active powerup name with letter-only constraint"""
        roi = ZwiftUILayoutFinal.POWERUP_NAME.extract(image)

        # Preprocess for colored text
        gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY)
        clahe = cv2.createCLAHE(clipLimit=2.0, tileGridSize=(8, 8))
        enhanced = clahe.apply(gray)
        _, binary = cv2.threshold(enhanced, 150, 255, cv2.THRESH_BINARY)
        processed = cv2.resize(binary, None, fx=2, fy=2, interpolation=cv2.INTER_CUBIC)

        result = self.ocr.ocr(processed, cls=True)

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

    def _extract_leaderboard_structured(
        self, image: np.ndarray
    ) -> List[Dict[str, Any]]:
        """Extract leaderboard with two-row structure per entry"""
        roi = ZwiftUILayoutFinal.LEADERBOARD_AREA.extract(image)

        # Preprocess for better OCR
        gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY)
        clahe = cv2.createCLAHE(clipLimit=2.0, tileGridSize=(8, 8))
        enhanced = clahe.apply(gray)

        result = self.ocr.ocr(enhanced, cls=True)
        if not result or not result[0]:
            return []

        # Helper function to identify names
        def is_likely_name(text: str) -> bool:
            if "KM" in text.upper() or "w/kg" in text.lower():
                return False
            if text.replace(".", "").isdigit():
                return False
            # Positive indicators
            if re.match(r"^[A-Z]\.[A-Za-z]", text) or text.count(".") >= 2:
                return True
            if re.match(r"^[A-Z][a-z]", text) or "(" in text or ")" in text:
                return True
            return False

        # Collect detections
        detections = []
        for detection in result[0]:
            bbox, (text, conf) = detection
            y_center = (bbox[0][1] + bbox[2][1]) / 2
            x_left = bbox[0][0]
            detections.append({"text": text.strip(), "y": y_center, "x": x_left})

        # Sort by Y position
        detections.sort(key=lambda d: d["y"])

        # Find names and build entries
        names = [d for d in detections if is_likely_name(d["text"])]
        entries = []

        for i, name_det in enumerate(names):
            if name_det["y"] < 180:  # Skip event title
                continue

            entry = {
                "position": len(entries) + 1,
                "name": name_det["text"],
                "time_delta": None,
                "watts_per_kg": None,
                "distance_km": None,
                "is_current_rider": False,
            }

            # Find data in row below name (10-40 pixels)
            data_row = [
                d
                for d in detections
                if name_det["y"] + 10 <= d["y"] <= name_det["y"] + 40
            ]

            for det in data_row:
                text = det["text"]
                # Time delta
                if ":" in text and any(c in text for c in ["+", "-"]):
                    entry["time_delta"] = text
                # Distance
                elif "KM" in text.upper():
                    match = re.search(r"(\d+\.?\d*)", text)
                    if match:
                        entry["distance_km"] = float(match.group(1))
                # W/kg (middle column position or has w/kg text)
                elif "w/kg" in text.lower():
                    match = re.search(r"(\d+\.?\d*)", text)
                    if match:
                        entry["watts_per_kg"] = float(match.group(1))
                elif 80 < det["x"] < 180 and "." in text:
                    match = re.search(r"(\d+\.?\d*)", text)
                    if match:
                        value = float(match.group(1))
                        if 0.5 <= value <= 7.0:
                            entry["watts_per_kg"] = value

            # Current rider has no time delta
            if entry["time_delta"] is None and (
                entry["watts_per_kg"] or entry["distance_km"]
            ):
                entry["is_current_rider"] = True

            if entry["watts_per_kg"] or entry["distance_km"] or entry["time_delta"]:
                entries.append(entry)

        return entries


def main():
    """Test the final OCR extraction"""
    import sys

    if len(sys.argv) > 1:
        image_path = sys.argv[1]
    else:
        print("Usage: python zwift_ocr_improved_final.py <image_path>")
        return

    extractor = ZwiftOCRExtractorFinal(debug=True)
    results = extractor.extract_telemetry(image_path)

    print("\nExtracted Telemetry:")
    print("-" * 40)

    # The actual values in the screenshot (based on OCR scan)
    actual_expected = {
        "speed": (results.get("speed"), 34),
        "distance": (results.get("distance"), 6.4),
        "altitude": (results.get("altitude"), 28),
        "race_time": (results.get("race_time"), "11:07"),
        "power": (results.get("power"), 268),
        "heart_rate": (results.get("heart_rate"), 160),
        "cadence": (results.get("cadence"), 72),
        "gradient": (results.get("gradient"), 3.0),
    }

    correct_count = 0
    for key, (actual, expected) in actual_expected.items():
        if actual is not None:
            if actual == expected:
                print(f"✓ {key}: {actual} (correct)")
                correct_count += 1
            else:
                print(f"✓ {key}: {actual} (expected: {expected})")
        else:
            print(f"✗ {key}: Not found (expected: {expected})")

    # Other fields
    for key in ["distance_to_finish", "powerup_name"]:
        value = results.get(key)
        if value is not None:
            print(f"✓ {key}: {value}")
        else:
            print(f"✗ {key}: Not found")

    # Leaderboard
    if results.get("leaderboard"):
        print(f"✓ leaderboard: {len(results['leaderboard'])} entries")
        for entry in results["leaderboard"]:
            marker = " ⬅️  YOU" if entry.get("is_current_rider") else ""
            time_str = entry.get("time_delta") or "---"
            name = entry.get("name", "Unknown")
            wkg = entry.get("watts_per_kg") or 0.0
            dist = entry.get("distance_km") or 0.0
            position = entry.get("position", "?")
            print(
                f"  {position}. {name:<15} {time_str:>6}  {wkg:>3.1f} w/kg  {dist:>4.1f} km{marker}"
            )
    else:
        print("✗ leaderboard: Not found")

    # Summary
    total_fields = (
        len(actual_expected) + 3
    )  # +3 for distance_to_finish, powerup_name, leaderboard
    successful = sum(1 for v in results.values() if v is not None and v != [])
    print(f"\nAccuracy: {correct_count}/{len(actual_expected)} correct values")
    print(
        f"Overall extraction: {successful}/{total_fields} fields found ({successful/total_fields*100:.1f}%)"
    )

    # Save results
    output_file = f"telemetry_final_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
    with open(output_file, "w") as f:
        json.dump(results, f, indent=2)
    print(f"\nResults saved to: {output_file}")


if __name__ == "__main__":
    main()
