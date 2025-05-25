// Extract race results from ZwiftPower profile page
// Make sure you're on the main profile page with your race history
(function() {
    const results = [];
    
    // Look for the results table - it usually has class "table" and contains race history
    const tables = document.querySelectorAll('table');
    let resultsTable = null;
    
    // Find the table that contains race results (look for date patterns)
    tables.forEach(table => {
        const firstCell = table.querySelector('td');
        if (firstCell && firstCell.textContent.match(/\d{4}-\d{2}-\d{2}/)) {
            resultsTable = table;
        }
    });
    
    if (!resultsTable) {
        // Try alternate selector for results
        resultsTable = document.querySelector('#profile_results table') || 
                      document.querySelector('.results table') ||
                      document.querySelector('table.table');
    }
    
    if (resultsTable) {
        const rows = resultsTable.querySelectorAll('tbody tr');
        console.log(`Found results table with ${rows.length} rows`);
        
        rows.forEach((row, index) => {
            const cells = row.querySelectorAll('td');
            if (cells.length >= 7) {
                // Typical columns: Date | Event | Cat | Pos | Time | W/kg | HR
                const dateText = cells[0]?.textContent?.trim() || '';
                const eventCell = cells[1];
                const eventName = eventCell?.textContent?.trim() || '';
                const eventLink = eventCell?.querySelector('a')?.href || '';
                const category = cells[2]?.textContent?.trim() || '';
                const position = cells[3]?.textContent?.trim() || '';
                const timeText = cells[4]?.textContent?.trim() || '';
                const wkg = cells[5]?.textContent?.trim() || '';
                
                // Extract event ID from link
                let eventId = null;
                const idMatch = eventLink.match(/[?&]id=(\d+)/);
                if (idMatch) eventId = idMatch[1];
                
                // Parse time to minutes (handle formats: "1:23:45" or "45:23" or "DNF")
                let minutes = 0;
                if (timeText && !timeText.includes('DNF') && !timeText.includes('DQ')) {
                    const timeParts = timeText.split(':');
                    if (timeParts.length === 3) {
                        // HH:MM:SS
                        minutes = parseInt(timeParts[0]) * 60 + parseInt(timeParts[1]);
                    } else if (timeParts.length === 2) {
                        // MM:SS
                        minutes = parseInt(timeParts[0]);
                    }
                }
                
                // Only add if we have a valid date and event name
                if (dateText.match(/\d{4}-\d{2}-\d{2}/) && eventName && eventName.length > 3) {
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
                    
                    if (index < 5) {
                        console.log(`Sample result: ${dateText} - ${eventName} - ${timeText}`);
                    }
                }
            }
        });
    } else {
        console.error("Could not find results table. Make sure you're on the profile page with race history.");
        console.log("Available tables:", tables.length);
        
        // Show what tables are on the page for debugging
        tables.forEach((table, i) => {
            const firstRow = table.querySelector('tr');
            console.log(`Table ${i}: ${firstRow?.textContent?.substring(0, 100)}`);
        });
    }
    
    console.log(`Found ${results.length} race results`);
    
    if (results.length > 0) {
        // Create downloadable JSON
        const json = JSON.stringify(results, null, 2);
        const blob = new Blob([json], {type: 'application/json'});
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = 'zwiftpower_results.json';
        a.click();
        
        // Also log to console
        console.log('Results:', results);
        console.log('First result:', results[0]);
        console.log('Last result:', results[results.length - 1]);
    } else {
        console.error("No results found. The page structure may have changed.");
    }
    
    return results;
})();