#!/usr/bin/env python3
"""Improved leaderboard extraction based on actual Zwift UI layout"""

import cv2
from paddleocr import PaddleOCR
from dataclasses import dataclass
from typing import List, Optional, Dict, Tuple
import re

@dataclass
class LeaderboardEntry:
    """Represents a single rider on the leaderboard"""
    name: str
    watts_per_kg: float
    distance_km: float
    time_gap: Optional[str] = None
    is_current_rider: bool = False
    y_position: int = 0  # For sorting


class LeaderboardExtractorV2:
    """Extract leaderboard data with better parsing"""
    
    def __init__(self, debug=False):
        self.ocr = PaddleOCR(use_angle_cls=True, lang='en', show_log=False)
        self.debug = debug
    
    def extract_leaderboard(self, image_path: str) -> List[LeaderboardEntry]:
        """Extract leaderboard entries from screenshot"""
        
        image = cv2.imread(image_path)
        if image is None:
            raise ValueError(f"Could not load image: {image_path}")
        
        # Run OCR on full image
        result = self.ocr.ocr(image, cls=True)
        
        if not result or not result[0]:
            return []
        
        # Find all relevant leaderboard elements
        time_gaps = {}  # y_position -> time_gap
        names = {}      # y_position -> name
        stats = {}      # y_position -> (w/kg, km)
        
        for line in result[0]:
            text = line[1][0]
            coords = line[0]
            x_min = min(p[0] for p in coords)
            y_min = min(p[1] for p in coords)
            
            # Only process items on the right side (leaderboard area)
            if x_min < 1400:
                continue
            
            # Time gaps
            if text.startswith('+') and ':' in text:
                time_gaps[y_min] = text
                if self.debug:
                    print(f"Time gap at y={y_min}: {text}")
            
            # Stats (w/kg and km, often concatenated)
            elif 'w/kg' in text.lower():
                # Parse w/kg and km from concatenated format
                w_kg = None
                km = None
                
                # Try to extract w/kg
                w_kg_match = re.search(r'(\d+\.?\d*)\s*w/kg', text.lower())
                if w_kg_match:
                    w_kg = float(w_kg_match.group(1))
                
                # Try to extract km
                km_match = re.search(r'(\d+\.?\d*)\s*km', text.lower())
                if km_match:
                    km = float(km_match.group(1))
                
                if w_kg is not None:
                    stats[y_min] = (w_kg, km)
                    if self.debug:
                        print(f"Stats at y={y_min}: {w_kg} w/kg, {km} km")
            
            # Names (contain dots and uppercase letters)
            elif self._is_likely_name(text):
                names[y_min] = text.strip()
                if self.debug:
                    print(f"Name at y={y_min}: {text}")
        
        # Combine into entries by matching Y positions
        entries = []
        
        # Process each name and find matching stats/time gap
        for name_y, name in names.items():
            entry = LeaderboardEntry(
                name=name,
                watts_per_kg=0.0,
                distance_km=0.0,
                y_position=name_y
            )
            
            # Find matching stats (within 30 pixels vertically)
            for stats_y, (w_kg, km) in stats.items():
                if abs(stats_y - name_y) < 30:
                    entry.watts_per_kg = w_kg
                    entry.distance_km = km or 0.0
                    break
            
            # Find matching time gap (within 30 pixels vertically)
            for gap_y, gap in time_gaps.items():
                if abs(gap_y - name_y) < 30:
                    entry.time_gap = gap
                    break
            
            # Only add entries with valid data
            if entry.watts_per_kg > 0:
                entries.append(entry)
        
        # Sort by Y position
        entries.sort(key=lambda e: e.y_position)
        
        # Identify current rider (no time gap or specific patterns)
        self._identify_current_rider(entries)
        
        return entries
    
    def _is_likely_name(self, text: str) -> bool:
        """Check if text is likely a rider name"""
        # Remove spaces and check
        text = text.strip()
        
        # Too short or too long
        if len(text) < 3 or len(text) > 25:
            return False
        
        # Must have at least one letter
        if not any(c.isalpha() for c in text):
            return False
        
        # Common patterns for names:
        # - Contains dots (J.Smith)
        # - Has uppercase letters
        # - Not purely numeric
        if '.' in text and any(c.isupper() for c in text):
            return True
        
        # Check for specific patterns
        # Avoid pure numbers, distances, etc.
        if re.match(r'^\d+\.?\d*$', text):  # Pure number
            return False
        
        if text.lower().endswith('km') or text.lower().endswith('mi'):
            return False
        
        # If it has mixed case and letters, probably a name
        if any(c.isupper() for c in text) and any(c.islower() for c in text):
            return True
        
        return False
    
    def _identify_current_rider(self, entries: List[LeaderboardEntry]):
        """Identify which entry is the current rider"""
        # Based on the example: J.Chidley is current (no time gap initially)
        # followed by C.J.Y.S with +0:00
        
        for i, entry in enumerate(entries):
            # Current rider typically has no time gap
            if entry.time_gap is None or entry.time_gap == "":
                # Verify next rider has +0:00 or similar
                if i + 1 < len(entries) and entries[i + 1].time_gap == "+0:00":
                    entry.is_current_rider = True
                    break


def main():
    """Test improved leaderboard extraction"""
    import sys
    
    if len(sys.argv) > 1:
        image_path = sys.argv[1]
    else:
        print("Usage: python leaderboard_extractor_v2.py <image_path>")
        return
    
    extractor = LeaderboardExtractorV2(debug=True)
    entries = extractor.extract_leaderboard(image_path)
    
    print("\n" + "="*60)
    print("Extracted Leaderboard:")
    print("="*60)
    
    for i, entry in enumerate(entries):
        current = " ← YOU" if entry.is_current_rider else ""
        gap = entry.time_gap if entry.time_gap else "    "
        
        print(f"{i+1:2d}. {gap:>6} | {entry.name:<20} | "
              f"{entry.watts_per_kg:3.1f} w/kg | "
              f"{entry.distance_km:4.1f} km{current}")
    
    print("="*60)
    
    # Summary
    if entries:
        current_rider = next((e for e in entries if e.is_current_rider), None)
        if current_rider:
            print(f"\nCurrent rider: {current_rider.name}")
            print(f"Performance: {current_rider.watts_per_kg:.1f} w/kg")
            print(f"Distance: {current_rider.distance_km:.1f} km")


if __name__ == "__main__":
    main()