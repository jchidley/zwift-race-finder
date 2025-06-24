#!/usr/bin/env python3
# ABOUTME: Simple regex-based WhatsOnZwift route data fetcher

import sqlite3
import time
import re
from pathlib import Path
import urllib.request
import argparse


def get_db_path():
    """Get the path to the database"""
    return Path.home() / ".local/share/zwift-race-finder/races.db"


def transform_slug(slug):
    """Transform database slug to WhatsOnZwift format"""
    if slug.endswith("-rev"):
        return slug[:-4] + "-reverse"
    # Special case for Bologna Time Trial
    if slug == "time-trial":
        return "bologna-time-trial"
    return slug


def fetch_and_parse_route(world, slug):
    """Fetch and parse route data from WhatsOnZwift"""
    woz_slug = transform_slug(slug)
    url = f"https://whatsonzwift.com/world/{world}/route/{woz_slug}"

    print(f"  URL: {url}")

    try:
        # Create request with user agent
        request = urllib.request.Request(url)
        request.add_header(
            "User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36"
        )

        with urllib.request.urlopen(request, timeout=10) as response:
            html = response.read().decode("utf-8")

        # Extract data using regex
        stats = {}

        # Look for the route details table
        # Pattern: <td><strong>Distance</strong></td><td>24.65 km / 15.31 mi</td>
        patterns = {
            "distance_km": r"<td><strong>Distance</strong></td><td>([\d.]+)\s*km",
            "elevation_m": r"<td><strong>Elevation gain</strong></td><td>([\d.]+)\s*m",
            "lead_in_distance_km": r"<td><strong>Lead-in distance</strong></td><td>([\d.]+)\s*km",
            "lead_in_elevation_m": r"<td><strong>Lead-in elevation gain</strong></td><td>([\d.]+)\s*m",
        }

        for key, pattern in patterns.items():
            match = re.search(pattern, html)
            if match:
                value = float(match.group(1))
                if "elevation" in key:
                    value = int(value)
                stats[key] = value

        return stats

    except urllib.error.HTTPError as e:
        print(f"  ✗ HTTP error {e.code}")
        return None
    except Exception as e:
        print(f"  ✗ Error: {e}")
        return None


def update_database(conn, route_id, stats):
    """Update route in database"""
    if not stats:
        return False

    updates = []
    params = []

    for key, value in stats.items():
        updates.append(f"{key} = ?")
        params.append(value)

    if updates:
        params.append(route_id)
        query = f"UPDATE routes SET {', '.join(updates)} WHERE route_id = ?"
        conn.execute(query, params)
        return True

    return False


def main():
    parser = argparse.ArgumentParser(description="Fetch WhatsOnZwift route data")
    parser.add_argument(
        "--limit", type=int, default=5, help="Number of routes to fetch"
    )
    parser.add_argument("--route-id", type=int, help="Specific route ID")
    parser.add_argument(
        "--dry-run", action="store_true", help="Show what would be done"
    )
    args = parser.parse_args()

    conn = sqlite3.connect(get_db_path())

    try:
        # Get routes to process
        if args.route_id:
            query = """
                SELECT route_id, name, world, slug 
                FROM routes 
                WHERE route_id = ? AND slug IS NOT NULL
            """
            routes = conn.execute(query, (args.route_id,)).fetchall()
        else:
            query = """
                SELECT route_id, name, world, slug 
                FROM routes 
                WHERE slug IS NOT NULL 
                AND (lead_in_distance_km IS NULL OR lead_in_distance_km = 0)
                ORDER BY name 
                LIMIT ?
            """
            routes = conn.execute(query, (args.limit,)).fetchall()

        print(f"Processing {len(routes)} routes\n")

        success = 0
        for route_id, name, world, slug in routes:
            print(f"\n{name} ({world})")
            stats = fetch_and_parse_route(world, slug)

            if stats:
                print("  ✓ Found data:")
                for key, value in stats.items():
                    label = key.replace("_", " ").title()
                    unit = "km" if "distance" in key else "m"
                    print(f"    {label}: {value} {unit}")

                if not args.dry_run:
                    if update_database(conn, route_id, stats):
                        conn.commit()
                        print("  ✓ Updated database")
                        success += 1
                else:
                    print("  [DRY RUN] Would update database")
                    success += 1

            time.sleep(1)  # Be polite

        print(f"\nProcessed {success}/{len(routes)} routes successfully")

    except KeyboardInterrupt:
        print("\nInterrupted")
    finally:
        conn.close()


if __name__ == "__main__":
    main()
