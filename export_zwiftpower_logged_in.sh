#!/bin/bash
# Export ZwiftPower race results after manual login
# Run this script after logging into ZwiftPower in your browser

set -euo pipefail

cat << 'EOF'
🚴 ZwiftPower Race Results Exporter
==================================

This script helps export your race results from ZwiftPower after you've logged in.

Instructions:
1. Open your browser and log into ZwiftPower
2. Navigate to your profile: https://zwiftpower.com/profile.php?z=YOUR_PROFILE_ID
3. Open the browser's Developer Tools (F12)
4. Go to the Console tab
5. Copy the JavaScript code from: ~/tools/rust/zwift-race-finder/extract_zwiftpower.js
   Or run: cat ~/tools/rust/zwift-race-finder/extract_zwiftpower.js | xclip -selection clipboard
6. Paste and run the code in the browser console
7. The file will download to: /mnt/c/Users/YOUR_USERNAME/Downloads/zwiftpower_results.json
8. Copy it here: cp /mnt/c/Users/YOUR_USERNAME/Downloads/zwiftpower_results.json ~/tools/rust/zwift-race-finder/
9. Run this script again with: ~/tools/rust/zwift-race-finder/export_zwiftpower_logged_in.sh import

JavaScript code location:
-------------------------------------------
~/tools/rust/zwift-race-finder/extract_zwiftpower.js
EOF

cat << 'JAVASCRIPT'
// Extract race results from ZwiftPower profile page
(function() {
    const results = [];
    const tables = document.querySelectorAll('table.table-striped');
    
    tables.forEach(table => {
        const rows = table.querySelectorAll('tbody tr');
        rows.forEach(row => {
            const cells = row.querySelectorAll('td');
            if (cells.length >= 5) {
                const dateText = cells[0]?.textContent?.trim() || '';
                const eventName = cells[1]?.textContent?.trim() || '';
                const eventLink = cells[1]?.querySelector('a')?.href || '';
                const category = cells[2]?.textContent?.trim() || '';
                const position = cells[3]?.textContent?.trim() || '';
                const timeText = cells[4]?.textContent?.trim() || '';
                const wkg = cells[5]?.textContent?.trim() || '';
                
                // Extract event ID from link
                let eventId = null;
                const idMatch = eventLink.match(/id=(\d+)/);
                if (idMatch) eventId = idMatch[1];
                
                // Parse time to minutes
                let minutes = 0;
                const timeParts = timeText.split(':');
                if (timeParts.length === 3) {
                    minutes = parseInt(timeParts[0]) * 60 + parseInt(timeParts[1]);
                } else if (timeParts.length === 2) {
                    minutes = parseInt(timeParts[0]) * 60 + parseInt(timeParts[1]);
                } else if (timeParts.length === 1 && !isNaN(parseInt(timeParts[0]))) {
                    minutes = parseInt(timeParts[0]);
                }
                
                if (eventName && minutes > 0) {
                    results.push({
                        date: dateText,
                        event_name: eventName,
                        event_id: eventId,
                        category: category,
                        position: position,
                        time_text: timeText,
                        time_minutes: minutes,
                        w_kg: wkg,
                        event_link: eventLink
                    });
                }
            }
        });
    });
    
    console.log(`Found ${results.length} race results`);
    
    // Create downloadable JSON
    const json = JSON.stringify(results, null, 2);
    const blob = new Blob([json], {type: 'application/json'});
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'zwiftpower_results.json';
    a.click();
    
    // Also log to console for copy/paste
    console.log('Results:', results);
    return results;
})();
JAVASCRIPT

echo ""
echo "-------------------------------------------"
echo ""

# Check if we're in import mode
if [[ "${1:-}" == "import" ]]; then
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    RESULTS_FILE="${SCRIPT_DIR}/zwiftpower_results.json"
    
    if [[ ! -f "$RESULTS_FILE" ]]; then
        echo "❌ Error: $RESULTS_FILE not found!"
        echo "Please follow the instructions above first."
        exit 1
    fi
    
    echo "Importing results from zwiftpower_results.json..."
    
    DB_PATH="${HOME}/.local/share/zwift-race-finder/races.db"
    
    # Convert JSON to SQL
    cd "$SCRIPT_DIR"
    jq -r '.[] | 
        "INSERT OR IGNORE INTO zwiftpower_results (date, event_name, category, position, time_minutes, time_text, event_id) VALUES (" +
        "\"" + .date + "\", " +
        "\"" + (.event_name | gsub("\""; "\"\"")) + "\", " +
        "\"" + .category + "\", " +
        "\"" + .position + "\", " +
        (.time_minutes | tostring) + ", " +
        "\"" + .time_text + "\", " +
        "\"" + (.event_id // "unknown") + "\");"' zwiftpower_results.json > import_results.sql
    
    # Add table creation
    {
        echo "CREATE TABLE IF NOT EXISTS zwiftpower_results ("
        echo "    id INTEGER PRIMARY KEY AUTOINCREMENT,"
        echo "    date TEXT NOT NULL,"
        echo "    event_name TEXT NOT NULL,"
        echo "    category TEXT,"
        echo "    position TEXT,"
        echo "    time_minutes INTEGER NOT NULL,"
        echo "    time_text TEXT,"
        echo "    event_id TEXT,"
        echo "    route_id INTEGER,"
        echo "    scraped_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP"
        echo ");"
        echo ""
        cat import_results.sql
    } > final_import.sql
    
    # Import to database
    sqlite3 "$DB_PATH" < final_import.sql
    
    # Show summary
    echo ""
    echo "✅ Import complete!"
    echo ""
    echo "Summary:"
    sqlite3 "$DB_PATH" <<EOF
.mode column
.headers on
SELECT COUNT(*) as total_races FROM zwiftpower_results;
SELECT MIN(time_minutes) as shortest_minutes, MAX(time_minutes) as longest_minutes, ROUND(AVG(time_minutes)) as avg_minutes FROM zwiftpower_results;

SELECT event_name, COUNT(*) as times, ROUND(AVG(time_minutes)) as avg_time
FROM zwiftpower_results 
GROUP BY event_name 
HAVING COUNT(*) > 1
ORDER BY times DESC 
LIMIT 10;
EOF
    
    # Clean up
    rm -f import_results.sql final_import.sql
    
    echo ""
    echo "Now we can use your actual race times for regression testing!"
    echo "Run: cargo test test_route_id_regression_with_actual_results"
else
    echo "After running the JavaScript:"
    echo "1. Copy the downloaded file:"
    echo "   cp /mnt/c/Users/YOUR_USERNAME/Downloads/zwiftpower_results.json ~/tools/rust/zwift-race-finder/"
    echo ""
    echo "2. Then import it:"
    echo "   ~/tools/rust/zwift-race-finder/export_zwiftpower_logged_in.sh import"
    echo ""
    echo "JavaScript code is saved in:"
    echo "  ~/tools/rust/zwift-race-finder/extract_zwiftpower.js"
fi