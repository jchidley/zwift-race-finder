<!-- code-truth: checkpoint 3e13a78 plus unpublished working-tree observations; see README.md -->
# Implementation Patterns

The selected examples below are adopted guidance for their named patterns. The
remaining entries describe observed implementation, not universal Rust prescriptions
or automatic approval of every existing API. This page includes unpublished
working-tree observations; see the [snapshot boundary](README.md).

## Selected first-party examples

For substantive changes, select the applicable example and inspect its matching
tests. Keep the pattern narrow; when it does not fit, explain the mismatch and
propose the smallest alternative rather than forcing a shared abstraction.

| Pattern to follow | Implementation and matching tests | What to retain / what not to assume |
|---|---|---|
| Test the actual HTTP adapter | [`api::fetch_events_from_url()`](../../src/api.rs), [`tests/api_tests.rs`](../../tests/api_tests.rs) | Loopback fixtures execute production request/decoding/error paths, including malformed JSON, HTTP errors and `u32` IDs. Do not replace these with generic `reqwest` tests; do not infer live endpoint health or add configurable production endpoints without a need. |
| Pure fitting with an explicit date | [`personal_estimation::predict()`](../../src/personal_estimation.rs) and its adjacent tests | Keep numerical fitting independent of HTTP/SQLite and pass the cutoff date explicitly. Test earlier-only chronology, support thresholds, known-zero versus unknown gain and unsupported inputs. Provenance/rider qualification belongs to the database adapter; public training structs alone do not enforce it. |
| One estimate policy across selection and output | [`event_filtering::estimate_event()`](../../src/event_filtering.rs), [`tests/duration_consistency_tests.rs`](../../tests/duration_consistency_tests.rs), [`tests/personal_estimation_tests.rs`](../../tests/personal_estimation_tests.rs) | Test priority and negative cases, not only arithmetic: fixed durations, selected subgroup, lead-in once, rounding, unavailable geometry, chronology and honest fallback. Independent callers still reread time/state; this is not a single immutable snapshot. |
| Validated, atomic measured-history ingestion | [`HistoryRecord::validate()`](../../src/history.rs), [`database_history.rs`](../../src/database_history.rs), [`import_zwiftpower_history`](../../src/bin/import_zwiftpower_history.rs), [`tests/history_import_tests.rs`](../../tests/history_import_tests.rs) | Validate complete batches, use database-owned transactions, preserve confirmed scores/history, reject unsupported schemas, prove rollback with a forced mid-batch failure, and prove unchanged-target preview and repeatability. This does not establish concurrent-writer or disk-full safety. |
| Client geometry and source preservation | [`Database::import_client_routes()`](../../src/database.rs), [`tests/client_route_import_tests.rs`](../../tests/client_route_import_tests.rs), [`tests/elevation_consistency_tests.rs`](../../tests/elevation_consistency_tests.rs) | Preserve `u32` identity, fractional measurements, provenance and unknown versus zero; test invalid batches and rollback. Do not copy the history preview's unchanged-target guarantee: client preview can initialize/migrate schema. |

These tests are inspected examples, not a claim they ran during documentation
adoption. Use the [testing guide](../howto/TESTING_GUIDE.md) for change-specific
checks, isolation and separate populated-data accuracy qualification.

## Observed patterns

| Pattern | Current form | Cost of breaking it |
|---|---|---|
| Route identity | Public and database APIs accept `u32 route_id`; aliases map alternate IDs to canonical rows. | Using names or signed 32-bit IDs can silently mismatch routes or overflow external values. |
| SQL ownership | Raw parameterized application SQL lives in `src/database.rs` and its `database_history.rs` child; callers use `Database` methods. | SQL elsewhere creates a second schema/query boundary and makes migrations harder to audit. |
| Estimation layering | Pure category/difficulty math in `duration_estimation.rs`, pure personal fitting plus a database adapter in `personal_estimation.rs`, route bridge in `estimation.rs`, and ordered estimate resolution in `event_filtering.rs`. | Combining layers can introduce database dependence into pure tests or change fallback priority accidentally. |
| Errors | `anyhow::Result` propagates technical failures; `src/errors.rs` formats selected user-facing errors and suggestions. | A new style in one path can duplicate messages or expose only low-level errors. |
| Configuration | Serde TOML structs with `Default`; first existing config file wins, then environment overrides. Secrets have a separate `Secrets` struct. | Merging files or storing secrets in TOML changes precedence and credential boundaries. |
| Optional OCR | OCR modules and four binaries are gated by Cargo's `ocr` feature. | Unconditional imports make default builds require native OCR dependencies. |
| Tests near logic | Core modules use `#[cfg(test)]`; cross-module behaviour lives under `tests/`; snapshots/golden files preserve outputs. | Moving private helpers can strand `super::*` tests or force an unintended public API. |
| Display | Event calculations feed either a compact row/table or verbose functions in `event_display.rs`. | Changing only one presentation path can make compact and verbose estimates disagree. |
| Schema and explicit imports | Database initialization migrates supported schemas transactionally; explicit importers own data ingestion and their distinct preview semantics. Historical operator SQL is not an automatic migration framework. | Assuming no automatic migration, or assuming all previews leave the target unchanged, can modify user state unexpectedly. |

## Naming and module shape

- Module files use snake_case and generally own one concern.
- Public function names are verb phrases (`fetch_events`, `get_route`, `filter_by_tags`, `estimate_duration_with_distance`).
- Database data structs live in `database.rs`; API/domain structs live in `models.rs`. Both define a `RouteData`, so imports often alias the database form.
- Constants used across calculation/display code live in `constants.rs`; category speeds are returned by category functions rather than a configuration file.

## Test patterns

- Pure numerical examples use direct assertions in the owning module.
- Property tests use Proptest and keep minimized regression seeds beside the test target.
- Snapshot tests use Insta files under `tests/snapshots/`.
- Accuracy tests read the local SQLite race history and print diagnostics with `--nocapture`.
- HTTP tests call production `fetch_events_from_url()` against Mockito; the CLI keeps its fixed endpoint.

## Notable absences

- No ORM or generic migration framework; supported initialization migrations exist in the database modules.
- No service/repository abstraction between domain logic and `Database` methods.
- No CLI endpoint override; the adapter accepts a URL for loopback testing.
- No hermetic full CLI fixture covering fetch → filter → display.
- No direct individual-rider physics model in the production estimator.
- No endpoint failover.
