<!-- code-truth: ad3e87c -->
# Implementation Decisions

These decisions are evidenced by current code and maintained project records. Historical rationale is linked rather than repeated.

## Structural decisions

### Route IDs are canonical identity

**Status:** active\
**What:** Routes are keyed by unsigned 32-bit Zwift route IDs; aliases resolve alternate event-context IDs.\
**Where:** `src/database.rs`, `src/estimation.rs` (the legacy alias SQL was retired; existing database aliases are preserved).\
**Consequences:** Names remain fallback/display values. Any route import, completion, result, or mapping change must preserve IDs above signed 32-bit range.

### SQLite is the durable local store

**Status:** active\
**What:** Route knowledge, race results, completion, rider stats, unknown routes, and discovery attempts live in one platform-local SQLite file.\
**Where:** `src/database.rs`.\
**Consequences:** The application needs no database service, but schema evolution must account for existing user files. Raw SQL and row mapping remain centralized.

### Duration uses empirical category speed and route difficulty

**Status:** active\
**What:** Duration is distance divided by empirical category speed multiplied by an elevation/category difficulty factor. Raw height, weight, FTP, field size, and draft state are not direct inputs.\
**Where:** `src/category.rs`, `src/duration_estimation.rs`, `src/estimation.rs`.\
**Why:** Calibration records indicate that empirical category speeds already include typical draft benefit and category-specific climbing penalties capture observed group differences. See [`PROJECT_WISDOM.md`](../../PROJECT_WISDOM.md) and [Algorithms](../reference/ALGORITHMS.md).\
**Consequences:** A physics-heavy or personalized estimator is a model replacement, not a local refactor. It needs comparative race-data evidence.

### Event estimation is an ordered fallback tree

**Status:** active\
**What:** Filtering returns on the first usable duration source, preferring explicit duration and structured route/subgroup data before description/name heuristics.\
**Where:** `src/event_filtering.rs::event_matches_duration`.\
**Consequences:** Reordering branches can change which events match even if each estimator is unchanged. Test the selection contract, not only arithmetic.

## Pragmatic decisions

### Binary-private adapters coexist with the library

**Status:** active\
**What:** The main binary keeps API, command, config, database, discovery, and ZwiftPower modules private while reusable logic is also exported from the library.\
**Where:** `src/main.rs`, `src/lib.rs`.\
**Consequences:** This avoids a larger reorganization but creates two crate contexts and import/visibility friction during moves.

### Configuration uses first-file-wins plus environment overrides

**Status:** active\
**What:** The first existing TOML file is deserialized; environment values then override selected fields. Secrets are loaded separately from environment variables.\
**Where:** `src/config.rs`.\
**Consequences:** This is not a layered merge of all files. Changing precedence can alter user behaviour and requires config tests and docs updates.

### OCR remains optional

**Status:** active\
**What:** OCR dependencies, modules, and binaries require the `ocr` feature.\
**Where:** `Cargo.toml`, `src/lib.rs`, OCR binaries/tests.\
**Consequences:** Keep the default race-finder build independent of native OCR prerequisites.

### Historical and current documentation are separated

**Status:** active\
**What:** Current work lives in `STATUS.md`/`TASKS.md`; stable facts in `docs/reference/`; implementation snapshots here; historical reports in `docs/project-history/` or `archive/`.\
**Consequences:** Do not copy volatile counts or old migration plans back into current reference and startup instructions.

## Open questions

- The documented ~200-event/~12-hour API limit is encoded as comments and user messaging, not a tested contract. Revalidate it when API behaviour changes. **[UNCERTAIN]**
- `Config::default()` includes personal-shaped weight and height defaults although production estimation does not consume them. Decide whether generic defaults or optional absence better represents the product. **[UNCERTAIN]**
- `src/estimation.rs::get_route_data()` leaks database strings to obtain `'static` references. It works for short CLI runs but should be understood before reuse in a long-lived process. **[UNCERTAIN]**
- The source contains a support URL pointing to the unrelated `anthropics/claude-code` issue tracker when unexpectedly many events are returned. Choose the intended project support destination before documenting it. **[UNCERTAIN]**

## Resolved during history migration — 2026-10-30

Application connections now enable `PRAGMA foreign_keys = ON` before initialization. Supported history migration and batch imports check constraints; synthetic preservation/rollback checks and the backed-up normal import pass with no foreign-key violations. This resolves the earlier connection-enforcement uncertainty.
