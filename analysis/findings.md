# Findings: DRY + YAGNI

## YAGNI — Dead Code

| File | LOC | Evidence | Confidence |
|------|-----|----------|------------|
| `compatibility.rs` | 308 | Not in `lib.rs`, not imported by any file. Zero references except comments. | **High** — dead |
| `secure_storage.rs` | 251+91 | In `lib.rs` as `pub mod`, but zero imports from any other module or test file. | **High** — dead |
| `test_utils.rs` | 46 | In `lib.rs` as `pub mod`, but zero imports from any file. | **High** — dead |
| `test_db.rs` | 94 | Not in `lib.rs`, not imported anywhere. `create_test_database` has zero callers. | **High** — dead |

**Total dead code: 790 lines (308 + 342 + 46 + 94)**

### Notes
- `ab_testing.rs` (133+64 LOC) is alive — imported by `tests/uom_integration_tests.rs` and `tests/behavioral/ab_testing.rs`
- `test_db.rs` is different from the golden test's inline `mod integration_with_test_db` — that's unrelated

## DRY — Repeated Patterns

### 1. Route name extraction (10 occurrences)
```rust
let route_name = event.route.as_deref().unwrap_or(&event.name);
```
6× in `event_display.rs`, 4× in `event_filtering.rs`. Could be a method on `ZwiftEvent` or a helper.

### 2. Duration estimation from distance (13 calls)
```rust
estimate_duration_for_category(distance_km, route_name, zwift_score)
```
7× in `event_display.rs`, 6× in `event_filtering.rs`. Each call follows the same pattern: extract distance → convert meters→km → get route name → estimate. The setup is repeated, not just the call.

### 3. Distance meters-to-km conversion (10 occurrences)
```rust
let distance_km = dist_m / METERS_PER_KILOMETER;
```
8× in `event_display.rs`, 2× in `event_filtering.rs`. One instance uses `/1000.0` instead of the constant (line 578 of event_display.rs).

### Assessment

DRY items 1 and 3 are single-expression patterns — extracting them would add function call overhead for minimal clarity gain. The repeated **setup pattern** around duration estimation (items 1+2+3 combined) is the real DRY issue, but it's deeply embedded in branching logic that varies per call site. Extracting it would require careful analysis of each call site's specific context.

**Recommendation:** DRY items are low priority. The YAGNI dead code is the clear win — 790 lines of dead code to remove.
