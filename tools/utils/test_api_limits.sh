#!/bin/bash

echo "Testing Zwift API limit parameter..."
echo

# Test different limit values
for limit in 50 100 150 200 250 300 400 500; do
    echo -n "Testing limit=$limit: "
    count=$(curl -s "https://us-or-rly101.zwift.com/api/public/events/upcoming?limit=$limit" | jq '. | length')
    echo "$count events returned"
done

echo
echo "Testing with offset..."
# Test offset with limit=100
for offset in 0 100 200 300; do
    echo -n "Testing limit=100&offset=$offset: "
    count=$(curl -s "https://us-or-rly101.zwift.com/api/public/events/upcoming?limit=100&offset=$offset" | jq '. | length')
    echo "$count events returned"
done