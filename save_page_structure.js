// Save ZwiftPower page structure for analysis
// Run this in the browser console on your profile page
(function() {
    // Get the page HTML
    const html = document.documentElement.outerHTML;
    
    // Also get table structure info
    const tables = document.querySelectorAll('table');
    const tableInfo = [];
    
    tables.forEach((table, index) => {
        const headers = Array.from(table.querySelectorAll('th')).map(th => th.textContent.trim());
        const firstRow = table.querySelector('tbody tr');
        const firstRowData = firstRow ? Array.from(firstRow.querySelectorAll('td')).map(td => td.textContent.trim().substring(0, 50)) : [];
        
        tableInfo.push({
            index: index,
            className: table.className,
            id: table.id,
            headers: headers,
            firstRowSample: firstRowData,
            rowCount: table.querySelectorAll('tbody tr').length
        });
    });
    
    // Create a summary object
    const pageSummary = {
        url: window.location.href,
        title: document.title,
        tableCount: tables.length,
        tables: tableInfo,
        // Save a clean version of the HTML (first 50KB to avoid huge files)
        htmlSnippet: html.substring(0, 50000)
    };
    
    // Download as JSON
    const json = JSON.stringify(pageSummary, null, 2);
    const blob = new Blob([json], {type: 'application/json'});
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'zwiftpower_page_structure.json';
    a.click();
    
    // Log table info to console
    console.log('Page Analysis:');
    console.log('Tables found:', tables.length);
    tableInfo.forEach(table => {
        console.log(`\nTable ${table.index}:`);
        console.log('  Class:', table.className);
        console.log('  Headers:', table.headers);
        console.log('  First row:', table.firstRowSample);
        console.log('  Total rows:', table.rowCount);
    });
    
    return pageSummary;
})();