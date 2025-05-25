// Extract ALL race results from ZwiftPower - Final version
// Uses table info to know exactly how many results to expect
(async function() {
    console.log("🚴 ZwiftPower Complete Extractor");
    console.log("================================");
    
    // Get total entries from table info
    const tableInfo = document.querySelector('#profile_results_info');
    const infoText = tableInfo?.textContent || '';
    const totalMatch = infoText.match(/of (\d+) entries/);
    const totalEntries = totalMatch ? parseInt(totalMatch[1]) : 0;
    
    console.log(`📊 Total entries to extract: ${totalEntries}`);
    
    const allResults = [];
    let currentPage = 1;
    
    // Function to extract from current page
    function extractFromCurrentPage() {
        const results = [];
        const resultsTable = document.querySelector('#profile_results');
        
        if (!resultsTable) {
            console.error("Could not find #profile_results table");
            return results;
        }
        
        const rows = resultsTable.querySelectorAll('tbody tr');
        
        rows.forEach((row) => {
            const cells = row.querySelectorAll('td');
            
            if (cells.length >= 18) {
                const category = cells[0]?.textContent?.trim() || '';
                const position = cells[1]?.textContent?.trim() || '';
                const dateText = cells[2]?.textContent?.trim() || '';
                const raceCell = cells[3];
                const raceName = raceCell?.textContent?.trim() || '';
                const raceLink = raceCell?.querySelector('a')?.href || '';
                const distanceText = cells[15]?.textContent?.trim() || '';
                const resultScore = cells[17]?.textContent?.trim() || '';
                
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
                let estimatedMinutes = 0;
                if (distance > 0) {
                    const speeds = { 'A': 40, 'B': 36, 'C': 33, 'D': 30, 'E': 28 };
                    const speed = speeds[category] || 30;
                    estimatedMinutes = Math.round((distance / speed) * 60);
                }
                
                // Look for route info in race name
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
                    zwift_score: resultScore,
                    event_link: raceLink,
                    route_name: routeName
                });
            }
        });
        
        return results;
    }
    
    // Function to click on specific page number
    function goToPage(pageNum) {
        // Find the page number link
        const pageLinks = document.querySelectorAll('.paginate_button a, .pagination a');
        for (const link of pageLinks) {
            if (link.textContent.trim() === String(pageNum)) {
                link.click();
                return true;
            }
        }
        return false;
    }
    
    // Extract from all pages
    const totalPages = Math.ceil(totalEntries / 50);
    console.log(`📄 Total pages: ${totalPages}`);
    
    for (let page = 1; page <= totalPages && page <= 10; page++) {
        if (page > 1) {
            console.log(`📄 Going to page ${page}...`);
            if (!goToPage(page)) {
                console.log(`   ❌ Could not navigate to page ${page}`);
                break;
            }
            // Wait for page to load
            await new Promise(resolve => setTimeout(resolve, 1500));
        }
        
        const pageResults = extractFromCurrentPage();
        allResults.push(...pageResults);
        console.log(`   Page ${page}: Found ${pageResults.length} results (Total so far: ${allResults.length})`);
        
        // Check if we've got all expected results
        if (allResults.length >= totalEntries) {
            console.log("✅ Collected all expected results!");
            break;
        }
    }
    
    console.log(`\n📊 Extraction complete!`);
    console.log(`   Expected: ${totalEntries} results`);
    console.log(`   Collected: ${allResults.length} results`);
    
    if (allResults.length > 0) {
        // Remove any duplicates (shouldn't be any, but just in case)
        const uniqueMap = new Map();
        allResults.forEach(r => {
            const key = `${r.date}-${r.event_name}-${r.category}-${r.position}`;
            uniqueMap.set(key, r);
        });
        const uniqueResults = Array.from(uniqueMap.values());
        
        // Sort by date (newest first)
        uniqueResults.sort((a, b) => b.date.localeCompare(a.date));
        
        console.log(`📊 Unique results: ${uniqueResults.length}`);
        
        // Create downloadable JSON
        const json = JSON.stringify(uniqueResults, null, 2);
        const blob = new Blob([json], {type: 'application/json'});
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = 'zwiftpower_results.json';
        a.click();
        
        // Log summary
        console.log('\n📈 Summary:');
        console.log(`Date range: ${uniqueResults[uniqueResults.length - 1].date} to ${uniqueResults[0].date}`);
        
        // Category breakdown
        const categories = {};
        uniqueResults.forEach(r => {
            categories[r.category] = (categories[r.category] || 0) + 1;
        });
        console.log('Races by category:', categories);
        
        // Sample results
        console.log('\nFirst 3 results:');
        uniqueResults.slice(0, 3).forEach(r => {
            console.log(`  ${r.date} - ${r.event_name} (${r.distance_km}km, Cat ${r.category})`);
        });
    } else {
        console.error("❌ No results found!");
    }
    
    return allResults;
})();