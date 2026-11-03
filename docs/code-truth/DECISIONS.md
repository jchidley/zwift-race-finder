<!-- code-truth: checkpoint 3e13a78 plus unpublished working-tree observations; see README.md -->
# Implementation Decisions

These decisions are evidenced by inspected code and maintained project records,
including unpublished changes after the [recorded checkpoint](README.md).
Historical rationale is linked rather than repeated. Documentation adoption does
not imply new mechanical enforcement or a current build/test pass.

## Structural decisions

### Route IDs are canonical identity

**Status:** active\
**What:** Routes are keyed by unsigned 32-bit Zwift route IDs; aliases resolve alternate event-context IDs.\
**Where:** `src/database.rs`, `src/estimation.rs` (the legacy alias SQL was retired; existing database aliases are preserved).\
**Consequences:** Names remain fallback/display values. Any route import, history or alias change must preserve IDs above signed 32-bit range; retired completion workflows are not a precedent for new work.

### SQLite is the durable local store

**Status:** active\
**What:** Current route knowledge, race results and unknown routes live in one platform-local SQLite file. Retired completion/rider/discovery tables are preserved in existing databases but no longer created.\
**Where:** `src/database.rs`.\
**Consequences:** The application needs no database service, but schema evolution must account for existing user files. Raw SQL and row mapping remain in `database.rs` and its `database_history.rs` child. Supported schema changes run transactionally during initialization with foreign keys enabled; unsupported history schemas fail rather than being discarded. Measured-history preview uses an in-memory copy without changing the target, while client preview may initialize/migrate it.

### Score selects the subgroup; history can estimate personal duration

**Status:** locally implemented at explicit owner request, accuracy qualification retained\
**What:** Fixed event durations win. Ordinary races use earlier source-qualified personal
performances where supported: comparable same-route pace, otherwise a fitted
personal distance/ascent model. Score is not an input to personal fitting.
Unavailable history/geometry and unsupported formats now return unknown duration,
excluded from duration matches with an omitted count. This supersedes the earlier
category fallback after owner approval of its removal.\
**Where:** `src/personal_estimation.rs`, `src/event_filtering.rs`, `src/event_display.rs`;
retained legacy helpers in `src/category.rs`, `src/duration_estimation.rs`, `src/estimation.rs`.\
**Consequences:** This is an authorized model integration, not evidence of demonstrated
selection superiority. Chronological shipped-code replay and synthetic integration
checks qualify results; no historical score inference, exact-profile claims or
training-residual confidence guarantees. See [Algorithms](../reference/ALGORITHMS.md)
and [live implementation](../project-history/LIVE_PERSONAL_ESTIMATOR_2026-11-02.md).

### Event estimation is an ordered fallback tree

**Status:** active\
**What:** `estimate_event()` resolves selected subgroup/geometry and fixed → supported personal → unknown priority; duration filtering and both display modes use that resolver.\
**Where:** `src/event_filtering.rs::resolve_event` (reason-bearing) and `estimate_event` (optional adapter), `event_matches_duration`, `src/event_display.rs`.\
**Consequences:** Reordering branches can change which events match even if each estimator is unchanged. Test selection, rounding, fallback visibility and prohibited cases, not only arithmetic. Calls still independently read time/database state; one immutable estimate per event is deferred, not a present guarantee.

## Pragmatic decisions

### Binary-private adapters coexist with the library

**Status:** active\
**What:** The main binary keeps only command and ZwiftPower adapters private; API, configuration and database implementations are shared library modules.\
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

## Standards adoption — documentation only

**Status:** D1 approved and documented; enforcement/migration not authorized.\
**Scope:** S1–S6 retain the concrete CLI/library design, clarify estimate authority and data/import invariants, select [first-party examples with tests](PATTERNS.md#selected-first-party-examples), separate validation evidence, and replace unsafe unconditional refactoring rollback/test authority. Durable rules live in the architecture/requirements/refactoring references and operational checks in the testing guide; they are not a second rulebook here.\
**Limits:** E1 (explicit populated-data accuracy gate), E2 (config-test subprocess isolation), and M1 (single-estimate pipeline migration) remain separate proposals, not adopted work. No code, tests or CI settings were changed by D1. Existing positive/negative tests support parts of the rules; the adoption itself adds documentation, not mechanical proof.

| Adopted ID | Canonical guidance |
|---|---|
| S1 — concrete boundaries and examples | [Architecture constraints](../reference/ARCHITECTURE.md#engineering-constraints), [selected examples](PATTERNS.md#selected-first-party-examples) |
| S2 — estimate authority and output agreement | [Duration requirements](../reference/REQUIREMENTS.md#duration-estimation), [algorithm priority](../reference/ALGORITHMS.md#estimation-priority) |
| S3 — identity, measurements and provenance | [Data requirements](../reference/REQUIREMENTS.md#data-and-persistence) |
| S4 — atomic imports and precise preview effects | [Data requirements](../reference/REQUIREMENTS.md#data-and-persistence), [import examples](PATTERNS.md#selected-first-party-examples) |
| S5 — separate validation evidence | [Testing guide](../howto/TESTING_GUIDE.md), [suite boundaries](../reference/TEST_SUITE_SUMMARY.md) |
| S6 — requirement-led refactoring and bounded recovery | [Refactoring rules](../reference/REFACTORING_RULES.md) |

## Open questions

- Explicit CLI duration/tolerance/days values equal to built-in defaults can be replaced by config preferences. The intended CLI-over-config contract is not fully enforced; see [Configuration](../reference/CONFIGURATION.md). No behaviour change was authorized by D1.
- The documented ~200-event/~12-hour API limit is encoded as comments and user messaging, not a tested contract. Revalidate it when API behaviour changes. **[UNCERTAIN]**
- The older below-20% “mean absolute error” product target has ambiguous metric wording. The focused test prints MAE in minutes and MAPE, asserts MAPE below 30%, and passes on zero usable comparisons. Do not reinterpret the target or treat an empty-data pass as accuracy evidence; E1 remains a separate enforcement proposal.
- `src/estimation.rs::get_route_data()` leaks database strings to obtain `'static` references. It works for short CLI runs but should be understood before reuse in a long-lived process. **[UNCERTAIN]**
- The source contains a support URL pointing to the unrelated `anthropics/claude-code` issue tracker when unexpectedly many events are returned. Choose the intended project support destination before documenting it. **[UNCERTAIN]**

## Resolved during history migration — 2026-10-30

Application connections now enable `PRAGMA foreign_keys = ON` before initialization. Supported history migration and batch imports check constraints; synthetic preservation/rollback checks and the backed-up normal import pass with no foreign-key violations. This resolves the earlier connection-enforcement uncertainty.
