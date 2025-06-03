#!/bin/bash
# Update rider stats in the database

DB_PATH="$HOME/.local/share/zwift-race-finder/races.db"

# Check if weight is provided as argument
if [ "$#" -eq 0 ]; then
    echo "Usage: $0 <weight_kg> [ftp_watts]"
    echo "Example: $0 86.0 250"
    exit 1
fi

WEIGHT=$1
FTP=${2:-NULL}  # Optional FTP parameter

# Insert or update rider stats
sqlite3 "$DB_PATH" <<EOF
INSERT OR REPLACE INTO rider_stats (id, height_m, weight_kg, ftp_watts, updated_at)
VALUES (1, 1.82, $WEIGHT, $FTP, datetime('now'));
EOF

echo "Updated rider stats: Height=1.82m, Weight=${WEIGHT}kg, FTP=${FTP}W"

# Show current stats
echo -e "\nCurrent rider stats:"
sqlite3 "$DB_PATH" "SELECT * FROM rider_stats;"