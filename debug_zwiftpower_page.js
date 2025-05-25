// Debug script to explore ZwiftPower page structure
// Run this in the browser console on your ZwiftPower profile page

(function() {
    console.log("=== ZwiftPower Page Structure Debug ===");
    
    // 1. Check page URL and title
    console.log("Page URL:", window.location.href);
    console.log("Page Title:", document.title);
    
    // 2. Find all tables on the page
    const tables = document.querySelectorAll('table');
    console.log(`\nFound ${tables.length} tables on the page:`);
    
    tables.forEach((table, i) => {
        const headers = Array.from(table.querySelectorAll('th')).map(th => th.textContent.trim());
        const firstRow = table.querySelector('tbody tr');
        const firstRowCells = firstRow ? Array.from(firstRow.querySelectorAll('td')).map(td => td.textContent.trim().substring(0, 20)) : [];
        
        console.log(`\nTable ${i}:`, {
            id: table.id || 'no-id',
            classes: table.className || 'no-classes',
            parent_id: table.parentElement?.id || 'no-parent-id',
            headers: headers.length ? headers : 'no-headers',
            rowCount: table.querySelectorAll('tbody tr').length,
            firstRowPreview: firstRowCells.length ? firstRowCells : 'no-data'
        });
    });
    
    // 3. Look for common div containers
    console.log("\n=== Looking for result containers ===");
    const possibleContainers = [
        '#profile_results',
        '#results',
        '#activities',
        '#races',
        '.profile-results',
        '.results-table',
        '.race-history',
        '[data-results]',
        '[data-races]'
    ];
    
    possibleContainers.forEach(selector => {
        const element = document.querySelector(selector);
        if (element) {
            console.log(`Found ${selector}:`, {
                tagName: element.tagName,
                classes: element.className,
                childrenCount: element.children.length,
                hasTable: !!element.querySelector('table')
            });
        }
    });
    
    // 4. Check for tabs or navigation
    console.log("\n=== Looking for tabs/navigation ===");
    const tabs = document.querySelectorAll('[role="tab"], .tab, .nav-link, a[data-toggle="tab"]');
    if (tabs.length) {
        console.log(`Found ${tabs.length} tab-like elements:`);
        tabs.forEach(tab => {
            console.log(`- ${tab.textContent.trim()} (${tab.tagName}, href: ${tab.href || 'none'})`);
        });
    }
    
    // 5. Check for AJAX data attributes
    console.log("\n=== Checking for dynamic content indicators ===");
    const ajaxElements = document.querySelectorAll('[data-url], [data-ajax], [data-load]');
    if (ajaxElements.length) {
        console.log(`Found ${ajaxElements.length} elements with data attributes that might load content`);
        ajaxElements.forEach(el => {
            console.log(`- ${el.tagName}.${el.className}`, el.dataset);
        });
    }
    
    // 6. Look for iframes
    const iframes = document.querySelectorAll('iframe');
    if (iframes.length) {
        console.log(`\nFound ${iframes.length} iframes - content might be inside`);
    }
    
    // 7. Sample text search for "race" or "result"
    const allText = document.body.innerText.toLowerCase();
    const raceCount = (allText.match(/race/g) || []).length;
    const resultCount = (allText.match(/result/g) || []).length;
    console.log(`\nPage contains the word "race" ${raceCount} times and "result" ${resultCount} times`);
    
    // 8. Check for pagination
    const pagination = document.querySelectorAll('.pagination, [class*="page"], a[href*="page="]');
    if (pagination.length) {
        console.log(`\nFound pagination elements - results might be paginated`);
    }
    
    console.log("\n=== Debug complete ===");
    console.log("If you see your race results on the page but they're not in any tables above,");
    console.log("they might be loaded dynamically. Try:");
    console.log("1. Check Network tab for XHR/Fetch requests");
    console.log("2. Wait a few seconds and run this script again");
    console.log("3. Click on different tabs/sections and run again");
})();