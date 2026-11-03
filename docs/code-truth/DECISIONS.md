# Implementation Decisions

Current behaviour is anchored by `v0.3.0`; check current source/diff before changes.
[Architecture](../reference/ARCHITECTURE.md) owns constraints,
[Algorithms](../reference/ALGORITHMS.md) owns numerical policy and
[Tasks](../../TASKS.md) owns unfinished decisions.

## Identity, data and imports

Use unsigned numeric route IDs and preserve aliases, optional measurements,
precise finish seconds, event geometry and source identity. SQL/schema evolution
belongs to `database.rs` and `database_history.rs`. Initialization migrates supported
schemas transactionally with foreign keys enabled; unsupported history schemas
fail without discarding data. Measured-history preview leaves the target unchanged;
client preview can initialize/migrate it. Existing retired tables remain preserved.

## Estimate authority

Score selects subgroup geometry, not personal pace. Fixed time wins; supported
ordinary races use earlier same-route or cross-course fitting; unavailable duration
is excluded and disclosed. No category-speed substitute. Filtering and rendering
call the same resolver independently, not an immutable once-per-event snapshot.
Test priority, prohibited matches, rounding and geometry as well as arithmetic.

## Boundaries

Concrete CLI/library modules remain appropriate; only command and ZwiftPower
adapters are binary-private. Configuration is first-file-wins plus environment
overrides, not layered merging. Secrets come from environment variables. OCR stays
feature-gated and is not a prerequisite for race selection.

Follow [selected examples](PATTERNS.md#selected-first-party-examples). Keep structural
invariants in types/APIs, use requirement-derived positive/negative tests, retain
unrelated work and use bounded recovery. Documentation guidance is not evidence of
new tests, CI enforcement, successful gates or model superiority.

## Current experiment

The [baseline/recency comparison](../project-history/RECENCY_BASELINE_2026-11-03.md)
preserves a usable active model and a separate R1 candidate. Recent examples improve
while the supporting historical comparison is mixed. R1 remains experimental;
current-form research remains open. Do not retune inspected outcomes or infer
physiological capacity from race performance alone.
