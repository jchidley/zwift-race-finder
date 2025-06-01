#!/bin/bash
# Debug script to examine Stage 4 event data

echo "Fetching Stage 4 event data..."
cargo run -- --debug 2>&1 | \
  awk '/Stage 4: Makuri May: Three Village Loop/{found=1} found && /^Event:/{if(count++) found=0} found{print}' | \
  head -200 > stage4_debug.txt

echo "Stage 4 debug data saved to stage4_debug.txt"

# Look for key fields
echo -e "\n=== Key Information ==="
grep -E "Distance:|Description:|laps:|Subgroups:" stage4_debug.txt || echo "No key fields found"

# Check for lap info in subgroups
echo -e "\n=== Subgroup Details ==="
grep -A5 "Subgroups:" stage4_debug.txt || echo "No subgroup info found"