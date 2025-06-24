#!/usr/bin/env python3
"""Import route data including lead-in distances from zwift-data-reference."""

import sqlite3
import os
import re
from pathlib import Path


def parse_routes_ts():
    """Parse routes.ts file and extract route data."""
    routes_file = Path("zwift-data-reference/src/routes.ts")
    if not routes_file.exists():
        print(f"Error: {routes_file} not found")
        return []

    with open(routes_file, "r") as f:
        content = f.read()

    # Find the routes array (note the opening parenthesis after =)
    routes_match = re.search(
        r"export const routes.*?=.*?\(\s*\[(.*?)\]\s*\)", content, re.DOTALL
    )
    if not routes_match:
        print("Error: Could not find routes array")
        return []

    routes_content = routes_match.group(1)

    # Parse individual route arrays
    route_pattern = r'\[\s*(\d+),\s*"([^"]+)",\s*"([^"]+)",\s*"([^"]+)",\s*(\w+),\s*([\d.]+),\s*(\d+),\s*([\d.]+|undefined),\s*(\d+|undefined),\s*([\d.]+|undefined),\s*(\d+|undefined),\s*([\d.]+|undefined),\s*(\d+|undefined),'

    routes = []
    for match in re.finditer(route_pattern, routes_content):
        route_id = int(match.group(1))
        name = match.group(2)
        slug = match.group(3)
        world = match.group(4)
        # Skip index 5 (eventOnly boolean)
        distance = float(match.group(6))
        elevation = int(match.group(7))

        # Lead-in distances (handle 'undefined' values)
        lead_in = float(match.group(8)) if match.group(8) != "undefined" else 0.0
        lead_in_elev = int(match.group(9)) if match.group(9) != "undefined" else 0
        lead_in_free = (
            float(match.group(10)) if match.group(10) != "undefined" else None
        )
        lead_in_free_elev = (
            int(match.group(11)) if match.group(11) != "undefined" else None
        )
        lead_in_meetup = (
            float(match.group(12)) if match.group(12) != "undefined" else None
        )
        lead_in_meetup_elev = (
            int(match.group(13)) if match.group(13) != "undefined" else None
        )

        routes.append(
            {
                "route_id": route_id,
                "name": name,
                "slug": slug,
                "world": world,
                "distance_km": distance,
                "elevation_m": elevation,
                "lead_in_distance_km": lead_in,
                "lead_in_elevation_m": lead_in_elev,
                "lead_in_distance_free_ride_km": lead_in_free,
                "lead_in_elevation_free_ride_m": lead_in_free_elev,
                "lead_in_distance_meetups_km": lead_in_meetup,
                "lead_in_elevation_meetups_m": lead_in_meetup_elev,
                "surface": "road",  # Default, can be updated later
            }
        )

    return routes


def update_database(routes):
    """Update SQLite database with route data."""
    db_path = os.path.expanduser("~/.local/share/zwift-race-finder/races.db")

    if not os.path.exists(db_path):
        print(f"Error: Database not found at {db_path}")
        return

    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()

    # Check if new columns exist, if not add them
    cursor.execute("PRAGMA table_info(routes)")
    columns = [col[1] for col in cursor.fetchall()]

    if "lead_in_distance_km" not in columns:
        print("Adding lead-in columns to routes table...")
        cursor.execute(
            "ALTER TABLE routes ADD COLUMN lead_in_distance_km REAL DEFAULT 0.0"
        )
        cursor.execute(
            "ALTER TABLE routes ADD COLUMN lead_in_elevation_m INTEGER DEFAULT 0"
        )
        cursor.execute(
            "ALTER TABLE routes ADD COLUMN lead_in_distance_free_ride_km REAL"
        )
        cursor.execute(
            "ALTER TABLE routes ADD COLUMN lead_in_elevation_free_ride_m INTEGER"
        )
        cursor.execute("ALTER TABLE routes ADD COLUMN lead_in_distance_meetups_km REAL")
        cursor.execute(
            "ALTER TABLE routes ADD COLUMN lead_in_elevation_meetups_m INTEGER"
        )
        cursor.execute("ALTER TABLE routes ADD COLUMN slug TEXT")
        conn.commit()

    # Update or insert routes
    updated = 0
    inserted = 0

    for route in routes:
        # Check if route exists
        cursor.execute(
            "SELECT route_id FROM routes WHERE route_id = ?", (route["route_id"],)
        )
        exists = cursor.fetchone() is not None

        if exists:
            # Update existing route with lead-in data
            cursor.execute(
                """
                UPDATE routes SET 
                    lead_in_distance_km = ?,
                    lead_in_elevation_m = ?,
                    lead_in_distance_free_ride_km = ?,
                    lead_in_elevation_free_ride_m = ?,
                    lead_in_distance_meetups_km = ?,
                    lead_in_elevation_meetups_m = ?,
                    slug = ?
                WHERE route_id = ?
            """,
                (
                    route["lead_in_distance_km"],
                    route["lead_in_elevation_m"],
                    route["lead_in_distance_free_ride_km"],
                    route["lead_in_elevation_free_ride_m"],
                    route["lead_in_distance_meetups_km"],
                    route["lead_in_elevation_meetups_m"],
                    route["slug"],
                    route["route_id"],
                ),
            )
            updated += 1
        else:
            # Insert new route
            cursor.execute(
                """
                INSERT INTO routes (
                    route_id, name, world, distance_km, elevation_m, surface,
                    lead_in_distance_km, lead_in_elevation_m,
                    lead_in_distance_free_ride_km, lead_in_elevation_free_ride_m,
                    lead_in_distance_meetups_km, lead_in_elevation_meetups_m,
                    slug
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
                (
                    route["route_id"],
                    route["name"],
                    route["world"],
                    route["distance_km"],
                    route["elevation_m"],
                    route["surface"],
                    route["lead_in_distance_km"],
                    route["lead_in_elevation_m"],
                    route["lead_in_distance_free_ride_km"],
                    route["lead_in_elevation_free_ride_m"],
                    route["lead_in_distance_meetups_km"],
                    route["lead_in_elevation_meetups_m"],
                    route["slug"],
                ),
            )
            inserted += 1

    conn.commit()
    conn.close()

    print("Database update complete:")
    print(f"  - Updated {updated} existing routes with lead-in data")
    print(f"  - Inserted {inserted} new routes")
    print(f"  - Total routes processed: {len(routes)}")


def main():
    """Main function."""
    print("Importing route data from zwift-data-reference...")

    routes = parse_routes_ts()
    if not routes:
        print("No routes found to import")
        return

    print(f"Found {len(routes)} routes")

    # Show some sample data
    print("\nSample routes with lead-in distances:")
    for route in routes[:5]:
        if route["lead_in_distance_km"] > 0:
            print(
                f"  - {route['name']}: {route['distance_km']} km + {route['lead_in_distance_km']} km lead-in"
            )

    update_database(routes)


if __name__ == "__main__":
    main()
