#!/bin/bash
# Fetch raw event data and filter for Epic Race

curl -s "https://us-or-rly101.zwift.com/api/public/events/upcoming" \
  -H "Content-Type: application/json" \
  -H "User-Agent: Zwift Race Finder" | \
  jq '.[] | select(.name | contains("Epic Race - Sacre Bleu"))'