// Safe extraction that stops at actual last page
(async function() {
    console.log("🚴 ZwiftPower Safe Extractor");
    console.log("============================");
    
    const allResults = [];
    let currentPage = 1;
    let lastPageFirstResult = null;
    
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
    
    // Check if next button is actually enabled
    function hasNextPage() {
        const nextButton = document.querySelector('#profile_results_next');
        return nextButton && !nextButton.classList.contains('disabled');
    }
    
    // Click next page
    function clickNext() {
        const nextButton = document.querySelector('#profile_results_next a');
        if (nextButton) {
            nextButton.click();
            return true;
        }
        return false;
    }
    
    // Main extraction loop
    console.log(`📄 Page ${currentPage}...`);
    let pageResults = extractFromCurrentPage();
    allResults.push(...pageResults);
    console.log(`   Found ${pageResults.length} results`);
    
    // Check if there's a first result to track
    if (pageResults.length > 0) {
        lastPageFirstResult = `${pageResults[0].date}-${pageResults[0].event_name}`;
    }
    
    // Continue while there's a next page
    while (hasNextPage() && currentPage < 10) {
        if (!clickNext()) {
            console.log("❌ Could not click next button");
            break;
        }
        
        // Wait for page to load
        await new Promise(resolve => setTimeout(resolve, 1500));
        
        currentPage++;
        console.log(`📄 Page ${currentPage}...`);
        
        pageResults = extractFromCurrentPage();
        
        // Check if we're seeing the same data (pagination stuck)
        if (pageResults.length > 0) {
            const thisPageFirstResult = `${pageResults[0].date}-${pageResults[0].event_name}`;
            if (thisPageFirstResult === lastPageFirstResult) {
                console.log("   ⚠️  Same data as previous page, stopping");
                break;
            }
            lastPageFirstResult = thisPageFirstResult;
        }
        
        if (pageResults.length === 0) {
            console.log("   No results found, stopping");
            break;
        }
        
        allResults.push(...pageResults);
        console.log(`   Found ${pageResults.length} results`);
    }
    
    // Check why we stopped
    if (!hasNextPage()) {
        console.log("✅ Reached last page");
    }
    
    console.log(`\n📊 Total results collected: ${allResults.length}`);
    
    if (allResults.length > 0) {
        // Remove duplicates
        const uniqueMap = new Map();
        allResults.forEach(r => {
            const key = `${r.date}-${r.event_name}-${r.category}`;
            uniqueMap.set(key, r);
        });
        const uniqueResults = Array.from(uniqueMap.values());
        
        console.log(`📊 Unique results: ${uniqueResults.length}`);
        
        // Sort by date (newest first)
        uniqueResults.sort((a, b) => b.date.localeCompare(a.date));
        
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
        console.log('Date range:', uniqueResults[uniqueResults.length - 1].date, 'to', uniqueResults[0].date);
        
        // Category breakdown
        const categories = {};
        uniqueResults.forEach(r => {
            categories[r.category] = (categories[r.category] || 0) + 1;
        });
        console.log('Races by category:', categories);
        
        // Show page info
        const pageInfo = document.querySelector('#profile_results_info');
        if (pageInfo) {
            console.log('\nTable info:', pageInfo.textContent);
        }
    }
    
    return uniqueResults;
})();