#!/usr/bin/env python3
# ABOUTME: Fetches route data from WhatsOnZwift using proper HTML parsing

import sqlite3
import time
import re
from pathlib import Path
import urllib.request
import urllib.error
from html.parser import HTMLParser
import argparse
import sys


class WhatsOnZwiftRouteParser(HTMLParser):
    """Parse route data table from WhatsOnZwift"""

    def __init__(self):
        super().__init__()
        self.found_route_details = False
        self.in_table = False
        self.in_row = False
        self.in_cell = False
        self.current_label = ""
        self.current_value = ""
        self.capture_value = False
        self.stats = {}
        self.text_buffer = ""

    def handle_starttag(self, tag, attrs):
        if tag == "h4":
            self.text_buffer = ""
        elif tag == "table" and self.found_route_details:
            # This should be the route details table
            self.in_table = True
            self.found_route_details = False  # Reset for next time
        elif tag == "tr" and self.in_table:
            self.in_row = True
            self.current_label = ""
            self.current_value = ""
        elif tag == "td" and self.in_row:
            self.in_cell = True
        elif tag == "strong" and self.in_cell:
            # This indicates we're in the label cell
            pass

    def handle_endtag(self, tag):
        if tag == "h4":
            # Check if this was the "Route details" heading
            if self.text_buffer.strip() == "Route details":
                self.found_route_details = True
            self.text_buffer = ""
        elif tag == "table":
            self.in_table = False
        elif tag == "tr" and self.in_row:
            # End of row - process the label/value pair
            if self.current_label and self.current_value:
                if self.current_label == "Distance":
                    self.stats["distance_km"] = self.parse_distance(self.current_value)
                elif self.current_label == "Elevation gain":
                    self.stats["elevation_m"] = self.parse_elevation(self.current_value)
                elif self.current_label == "Lead-in distance":
                    self.stats["lead_in_distance_km"] = self.parse_distance(
                        self.current_value
                    )
                elif self.current_label == "Lead-in elevation gain":
                    self.stats["lead_in_elevation_m"] = self.parse_elevation(
                        self.current_value
                    )
            self.in_row = False
        elif tag == "td" and self.in_cell:
            self.in_cell = False
            if not self.current_label:
                # First TD was the label
                self.capture_value = True
            else:
                # Second TD was the value
                self.capture_value = False

    def handle_data(self, data):
        # Capture text for h4 tags
        if hasattr(self, "text_buffer") and self.text_buffer is not None:
            self.text_buffer += data

        if self.in_cell:
            text = data.strip()
            if text:
                if not self.current_label and not self.capture_value:
                    # This is the label
                    self.current_label = text
                elif self.capture_value:
                    # This is the value
                    self.current_value = text

    def parse_distance(self, text):
        """Extract km value from text like '9.19 km / 5.71 mi'"""
        match = re.search(r"([\d.]+)\s*km", text)
        if match:
            return float(match.group(1))
        return None

    def parse_elevation(self, text):
        """Extract meter value from text like '108.9 m / 357.3 ft'"""
        match = re.search(r"([\d.]+)\s*m", text)
        if match:
            return int(float(match.group(1)))
        return None


def get_db_path():
    """Get the path to the database"""
    return Path.home() / ".local/share/zwift-race-finder/races.db"


def transform_slug(slug):
    """Transform database slug to WhatsOnZwift format"""
    # Convert abbreviated reverse suffix to full
    if slug.endswith("-rev"):
        return slug[:-4] + "-reverse"
    return slug


def fetch_route_data(world, slug):
    """Fetch route data from WhatsOnZwift"""
    woz_slug = transform_slug(slug)
    url = f"https://whatsonzwift.com/world/{world}/route/{woz_slug}"

    print(f"  Fetching: {url}")

    try:
        # Create request with user agent
        request = urllib.request.Request(url)
        request.add_header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
        )

        with urllib.request.urlopen(request, timeout=10) as response:
            html = response.read().decode("utf-8")

        parser = WhatsOnZwiftRouteParser()
        parser.feed(html)

        if not parser.stats:
            print("  ✗ No route data found in HTML")
            # Debug: print first few lines to see structure
            lines = html.split("\n")[:5]
            for line in lines:
                if line.strip():
                    print(f"    {line.strip()[:80]}...")

        return parser.stats if parser.stats else None

    except urllib.error.HTTPError as e:
        if e.code == 404:
            print("  ✗ Route not found (404)")
        else:
            print(f"  ✗ HTTP error: {e.code}")
        return None
    except urllib.error.URLError as e:
        print(f"  ✗ Network error: {e}")
        return None
    except Exception as e:
        print(f"  ✗ Parse error: {e}")
        return None


def update_route_data(conn, route_id, data):
    """Update route data in database"""
    updates = []
    params = []

    if "distance_km" in data and data["distance_km"] is not None:
        updates.append("distance_km = ?")
        params.append(data["distance_km"])

    if "elevation_m" in data and data["elevation_m"] is not None:
        updates.append("elevation_m = ?")
        params.append(data["elevation_m"])

    if "lead_in_distance_km" in data and data["lead_in_distance_km"] is not None:
        updates.append("lead_in_distance_km = ?")
        params.append(data["lead_in_distance_km"])

    if "lead_in_elevation_m" in data and data["lead_in_elevation_m"] is not None:
        updates.append("lead_in_elevation_m = ?")
        params.append(data["lead_in_elevation_m"])

    if updates:
        params.append(route_id)
        query = f"UPDATE routes SET {', '.join(updates)} WHERE route_id = ?"
        conn.execute(query, params)
        return True

    return False


def main():
    parser = argparse.ArgumentParser(
        description="Fetch route data from WhatsOnZwift using proper HTML parsing"
    )
    parser.add_argument("--route-id", type=int, help="Fetch data for specific route ID")
    parser.add_argument(
        "--limit", type=int, default=10, help="Number of routes to process"
    )
    parser.add_argument(
        "--dry-run", action="store_true", help="Show what would be updated"
    )
    parser.add_argument(
        "--all-routes", action="store_true", help="Process all routes with slugs"
    )
    args = parser.parse_args()

    conn = sqlite3.connect(get_db_path())

    try:
        # Get routes with slugs
        if args.route_id:
            query = "SELECT route_id, name, world, slug FROM routes WHERE route_id = ? AND slug IS NOT NULL"
            routes = conn.execute(query, (args.route_id,)).fetchall()
        elif args.all_routes:
            query = """
                SELECT route_id, name, world, slug 
                FROM routes 
                WHERE slug IS NOT NULL 
                ORDER BY name
            """
            routes = conn.execute(query).fetchall()
        else:
            query = """
                SELECT route_id, name, world, slug 
                FROM routes 
                WHERE slug IS NOT NULL 
                AND (lead_in_distance_km = 0 OR lead_in_distance_km IS NULL)
                ORDER BY name
                LIMIT ?
            """
            routes = conn.execute(query, (args.limit,)).fetchall()

        print("WhatsOnZwift Route Data Parser")
        print("==============================")
        print(f"Found {len(routes)} routes to process\n")

        success_count = 0
        error_count = 0

        for i, (route_id, name, world, slug) in enumerate(routes):
            print(f"[{i + 1}/{len(routes)}] {name} ({world}/{slug})")

            # Fetch data from WhatsOnZwift
            data = fetch_route_data(world, slug)

            if data:
                print("  ✓ Found data:")
                print(f"    Distance: {data.get('distance_km', 'N/A')} km")
                print(f"    Elevation: {data.get('elevation_m', 'N/A')} m")
                print(f"    Lead-in: {data.get('lead_in_distance_km', 'N/A')} km")
                print(
                    f"    Lead-in elevation: {data.get('lead_in_elevation_m', 'N/A')} m"
                )

                if not args.dry_run:
                    if update_route_data(conn, route_id, data):
                        print("  ✓ Updated database")
                        conn.commit()
                        success_count += 1
                    else:
                        print("  - No updates needed")
                else:
                    print("  [DRY RUN] Would update database")
                    success_count += 1
            else:
                error_count += 1

            # Be polite to the server
            if i < len(routes) - 1:
                time.sleep(1)

        print(f"\nSummary: {success_count} successful, {error_count} errors")

    except KeyboardInterrupt:
        print("\n\nInterrupted by user")
        sys.exit(1)
    finally:
        conn.close()


if __name__ == "__main__":
    main()
