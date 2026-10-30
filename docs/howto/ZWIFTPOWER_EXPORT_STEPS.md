# Export and import your ZwiftPower history

## Export measured observations

Open **your own logged-in profile/results page** on ZwiftPower. Use
`tools/import/zwiftpower/export_profile_history.js` in the browser console, or
have an explicitly authorized browser-automation session evaluate it there.
Only you handle login/verification. No cookie extraction, request-header capture,
or HAR export is needed.

The script verifies the selected profile ID, complete client-side DataTable and
owner of every result, then downloads `zwiftpower-history-<timestamp>.json`.
It includes all loaded rows across pagination, not just the visible page.
It preserves source fields rather than guessing finish times or Racing Scores.
If the site's schema/pagination changes, it fails rather than exporting a partial
history. Keep the downloaded file private and **outside the repository**.

The older profile extractors/import shell scripts are not this workflow. In
particular, do not use estimated minutes as actual race results or run a script
that clears existing history. The legacy scripts remain unsupported.

## Build

From the project root:

```bash
cargo build --release --bin zwift-race-finder --bin import_client_routes --bin import_zwiftpower_history
```

## Back up, preview, apply

Transfer the export as bytes if moving between Windows and WSL; verify SHA-256
at each end. Do not transfer JSON through a console that may recode Unicode.

Back up the existing database with SQLite's backup mechanism before applying a
schema migration or import. Example for the normal WSL database:

```bash
umask 077
state="${XDG_DATA_HOME:-$HOME/.local/share}/zwift-race-finder"
mkdir -p "$state/backups"
chmod 600 "$state/races.db"
backup=$(mktemp "$state/backups/races-before-history-XXXXXXXX.db")
sqlite3 -readonly "$state/races.db" ".backup '$backup'"
sqlite3 -readonly "$backup" 'PRAGMA integrity_check;'
```

Then preview and apply your chosen private input:

```bash
./target/release/import_zwiftpower_history --input /absolute/path/to/zwiftpower-history.json --dry-run
./target/release/import_zwiftpower_history --input /absolute/path/to/zwiftpower-history.json
```

Use `--database /absolute/path/to/races.db` to explicitly select a test copy or
another target. The default is the application's platform-local database.
Unlike the older client-route import preview, this history preview clones the
target into memory: **it does not create, migrate or write the target**.
The apply command also preflights the complete batch in memory first.

## What is preserved

- Original event dates/UTC timestamps and measured finish seconds.
- Event distance and supplied laps, independently of base route distance.
- Source rider/result/event IDs, export hash and original source-row JSON.
- Existing row IDs, notes, creation timestamps and confirmed historical scores.
- Existing route metadata, completion state and other application tables.

Unknown historical scores become SQL NULL, not a default or category-derived
score. Exact seconds remain queryable; `actual_minutes` is retained as a rounded
compatibility field. Gun time is stored separately and never substituted for
finish duration. Event type/subgroup ID remain unknown unless independently
established; source category is only a result label.

Source identity prevents duplicate results. An identical repeat is a no-op;
missing scores do not erase confirmed scores. Conflicting observations or
ambiguous legacy matches stop the import, rather than overwrite history.
Known structured provenance from earlier imports allows safe enrichment of those
rows. Manual records without source identity may require explicit reconciliation.

Routes must already exist under their numeric IDs; refresh verified client route
metadata first if any are missing. Do not create guessed mappings. If a legacy
database contains custom columns, dependent tables/views, or implicit constraints,
the migration refuses to discard them and requests explicit review.

## Storage is not a new prediction model

Importing history does not automatically change the production duration formula.
The current score-based regression explicitly reports/excludes unknown-score rows;
score-independent history queries still retain them all. See
[the history data model](../explanation/HISTORY_DATA_MODEL.md) for the separate
history-based prediction phase.

## Validation

```bash
cargo test --test history_import_tests
cargo test --lib history::
node tests/history_export_tests.js
```

Use isolated HOME/XDG state for the broader suite. Personal exports are never
committed as fixtures; all checked-in history tests use synthetic data.
