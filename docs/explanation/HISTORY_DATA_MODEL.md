# Performance history independent of Racing Score

**Implementation status (2026-10-30):** the history foundation and full backed-up import are complete. The production predictor is unchanged; the history-based estimator remains a separate future phase. See [implementation and verification](../project-history/HISTORY_MODEL_2026-10-30.md).

## Decision

Separate three concerns:

1. **Eligibility:** current rider score and an event's published subgroup rules.
2. **Observed performance:** what happened, when, on which course and over what distance.
3. **Prediction:** a derived estimate, with supporting observations and uncertainty.

Racing Score is optional historical context, not a prerequisite for retaining a result. Do not substitute today's score, infer one from category, or repurpose another ZwiftPower ranking. Current rider state must not be stamped onto past results.

## Implementation scope

First migrate history storage and import the already approved private export. Keep the route catalogue, provenance, existing stored completion records, and production duration model intact. Existing rows, IDs, notes, timestamps and confirmed scores must survive. No private export, rider identifiers, per-race measurements or score confirmations belong in Git.

`race_results` remains the canonical results table. It gains queryable observation and source fields rather than a competing staging table:

- Nullable historical `zwift_score`; SQL NULL means unknown, distinct from a legitimate zero.
- `actual_seconds` for measured precision; retain `actual_minutes` as a rounded compatibility field. Old minute-only records retain unknown exact seconds rather than fabricated precision.
- UTC event start timestamp, observed event distance in kilometres, numeric laps when supplied, and source result-category label.
- Nullable event type and subgroup ID: do not claim the source supplies these when it does not.
- Source service, rider/result/event IDs, export SHA-256 and source-row JSON for traceability.
- Gun time retained separately from finish duration, not silently substituted.

Identity is `(source, source_rider_id, source_result_id)`, not event name or date. Repeated imports must not duplicate results, erase confirmed scores, or silently change measured facts. Ambiguous legacy matches and conflicting source records fail with an actionable error. The six previously imported rows are reconciled through their structured provenance, not guessed by title.

Route distance and observed event distance are different: multiple laps, lead-ins, partial courses and formats matter. A current route definition does not establish an old event's full geometry. Preserve source observations and missingness rather than deriving historical facts from today's catalogue.

## Source qualification

The inspected ZwiftPower profile DataTable exposes all loaded results across pagination. Its `time[0]` is rendered through `convertSecondsToTime`; `distance` is rendered in kilometres; `event_date` is a Unix timestamp. `rt` values match the client route catalogue for this export. Source category is a result label, not proof of a modern Racing Score band or subgroup eligibility. Missing or malformed measurements must not turn into zero. Lap values and other field coverage must be checked before normalization.

Source rows and SHA-256 receipts stay under the private application data directory. The importer accepts the explicit versioned export envelope, validates the full batch before writing, and never accesses credentials or a network service.

## Migration and validation

Use a transaction and preserve AUTOINCREMENT history, row IDs, timestamps, indexes and triggers where the supported schema permits it. Refuse unrecognized columns or dependent schemas instead of silently discarding data. Back up the real SQLite database and verify the migration/import first on a copy. A preview must not create or migrate the target database.

Required checks: null versus zero score, exact seconds and fractional distance, unsigned route IDs, original dates, known-score preservation, provenance uniqueness, repeat import, conflicting input, unsupported schema, rollback on mid-batch failure, and unchanged non-history data. Synthetic fixtures belong in Git; actual records do not.

Existing score-dependent regression checks must report and exclude records whose score is unknown, rather than dropping them from storage or inventing a score. History queries remain available without score filtering.

## Next phase: history-based duration estimation

Do not couple this migration to a prediction-formula change. Evaluate a small personal estimator separately:

1. Declared fixed event duration wins.
2. Prefer recent comparable results on the same course and equivalent distance/laps/format.
3. Broaden to similar distance and terrain, weighting recency without treating old fitness as current.
4. Fall back to the existing route/category model when evidence is insufficient.
5. Display evidence count, estimation basis and uncertainty rather than false precision.

Compare against the existing model chronologically: each prediction may use only earlier observations. Similar duration alone is insufficient and can create circular matching. Course profile, drafting/solo format, finish/start timing and effort context remain important limitations. Persisting predictions/model versions is unnecessary until that workflow exists.
