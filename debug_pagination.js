// Debug pagination on ZwiftPower
(function() {
    console.log("=== Pagination Debug ===");
    
    // Check current page info
    const pageInfo = document.querySelector('.dataTables_info');
    console.log("Page info text:", pageInfo?.textContent);
    
    // Look for all pagination elements
    const paginationContainers = document.querySelectorAll('.dataTables_paginate, .pagination');
    console.log(`Found ${paginationContainers.length} pagination containers`);
    
    paginationContainers.forEach((container, i) => {
        console.log(`\nContainer ${i}:`, container.className);
        const buttons = container.querySelectorAll('a, button');
        buttons.forEach(btn => {
            console.log(`  Button: "${btn.textContent.trim()}" - classes: ${btn.className} - disabled: ${btn.classList.contains('disabled')}`);
        });
    });
    
    // Check if we're on the last page
    const nextButton = document.querySelector('#profile_results_next');
    console.log("\nNext button:", {
        exists: !!nextButton,
        classes: nextButton?.className,
        disabled: nextButton?.classList.contains('disabled')
    });
    
    // Get current results count
    const resultsTable = document.querySelector('#profile_results');
    const currentRows = resultsTable?.querySelectorAll('tbody tr').length;
    console.log(`\nCurrent visible rows: ${currentRows}`);
    
    // Check for total results info
    const totalInfo = document.querySelector('#profile_results_info');
    console.log("Results info:", totalInfo?.textContent);
})();