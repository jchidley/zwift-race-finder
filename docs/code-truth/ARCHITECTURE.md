<!-- code-truth: ad3e87c -->
# Implementation Architecture

This page describes current code boundaries and change propagation. Domain physics is explained under [`docs/explanation/`](../explanation/); user-facing algorithms are specified in [`docs/reference/ALGORITHMS.md`](../reference/ALGORITHMS.md).

## Executable and library boundary

`src/main.rs` is both CLI orchestration and a consumer of the package library. It declares binary-private modules for `api`, `commands`, `config`, `database` and `zwiftpower`, while importing reusable estimation/filtering/display modules through the `zwift_race_finder` library crate.

Some files therefore compile in two module contexts: through `src/lib.rs` and as private modules of the binary. Imports using `crate::...` and `zwift_race_finder::...` are not interchangeable. Moving code across this boundary can break visibility or resolve a type from the wrong crate context.

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
       │         └─ duration calculation/fallbacks
       └─ event_display ──> compact table or verbose output
```

The normal path is asynchronous only around external HTTP work. Filtering, estimation, SQLite access, and display preparation are synchronous. There is no long-lived service, worker pool, or shared in-process database connection.

## Persistence boundaries

`src/database.rs` owns all application SQL. `Database::new()` opens the platform-local data path, creates tables, and leaves fresh route tables empty until an explicit `import_client_routes` import. Existing route rows and race history are not deleted by retirement. Callers frequently create short-lived `Database` instances rather than sharing one connection.

| Store | Writer/readers | Authority |
|---|---|---|
| `races.db` | `database.rs`, called by commands, estimation, filtering, display, import, regression | Durable local route/results truth |
| `user_stats.json` cache | `cache.rs`, used by `zwiftpower.rs` | Transient score/category cache; valid for 24 hours |
| TOML config | loaded by `config.rs`; `Config::save()` can write user data-dir config | Non-secret user preferences/defaults |
| Environment | read by `config.rs` and `zwiftpower.rs` | Secrets and highest-priority overrides |
| Private client JSON export outside Git | `scripts/export_client_routes.ps1` → `src/bin/import_client_routes.rs` → `Database::import_client_routes()` | Explicitly selected client cache plus extracted XML for missing IDs; no bundled dataset |
| `sql/` | operator-run scripts | Migration/mapping inputs; not automatically applied by a migration framework |

### Schema propagation

A schema change starts in `src/database.rs::create_tables()`, but existing databases may need explicit migration SQL because `CREATE TABLE IF NOT EXISTS` does not alter existing columns. Update every affected query and row mapping in `database.rs`, then update `docs/reference/DATABASE.md` and relevant import/mapping scripts.

## Duration decision boundary

Local update after the commit-scoped snapshot: `event_filtering::estimate_event()` owns the ordered fallback tree and returns `EventEstimate` (minutes, optional distance/elevation, fixed status). It may use fixed event duration, subgroup lap data, event/subgroup distance, known route data, description parsing, or an event-name distance guess. `event_matches_duration()` compares its minutes using inclusive tolerance; compact rows and verbose duration formatting use the same resolver rather than separate formulas. The final subgroup fallback uses one selected subgroup, not any matching subgroup. See `tests/duration_consistency_tests.rs` for fixed-duration, fallback visibility, metadata, lead-in, and display/filter agreement regressions.

`src/estimation.rs` bridges route lookup to pure calculations. `estimate_duration_from_route_id()` adds the stored lead-in. `estimate_duration_with_distance()` trusts its supplied distance and does not add lead-in. The multi-lap filtering branch explicitly adds lead-in before calling it; event-provided-distance branches do not. Local elevation update: route and lead-in elevation are optional, with SQLite knowledge flags distinguishing legacy unknown zeros from explicitly known zero. Pure helpers in `estimation.rs` compute exact lap totals and approximate distance-limited totals; the latter carry an approximation flag in `EventEstimate`. Client-only updates use `scripts/export_client_routes.ps1` and `src/bin/import_client_routes.rs`; `Database::import_client_routes()` atomically writes route metadata and `route_sources` provenance, with selected cache > extracted XML precedence; legacy rows remain until explicitly updated. Elevation fields are optional `f64`, lead-in distance is optional, and total elevation is rounded only at the presentation boundary. See [client-only route data](../howto/CLIENT_ROUTE_DATA.md).

### Change propagation

A formula or threshold change can affect:

- filtering inclusion in `event_filtering.rs`;
- displayed predictions in `event_display.rs`;
- snapshots and property expectations;
- the local-database accuracy regression;
- algorithm and physical-stat references.

Run the focused accuracy regression and compare results before and after.

## External boundaries

- Zwift events: one hard-coded regional public endpoint in `src/api.rs`; 30-second client timeout; no endpoint failover.
- ZwiftPower: profile-page scraping in `src/zwiftpower.rs`; authenticated URL places the session value in the query string. Secrets originate from environment variables.
- Route metadata: only explicitly selected client cache and extracted client XML for missing IDs. Exports are private outside Git; latest/active cache status is not verified here.
- Discovery and description-analysis commands are removed. The leftover route-name guessing module is also removed. Event-distance/elevation/lap parsing in `src/parsing.rs` remains supported.
- OCR: optional local native libraries and model files; not part of default features.

## Implicit contracts

- Route identity is `u32` even though SQLite uses signed 64-bit integers and some external representations can appear negative.
- Route aliases must point to a canonical route row; lookup tries direct ID before alias resolution.
- Category strings such as `E`, `D`, `C`, `B`, `A`, and `A++` connect category mapping to speed and climbing-factor matches.
- Description parsing depends on human-formatted event text and units.
- Config precedence depends on first existing TOML path, then environment override; partial TOML sections can replace defaults differently from a missing section.
- The regression result depends on the contents of the normal local database, not a versioned fixture.
- SQLite foreign-key declarations exist, but connection setup does not visibly enable `PRAGMA foreign_keys = ON`. Enforcement should be verified before relying on it. **[UNCERTAIN]**
- The code comments treat the event endpoint as limited to roughly 200 events/~12 hours; this is an external observation, not enforced by the response type. **[UNCERTAIN]**

## Performance-history migration/import

`src/history.rs` validates the private versioned profile export and normalizes measured observations without score inference. `src/database_history.rs`, a child of `database.rs`, owns the nullable-score history migration, source uniqueness, reconciliation and atomic import. `src/bin/import_zwiftpower_history.rs` hashes input bytes and preflights migration/import on an in-memory SQLite backup before writes. Normal database initialization migrates supported old history schemas transactionally; unsupported custom/dependent schemas fail. `Database::performance_history()` is the score-independent query boundary. Manual result recording and completion presentation are removed; history ingestion uses the source-qualified importer. The production duration estimator is unchanged by history ingestion.
