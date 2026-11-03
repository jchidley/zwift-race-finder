(function() {
    console.warn('Standalone metrics export only; database ingestion of this format is unsupported.');
    const results = [];

    const resultsTable = document.querySelector('#profile_results');

    if (!resultsTable) {
        console.error("Could not find #profile_results table");
        return [];
    }

    const rows = resultsTable.querySelectorAll('tbody tr');
    console.log(`Found ${rows.length} race results`);

    rows.forEach((row, index) => {
        const cells = row.querySelectorAll('td');


        if (cells.length >= 16) {
            const dateText = cells[2]?.textContent?.trim() || '';
            const raceCell = cells[3];
            const raceName = raceCell?.textContent?.trim() || '';
            const raceLink = raceCell?.querySelector('a')?.href || '';

            const power20m = cells[4]?.textContent?.trim() || '';
            const power5m = cells[5]?.textContent?.trim() || '';
            const power1m = cells[6]?.textContent?.trim() || '';
            const power15s = cells[7]?.textContent?.trim() || '';
            const power5s = cells[8]?.textContent?.trim() || '';
            const avgPower = cells[9]?.textContent?.trim() || '';
            const normalizedPower = cells[10]?.textContent?.trim() || '';
            const avgWkg = cells[11]?.textContent?.trim() || '';

            const weight = cells[12]?.textContent?.trim() || '';
            const avgHR = cells[13]?.textContent?.trim() || '';
            const maxHR = cells[14]?.textContent?.trim() || '';
            const distanceText = cells[15]?.textContent?.trim() || '';

            let eventId = null;
            const idMatch = raceLink.match(/[?&]id=(\d+)/);
            if (idMatch) eventId = idMatch[1];

            const distance = parseFloat(distanceText.replace('km', '')) || 0;
            const avgPowerNum = parseFloat(avgPower.replace('w', '')) || 0;
            const avgWkgNum = parseFloat(avgWkg.replace('w/kg', '')) || 0;

            let formattedDate = dateText;
            if (dateText.match(/^\d{1,2}\/\d{1,2}\/\d{2}$/)) {
                const [month, day, shortYear] = dateText.split('/');
                const year = parseInt(shortYear) < 50 ? `20${shortYear}` : `19${shortYear}`;
                formattedDate = `${year}-${month.padStart(2, '0')}-${day.padStart(2, '0')}`;
            }


            results.push({
                date: formattedDate,
                event_name: raceName,
                event_id: eventId,
                distance_km: distance,
                power_20m: power20m,
                power_5m: power5m,
                power_1m: power1m,
                power_15s: power15s,
                power_5s: power5s,
                avg_power: avgPower,
                normalized_power: normalizedPower,
                avg_wkg: avgWkg,
                weight: weight,
                avg_hr: avgHR,
                max_hr: maxHR,
                event_link: raceLink
            });

            if (index < 5) {
                console.log(`Sample: ${formattedDate} - ${raceName} - ${distance}km - ${avgPower} (${avgWkg})`);
            }
        }
    });

    console.log(`\nExtracted ${results.length} race results`);

    const dataStr = JSON.stringify(results, null, 2);
    const dataUri = 'data:application/json;charset=utf-8,'+ encodeURIComponent(dataStr);

    const exportFileDefaultName = 'zwiftpower_results.json';

    const linkElement = document.createElement('a');
    linkElement.setAttribute('href', dataUri);
    linkElement.setAttribute('download', exportFileDefaultName);
    linkElement.click();

    return results;
})();