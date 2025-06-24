#!/usr/bin/env python3
"""
Analyze Strava activities to understand Zwift race performance patterns
"""

import json
import sqlite3
from pathlib import Path
from typing import List, Dict, Any


def load_activities(
    filepath: str = "strava_zwift_activities.json",
) -> List[Dict[str, Any]]:
    """Load Strava activities from JSON file"""
    with open(filepath, "r") as f:
        return json.load(f)


def analyze_race_speeds(activities: List[Dict[str, Any]]) -> None:
    """Analyze actual race speeds to calibrate predictions"""
    print("🏁 Zwift Race Speed Analysis")
    print("=" * 50)

    # Filter for races
    races = [a for a in activities if is_race(a["name"])]

    if not races:
        print("No races found in activities")
        return

    # Calculate statistics
    speeds = [a["average_speed_kmh"] for a in races if a.get("average_speed_kmh")]
    durations = [a["moving_time_minutes"] for a in races]
    distances = [a["distance_km"] for a in races]

    print(f"\n📊 Found {len(races)} races")
    print("\nSpeed Statistics (km/h):")
    print(f"  Average: {sum(speeds) / len(speeds):.1f}")
    print(f"  Min: {min(speeds):.1f}")
    print(f"  Max: {max(speeds):.1f}")

    print("\nDuration Statistics (minutes):")
    print(f"  Average: {sum(durations) / len(durations):.0f}")
    print(f"  Min: {min(durations)}")
    print(f"  Max: {max(durations)}")

    # Group by duration ranges
    print("\n🕐 Speed by Race Duration:")
    duration_buckets = {
        "Short (< 45 min)": [],
        "Medium (45-75 min)": [],
        "Long (> 75 min)": [],
    }

    for race in races:
        minutes = race["moving_time_minutes"]
        speed = race.get("average_speed_kmh", 0)

        if minutes < 45:
            duration_buckets["Short (< 45 min)"].append(speed)
        elif minutes <= 75:
            duration_buckets["Medium (45-75 min)"].append(speed)
        else:
            duration_buckets["Long (> 75 min)"].append(speed)

    for bucket, speeds in duration_buckets.items():
        if speeds:
            avg_speed = sum(speeds) / len(speeds)
            print(f"  {bucket}: {avg_speed:.1f} km/h (n={len(speeds)})")

    # Show recent races for context
    print("\n📅 Recent Race Results:")
    for race in sorted(races, key=lambda x: x["start_date"], reverse=True)[:10]:
        date = race["start_date"].split("T")[0]
        name = race["name"][:40]
        minutes = race["moving_time_minutes"]
        km = race["distance_km"]
        speed = race.get("average_speed_kmh", 0)
        print(f"  {date} - {name:40} - {minutes:3}min - {km:4}km - {speed:4.1f}km/h")


def is_race(name: str) -> bool:
    """Determine if an activity is likely a race based on name"""
    race_keywords = ["race", "racing", "tt", "time trial", "fondo", "series"]
    exclude_keywords = [
        "group ride",
        "easy",
        "recovery",
        "endurance",
        "zone 2",
        "meetup",
        "workout",
    ]

    name_lower = name.lower()

    # Exclude non-races
    for keyword in exclude_keywords:
        if keyword in name_lower:
            return False

    # Include races
    for keyword in race_keywords:
        if keyword in name_lower:
            return True

    return False


def update_database_with_insights(
    db_path: str, activities: List[Dict[str, Any]]
) -> None:
    """Update database with insights from Strava analysis"""
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()

    # Calculate Jack's actual average race speed
    races = [a for a in activities if is_race(a["name"])]
    if races:
        avg_speed = sum(r["average_speed_kmh"] for r in races) / len(races)
        print(f"\n💡 Your average race speed: {avg_speed:.1f} km/h")
        print("   This includes draft benefit in races")
        print("   Use this to calibrate speed estimates!")

    conn.close()


def main():
    """Main analysis function"""
    # Load activities
    activities_file = Path(__file__).parent / "strava_zwift_activities.json"

    if not activities_file.exists():
        print("❌ No activities file found. Run ./strava_fetch_activities.sh first")
        return

    activities = load_activities(str(activities_file))

    # Run analysis
    analyze_race_speeds(activities)

    # Update database insights
    db_path = Path.home() / ".local/share/zwift-race-finder/races.db"
    if db_path.exists():
        update_database_with_insights(str(db_path), activities)


if __name__ == "__main__":
    main()
