#!/bin/bash
# Collect unknown route IDs from Zwift API for mapping

echo "Collecting unknown route IDs from current Zwift events..."
echo "=================================================="

# Fetch all cycling events and extract unique route IDs
curl -s "https://us-or-rly101.zwift.com/api/public/events/upcoming" | \
jq -r '.[] | 
  select(.sport == "CYCLING") | 
  select(.routeId != null) | 
  "\(.routeId)\t\(.eventType)\t\(.name)\t\(.route // "Unknown")\t\(.distanceInMeters // 0)"' | \
sort -n | uniq > route_data.tmp

echo "Route ID	Type	Event Name	Route Name	Distance(m)"
echo "--------	----	----------	----------	-----------"

# Check which routes we don't have mapped yet
while IFS=$'\t' read -r route_id event_type event_name route_name distance; do
  # Check if this route_id exists in our Rust code
  if ! grep -q "$route_id =>" ../src/main.rs 2>/dev/null; then
    printf "%s\t%s\t%-40s\t%-30s\t%s\n" "$route_id" "$event_type" "${event_name:0:40}" "${route_name:0:30}" "$distance"
  fi
done < route_data.tmp

rm -f route_data.tmp

echo ""
echo "To add these routes to the tool:"
echo "1. Research the actual route distance and elevation on ZwiftHacks or Zwift Insider"
echo "2. Add to get_route_data() function in src/main.rs"
echo "3. Get Jack's actual completion times for regression testing"