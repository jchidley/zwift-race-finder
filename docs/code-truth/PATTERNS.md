<!-- code-truth: ad3e87c -->
# Implementation Patterns

These are observed repository patterns, not universal Rust prescriptions.

| Pattern | Current form | Cost of breaking it |
|---|---|---|
| Route identity | Public and database APIs accept `u32 route_id`; aliases map alternate IDs to canonical rows. | Using names or signed 32-bit IDs can silently mismatch routes or overflow external values. |
| SQL ownership | Raw parameterized SQL lives in `src/database.rs`; callers use `Database` methods. | SQL elsewhere creates a second schema/query boundary and makes migrations harder to audit. |
| Estimation layering | Pure category/difficulty math in `duration_estimation.rs`; route lookup bridge in `estimation.rs`; ordered event fallbacks in `event_filtering.rs`. | Combining layers can introduce database dependence into pure tests or change fallback priority accidentally. |
| Errors | `anyhow::Result` propagates technical failures; `src/errors.rs` formats selected user-facing errors and suggestions. | A new style in one path can duplicate messages or expose only low-level errors. |
| Configuration | Serde TOML structs with `Default`; first existing config file wins, then environment overrides. Secrets have a separate `Secrets` struct. | Merging files or storing secrets in TOML changes precedence and credential boundaries. |
| Optional OCR | OCR modules and four binaries are gated by Cargo's `ocr` feature. | Unconditional imports make default builds require native OCR dependencies. |
| Tests near logic | Core modules use `#[cfg(test)]`; cross-module behaviour lives under `tests/`; snapshots/golden files preserve outputs. | Moving private helpers can strand `super::*` tests or force an unintended public API. |
| Display | Event calculations feed either a compact row/table or verbose functions in `event_display.rs`. | Changing only one presentation path can make compact and verbose estimates disagree. |
| Operator data changes | Imports and mappings use scripts plus SQL files; there is no automatic migration framework. | Assuming migrations run automatically leaves existing local databases on old schema/data. |

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
- HTTP tests construct Mockito servers, but currently test generic `reqwest` calls rather than injecting a base URL into production `fetch_events()`.

## Notable absences

- No ORM or migration runner.
- No service/repository abstraction between domain logic and `Database` methods.
- No injectable production API base URL.
- No hermetic full CLI fixture covering fetch → filter → display.
- No direct individual-rider physics model in the production estimator.
- No endpoint failover.
