#!/usr/bin/env python3
# ABOUTME: Fetches accurate route data from WhatsOnZwift using route slugs

import sqlite3
import time
import re
from pathlib import Path
import urllib.request
import urllib.error
from html.parser import HTMLParser
import argparse


def get_db_path():
    """Get the path to the database"""
    return Path.home() / ".local/share/zwift-race-finder/races.db"


def parse_distance(text):
    """Parse distance from text like '41.27 km / 25.64 mi' to float km"""
    if not text:
        return None
    match = re.search(r"([\d.]+)\s*km", text)
    if match:
        return float(match.group(1))
    return None


def parse_elevation(text):
    """Parse elevation from text like '1,152 m / 3,779.5 ft' to int meters"""
    if not text:
        return None
    # Remove commas and extract number before 'm'
    text = text.replace(",", "")
    match = re.search(r"([\d.]+)\s*m", text)
    if match:
        return int(float(match.group(1)))
    return None


class RouteDataParser(HTMLParser):
    """Parse route data from WhatsOnZwift HTML"""

    def __init__(self):
        super().__init__()
        self.stats = {}
        self.in_stat_item = False
        self.in_label = False
        self.in_value = False
        self.current_label = ""
        self.current_value = ""

    def handle_starttag(self, tag, attrs):
        if tag == "div":
            classes = dict(attrs).get("class", "")
            if "route-stat-item" in classes:
                self.in_stat_item = True
            elif "route-stat-label" in classes and self.in_stat_item:
                self.in_label = True
            elif "route-stat-value" in classes and self.in_stat_item:
                self.in_value = True

    def handle_endtag(self, tag):
        if tag == "div":
            if self.in_label:
                self.in_label = False
            elif self.in_value:
                self.in_value = False
                # Process the label/value pair
                label = self.current_label.strip().lower()
                value = self.current_value.strip()

                if "distance" in label and "lead-in" not in label:
                    self.stats["distance_km"] = parse_distance(value)
                elif "elevation gain" in label and "lead-in" not in label:
                    self.stats["elevation_m"] = parse_elevation(value)
                elif "lead-in distance" in label:
                    self.stats["lead_in_distance_km"] = parse_distance(value)
                elif "lead-in elevation" in label:
                    self.stats["lead_in_elevation_m"] = parse_elevation(value)

                self.current_label = ""
                self.current_value = ""
            elif self.in_stat_item:
                self.in_stat_item = False

    def handle_data(self, data):
        if self.in_label:
            self.current_label += data
        elif self.in_value:
            self.current_value += data


def fetch_route_data(world, slug):
    """Fetch route data from WhatsOnZwift"""
    url = f"https://whatsonzwift.com/world/{world}/route/{slug}"

    try:
        with urllib.request.urlopen(url, timeout=10) as response:
            html = response.read().decode("utf-8")

        parser = RouteDataParser()
        parser.feed(html)

        return parser.stats if parser.stats else None

    except urllib.error.URLError as e:
        print(f"Error fetching {url}: {e}")
        return None
    except Exception as e:
        print(f"Error parsing data from {url}: {e}")
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
    parser = argparse.ArgumentParser(description="Fetch route data from WhatsOnZwift")
    parser.add_argument("--route-id", type=int, help="Fetch data for specific route ID")
    parser.add_argument(
        "--limit", type=int, default=10, help="Number of routes to process"
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Show what would be updated without making changes",
    )
    args = parser.parse_args()

    conn = sqlite3.connect(get_db_path())

    try:
        # Get routes with slugs
        if args.route_id:
            query = "SELECT route_id, name, world, slug FROM routes WHERE route_id = ? AND slug IS NOT NULL"
            routes = conn.execute(query, (args.route_id,)).fetchall()
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

        print(f"Found {len(routes)} routes to process")

        for route_id, name, world, slug in routes:
            print(f"\nProcessing: {name} ({world}/{slug})")

            # Fetch data from WhatsOnZwift
            data = fetch_route_data(world, slug)

            if data:
                print(f"  Found data: {data}")

                if not args.dry_run:
                    if update_route_data(conn, route_id, data):
                        print(f"  ✓ Updated route {route_id}")
                        conn.commit()
                    else:
                        print(f"  - No updates needed for route {route_id}")
                else:
                    print(f"  [DRY RUN] Would update route {route_id}")
            else:
                print("  ✗ No data found")

            # Be polite to the server
            time.sleep(1)

    finally:
        conn.close()


if __name__ == "__main__":
    main()
