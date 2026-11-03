#!/bin/bash

echo "Fetching Stage 4 event data..."
cargo run --bin zwift-race-finder -- --debug 2>&1 | \
  awk '/Stage 4: Makuri May: Three Village Loop/{found=1} found && /^Event:/{if(count++) found=0} found{print}' | \
  head -200 > stage4_debug.txt

echo "Stage 4 debug data saved to stage4_debug.txt"

echo -e "\n=== Key Information ==="
grep -E "Distance:|Description:|laps:|Subgroups:" stage4_debug.txt || echo "No key fields found"

echo -e "\n=== Subgroup Details ==="
grep -A5 "Subgroups:" stage4_debug.txt || echo "No subgroup info found"