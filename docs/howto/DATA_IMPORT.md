# How to Import Race and Route Data

## Overview

Zwift Race Finder imports verified client route metadata and measured performance history. History is stored for evaluation; importing it does not automatically personalize production predictions.

For route metadata, use [the client-only import workflow](CLIENT_ROUTE_DATA.md): explicitly selected installed-client cache, then extracted client XML for missing IDs. Exports remain private outside Git; no route dataset is bundled. Race-history imports below are separate.

## ZwiftPower history

Use [the measured-history export/import workflow](ZWIFTPOWER_EXPORT_STEPS.md).
The supported binary is `import_zwiftpower_history`, which previews on an
in-memory copy and imports measured results with optional historical scores.
The legacy `import_zwiftpower.sh`, `import_zwiftpower_dev.sh`, and
`zwiftpower_profile_extractor.js` have been removed: they used estimated durations,
wrote a different table, or deleted existing results. Do not substitute estimated
minutes for measured finish seconds.

`tools/import/zwiftpower/zwiftpower_event_extractor.js` is retained only as a
standalone export of displayed power, heart-rate, and distance metrics. It has no
finish-time field, does not map routes or write the database, and its output is
**not** accepted by `import_zwiftpower_history`. Its positional table parsing has
not been validated against the current site; use `export_profile_history.js` for
the supported history export.

## Strava history

Strava **database ingestion is unsupported**. The old `strava_import_to_db.sh`
and its generated root `match_activities.sql` have been removed. Moving time is
not automatically race finish time, and title/date matching is not route/event
identity.

Fetch, authentication, and standalone analysis tools remain under
`tools/import/strava/`; they are not a supported history-update path. Their
credential handling and live API behavior have not been revalidated or reconciled
with the approved secret-storage workflow. The analyzer's legacy database-open
helper only prints a speed summary; it does not ingest results. A future adapter
needs explicit source IDs, timing semantics, duplicate handling and approved
credential storage.

## Route Data Import

Use `import_client_routes` as described in [Client-only route data](CLIENT_ROUTE_DATA.md).
Community/zwift-offline imports, manual third-party metadata insertion, and website
route scraping are no longer supported inputs. Fresh databases have no seeded
routes; existing route rows and history remain until explicitly updated.
The discovery and description-analysis CLI options are removed. Internal
event-description parsing remains supported.

## Route identity and race-history mapping

Refresh missing route metadata through [the client workflow](CLIENT_ROUTE_DATA.md).
Use `--show-unknown-routes` to list unresolved IDs. Legacy website-derived SQL,
placeholder IDs, and event-name-based mapping recipes are retired; do not guess a
race's route from its series name. Existing race results and aliases are preserved,
not automatically corrected by this cleanup. Any historical-result remapping needs
separate event-specific evidence, a preview, and approval before writes.

## Screenshot telemetry (separate from history ingestion)

Use the retained Rust/Python compact screenshot extractors described in
[the OCR README](../../tools/ocr/README.md) and
[setup guide](../../tools/ocr/SETUP_GUIDE.md). The missing-module video and v3
visualizer wrappers and prototype console entrypoint are retired. OCR/calibration
accuracy on current images is unvalidated; telemetry output is not automatically
a measured finish result and has no supported race-history database importer.

## Event Description Fetching

`fetch_event_descriptions.sh` is a separate inspection utility, not a history or
route-metadata importer. Its live behavior has not been validated by this cleanup.
For Racing Score events that need description inspection:

```bash
# From the repository root; inspect the script before any live use
bash tools/import/routes/fetch_event_descriptions.sh

# This helps identify:
# - Distance information in descriptions
# - Multi-stage event details
# - Special event formats
```

## Data Validation

### Verify Import Success

```bash
sqlite3 ~/.local/share/zwift-race-finder/races.db <<'SQL'
SELECT COUNT(*) AS total_races FROM race_results;
SELECT COUNT(*) AS total_routes FROM routes;
SELECT route_id, event_name, times_seen
FROM unknown_routes
ORDER BY times_seen DESC
LIMIT 10;
SQL
```

### Test Predictions

```bash
cargo test --lib test_race_predictions_accuracy -- --nocapture
```

The regression reads the normal application database. Compare results only when the database contents are known and equivalent.

## Troubleshooting

### Common Issues

1. **Duplicate Imports**:
   Preview possible duplicates without changing data:
   ```sql
   SELECT event_name, race_date, route_id, COUNT(*) AS copies
   FROM race_results
   GROUP BY event_name, race_date, route_id
   HAVING COUNT(*) > 1;
   ```
   Confirm which rows are truly duplicates and back up the database before deleting anything.

2. **Missing Route Mappings**:
   - Refresh the selected client export and match numeric route IDs.
   - Leave unresolved historical results unmapped until event-specific evidence exists.

3. **Import Script Fails**:
   - Check file permissions
   - Verify JSON format
   - Check database path

### Data Cleanup

Bulk deletion is intentionally not prescribed here: it irreversibly changes the race history used for calibration. Back up the database, preview rows with `SELECT`, and confirm the exact retention rule before issuing a `DELETE`. The `race_results` timestamp column is `race_date`; see the [Database Reference](../reference/DATABASE.md).

## Best Practices

1. **Regular Imports**: Weekly for active racers
2. **Backup First**: Always backup before bulk imports
3. **Verify Data**: Check a few records manually
4. **Map Routes**: Investigate unknown routes promptly
5. **Keep Exports Private**: Do not commit rider history, credentials, or client exports
