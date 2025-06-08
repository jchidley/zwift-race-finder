#!/usr/bin/env python3
"""Extract leaderboard data from Zwift screenshots"""

import cv2
import numpy as np
from paddleocr import PaddleOCR
from dataclasses import dataclass
from typing import List, Optional, Dict, Tuple
import re

@dataclass
class LeaderboardEntry:
    """Represents a single rider on the leaderboard"""
    position: Optional[int] = None
    time_gap: Optional[str] = None  # e.g., "+0:00", "+6:12", None for current rider
    name: str = ""
    watts_per_kg: Optional[float] = None
    distance_km: Optional[float] = None
    is_current_rider: bool = False
    

class LeaderboardExtractor:
    """Extract leaderboard data from Zwift UI"""
    
    def __init__(self, debug=False):
        self.ocr = PaddleOCR(use_angle_cls=True, lang='en', show_log=False)
        self.debug = debug
        
        # Leaderboard is typically on the right side
        # Based on findings: x > 1500, y between 300-800
        self.leaderboard_region = (1500, 300, 420, 500)  # x, y, width, height
    
    def extract_leaderboard(self, image_path: str) -> List[LeaderboardEntry]:
        """Extract all leaderboard entries from the screenshot"""
        
        # Load image
        image = cv2.imread(image_path)
        if image is None:
            raise ValueError(f"Could not load image: {image_path}")
        
        # Extract leaderboard region
        x, y, w, h = self.leaderboard_region
        roi = image[y:y+h, x:x+w]
        
        # Run OCR on the region
        result = self.ocr.ocr(roi, cls=True)
        
        if not result or not result[0]:
            return []
        
        # Group OCR results by Y coordinate (same row)
        rows = self._group_by_row(result[0])
        
        # Parse each row into a leaderboard entry
        entries = []
        for row_data in rows:
            entry = self._parse_row(row_data)
            if entry and entry.name:  # Only add if we found a name
                entries.append(entry)
        
        # Mark current rider (usually 3rd or 4th entry, has no time gap initially)
        self._identify_current_rider(entries)
        
        return entries
    
    def _group_by_row(self, ocr_results: List) -> List[List]:
        """Group OCR results by Y coordinate (same row)"""
        # Sort by Y coordinate
        sorted_results = sorted(ocr_results, key=lambda x: min(p[1] for p in x[0]))
        
        rows = []
        current_row = []
        last_y = -1
        
        for result in sorted_results:
            coords = result[0]
            y_min = min(p[1] for p in coords)
            
            # If Y coordinate is close to previous (within 30 pixels), same row
            if last_y == -1 or abs(y_min - last_y) < 30:
                current_row.append(result)
            else:
                # New row
                if current_row:
                    rows.append(current_row)
                current_row = [result]
            
            last_y = y_min
        
        # Add last row
        if current_row:
            rows.append(current_row)
        
        return rows
    
    def _parse_row(self, row_data: List) -> Optional[LeaderboardEntry]:
        """Parse a single row of leaderboard data"""
        entry = LeaderboardEntry()
        
        # Sort by X coordinate (left to right)
        row_data.sort(key=lambda x: min(p[0] for p in x[0]))
        
        # Extract text from each element
        texts = []
        for item in row_data:
            text = item[1][0]
            texts.append(text)
        
        if self.debug:
            print(f"Row texts: {texts}")
        
        # Parse the row
        for text in texts:
            # Time gap (e.g., "+0:00", "+6:12")
            if text.startswith('+') and ':' in text:
                entry.time_gap = text
            
            # Watts per kg (e.g., "3.6w/kg", "3.6 w/kg")
            elif 'w/kg' in text.lower():
                match = re.search(r'(\d+\.?\d*)\s*w/kg', text.lower())
                if match:
                    entry.watts_per_kg = float(match.group(1))
            
            # Distance (e.g., "18.4km", "18.4 km")
            elif 'km' in text.lower():
                match = re.search(r'(\d+\.?\d*)\s*km', text.lower())
                if match:
                    entry.distance_km = float(match.group(1))
            
            # Name (contains dots or uppercase letters)
            elif '.' in text or any(c.isupper() for c in text):
                # Check if it's not a number with decimal
                if not re.match(r'^\d+\.\d+$', text):
                    # Clean up name
                    name = text.strip()
                    # Remove trailing numbers if they got concatenated
                    name = re.sub(r'\d+$', '', name).strip()
                    if name and len(name) > 2:
                        entry.name = name
        
        # Sometimes w/kg and distance are concatenated like "3.6w/kg18.4km"
        for text in texts:
            if 'w/kg' in text.lower() and 'km' in text.lower():
                # Parse concatenated format
                match = re.match(r'(\d+\.?\d*)\s*w/kg\s*(\d+\.?\d*)\s*km', text.lower())
                if match:
                    entry.watts_per_kg = float(match.group(1))
                    entry.distance_km = float(match.group(2))
        
        return entry
    
    def _identify_current_rider(self, entries: List[LeaderboardEntry]):
        """Identify which entry is the current rider"""
        # Current rider typically:
        # 1. Has no time gap (or shows as leader)
        # 2. Is highlighted (but we can't detect that from OCR)
        # 3. Usually in middle of visible leaderboard
        
        # Look for entry with no time gap in middle positions
        for i, entry in enumerate(entries):
            if i > 0 and i < len(entries) - 1:  # Not first or last
                if entry.time_gap is None or entry.time_gap == "":
                    entry.is_current_rider = True
                    entry.position = i + 1
                    break
        
        # Assign positions to all entries
        for i, entry in enumerate(entries):
            if entry.position is None:
                entry.position = i + 1


def main():
    """Test leaderboard extraction"""
    import sys
    
    if len(sys.argv) > 1:
        image_path = sys.argv[1]
    else:
        print("Usage: python leaderboard_extractor.py <image_path>")
        return
    
    extractor = LeaderboardExtractor(debug=True)
    entries = extractor.extract_leaderboard(image_path)
    
    print("\nLeaderboard Entries:")
    print("-" * 60)
    
    for entry in entries:
        current = " [YOU]" if entry.is_current_rider else ""
        gap = entry.time_gap if entry.time_gap else "---"
        
        print(f"{entry.position}. {gap:>6} {entry.name:<20} "
              f"{entry.watts_per_kg or 0:.1f} w/kg  "
              f"{entry.distance_km or 0:.1f} km{current}")
    
    # Also extract from full image to show what we found
    print("\n\nFull image analysis:")
    image = cv2.imread(image_path)
    result = extractor.ocr.ocr(image, cls=True)
    
    if result and result[0]:
        for line in result[0]:
            text = line[1][0]
            coords = line[0]
            x_min = min(p[0] for p in coords)
            y_min = min(p[1] for p in coords)
            
            # Only show items on right side that look like leaderboard
            if x_min > 1400 and ('w/kg' in text or 'km' in text.lower() or 
                                 '.' in text or text.startswith('+')):
                print(f"({int(x_min)}, {int(y_min)}): {text}")


if __name__ == "__main__":
    main()