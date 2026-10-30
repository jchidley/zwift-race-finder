# Performance-history model and import — 2026-10-30

## Authorized scope and result

The owner approved separating event eligibility, measured performance and future
prediction, then implementing the history foundation. Historical Racing Score is
now optional. The normal database contains the complete approved export: 194
source-qualified observations, six preserved confirmed scores and 188 unknown
scores. No score was inferred from category, current score or ranking fields.
Personal rows, source IDs, measurements, exports and detailed receipts remain
outside Git. The production duration formula was not changed in this phase.

## Implementation

- `src/history.rs`: strict versioned-envelope parser, source identity, UTC dates,
  fractional finish seconds, supplied event distance/laps/category/gun time,
  export hash and original row JSON. Missing values remain missing.
- `src/database_history.rs`: supported-schema migration preserving IDs, notes,
  timestamps, AUTOINCREMENT sequence and explicit indexes/triggers; unsupported
  schema dependencies fail rather than being discarded. Source-identity
  uniqueness, conservative legacy reconciliation and atomic batch ingestion.
- `src/bin/import_zwiftpower_history.rs`: input-byte SHA-256, in-memory preflight,
  genuinely non-writing preview and explicit offline application.
- Manual recording no longer invents a historical score when omitted.
  `Database::performance_history()` exposes observations without score filtering.
- The reusable browser exporter verifies a complete client-side results table and
  matching profile owner. It exports observations without network/credential
  access or copying unrelated URL parameters.

## Validation and corrections

- Seven synthetic parser tests and six database/CLI integration tests added.
  The latter caught nested initialization/elevation transactions; initialization
  now owns a single transaction boundary. They cover unknown versus zero,
  fractional/u32 data, preserving old rows and schema objects, sequence retention,
  idempotence, conflicts, unsupported schema, forced mid-batch rollback and
  byte-unchanged dry-run. Missing preview targets are not created.
- Full isolated default suite: **195 passed, 0 failed, 7 ignored**. The first run
  exceeded a 60-second command limit near its end; a complete longer run passed.
  No snapshots were regenerated.
- Release builds pass for the main CLI, client importer and history importer.
  All-feature test targets compile. This does not resolve or runtime-validate the
  previously recorded OCR failures. Existing formatting/Clippy debt remains.
- Synthetic JavaScript checks cover a valid download and six rejection cases.
  The packaged browser helper was not rerun in a live browser; the approved
  private source export had already been collected and inspected separately.
- Fresh-context same-family read-only review (`openai/gpt-6-luna`, requested and
  used) found no concrete consequential issue in the migration/import scope.
  It did not run tests or inspect private data. Parent validation is independent.

## Backed-up normal-state application

Before normal writes, a SQLite backup was verified against all table/schema
fingerprints. The full import was exercised on a private copy. Every imported
observation and original source payload matched the export. Original history
fields, including IDs, notes, scores and creation times, remained identical.
Other application tables were unchanged. Repeat import changed nothing.

The normal database was checked again against the preparation fingerprint,
previewed without writes, then migrated/imported. Its observations and schema
match the verified trial. Integrity is OK, there are no foreign-key violations,
and a repeat reports **0 new, 0 enriched, 194 unchanged** with unchanged table and
schema fingerprints. The personal database was tightened to owner-only mode
0600; the new backup and detailed receipt are also owner-only.

Backup: `~/.local/share/zwift-race-finder/backups/races-before-history-model-20261030T124556016354Z.db`.
Private receipt: `~/.local/share/zwift-race-finder/history-imports/migration-20261030T124556016354Z/applied.json`.

## Accuracy boundary

On the same six scored records, before/after **mean absolute percentage error
(MAPE) is 13.3% → 13.3%**. The after run explicitly excludes 188 unknown-score
observations from this old score-dependent check; they remain available to
score-independent analysis. This is not MAE in minutes, a large benchmark, or
validation of a history-based predictor. The existing regression still has
imperfect event-distance/lap interpretation and minute-rounded observations.

Chronological, geometry-aware evaluation and a separately developed history-based
estimator remain future work. No calibration improvement is claimed. See
[the design](../explanation/HISTORY_DATA_MODEL.md) and
[the supported workflow](../howto/ZWIFTPOWER_EXPORT_STEPS.md).

No archive edits, staging, commits or pushes were made.
