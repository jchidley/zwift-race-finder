// Extract ALL race results from ZwiftPower profile page (handles pagination)
// This version collects results from all pages
(async function() {
    console.log("🚴 ZwiftPower Multi-Page Extractor");
    console.log("==================================");
    
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
    
    // Function to find and click next page
    function findNextButton() {
        // Look for pagination controls
        // Common patterns: "Next", ">", "»", page numbers
        const paginationSelectors = [
            '#profile_results_next',  // DataTables default
            '.paginate_button.next:not(.disabled)',
            'a[aria-label="Next"]',
            'button:contains("Next")',
            'a:contains("Next")',
            '.pagination a:contains("»")',
            '.dataTables_paginate .next'
        ];
        
        for (const selector of paginationSelectors) {
            try {
                const button = document.querySelector(selector);
                if (button && !button.classList.contains('disabled')) {
                    return button;
                }
            } catch (e) {
                // Try jQuery selectors if available
                if (typeof $ !== 'undefined') {
                    const $button = $(selector).filter(':visible:not(.disabled)');
                    if ($button.length) {
                        return $button[0];
                    }
                }
            }
        }
        
        // Try looking for page numbers
        const pageLinks = document.querySelectorAll('.paginate_button:not(.current)');
        for (const link of pageLinks) {
            const pageNum = parseInt(link.textContent);
            if (pageNum === currentPage + 1) {
                return link;
            }
        }
        
        return null;
    }
    
    // Main extraction loop
    console.log(`📄 Extracting page ${currentPage}...`);
    let pageResults = extractFromCurrentPage();
    allResults.push(...pageResults);
    console.log(`   Found ${pageResults.length} results on page ${currentPage}`);
    
    // Check for more pages
    let nextButton = findNextButton();
    
    while (nextButton && currentPage < 10) { // Safety limit of 10 pages
        console.log(`📄 Moving to page ${currentPage + 1}...`);
        
        // Click next page
        nextButton.click();
        
        // Wait for page to load
        await new Promise(resolve => setTimeout(resolve, 1500));
        
        currentPage++;
        pageResults = extractFromCurrentPage();
        
        if (pageResults.length === 0) {
            console.log("   No results found, stopping");
            break;
        }
        
        allResults.push(...pageResults);
        console.log(`   Found ${pageResults.length} results on page ${currentPage}`);
        
        nextButton = findNextButton();
    }
    
    console.log(`\n✅ Extraction complete!`);
    console.log(`📊 Total results: ${allResults.length}`);
    
    if (allResults.length > 0) {
        // Remove duplicates (in case of any issues)
        const uniqueResults = Array.from(new Map(
            allResults.map(r => [`${r.date}-${r.event_name}`, r])
        ).values());
        
        console.log(`📊 Unique results: ${uniqueResults.length}`);
        
        // Create downloadable JSON
        const json = JSON.stringify(uniqueResults, null, 2);
        const blob = new Blob([json], {type: 'application/json'});
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = 'zwiftpower_results_all.json';
        a.click();
        
        // Log summary
        console.log('\n📈 Summary:');
        console.log('First result:', uniqueResults[0]);
        console.log('Last result:', uniqueResults[uniqueResults.length - 1]);
        console.log(`Date range: ${uniqueResults[uniqueResults.length - 1].date} to ${uniqueResults[0].date}`);
        
        // Category breakdown
        const categories = {};
        uniqueResults.forEach(r => {
            categories[r.category] = (categories[r.category] || 0) + 1;
        });
        console.log('\nRaces by category:', categories);
    } else {
        console.error("❌ No results found!");
        console.log("Make sure you're on the profile page with the results table visible");
    }
    
    return allResults;
})();