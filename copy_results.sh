#!/bin/bash
# Copy ZwiftPower results from Windows Downloads to zwift-race-finder directory

DOWNLOAD_FILE="/mnt/c/Users/YOUR_USERNAME/Downloads/zwiftpower_results.json"
TARGET_FILE="$HOME/tools/rust/zwift-race-finder/zwiftpower_results.json"

if [[ -f "$DOWNLOAD_FILE" ]]; then
    cp "$DOWNLOAD_FILE" "$TARGET_FILE"
    echo "✅ Copied zwiftpower_results.json from Downloads"
    echo ""
    echo "File size: $(wc -l < "$TARGET_FILE") lines"
    echo ""
    echo "Next step: ~/tools/rust/zwift-race-finder/export_zwiftpower_logged_in.sh import"
else
    echo "❌ File not found: $DOWNLOAD_FILE"
    echo ""
    echo "Make sure you:"
    echo "1. Ran the JavaScript in ZwiftPower console"
    echo "2. The file downloaded successfully"
fi