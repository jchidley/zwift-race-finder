#!/usr/bin/env python3

import requests
import json
from datetime import datetime, timedelta

def fetch_events():
    end_time = datetime.utcnow() + timedelta(hours=6)

    url = "https://us-or-rly101.zwift.com/api/public/events/upcoming"
    params = {
        "start": 0,
        "limit": 20,
        "eventStartsAfter": datetime.utcnow().isoformat() + "Z",
        "eventStartsBefore": end_time.isoformat() + "Z",
    }

    response = requests.get(url, params=params)
    events = response.json()

    for event in events:
        if "lap" in event.get("name", "").lower() or len(event.get("eventSubGroups", [])) > 0:
            print(f"\n{'='*80}")
            print(f"Event: {event.get('name')}")
            print(f"Route ID: {event.get('routeId')}")
            print(f"Distance: {event.get('distanceInMeters', 0)/1000:.1f} km")

            print("\nFull event data:")
            print(json.dumps(event, indent=2, sort_keys=True))

            if event.get("eventSubGroups"):
                print("\nSubgroups (per category):")
                for sg in event["eventSubGroups"]:
                    print(f"  - {sg.get('name')}: {sg.get('distanceInMeters', 0)/1000:.1f} km")

            break

if __name__ == "__main__":
    fetch_events()