# Project Status

`STATUS.md` is the canonical current-work record. Durable facts belong in `docs/reference/`; unresolved work belongs in `TASKS.md`.

## Current focus

The v0.2.0 CLI has been simplified at owner request: removed completion marking/filtering, description analysis (including its standalone binary), and the retired discovery command, along with their dedicated code/tests. A subsequent cleanup also removes completion progress/indicators and manual result recording, including their database APIs and dedicated tests. Existing stored data remains intact; history ingestion uses the source-qualified importer, and shared event-description parsing still supports display/estimation. The leftover route-name guessing module, its test, obsolete discovery comments and stale coverage-helper entries are also removed. Unknown routes are recorded by ID and event name for client-data refresh, without guessed mapping annotations. Current default suite: **186 passed, 7 ignored**; release binaries rebuilt. Earlier verification counts below describe their respective implementation stages.

The approved performance-history foundation is complete: 194 observations imported, six confirmed scores preserved, 188 unknown scores stored as NULL. Exact seconds, event geometry and source provenance are retained independently of eligibility. Production prediction is unchanged. At the history-import stage the default isolated suite passed **195 tests, 7 ignored**; the same six scored records retain **13.3% MAPE before/after**, with 188 unknown-score observations explicitly excluded only from that legacy check. See [history implementation audit](docs/project-history/HISTORY_MODEL_2026-10-30.md) and [design](docs/explanation/HISTORY_DATA_MODEL.md).

The authorized duration and elevation fixes are implemented and the release binaries rebuilt. Unknown elevation is distinct from known zero; lap totals count event lead-in climbing once; proportional distance totals are labelled approximate. Supported route ingestion is now client-only: selected installed-client cache, then extracted XML for missing IDs. Old bundled datasets, automatic seeds, community importers, website discovery, and guessed mapping recipes have been retired. The verified private export contains 386 routes and preserves fractional climbing and source hashes. Isolated live output resolves California Dreamin' Stage 4 at 21.2 km / 106 m / estimated 39 minutes. Client precision gives 249 m for 15 Downtown Dolphin laps, superseding the community-rounded 255 m. The owner approved and the client import was applied to the normal database on 2026-10-30 after a verified SQLite backup. All 386 client records and provenance were verified; all seven non-route tables, including race history, are unchanged. Default Rust testing passes; OCR retains its previously recorded failures. Broader cleanup and credential decisions remain deferred. See [the test audit](docs/project-history/TEST_AUDIT_2026-10-29.md) for executed checks, evidence, removals, and limits.

## Working tree

The checkout remains on `main` at `ad3e87c`, with unpublished documentation rationalisation and authorized route-data cleanup:

- Removed two empty benchmark files and their Cargo declarations, unused Criterion dependency, and unused lockfile packages.
- Removed inactive UOM test scaffolding referring to an undeclared feature and absent implementations; retained the real A/B utility and its unit tests.
- Corrected test documentation to stop claiming benchmark/UOM coverage that did not exist.

The earlier documentation set remains preserved: concise agent instructions, removal of duplicate/pointer-only documents, current-state/task governance, code-truth map, current references and guides, historical 2025 records, returning-rider guide, and Pi audit/evidence. Its complete review is not finished; the repository audit found stale import paths and contradictory reference claims still requiring correction.

Duration presentation now uses the filter's estimate rather than its own formula; the category speed/difficulty formulas are unchanged. The late subgroup fallback consistently selects one subgroup. The code now migrates the route schema with elevation-knowledge flags and uses non-deleting UPSERTs. Credentials and Rust toolchain components were not changed. The normal database received the explicitly approved client route import after backup; non-route data was verified unchanged. The owner-approved native packages and their dependencies were installed in two steps (first: 50 new packages and 9 related upgrades; second: 14 new packages, no upgrades). Tests used temporary application state. No commit or publication was made.

## Verification and blockers

- **Earlier client-only retirement:** 182 Rust tests passed, 0 failed, 7 ignored with fresh isolated state; CLI/client importer release builds and all-feature test compilation pass. A real isolated import has exactly 386 routes/provenance records, integrity OK, and repeat preview zero writes. No normal DB writes or snapshot regeneration. [Retirement audit](docs/project-history/CLIENT_ONLY_ROUTES_2026-10-30.md) records removals, test-count changes, and cache-selection limits.

- Historical pre-retirement default Rust suite after client-primary changes passed with isolated state and one test thread: 196 passed, 0 failed, 7 ignored. New coverage includes unknown/zero migration, repeatability/history preservation, lap/lead-in totals, duration gradient inputs, route-ID bounds, and importer repair/preservation/idempotence. Snapshots were not regenerated. This does not establish concurrent-test safety.
- Historical pre-retirement checks: CLI, client importer, and community importer release builds passed; all-feature test compilation passes. PowerShell collector: 90 assertions passed; Python legacy-import protection: two tests passed. A fresh isolated client import writes 386 records, reconciling all ten seeds; a repeat changes none. Forced transaction failure preserves earlier rows/history. A same-family fresh-context read-only Rust review found no concrete regressions; it did not run tests or inspect the collector. Parent independently ran collector tests and verified the real export/import.
- Private export: `~/.local/share/zwift-race-finder/client-routes-0862ca71d9b4.json`. Cross-OS SHA-256 matches; plain console text transfer was rejected due to IBM850 recoding and replaced with byte-preserving transfer. Normal route DB was subsequently updated with approval: 63 → 387 routes, including 386 verified client records and one preserved unmatched route. Integrity and foreign-key checks pass; repeat preview reports zero changes.
- The explicitly enabled live-API test still has a stale `/api/public/events` URL (404). The production release CLI now passes live consistency checks: 28 rows within 15–45 minutes and 2 rows within 45–75 minutes, using imported route data in isolated state. KRG's declared fixed 45 minutes appears consistently in compact/verbose output.
- The accuracy regression ran before and after against isolated data and found no mapped race results; before/after MAE are unavailable. The user's normal database was not used.
- All-feature/OCR test build passes after installing libclang and English Tesseract data. Earlier all-feature execution, before the core fixes, reported 232 passed, 2 failed, 16 ignored (includes core tests); this stage rebuilt but did not rerun those known-failing OCR tests.
- OCR failures: leaderboard distance parsing absorbs a preceding time-delta digit; the name-detection snapshot contradicts current output and existing unit-test expectations.
- Explicit ignored OCR checks report 6 missing-model failures and 2 misleading passes that swallow model-load errors. The code-provided detection-model download URL returns HTTP 403; image baselines are absent too. No expected outputs were regenerated.
- Clippy now runs and fails on existing lint errors (243 library / 393 library-test reports, overlapping).
- `rustfmt` is installed. Formatting check runs but fails on existing source/test formatting; the previous missing-rustfmt diagnosis is superseded.
- Syntax checks passed for 25 Python, 26 Bash, and 5 JavaScript files. ShellCheck reported 17 findings across 5 scripts.
- The public API responded with 200 events for `limit=200`; `limit=250` returned HTTP 400. The diagnostic script misleadingly reports the error object as one event and exits zero.
- Python OCR dependencies are missing from the probed uv-managed interpreter. Mutation/coverage tools remain unavailable; Tesseract/Leptonica development libraries are installed.
- Earlier documentation link/audit checks are historical evidence, not proof that every current command or claim is valid.

## Approved history migration and full import — 2026-10-30

The initial six-result import is superseded by the supported nullable-score model and full private export import. All original row IDs, notes, creation timestamps and confirmed scores were preserved; precise observations and source identity were added. The other 188 observations were inserted without fabricated scores. All 194 match the source export, non-history tables are unchanged, integrity/foreign-key checks pass, and repeat import makes no changes. A private SQLite backup and detailed receipt are recorded in the [implementation audit](docs/project-history/HISTORY_MODEL_2026-10-30.md).

The supported workflow is [measured ZwiftPower history](docs/howto/ZWIFTPOWER_EXPORT_STEPS.md). The production estimator is still the existing route/category formula; history ingestion is not prediction calibration. Future work must use equivalent event geometry and chronological holdout evaluation without future leakage.

## Immediate next action

Evaluate a separate history-based estimator using the now-complete observation store; do not change production until chronological, geometry-aware comparisons show a reliable improvement. Current Racing Score remains an eligibility input, not a speed measurement.

The normal-state client import is complete. Backup: `~/.local/share/zwift-race-finder/backups/races-before-client-import-20261030T075234091278Z.db`. The rebuilt CLI now uses the imported client route metadata. The selected archive hash is unchanged and installed game version is 1.124.0 (167761), but the installed manifest did not establish an active-cache link. See `docs/howto/CLIENT_ROUTE_DATA.md` for future refreshes. Client imports replace conflicting legacy records without deleting history. Fresh databases no longer seed routes, and unsupported import/scrape paths are retired. Remaining core issues include active-cache/version selection, multi-stage description parsing for unresolved IDs, subgroup range selection, incomplete route coverage, the stale live-API test, empty-data accuracy passes, and lint failures. OCR needs separate parsing/snapshot fixes and a trustworthy model source/baselines. Do not regenerate expected outputs merely to make tests pass. Preserve credentials and personal data; their disposition remains deferred.

## Coaching project handoff

The owner requested a separate `~/git/zwift-race-coach` project. Local instructions, scope, task queue, and documentation are prepared there. Ecosystem/multi-instance references and the historical simulation catalogue were moved there with navigation repaired; route collection/import, source provenance, and production race selection remain here. The owner approved the coaching project's GitHub skill preview: private `jchidley/zwift-race-coach` now exists with Actions disabled and matching README/MIT/Apache template files locally and remotely. Its remaining local docs have not been committed or pushed. Continue coaching development from a new Pi session at that canonical root; do not stage or publish this dirty checkout as part of its setup.

## Retained uncertainty and boundaries

- Earlier empty-data regressions remain historical. The six-scored-record baseline now gives unchanged 13.3% MAPE, not MAE; it is small and still has event-interpretation limits. It does not validate a history-based model. Category speeds/difficulty breakpoints are unchanged.
- Returning-rider eligibility/seeding claims and optional practical advice still need editorial/source review before publication.
- Legacy requirements/security/UOM recommendations remain historical and untriaged; they are not current acceptance evidence.
- OCR, Garmin, simulation, import workflows, and reference-repository support boundaries still need decisions and documentation.
- Historical `archive/` files remain read-only. Existing unpublished work is preserved. Privileged installation, credential changes, history rewriting, commit, and publication require separate approval.
