# Current Requirements

This page records current product and engineering requirements. Implementation details are described in [Architecture](ARCHITECTURE.md), [Algorithms](ALGORITHMS.md), and [Database](DATABASE.md). Open enhancements and known limitations belong in [`TASKS.md`](../../TASKS.md); the earlier 2025 requirements record is preserved in [project history](../project-history/REQUIREMENTS_2025.md).

## Product outcome

Given a rider's Zwift Racing Score and a target duration, the CLI must find upcoming cycling events whose estimated duration falls within the requested tolerance.

## Event discovery and filtering

The main CLI must:

- fetch upcoming events from the Zwift public events endpoint;
- exclude non-cycling and out-of-range events;
- filter races, time trials, group rides, workouts, fondos, included tags, and excluded tags;
- identify the rider's applicable traditional or Racing Score subgroup;
- use fixed event durations when supplied;
- obtain distance from subgroup data, event data, descriptions, route data, or the documented name fallback;
- report useful no-result guidance rather than failing silently.

The CLI must continue to identify its single-endpoint and limited-event-window assumptions to users when they affect a search.

## Duration estimation

The estimator must:

- prioritize fixed durations, then supported ordinary-race personal history, otherwise report unknown duration and exclude it from time-window matches as specified in [Algorithms](ALGORITHMS.md);
- use only source-qualified, single-rider observations from UTC dates strictly before `min(today, event day)` for personal fitting; never use Racing Score as a personal pace feature;
- never substitute category-derived speed for an unavailable personal duration; retain legacy formula helpers only for separate callers/regression;
- resolve routes by `route_id`, including aliases, rather than by display name;
- handle multi-lap distance when lap data is available;
- allow supported same-route evidence without known ascent; report unavailable geometry when neither personal method is supported;
- keep prediction changes measurable against recorded race results;
- use the same estimate resolver for duration filtering and compact/verbose presentation, with inclusive tolerance and no independent display formula;
- preserve unknown measurements separately from known zero, measured seconds separately from rounded minutes, and event distance/laps separately from base route metadata;
- report personal method/support or an explicit unavailability reason; fitted residuals are descriptive, not confidence intervals.

Any formula change requires before/after MAE in minutes on the same approved populated data. The focused regression checks the retained legacy category formula, not personal-estimator accuracy. Personal prediction claims additionally require chronological evaluation of the shipped implementation, comparable event geometry, and out-of-sample MAE and time-window inclusion results; do not tune on a holdout and present it as independent validation.

The existing product target was recorded as below 20% “mean absolute error.” That percentage wording does not specify MAE in minutes; its metric interpretation remains unresolved. Do not silently reinterpret it or substitute a new target. The current focused test reports MAE and MAPE, asserts MAPE below 30%, and passes with zero usable comparisons. These implementation facts are not the product target or sufficient qualification. Report sample counts and missing-data exclusions; zero samples mean unqualified accuracy. See [the testing guide](../howto/TESTING_GUIDE.md#run-the-accuracy-regression) and [dated measurements](ALGORITHMS.md#validation-and-calibration).

## Data and persistence

The application must:

- store route metadata/provenance, aliases, measured race history and unknown routes in local SQLite; preserve existing legacy completion/rider-stat/discovery data without recreating or using those retired features;
- use unsigned 32-bit route IDs in Rust and SQLite integer storage without truncation;
- create and migrate supported schemas transactionally and idempotently; reject unsupported history schemas rather than discarding data;
- validate complete import batches before writes, apply them atomically, preserve existing history and confirmed scores, and make repeated supported imports idempotent;
- preserve source/rider/result identity and source hashes; never infer historical Racing Score from category, current score or other rankings;
- document preview effects separately: measured-history preview leaves the target unchanged; client-route preview can initialize/migrate schema;
- avoid using route names as identity;
- accept route metadata only from an explicitly selected client cache and extracted client XML for IDs absent from that cache; keep exports private outside Git with no bundled dataset;
- leave fresh databases without route rows until explicit `import_client_routes` import, while retaining existing rows/history;
- obtain route metadata through explicit client imports, not discovery or description-analysis commands; retain internal event-description parsing;
- keep credentials outside source code and committed configuration;
- accept ZwiftPower credentials only from environment variables.

Application SQL and schema evolution belong in `src/database.rs` and its `src/database_history.rs` child; callers use `Database` methods. [Database Reference](DATABASE.md) mirrors storage, but its older route-ID cast note is not permission to bypass checked input conversion. Back up existing data before imports or schema upgrades; see the [client-route](../howto/CLIENT_ROUTE_DATA.md) and [measured-history](../howto/ZWIFTPOWER_EXPORT_STEPS.md) procedures. The latter's storage-only prediction discussion predates the active personal estimator; [Algorithms](ALGORITHMS.md) owns current prediction policy.

## CLI and configuration

The main binary must:

- run without configuration by using documented defaults;
- accept command-line values for score, duration, tolerance, event type, days, tags, and display mode; ingest measured history through the source-qualified importer rather than manual result entry;
- load non-secret configuration using the precedence documented in the [Configuration Reference](CONFIGURATION.md);
- keep explicit command-line values ahead of loaded preferences;
- produce actionable errors for invalid scores, API failures, and database failures.

The CLI-over-config requirement is not fully met: duration/tolerance/days currently
use equality with the built-in default as the override test, so an explicitly
supplied default value is indistinguishable from absence. See
[Configuration](CONFIGURATION.md); fixing that behaviour is outside documentation adoption.

## Quality and safety

- Default-feature code must build and pass `cargo test` under the documented system prerequisites.
- Formula changes must run the accuracy regression and report the before/after result.
- Route-ID parsing and storage must support values above signed 32-bit range.
- Separate deterministic correctness, live-service compatibility, populated-data accuracy and optional OCR qualification. Identify non-hermetic tests; isolate writable HOME/XDG state and exclude inherited credentials. Populated accuracy uses a deliberately approved dataset instead of fresh empty data, still without inherited credentials.
- Use focused requirement-derived positive and negative tests, then applicable suite/format/lint gates. Existing failures remain visible; do not weaken assertions, broadly suppress lints, or replace expected outputs simply to obtain a pass.
- Credentials, personal IDs, tokens, and private activity exports must not be committed.
- Files under `archive/` must remain historical and unmodified.

## Supported environment

Linux and WSL are the primary environments. Debian/Ubuntu builds require Rust, `libssl-dev`, and `pkg-config`. OCR tooling is optional and requires the `ocr` Cargo feature plus its native dependencies.
