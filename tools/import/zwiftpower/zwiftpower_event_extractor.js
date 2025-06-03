// Extract race results from ZwiftPower profile page - Version 3
// Captures meaningful metrics (power, w/kg, HR) while excluding unreliable fields
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
        // 3: Race name
        // 4: 20m power (e.g., "199w")
        // 5: 5m power
        // 6: 1m power
        // 7: 15s power
        // 8: 5s power
        // 9: Avg power
        // 10: NP (Normalized Power)
        // 11: Avg W/kg (e.g., "2.7w/kg")
        // 12: Weight (86.0kg)
        // 13: Avg HR (e.g., "155bpm")
        // 14: Max HR
        // 15: Distance (56km)
        // 16: Empty
        // 17: Result/Score - SKIP THIS
        // 18-19: Gain and other - SKIP THESE
        
        if (cells.length >= 16) {
            // Skip category (cells[0]) - not meaningful for Zwift Racing Score system
            // Skip position (cells[1]) - depends on field size, not useful
            const dateText = cells[2]?.textContent?.trim() || '';
            const raceCell = cells[3];
            const raceName = raceCell?.textContent?.trim() || '';
            const raceLink = raceCell?.querySelector('a')?.href || '';
            
            // Power metrics
            const power20m = cells[4]?.textContent?.trim() || '';
            const power5m = cells[5]?.textContent?.trim() || '';
            const power1m = cells[6]?.textContent?.trim() || '';
            const power15s = cells[7]?.textContent?.trim() || '';
            const power5s = cells[8]?.textContent?.trim() || '';
            const avgPower = cells[9]?.textContent?.trim() || '';
            const normalizedPower = cells[10]?.textContent?.trim() || '';
            const avgWkg = cells[11]?.textContent?.trim() || '';
            
            // Physical metrics
            const weight = cells[12]?.textContent?.trim() || '';
            const avgHR = cells[13]?.textContent?.trim() || '';
            const maxHR = cells[14]?.textContent?.trim() || '';
            const distanceText = cells[15]?.textContent?.trim() || '';
            
            // Extract event ID from link
            let eventId = null;
            const idMatch = raceLink.match(/[?&]id=(\d+)/);
            if (idMatch) eventId = idMatch[1];
            
            // Extract numeric values
            const distance = parseFloat(distanceText.replace('km', '')) || 0;
            const avgPowerNum = parseFloat(avgPower.replace('w', '')) || 0;
            const avgWkgNum = parseFloat(avgWkg.replace('w/kg', '')) || 0;
            
            // Parse date - convert from M/D/YY to YYYY-MM-DD
            let formattedDate = dateText;
            if (dateText.match(/^\d{1,2}\/\d{1,2}\/\d{2}$/)) {
                const [month, day, shortYear] = dateText.split('/');
                const year = parseInt(shortYear) < 50 ? `20${shortYear}` : `19${shortYear}`;
                formattedDate = `${year}-${month.padStart(2, '0')}-${day.padStart(2, '0')}`;
            }
            
            // We can't accurately calculate race duration without knowing the actual time
            // The actual_minutes field should come from race timing data, not estimates
            
            results.push({
                date: formattedDate,
                event_name: raceName,
                event_id: eventId,
                distance_km: distance,
                // Power metrics
                power_20m: power20m,
                power_5m: power5m,
                power_1m: power1m,
                power_15s: power15s,
                power_5s: power5s,
                avg_power: avgPower,
                normalized_power: normalizedPower,
                avg_wkg: avgWkg,
                // Physical metrics
                weight: weight,
                avg_hr: avgHR,
                max_hr: maxHR,
                // Metadata
                event_link: raceLink
            });
            
            if (index < 5) {
                console.log(`Sample: ${formattedDate} - ${raceName} - ${distance}km - ${avgPower} (${avgWkg})`);
            }
        }
    });
    
    console.log(`\nExtracted ${results.length} race results`);
    
    // Create download
    const dataStr = JSON.stringify(results, null, 2);
    const dataUri = 'data:application/json;charset=utf-8,'+ encodeURIComponent(dataStr);
    
    const exportFileDefaultName = 'zwiftpower_results.json';
    
    const linkElement = document.createElement('a');
    linkElement.setAttribute('href', dataUri);
    linkElement.setAttribute('download', exportFileDefaultName);
    linkElement.click();
    
    return results;
})();