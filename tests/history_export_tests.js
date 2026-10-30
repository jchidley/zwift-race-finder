// Synthetic browser surface: no browser, website, profile, or network access.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const vm = require('node:vm');
const source = fs.readFileSync('tools/import/zwiftpower/export_profile_history.js', 'utf8');
function run(options = {}) {
    const rows = options.rows ?? [{ zwid: 42, res_id: 'synthetic', time: [1800.25, 0], skill: 999 }];
    let saved, clicked = false;
    const table = {};
    const jq = () => ({ DataTable: () => ({
        rows: () => ({ data: () => ({ toArray: () => rows }) }),
        page: { info: () => ({ serverSide: options.serverSide ?? false, recordsTotal: options.total ?? rows.length }) },
    }) });
    jq.fn = { dataTable: { isDataTable: () => options.loaded !== false } };
    class TestURL extends URL {}
    TestURL.createObjectURL = blob => { saved = JSON.parse(blob.parts[0]); return 'blob:synthetic'; };
    TestURL.revokeObjectURL = () => {};
    const result = vm.runInNewContext(source, {
        location: { hostname: options.host ?? 'zwiftpower.com', pathname: '/profile.php', href: 'https://zwiftpower.com/profile.php?z=42&unrelated=omit#ignored' },
        window: { jQuery: jq },
        document: { querySelector: () => table, createElement: () => ({ click: () => { clicked = true; } }) },
        URL: TestURL, Blob: class { constructor(parts) { this.parts = parts; } }, Date,
        setTimeout: fn => fn(),
    });
    return { saved, clicked, result };
}
const { saved, clicked, result } = run();
assert(clicked); assert.equal(result.exported_results, 1);
assert.equal(saved.schema_version, 1);
assert.equal(saved.profile_url, 'https://zwiftpower.com/profile.php?z=42');
assert.equal(saved.results[0].time[0], 1800.25);
assert(!('estimated_minutes' in saved.results[0]));
assert(!('zwift_score' in saved.results[0]));
for (const args of [{ host: 'example.com' }, { loaded: false }, { serverSide: true }, { total: 2 }, { rows: [] }, { rows: [{ zwid: 99 }] }]) {
    assert.throws(() => run(args));
}
console.log('History exporter: valid export plus six rejection cases passed.');
