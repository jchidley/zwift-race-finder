# Implementation Architecture

This page maps call boundaries and change propagation for the current finder.
The behavioural baseline is `v0.3.0`; comment/document cleanup does not replace the
estimator. [Architecture constraints](../reference/ARCHITECTURE.md) and
[selected examples](PATTERNS.md#selected-first-party-examples) direct new work.

## Executable and library boundary

`src/main.rs` is both CLI orchestration and a consumer of the package library. It keeps only `commands` and `zwiftpower` binary-private; API, configuration, database and reusable estimation/filtering/display modules are shared through the `zwift_race_finder` library crate.

API/configuration/database now compile once as library modules rather than duplicated private CLI copies. Library internals use `crate::...`; binary-private adapters use the shared library modules.

## Main runtime flow

```text
Args::parse
  ├─ special command? ──> commands.rs ──> database
  └─ normal search
       ├─ FullConfig::load ──> TOML + environment secrets
       ├─ get_user_stats ──> 24h cache → ZwiftPower → configured/default score
       ├─ api::fetch_events ──> fixed Zwift public endpoint
       ├─ filter_events
       │    ├─ sport, time, event type, tags
       │    └─ event_matches_duration
       │         ├─ subgroup and description parsing
       │         ├─ database route/alias lookup
       │         └─ fixed time / personal estimate / unavailable
       └─ event_display ──> compact table or verbose output
```

The normal path is asynchronous only around external HTTP work. Filtering, estimation, SQLite access, and display preparation are synchronous. There is no long-lived service, worker pool, or shared in-process database connection.

## Persistence boundaries

`src/database.rs` and its `database_history.rs` child own application SQL. `Database::new()` opens the platform-local data path, creates tables, and leaves fresh route tables empty until an explicit `import_client_routes` import. Existing route rows and race history are not deleted by retirement. Callers frequently create short-lived `Database` instances rather than sharing one connection.

| Store | Writer/readers | Authority |
|---|---|---|
| `races.db` | `database.rs`, called by commands, estimation, filtering, display, import, regression | Durable local route/results truth |
| `user_stats.json` cache | `cache.rs`, used by `zwiftpower.rs` | Transient score/category cache; valid for 24 hours |
| TOML config | loaded read-only by `config.rs` | Non-secret user preferences/defaults |
| Environment | read by `config.rs` and `zwiftpower.rs` | Secrets and highest-priority overrides |
| Private client JSON export outside Git | `scripts/export_client_routes.ps1` → `src/bin/import_client_routes.rs` → `Database::import_client_routes()` | Explicitly selected client cache plus extracted XML for missing IDs; no bundled dataset |
| `sql/` | operator-run scripts | Migration/mapping inputs; not automatically applied by a migration framework |

### Schema propagation

`Database::from_connection()` enables foreign keys and transactionally runs table creation, elevation-knowledge migration and supported history migration. `CREATE TABLE IF NOT EXISTS` alone does not evolve existing columns. Extend the owning database module's migration and affected queries/row mapping; unsupported custom/dependent history schemas must fail without destruction. Pair changes with synthetic legacy preservation and forced-rollback tests, and update `docs/reference/DATABASE.md` and affected importer documentation. Back up real state before initialization upgrades or imports; historical operator SQL is not a substitute for the application's migration path.

## Duration decision boundary

The active duration boundary:
`event_filtering::resolve_event()` owns `Result<EventEstimate, EstimateUnavailable>`.
`estimate_event()` is its optional adapter. Fixed event/subgroup time wins after
subgroup eligibility; otherwise supported ordinary races use `personal_estimation`.
An error exposes no eligible subgroup, missing geometry or a personal unavailability
reason. A successful estimate carries fixed status or personal method/support,
not a category fallback reason. Unsupported personal durations remain unknown.
Duration filtering uses inclusive tolerance; compact and verbose output call the
same resolver independently. See [Algorithms](../reference/ALGORITHMS.md#estimation-priority)
and `tests/duration_consistency_tests.rs`.

Geometry prefers the selected subgroup's route ID, then positive finite subgroup
distance, positive subgroup laps plus one known lead-in, event distance, known
route/multi-lap metadata, Racing Score description distance, then name-distance
fallback. Explicit total distance gets no additional lead-in. Zero laps cannot
produce a lead-in-only estimate. Optional route/lead-in climbing distinguishes
unknown from known zero; proportional distance-limited climbing is approximate.
`src/estimation.rs` supplies geometry helpers as well as retained standalone
category-duration helpers; the active filtering branch does not call the latter
for personal fallback. Client metadata import and provenance are described in
[client-only route data](../howto/CLIENT_ROUTE_DATA.md).

Personal estimation uses strictly earlier dates, same-route pace first, otherwise
distance/ascent fitting. Its one-year weighting and support thresholds are
unchanged; new recency policies remain proposals. Resolved minutes retain fractional precision for filtering; only display rounds
to whole minutes. Both CLI and library call `EventEstimate::matches_duration()`. The disabled profile prototype is not this
active cross-course fit. See [Algorithms](../reference/ALGORITHMS.md#personal-duration-estimates)
for exact thresholds and fitting rules.

### Change propagation

A formula or threshold change can affect:

- filtering inclusion in `event_filtering.rs`;
- displayed predictions in `event_display.rs`;
- snapshots and property expectations;
- the local-database accuracy regression;
- algorithm and physical-stat references.

Run focused contract tests, then applicable gates with isolated state. Formula changes also require before/after MAE on the same approved populated data; the category regression does not qualify personal fitting. Report chronological shipped-code personal evaluation separately. See the [testing guide](../howto/TESTING_GUIDE.md).

The shared resolver is called independently by filtering and rendering and reads
`Utc::now()` and current database state. Tests establish agreement for their fixtures,
not a once-per-event snapshot across concurrent changes or a UTC date boundary.
Passing an immutable estimate through the pipeline is deferred application work.

## External boundaries

- Zwift events: one hard-coded regional public endpoint in `src/api.rs`; 30-second client timeout; no endpoint failover.
- ZwiftPower: profile-page scraping in `src/zwiftpower.rs`; authenticated URL places the session value in the query string. Secrets originate from environment variables.
- Route metadata: only explicitly selected client cache and extracted client XML for missing IDs. Exports are private outside Git; latest/active cache status is not verified here.
- Discovery and description-analysis commands are removed. The leftover route-name guessing module is also removed. Event-distance/elevation/lap parsing in `src/parsing.rs` remains supported.
- OCR: optional local native libraries and model files; not part of default features.

## Implicit contracts

- Route identity is `u32` even though SQLite uses signed 64-bit integers and some external representations can appear negative.
- Route aliases must point to a canonical route row; lookup tries direct ID before alias resolution.
- Category strings connect retained legacy speed/climbing helpers; event subgroup selection does not supply personal pace.
- Description parsing depends on human-formatted event text and units.
- Config precedence depends on first existing TOML path, then environment override; partial TOML sections can replace defaults differently from a missing section.
- The legacy regression result depends on the approved populated database copy; an empty test state does not qualify accuracy.
- Application connections enable `PRAGMA foreign_keys=ON` before initialization; a foreign-key check guards migrated history. Tests cover supported preservation/rollback paths, not arbitrary corruption or concurrent writers.
- The CLI assumes roughly 200 events/~12 hours from the endpoint; this is an external observation, not enforced by the response type.

## Performance-history migration/import

`src/history.rs` validates the private versioned profile export and normalizes measured observations without score inference. `src/database_history.rs`, a child of `database.rs`, owns the nullable-score history migration, source uniqueness, reconciliation and atomic import. `src/bin/import_zwiftpower_history.rs` hashes input bytes and preflights migration/import on an in-memory SQLite backup before writes. Normal database initialization migrates supported old history schemas transactionally; unsupported custom/dependent schemas fail. `Database::performance_history()` is the score-independent query boundary. Manual result recording and completion presentation are removed; history ingestion uses the source-qualified importer. History ingestion does not itself calibrate prediction. Subsequent owner-authorized personal estimation reads these observations as described above; it does not infer missing scores or establish improved accuracy.
