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

- use the category speeds and piecewise elevation multipliers documented in [Algorithms](ALGORITHMS.md);
- apply category-specific climbing penalties above the documented threshold;
- resolve routes by `route_id`, including aliases, rather than by display name;
- handle multi-lap distance when lap data is available;
- preserve explicit fallback behaviour when route elevation data is unavailable;
- keep prediction changes measurable against recorded race results.

Any formula change requires the focused accuracy regression. The accepted product target is below 20% mean absolute error on the owner's maintained race-result dataset; the test threshold and latest measured result are separate facts documented in [Algorithms](ALGORITHMS.md).

## Data and persistence

The application must:

- store route, alias, race-result, rider-stat, and unknown-route data in local SQLite;
- use unsigned 32-bit route IDs in Rust and SQLite integer storage without truncation;
- create its schema idempotently;
- avoid using route names as identity;
- accept route metadata only from an explicitly selected client cache and extracted client XML for IDs absent from that cache; keep exports private outside Git with no bundled dataset;
- leave fresh databases without route rows until explicit `import_client_routes` import, while retaining existing rows/history;
- obtain route metadata through explicit client imports, not discovery or description-analysis commands; retain internal event-description parsing;
- keep credentials outside source code and committed configuration;
- accept ZwiftPower credentials only from environment variables.

The database schema is authoritative in `src/database.rs`; [Database Reference](DATABASE.md) is its human-readable mirror.

## CLI and configuration

The main binary must:

- run without configuration by using documented defaults;
- accept command-line values for score, duration, tolerance, event type, days, tags, and display mode; ingest measured history through the source-qualified importer rather than manual result entry;
- load non-secret configuration using the precedence documented in the [Configuration Reference](CONFIGURATION.md);
- keep explicit command-line values ahead of loaded preferences;
- produce actionable errors for invalid scores, API failures, and database failures.

## Quality and safety

- Default-feature code must build and pass `cargo test` under the documented system prerequisites.
- Formula changes must run the accuracy regression and report the before/after result.
- Route-ID parsing and storage must support values above signed 32-bit range.
- Tests that invoke the network or normal user database must be identified as non-hermetic.
- Credentials, personal IDs, tokens, and private activity exports must not be committed.
- Files under `archive/` must remain historical and unmodified.

## Supported environment

Linux and WSL are the primary environments. Debian/Ubuntu builds require Rust, `libssl-dev`, and `pkg-config`. OCR tooling is optional and requires the `ocr` Cargo feature plus its native dependencies.
