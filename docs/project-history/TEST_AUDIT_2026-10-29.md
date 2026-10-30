# Test audit — 2026-10-29

## Client-primary source follow-up

The owner requested installed-client data first, then the client XML extracted in session `01a189b5-c35a-721d-ad71-487d9dff54f0`. Implemented explicit precedence: selected cache ZIP → extracted XML for missing IDs → community fallback. This does not establish which cached archive the running game currently selects.

- `scripts/export_client_routes.ps1` reads the selected archive and extracted XML without unpacking/executing assets or writing inputs. It preserves numeric-ID identity, fractional metres, nullable fields, availability flags, and source hashes. Parent reran its 90 PowerShell assertions successfully after inspecting implementation/tests.
- Actual selected archive has 386 unique routes. All 171 extracted Watopia IDs are already present, so this export has 386 primary records and no additional fallback records. The secondary-source behavior is covered by synthetic tests.
- `import_client_routes` imports those records atomically and stores provenance in `route_sources`. Isolated import reconciled all ten legacy seed IDs; total table size is 386. Reimport writes zero rows. Community Rust imports, discovery, and the legacy community Python importer are protected against downgrading client records.
- Optional fractional elevations and lead-in distance retain unknown/zero distinctions. Fixed event time remains unchanged; total gain is rounded only for display. Downtown Dolphin client data yields about 249.2654 m for 15 laps, displayed as 249 m rather than 255 m from rounded community input. The incorrect hardcoded Bell Lap fallback was removed.
- Final isolated default suite: **196 passed, 0 failed, 7 ignored**. PowerShell: **90 assertions passed**. Python importer protection: **2 tests passed**. CLI and both importer release builds pass; all-feature test compilation passes. Existing OCR failures were not rerun/repaired. No snapshots regenerated.
- New tests cover primary/extracted/community precedence, precision, seed correction, preserved history/timestamps, actual forced transaction rollback, dry-run, nullable lead-in distance, and Unicode. One legacy test needed REAL-compatible query/assertion types after fresh-schema elevation columns changed to REAL.
- A plain console transfer initially recoded UTF-8 through Windows IBM850; the importer rejected the invalid JSON before opening a database. Byte-preserving transfer resolved this, and parent verified identical Windows/Linux SHA-256: `0862ca71d9b44aaea32424f270a196446fc7017aa69e1f2d443e55cadf0c8c40`.
- Private export saved outside the repo at `~/.local/share/zwift-race-finder/client-routes-0862ca71d9b4.json`. Source archive SHA-256: `1d96e52198472351b2e519a42972f51461f245f924a1c1624501b63ef62a24ad`.
- Isolated live CLI resolved Stage 4 California Dreamin' by ID: **21.2 km / 106 m / estimated 39 minutes**. KRG was no longer in this captured upcoming-event sample; its new gain is covered by fractional-total tests, not a claimed new live KRG check.
- Accuracy checks before/after again found no mapped races: **both MAEs unavailable**. Category speed/difficulty breakpoints are unchanged; input precision/data changed.
- Same-family fresh-context Rust review (`clientdata-review`, `openai/gpt-6-luna`) found no evidenced regressions. It did not run tests or inspect the collector; parent verified collector/data integration and subsequent nullable-distance/legacy-import protection changes separately.
- Evidence: `/tmp/zwift-client-primary-KFIUAA/`. Normal database, credentials, personal rider records, and `archive/` remain untouched. No commits/publication. Applying the client export to normal state after backup remains pending.

## Elevation correctness follow-up

The owner authorized implementation after the zero-elevation diagnosis. All 309 exported records contained zero elevation, including the usable-distance routes. The fix preserves unknown versus known zero through optional Rust fields and repeatable SQLite knowledge-flag migration; legacy positives remain available and legacy zeros become unknown.

- Independently checked the pinned `andipaetzold/zwift-data` tuple mapping, type definitions, and MIT license. Contrary to the initial scout summary, the source **does** contain event lead-in distance/elevation. Bundled normalized numeric-ID data, attribution, source hash, and a non-executing reproducible converter are in `data/route_reference/` and `scripts/build_route_reference.py`.
- Exact lap totals add lap climbing × laps plus event lead-in climbing once. Distance-limited totals are proportional and explicitly marked `~`; missing climbing remains unknown. Duration uses the existing speed/difficulty rules with corrected event-gradient inputs when available. Historical DB multi-lap metadata no longer repeats lead-in distance or rounded single-lap time. Fixed declared durations are unchanged.
- Import enriches exact exported IDs, repairs unknown elevation by default, and preserves existing nonzero metadata unless `--update` is supplied. Incomplete imports do not erase known elevation/auxiliary lead-ins. UPSERT preserves route creation timestamps and dependent history instead of deleting/replacing rows.
- Final isolated default suite: **191 passed, 0 failed, 7 ignored**. Release CLI/importer builds and all-feature test compilation pass. Snapshots/baselines were not regenerated. Existing OCR assertion/model failures were not repaired or rerun as an all-feature execution suite.
- Fresh isolated import: 267 new routes plus ten seeds; 42 exported records skipped. Repeating the import changes no rows. The end-to-end importer test separately exercises legacy-zero repair, explicit updates, preservation under incomplete input, high IDs, and rejection of name-based identity.
- Isolated live CLI: KRG displays **29.6 km / 255 m / fixed 00:45**. Its 15 laps use source values 1.965 km / 17 m per lap and 0.094 km / 0 m lead-in.
- Accuracy regression ran before/after: both found no mapped races. **Before MAE: unavailable; after MAE: unavailable.** No measured accuracy improvement is claimed.
- Same-family fresh-context read-only review (`elevation-review`, `openai/gpt-6-luna`) reported no concrete regressions. It inspected code/data but ran no tests; this was not cross-family verification. Parent ran the validation above.
- Evidence directory: `/tmp/zwift-elevation-fix-fLQ1ZH/` (including final test, build, accuracy, import, and live-output logs).
- Normal user database, credentials, personal data, and historical `archive/` were untouched. No commit or publication. Applying the importer to normal state after backup remains pending.
- Remaining limits: community metadata is not independently measured event geometry; partial-route climbing is approximate; source coverage is incomplete; existing nonzero seed IDs/names/distances and the hardcoded fallback conflict with current source identity and require explicit reconciliation.

## Duration filter/display consistency fix

User-reported output showed KRG at 53 minutes inside a 15–45 minute search, plus matched events displaying unknown duration. A captured public event response established that KRG declares `durationInSeconds: 2700` as well as subgroup lap counts. The filter used 45 fixed minutes; display independently calculated a lap-based duration. Description/name/subgroup fallbacks were also omitted by compact display.

- Added `event_filtering::estimate_event()` / `EventEstimate`; filtering, compact rows, verbose duration formatting, and subgroup estimate annotations now share this resolver.
- Preserved the category speed/difficulty formulas and fallback priority. The late subgroup fallback now selects one subgroup consistently instead of matching any eligible subgroup. Display-only surface/lead-in calculations were removed.
- Fixed duration does not erase explicit distance or subgroup-lap metadata; metadata never overrides fixed minutes.
- Added seven focused regression tests: before the fix six failed and one passed; after the fix all seven pass. Tests cover 45-minute fixed-event precedence, exclusion of a genuine 53-minute duration from 30 ± 15, fixed-event metadata, description/name/subgroup fallbacks, explicit-distance and lap/route-only lead-in rules, verbose text, and unknown rejection.
- Default suite after the final changes: **177 passed, 0 failed, 7 ignored**, using isolated application state and one test thread. Release build passed.
- Live release checks after importing the repository's routes into temporary state: 30 ± 15 returned 28 rows, all with displayed durations in range; 60 ± 15 returned 2 rows, both in range. KRG showed 45 minutes in compact and verbose output, retaining 30.4 km distance metadata.
- Focused accuracy regression ran before and after; both reported no mapped races. **Before MAE: unavailable; after MAE: unavailable.** No calibration claim is made.
- A standalone same-family reviewer identified lost fixed-event metadata and test gaps; these were addressed and the full default suite rerun. This was context-isolated, not cross-family verification.
- A constants import already removed from production scope in `src/main.rs` broke its test build. Added the two required imports within the test module without restoring the production wildcard.
- Targeted rustfmt and `git diff --check` passed. Existing OCR failures and broader repository lint/format issues were not rerun because their source was unchanged.
- Remaining limitations: multi-stage descriptions still use the first matching distance, subgroup eligibility matching is unchanged, and missing route/elevation data limits accuracy. Consistent estimates are not evidence of calibrated accuracy.

Evidence: `/tmp/zwift-duration-fix-FYxZqi/` (captured public events, red/green regressions, suite/build/accuracy logs, and live compact/verbose output). Credentials and the normal user database were not used or modified.

## OCR follow-up after approved libclang/model-data installation

The owner subsequently approved `libclang-dev` and `tesseract-ocr-eng`. Installation succeeded (14 new packages including dependencies/recommendations, no upgrades/removals); Tesseract reports `eng` and `osd`. This supersedes the missing-libclang and missing-English-data blockers below.

- `cargo test --locked --offline --all-features --no-run`: **passed**. All Rust feature combinations selected by `--all-features` compile together, including OCR.
- `cargo test --locked --offline --all-features --no-fail-fast -- --test-threads=1`: **232 passed, 2 failed, 16 ignored**, with fresh isolated HOME/XDG directories, a cleared environment, and snapshot/proptest persistence disabled. These counts include the core tests, not 232 additional tests.
- `fuzz_tests::fuzz_leaderboard_numeric_combinations` failed at `tests/ocr_fuzz_tests.rs:208`: a zero-space combination of delta `+00:01` and distance rounded to `451.3 km` was parsed as `1451.3 km`. The minimized input used distance `451.32161436149653`, no power, and zero joining spaces. This is a real numeric-boundary parsing failure; no parser change made.
- `snapshot_tests::snapshot_is_likely_name_variations` failed at `tests/ocr_snapshot_tests.rs:75`: ordinary names return true while the stored snapshot expects false in nine cases. The existing `test_is_likely_name` unit test explicitly expects several of those names to return true. The snapshot and unit-test contracts conflict; the snapshot was not regenerated.
- Image/model tests were then explicitly run with `--ignored --skip generate_golden_data` across `ocr_tests`, `ocr_snapshot_tests`, and `ocr_integration_tests`: **2 passed, 6 failed**. All six failures were missing OCR detection-model errors, not demonstrated recognition mismatches.
- The two nominal passes (`ocrs_tests::test_create_engine` and `ocrs_tests::test_extract_text_from_region`) caught and printed missing-model errors, then returned success. They do not verify that OCR works; the second also contains an always-true assertion. Treat them as test-quality defects, not successful model validation.
- The model download endpoint given in `src/ocr_ocrs.rs`, `https://ocrs-models.s3-accelerate.amazonaws.com/text-detection.rten`, returned HTTP 403 during an attempted download into isolated temporary storage. The recognition download was not attempted after that failure. No substitute source was guessed and no model files were installed.
- The ignored image snapshots and OCR golden baseline are also absent from the checkout. Missing models prevented execution from reaching those comparisons; no baselines were generated to conceal this gap.

Logs: `/tmp/zwift-ocr-tests-akazOb/` (`install.log`, `build.log`, `tests.log`, `ignored-ocr-tests.log`). No application/test source, credentials, normal database, or expected outputs were changed in this installation/testing follow-up. Python OCR and optional mutation/coverage workflows remain unexecuted for the previously recorded reasons. `git diff --check` passed.

## Follow-up after approved native package installation

The owner approved installing `pkg-config`, `libssl-dev`, `libtesseract-dev`, and `libleptonica-dev`. APT installed those and their dependencies (50 new packages, 9 dependency-related upgrades, no removals). Detected library versions: OpenSSL 3.5.7, Tesseract 5.5.0, Leptonica 1.84.1. The initial pre-installation results below are retained as history and are superseded where noted here.

- **Default suite passed:** `cargo test --locked --offline --no-fail-fast -- --test-threads=1`: 170 passed, 0 failed, 7 ignored. Includes 86 library tests, 36 main-binary tests, 6 API-mock tests, 6 config tests, 12 CLI integration tests, 20 property tests, and 4 snapshot tests. Golden tests remain explicitly ignored; OCR targets have zero tests without the feature.
- Tests ran with a cleared environment (`env -i`), temporary HOME/XDG data/config/cache directories, original Cargo/Rustup toolchain directories, `INSTA_UPDATE=no`, and disabled proptest failure persistence. Single-threaded execution avoids known process-global cwd/environment test interference; concurrent-test safety was not established. CLI tests made public API requests.
- **Explicit ignored live-API test failed:** `test_real_zwift_api_connection` received HTTP 404 from its stale `/api/public/events` URL. Production uses `/api/public/events/upcoming`; this test does not call the production fetch function.
- **Actual CLI smoke run exited successfully:** fetched 200 events and displayed 29 race matches for score 195, duration 30, tolerance 15. All 29 displayed unknown distance/elevation/duration in the isolated state. This verifies fetching and rendering, not useful duration predictions for those events; investigate route-data coverage and filtering/display consistency.
- **Accuracy remains unverified:** no normal race database exists on this machine. An explicit accuracy regression on a fresh isolated database returned success with “No races with mapped routes found for regression testing.” That is an empty-data pass, not an MAE result. The suite's shared temporary database had only two test-created rows.
- **All-feature build now fails at a different prerequisite:** `leptonica-sys`/bindgen cannot find `libclang`. `libclang-dev` is not installed. English Tesseract traineddata (`tesseract-ocr-eng`) is also absent. No OCR tests have executed.
- **Clippy now executes and fails:** with `--all-targets -- -D warnings`, the library and library-test compilations reported 243 and 393 lint errors respectively (overlapping reports, not 636 distinct defects). Examples include pedantic literal/style warnings, missing API error documentation, and unused/dead code. No bulk lint fixes applied.
- Credentials and normal user state were not used or changed. No formula, source behavior, or expected test output was changed during this follow-up.

Follow-up logs: `/tmp/zwift-runtime-DjEPrp/`. Next approval needed for `libclang-dev` and `tesseract-ocr-eng` to continue OCR testing. Python OCR dependencies/models and optional mutation/coverage tools remain separate limitations.

## Initial pre-installation scope

User requested testing before broader cleanup or credential changes, and removal of no-op placeholders. Base commit: `ad3e87c`. Existing unpublished documentation was preserved. No prediction formula, application schema, credentials, user database, system packages, or Rust toolchain components were changed. Cargo downloaded dependencies; build outputs were generated locally. No commit or publication was made.

## Executed checks

| Check | Result |
|---|---|
| Default Rust test build: `cargo test --locked --no-run` | Blocked in `openssl-sys`: missing `pkg-config` and OpenSSL development prerequisites. No Rust tests executed. |
| All-feature test build: `cargo test --locked --all-features --no-run` | Same OpenSSL blocker, after downloading uncached dependencies. OCR native dependencies also absent. |
| `cargo clippy --locked --offline --all-targets -- -D warnings` | Blocked by OpenSSL before application lint results. |
| `cargo fmt --all -- --check` | Failed: initial output reported differences in 37 source/test files. No bulk formatting applied. `rustfmt` is installed; the earlier status claiming it was missing is superseded. |
| Python AST syntax, all tracked non-archive `.py` files | 25/25 passed. Invalid escape-sequence warning in `tools/ocr/leaderboard_extractor_v3.py:206`. No imports or OCR processing performed. |
| Bash syntax, all tracked non-archive `.sh` files | 26/26 passed. |
| Node syntax, all tracked non-archive `.js` files | 5/5 passed. |
| ShellCheck on those 26 shell scripts | 17 findings across 5 files: 6 warnings, 10 informational, 1 style. No error-level findings. |
| `bash tools/utils/test_api_limits.sh`, bounded by 60-second timeout | Completed, but not a trustworthy pass/fail test; see below. |

Initial offline builds also encountered uncached crates. Retrying online exposed the actual native build blocker; this is not a Rust test assertion failure.

### ShellCheck findings

- `tools/import/strava/strava_auth_secure.sh`: SC2086 and SC2162.
- `tools/import/strava/strava_auth.sh`: SC2086 and SC2162.
- `tools/import/routes/fetch_whatsonzwift_route_data.sh`: SC2155.
- `tools/import/routes/fetch_event_descriptions.sh`: SC2181.
- `tools/debug/debug_event_tags.sh`: SC2034.

### Public API diagnostic

The script reported array lengths 50, 100, 150, and 200 for those requested limits. For limits 250–500 it printed “1 events returned.” Independent status/type checks established:

- `limit=200`: HTTP 200, JSON array of 200 events.
- `limit=250`: HTTP 400, JSON object with a `message` key.

The script uses `jq length` without checking HTTP status or response type, so an error object is mislabeled as an event count. Its exit status was zero. Offset requests returned arrays of 100 according to the script; whether offsets actually change the returned event identities was not tested. These observations do not establish the API time horizon.

## Removed no-op infrastructure

- `benches/performance.rs` and `benches/ocr_benchmarks.rs`: both consisted of comments and empty `main()` functions.
- Corresponding Cargo benchmark declarations, unused Criterion dependency, and exclusively unused lockfile packages.
- `tests/uom_integration_tests.rs` and `tests/behavioral/{mod.rs,ab_testing.rs}`: inactive scaffolding behind undeclared `uom-migration`, referring to absent implementations. The behavioral directory was not wired into a test root either. One proposed comparison simply called the same database method on both sides.

The functioning A/B utility in `src/ab_testing.rs`, golden/property tests, and separate `zwift_ocr_benchmark` executable were retained. Testing references were corrected so empty benchmarks and inactive migration tests are not presented as coverage.

## Not executed / limits

- Rust unit, API-mock, CLI, property, snapshot, golden, accuracy-regression, and OCR test cases: build blocked. Repeating each target separately cannot bypass the shared native dependency.
- Accuracy MAE: not measured; no formula change was made.
- Ignored baseline generators: not run because they write new expectations rather than validate existing behavior.
- Ignored database validators and `tools/utils/validate_test_data.sh`: Rust build blocked; wrapper also suppresses test failures and can prompt to regenerate baselines. Future runs should use isolated database copies and direct test commands.
- Mutation/coverage tools: `cargo-mutants`, nextest, tarpaulin, and llvm-cov not available; mutation wrapper uses in-place source edits. Native build failure would block these regardless.
- Python OCR smoke/performance/comparison tools: Python 3.12 dependency probe found no PaddleOCR, OpenCV, Pillow, or pytest. The OCR task named `test` is an extractor smoke run, not an assertion suite. No large ML packages/models were installed.
- Garmin execution: `garminconnect` and `rich` absent from the probed interpreter; no authentication attempted.
- PowerShell screen recorder: not a test; not executed.
- SQL/import/debug workflows that mutate data, credentials, or external systems were not treated as tests.

## Final cleanup validation

- `cargo metadata --locked --offline --no-deps --format-version 1`: passed; 1 library, 8 binaries, 13 integration-test targets, no benchmark targets. Target discovery is not test execution.
- Lockfile comparison: 16 packages removed, none added; all retained dependency package blocks unchanged. Only the root package dependency list changed.
- `git diff --check`: passed.
- Final formatting check: still failed, now reporting 36 files after removal of the inactive UOM test file. No source formatting was changed.

## Evidence and next step

Raw logs from this local session: `/tmp/zwift-tests-xKD4HH/` (temporary, not a durable project dependency). Durable results are summarized above.

Install approved native prerequisites before attempting runtime validation: `pkg-config`, `libssl-dev`, and for OCR `libtesseract-dev`/`libleptonica-dev` plus an available Tesseract language/model setup. Installation requires owner approval and may require elevated privileges. Keep credentials unchanged until functional validation is complete.
