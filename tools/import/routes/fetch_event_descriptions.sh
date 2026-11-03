#!/bin/bash

ROUTE_IDS=("1917017591" "2128890027" "3366225080")

echo "Fetching events from Zwift API..."
EVENTS=$(curl -s "https://us-or-rly101.zwift.com/api/public/events/upcoming")

if [ $? -ne 0 ]; then
    echo "Error: Failed to fetch events from API"
    exit 1
fi

for ROUTE_ID in "${ROUTE_IDS[@]}"; do
    echo -e "\n========================================"
    echo "Route ID: $ROUTE_ID"
    echo "========================================"

    ROUTE_EVENTS=$(echo "$EVENTS" | jq -r --arg rid "$ROUTE_ID" '
        .[] | select(.routeId == ($rid | tonumber)) | 
        {
            name: .name,
            description: .description,
            eventStart: .eventStart,
            id: .id
        }'
    )

    if [ -z "$ROUTE_EVENTS" ]; then
        echo "No events found for route ID $ROUTE_ID"
        continue
    fi

    echo "$ROUTE_EVENTS" | jq -r '
        "\nEvent: \(.name)\n" +
        "ID: \(.id)\n" +
        "Start: \(.eventStart)\n" +
        "Description:\n\(.description)\n" +
        "-----------------------------------------"
    '

    echo -e "\nURLs found in descriptions for route $ROUTE_ID:"
    echo "$ROUTE_EVENTS" | jq -r '.description' | grep -Eo 'https?://[^[:space:]]+' | sort -u || echo "No URLs found"
done

echo -e "\n\nDone!"