#!/usr/bin/env python3
# /// script
# dependencies = ["requests"]
# ///

import requests
import json
from datetime import datetime, timedelta

# Fetch upcoming events from Zwift API
def fetch_events():
    # Only get next 6 hours of events to reduce noise
    end_time = datetime.utcnow() + timedelta(hours=6)
    
    url = "https://api.zwift.com/events"
    params = {
        "start": 0,
        "limit": 20,
        "eventStartsAfter": datetime.utcnow().isoformat() + "Z",
        "eventStartsBefore": end_time.isoformat() + "Z",
    }
    
    response = requests.get(url, params=params)
    events = response.json()
    
    # Find events with multiple laps or interesting structure
    for event in events:
        if "lap" in event.get("name", "").lower() or len(event.get("eventSubGroups", [])) > 0:
            print(f"\n{'='*80}")
            print(f"Event: {event.get('name')}")
            print(f"Route ID: {event.get('routeId')}")
            print(f"Distance: {event.get('distanceInMeters', 0)/1000:.1f} km")
            
            # Pretty print the full event structure
            print("\nFull event data:")
            print(json.dumps(event, indent=2, sort_keys=True))
            
            # Show subgroups if any
            if event.get("eventSubGroups"):
                print("\nSubgroups (per category):")
                for sg in event["eventSubGroups"]:
                    print(f"  - {sg.get('name')}: {sg.get('distanceInMeters', 0)/1000:.1f} km")
            
            # Only show first 3 matching events
            break

if __name__ == "__main__":
    fetch_events()