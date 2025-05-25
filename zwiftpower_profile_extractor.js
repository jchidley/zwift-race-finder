// Extract race results from ZwiftPower profile page - Version 2
// Based on actual page structure analysis
(function() {
    const results = [];
    
    // Target the profile_results table directly
    const resultsTable = document.querySelector('#profile_results');
    
    if (!resultsTable) {
        console.error("Could not find #profile_results table");
        return [];
    }
    
    // Get all data rows (skip header)
    const rows = resultsTable.querySelectorAll('tbody tr');
    console.log(`Found ${rows.length} race results`);
    
    rows.forEach((row, index) => {
        const cells = row.querySelectorAll('td');
        
        // Expected columns based on the headers:
        // 0: Category (D)
        // 1: Position (#17)
        // 2: Date (5/24/25)
        // 3: Race name (2025 SISU Pinkki - Stage 5)
        // 4-11: Various power metrics
        // 12: Weight (86.0kg)
        // 13-14: HR metrics
        // 15: Distance (56km)
        // 16: Empty
        // 17: Result/Score (564.02)
        // 18-19: Gain and other
        
        if (cells.length >= 18) {
            const category = cells[0]?.textContent?.trim() || '';
            const position = cells[1]?.textContent?.trim() || '';
            const dateText = cells[2]?.textContent?.trim() || '';
            const raceCell = cells[3];
            const raceName = raceCell?.textContent?.trim() || '';
            const raceLink = raceCell?.querySelector('a')?.href || '';
            const distanceText = cells[15]?.textContent?.trim() || '';
            // Skip resultScore (cells[17]) as it's unreliable
            
            // Extract event ID from link
            let eventId = null;
            const idMatch = raceLink.match(/[?&]id=(\d+)/);
            if (idMatch) eventId = idMatch[1];
            
            // Extract distance (remove 'km' suffix)
            const distance = parseFloat(distanceText.replace('km', '')) || 0;
            
            // Parse date - convert from M/D/YY to YYYY-MM-DD
            let formattedDate = dateText;
            if (dateText.match(/^\d{1,2}\/\d{1,2}\/\d{2}$/)) {
                const [month, day, shortYear] = dateText.split('/');
                const year = parseInt(shortYear) < 50 ? `20${shortYear}` : `19${shortYear}`;
                formattedDate = `${year}-${month.padStart(2, '0')}-${day.padStart(2, '0')}`;
            }
            
            // Calculate approximate duration from distance and category
            // Very rough estimate: D cat ~30km/h, C ~33km/h, B ~36km/h, A ~40km/h
            let estimatedMinutes = 0;
            if (distance > 0) {
                const speeds = { 'A': 40, 'B': 36, 'C': 33, 'D': 30, 'E': 28 };
                const speed = speeds[category] || 30;
                estimatedMinutes = Math.round((distance / speed) * 60);
            }
            
            // Look for route info in race name
            // Common patterns: "Route: Name" or "(Route Name)"
            let routeName = null;
            const routeMatch = raceName.match(/Route:\s*([^,\)]+)|^\s*\(([^\)]+)\)/);
            if (routeMatch) {
                routeName = (routeMatch[1] || routeMatch[2]).trim();
            }
            
            results.push({
                date: formattedDate,
                event_name: raceName,
                event_id: eventId,
                category: category,
                position: position,
                distance_km: distance,
                estimated_minutes: estimatedMinutes,
                // zwift_score excluded - unreliable field
                event_link: raceLink,
                route_name: routeName
            });
            
            if (index < 5) {
                console.log(`Sample: ${formattedDate} - ${raceName} - ${distance}km - Cat ${category}`);
            }
        }
    });
    
    console.log(`Extracted ${results.length} race results`);
    
    if (results.length > 0) {
        // Create downloadable JSON
        const json = JSON.stringify(results, null, 2);
        const blob = new Blob([json], {type: 'application/json'});
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = 'zwiftpower_results.json';
        a.click();
        
        // Log summary
        console.log('First result:', results[0]);
        console.log('Last result:', results[results.length - 1]);
        console.log(`Date range: ${results[results.length - 1].date} to ${results[0].date}`);
    }
    
    return results;
})();