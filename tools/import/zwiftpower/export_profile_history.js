// Run only on your own logged-in ZwiftPower profile. No network/credential access.
// Exports source observations, never estimated times or inferred Racing Scores.
(() => {
    if (location.hostname !== 'zwiftpower.com' || location.pathname !== '/profile.php') {
        throw new Error('Open your own ZwiftPower profile before exporting.');
    }
    const profileId = new URL(location.href).searchParams.get('z');
    if (!profileId || !/^\d+$/.test(profileId)) {
        throw new Error('Expected an explicit numeric profile ID in the profile URL.');
    }
    const jq = window.jQuery;
    const table = document.querySelector('#profile_results');
    if (!table || !jq?.fn?.dataTable?.isDataTable(table)) {
        throw new Error('Wait for the profile results table to load.');
    }
    const dt = jq(table).DataTable();
    const rows = dt.rows().data().toArray();
    const page = dt.page.info();
    if (page.serverSide || !rows.length || rows.length !== page.recordsTotal) {
        throw new Error('Complete client-side results are required; refusing a partial export.');
    }
    if (!rows.every(row => String(row.zwid) === profileId)) {
        throw new Error('Result owner does not match the selected profile.');
    }
    const exportedAt = new Date().toISOString();
    const payload = {
        schema_version: 1,
        source: 'ZwiftPower profile results DataTable',
        exported_at: exportedAt,
        // Deliberately omit all other URL parameters/fragments.
        profile_url: `https://zwiftpower.com/profile.php?z=${profileId}`,
        row_count: rows.length,
        results: rows,
    };
    const url = URL.createObjectURL(new Blob([JSON.stringify(payload, null, 2)], { type: 'application/json' }));
    const link = document.createElement('a');
    link.href = url;
    link.download = `zwiftpower-history-${exportedAt.replace(/[:.]/g, '-')}.json`;
    link.click();
    setTimeout(() => URL.revokeObjectURL(url), 1000);
    return { exported_results: rows.length, filename: link.download };
})();
