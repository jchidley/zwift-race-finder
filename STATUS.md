# Project Status

`STATUS.md` is the canonical current-work record. Durable facts belong in `docs/reference/`; unresolved work belongs in `TASKS.md`.

## Current focus

**Personal-only duration policy implemented — 2026-11-03.** P1/P2 remain
complete. Owner approved history reconciliation, category-fallback removal and
release-gate preparation. The finder now uses declared time or supported personal
same-route/cross-course estimates; unavailable estimates do not enter duration
matches. Score selects geometry only. Personal fitting parameters are unchanged.

**History refresh is blocked on a fresh supported export.** Only the 30 October
export was found in the application's private history-import directory. No later
observation was manufactured or imported. Follow the
[measured-history procedure](docs/howto/ZWIFTPOWER_EXPORT_STEPS.md), then preview
and reconcile a supplied private export before applying to backed-up normal state.

**Release gates remain blocked by pre-existing repository-wide debt.** Baseline
Clippy reported 165 library errors; after this bounded correction it reports 160.
Formatting initially failed in 43 files, including optional OCR. Changed Rust files
are formatted; no global suppressions, lint-policy weakening or unrelated
repository-wide formatting was applied. This is not a clean release checkpoint.

### Personal-only policy validation

- Final isolated default suite passed: **195 tests, 7 ignored**.
- Focused tests cover personal cross-course/same-route estimation, fixed-time
  priority, unknown reasons, no unsupported duration matches, geometry and display
  agreement. An additional binary regression checks unknown omission counts and
  excludes ineligible subgroups from that count.
- Retained legacy formula regression on the same populated copy: **2.093 → 2.093
  minutes MAE**, six score-known observations; not personal accuracy qualification.
- Offline locked release build passed. An isolated copy of normal history returned
  **19 personal-history matches in 40–50 minutes**. A fresh empty state disclosed
  **30 unknown-duration omissions**, returned no matches, and substituted no
  category-speed durations. Live requests are separate observations, not a fixed
  event snapshot or predictive-accuracy result.
- Private logs and copied data: `/tmp/zrf-personal-only-VFDrQaqf/`; no normal database import,
  commit or publication. Fresh-context same-family review is separate from
  cross-family verification. Review findings were corrected: unknown/ineligible
  exclusions are separate from known out-of-window estimates, and new resolver
  documentation lints are fixed. Final release rebuild and scoped Rust formatting
  passed; all 126 checked local documentation link paths exist (anchor semantics
  were not exhaustively checked).

### Prior P1/P2 checkpoint

The following acceptance results describe the earlier, unchanged-model run,
including its then-active category fallback; the personal-only policy above
supersedes that fallback, not the historical evidence.

The product flow is **score → relevant subgroup/course → personal estimate from
actual event geometry and earlier measured performances → time-window filter**.
Recent/comparable performance matters to the owner; three same-route observations,
twelve global observations and a one-year half-life are implementation choices,
not permanently adopted requirements. Whole-minute presentation is not a promise
of ±1-minute accuracy.

| State | Work |
|---|---|
| Delivered locally | Client-route and source-qualified history foundation, active personal estimator, simplified CLI, README Quick Start, D1 standards documentation |
| Completed acceptance | P1 reconciled this record and `TASKS.md`; P2 checked the executable, estimate basis, recent/older influence, fallback and latest-race coverage |
| Unproven outcome | Improved personal prediction and time-window selection; previous holdouts have already been inspected |
| Separate, unapproved proposals | E1 explicit populated-data accuracy gate; E2 config-test subprocess isolation; M1 once-per-event immutable estimate |
| Deferred finder work | Detailed profiles/physics, drafting forecasts, recorder improvements and OCR; none is a prerequisite for the active finder |
| Separate project | Coaching, pacing strategy and simulation in `zwift-race-coach` |

D1 changed documentation only; its static link/syntax checks are not application
qualification. See [adoption scope](docs/code-truth/DECISIONS.md#standards-adoption--documentation-only).
The FIT/Sauce work remains prior recording/alignment evidence, not a deployed
drafting predictor. Reuse it only to investigate a concrete discrepancy; no new
recording is required by this checkpoint.

### P2 acceptance evidence

| Case / expected behaviour | Observed result | Evidence / disposition |
|---|---|---|
| Known course: earlier same-route history drives estimate and output | Synthetic measured history produced 60 minutes; filter and both presentation modes agreed, aliases resolved, changing score alone did not change personal pace | Existing personal integration test passed; supports the contract, not real-world accuracy |
| Unseen course: use personal distance/ascent rather than require historical score | Synthetic varying-terrain history produced the independently specified 43.2-minute fit; conflicting historical laps did not replace reported event distance | Existing adapter/integration test passed; flat unseen-route probe also selected the global fit |
| Changing form: newer observations carry greater weight | Actual Rust probe: 12 year-old 60-minute races + 3 recent 40-minute races predicted 60 minutes; 3 old + 3 recent predicted 40. Recent weight share in the first case was 33.5% | Weighting works as implemented, but can be dominated by older volume. No half-life/threshold tuning or adaptation-superiority claim |
| Insufficient support: expose fallback rather than fabricate personal evidence | Empty history and missing geometry produced labelled fallback; two same-route observations alone returned insufficient support (2/12). Fixed duration bypassed broken history | Existing tests plus pure probe passed. Current support policy is visible, not newly endorsed as optimal |
| Actual command: subgroup/geometry, duration, filter and output agree | Release short-option compact and long-option verbose searches each returned 19 personal-history matches within 40–50 minutes. All ordered displayed names/distances/durations agreed | Separate live API requests, not identical captured payloads. Subgroup/lead-in contracts additionally checked by 17 synthetic consistency tests; no immutable-snapshot claim |
| Latest race: establish stored coverage without importing | Read-only backup: 194 history rows, latest date 2026-10-29, no 2026-10-31 observation | Later supplied race is absent from checked normal history. No import or fabricated provenance |
| Executable corresponds to current build | Offline locked release build passed; existing executable SHA-256 remained `4e22a3387ee2344d414b8ad168fb0d9ffe035d4be043b6309c5ee54b3b8397be` | No stale-binary discrepancy found; `--help` succeeded without creating a DB |
| Completion gates remain honest | 192 default tests passed, 7 ignored; format exit 1, Clippy exit 101 | Not a clean all-gates pass. No suppressions, auto-formatting or baseline changes |

Focused validation ran first: 17 duration-consistency, 6 elevation-consistency,
1 personal integration and 9 personal unit tests. The full default suite then ran
serially with fresh HOME/XDG, preserved Cargo/Rustup homes and excluded credentials;
some integration cases call the live API. The empty-state accuracy test in that
suite provides **no populated accuracy qualification**. No formula was changed,
so no new before/after personal or category accuracy claim is made.

On the copied history, a descriptive source/type/positive-measurement/date screen
found 182 ordinary candidates, six from the last 90 days, supplying 16.9% of base
recency weight. This is not target-specific fitting influence: same-route support
and the global feature-weighted loss also matter. The missing later race and
sparse recent coverage justify reconciling data before tuning the model.

Normal database checksum and application/test/Cargo inputs were preserved.
Private logs, isolated states, copy and the external Rust probe remain under
`/tmp/zrf-acceptance-20261103-NdVICB/` (temporary evidence, outside Git).
Key logs: `focused.log`, `pure.log`, `suite.log`, `build.log`, `probe.log`,
`cli-comparison.json`, `fmt.log`, `clippy.log`. No private rows or exports were
added to the repository; no normal-state imports, commits or publication occurred.

## Historical implementation and investigation checkpoints

The following reports describe their dated stages, not today's test results.
Statements that the category estimator remained active are superseded by the
later owner-authorized personal integration.

**Live personal estimates implemented (2026-11-02, explicit owner request).**
Score selects subgroup geometry; supported ordinary races now use earlier
same-route pace or a personal distance/ascent fit for duration and filtering.
Compact output labels history estimates experimental and exposes category fallback
reasons; verbose output adds method, sample count and descriptive training error.
Fixed durations remain authoritative. Fresh-state tests: **192 passed, 7 ignored**;
release CLI rebuilt and live40–50-minute search returned26 history-based matches.
Actual shipped Rust chronological replay uses history on170/182 ordinary results;
all170 predictions match independent arithmetic. Six-score personal MAE2.159 min
versus category2.093 min does **not** establish improved accuracy. Normal DB unchanged.
See [live implementation and qualification](docs/project-history/LIVE_PERSONAL_ESTIMATOR_2026-11-02.md).

Earlier stages below remain historical evidence; their statements that no personal
model is enabled are superseded by this explicitly requested implementation.


Failure-led follow-up (2026-11-02): implemented actual event-resolution fixes:
numeric Racing Score subgroup selection, subgroup route identity, positive
subgroup distance before positive laps, zero-lap rejection, shared fixed/calculated
geometry and subgroup API seconds. Captured live-feed replay verifies filter/display
agreement and exposes five old lead-in-only calculations (for example 47.6 km was
calculated as 0 minutes; corrected geometry estimates 90 minutes at score195).
Fresh-state default suite: **182 passed, 7 ignored**. Review-found blank-range and
verbose route-metadata regressions are corrected; eight captured traditional
races/time trials remain estimable and four subgroup-specific metadata routes agree. Accuracy formula regression
is unchanged: **2.093 → 2.093 min MAE**, n=6. Release CLI rebuilt. A new personal
profile model fitted to comparable-length ordinary races recovers three short-window
misses but adds false inclusions and misses longer suitable races; it is not enabled.
Source payload format tags were recovered; exported whole-kilometre distances must
not be described as exact historical geometry. See [diagnosis and implementation](docs/project-history/PERSONAL_ESTIMATOR_DIAGNOSIS_2026-11-02.md).


Window-selection follow-up (2026-11-02): recovered the previous course-model
experiment from Pi session `01a18e10-2eee-7650-8e82-35dda9b7ad8d` and tested
strictly earlier-date prediction-error probabilities for duration-window inclusion.
The fixed 50% rule did not improve the profile model's 40–50-minute selection:
7 suitable shown, 1 unsuitable shown, 4 suitable missed on 38 later observations.
At the same list size as the neutral reference, profile probability ranking gave
8/4/3 versus neutral point selection's 10/2/1. The neutral reference is not a
historical-score production reconstruction. Six genuinely score-comparable rows
show no filtering gain. Production remains unchanged; this is exploratory reuse
of an already-inspected holdout, not independent validation. Private protocol,
code, tests and results are under
`~/.local/share/zwift-race-finder/investigations/20261102-window-selection/`.
All 12 synthetic tests pass; a same-family context-isolated review reproduced all
288 count dictionaries and checked source chronology. Database checksum is unchanged.
The six-score check compares production point selection, not production probability
calibration; there are too few earlier scored residuals to calibrate production.

Practical-estimator update (2026-11-02): calculated estimates round to the nearest
whole minute rather than truncate; filtering and presentation still share one
estimate. No category speeds or difficulty factors changed. The personal last-five
pace adjustment was evaluated chronologically and rejected: rounded MAE was 2.59
minutes versus the production formula's 2.09 on the six known-score comparisons.
All 194 observations were retained; no missing scores were inferred. The focused
regression now uses recorded event distances and precise seconds where available,
fixing its older route/name/whole-minute comparison. Fresh-state default tests
passed 173, with 7 ignored, and the release CLI rebuilt. Changes are local and
unpublished. See [the practical evaluation](docs/project-history/PRACTICAL_ESTIMATOR_2026-11-02.md).

Approved checkpoint **`3e13a78`** is pushed to `origin/main`. Subsequent obsolete-code cleanup is implemented locally and **not committed/pushed**. It removes unsafe legacy history importers, inert configuration/database APIs, dormant output and broken OCR wrappers; shares API/config/database through the library; and replaces misleading tests with real adapter and compatibility checks. See the [cleanup audit](docs/project-history/OBSOLETE_CODE_CLEANUP_2026-10-30.md) for validation and boundaries.

The v0.2.0 CLI has been simplified at owner request: removed completion marking/filtering, description analysis (including its standalone binary), and the retired discovery command, along with their dedicated code/tests. A subsequent cleanup also removes completion progress/indicators and manual result recording, including their database APIs and dedicated tests. Existing stored data remains intact; history ingestion uses the source-qualified importer, and shared event-description parsing still supports display/estimation. The leftover route-name guessing module, its test, obsolete discovery comments and stale coverage-helper entries are also removed. Unknown routes are recorded by ID and event name for client-data refresh, without guessed mapping annotations. At the cleanup checkpoint the default suite passed **170 tests, with 7 ignored**; release binaries were rebuilt. Earlier verification counts below describe their respective implementation stages.

The approved performance-history foundation is complete: 194 observations imported, six confirmed scores preserved, 188 unknown scores stored as NULL. Exact seconds, event geometry and source provenance are retained independently of eligibility. Production prediction is unchanged. At the history-import stage the default isolated suite passed **195 tests, 7 ignored**; the same six scored records retain **13.3% MAPE before/after**, with 188 unknown-score observations explicitly excluded only from that legacy check. See [history implementation audit](docs/project-history/HISTORY_MODEL_2026-10-30.md) and [design](docs/explanation/HISTORY_DATA_MODEL.md).

The authorized duration and elevation fixes are implemented and the release binaries rebuilt. Unknown elevation is distinct from known zero; lap totals count event lead-in climbing once; proportional distance totals are labelled approximate. Supported route ingestion is now client-only: selected installed-client cache, then extracted XML for missing IDs. Old bundled datasets, automatic seeds, community importers, website discovery, and guessed mapping recipes have been retired. The verified private export contains 386 routes and preserves fractional climbing and source hashes. Isolated live output resolves California Dreamin' Stage 4 at 21.2 km / 106 m / estimated 39 minutes. Client precision gives 249 m for 15 Downtown Dolphin laps, superseding the community-rounded 255 m. The owner approved and the client import was applied to the normal database on 2026-10-30 after a verified SQLite backup. All 386 client records and provenance were verified; all seven non-route tables, including race history, are unchanged. Default Rust testing passes; OCR retains its previously recorded failures. The identified obsolete-code cleanup is now complete; historical credential/data cleanup decisions remain deferred. See [the test audit](docs/project-history/TEST_AUDIT_2026-10-29.md) for executed checks, evidence, removals, and limits.

## Working tree

The checkout is on `main` at pushed checkpoint `3e13a78`. Subsequent cleanup, personal estimation and documentation changes are local and uncommitted; this is not a clean published snapshot. The checkpoint includes the prior documentation rationalisation and route-data cleanup:

- Removed two empty benchmark files and their Cargo declarations, unused Criterion dependency, and unused lockfile packages.
- Removed inactive UOM test scaffolding referring to an undeclared feature and absent implementations; retained the real A/B utility and its unit tests.
- Corrected test documentation to stop claiming benchmark/UOM coverage that did not exist.

The earlier documentation set remains preserved: concise agent instructions, removal of duplicate/pointer-only documents, current-state/task governance, code-truth map, current references and guides, historical 2025 records, returning-rider guide, and Pi audit/evidence. Its complete review is not finished; the repository audit found stale import paths and contradictory reference claims still requiring correction.

Duration presentation now uses the filter's estimate rather than its own formula; the category speed/difficulty formulas are unchanged. The late subgroup fallback consistently selects one subgroup. The code now migrates the route schema with elevation-knowledge flags and uses non-deleting UPSERTs. Credentials and Rust toolchain components were not changed. The normal database received the explicitly approved client route import after backup; non-route data was verified unchanged. The owner-approved native packages and their dependencies were installed in two steps (first: 50 new packages and 9 related upgrades; second: 14 new packages, no upgrades). Tests used temporary application state. No commit or publication was made.

## Earlier verification and recorded blockers

These are historical observations; do not treat their test counts or failure lists
as a fresh acceptance run. Current P2 results belong above. Formatting/lint debt,
OCR prerequisites and external-service limitations remain distinct concerns.

- **Earlier client-only retirement:** 182 Rust tests passed, 0 failed, 7 ignored with fresh isolated state; CLI/client importer release builds and all-feature test compilation pass. A real isolated import has exactly 386 routes/provenance records, integrity OK, and repeat preview zero writes. No normal DB writes or snapshot regeneration. [Retirement audit](docs/project-history/CLIENT_ONLY_ROUTES_2026-10-30.md) records removals, test-count changes, and cache-selection limits.

- Historical pre-retirement default Rust suite after client-primary changes passed with isolated state and one test thread: 196 passed, 0 failed, 7 ignored. New coverage includes unknown/zero migration, repeatability/history preservation, lap/lead-in totals, duration gradient inputs, route-ID bounds, and importer repair/preservation/idempotence. Snapshots were not regenerated. This does not establish concurrent-test safety.
- Historical pre-retirement checks: CLI, client importer, and community importer release builds passed; all-feature test compilation passes. PowerShell collector: 90 assertions passed; Python legacy-import protection: two tests passed. A fresh isolated client import writes 386 records, reconciling all ten seeds; a repeat changes none. Forced transaction failure preserves earlier rows/history. A same-family fresh-context read-only Rust review found no concrete regressions; it did not run tests or inspect the collector. Parent independently ran collector tests and verified the real export/import.
- Private export: `~/.local/share/zwift-race-finder/client-routes-0862ca71d9b4.json`. Cross-OS SHA-256 matches; plain console text transfer was rejected due to IBM850 recoding and replaced with byte-preserving transfer. Normal route DB was subsequently updated with approval: 63 → 387 routes, including 386 verified client records and one preserved unmatched route. Integrity and foreign-key checks pass; repeat preview reports zero changes.
- The explicitly enabled live-API test still has a stale `/api/public/events` URL (404). The production release CLI now passes live consistency checks: 28 rows within 15–45 minutes and 2 rows within 45–75 minutes, using imported route data in isolated state. KRG's declared fixed 45 minutes appears consistently in compact/verbose output.
- The accuracy regression ran before and after against isolated data and found no mapped race results; before/after MAE are unavailable. The user's normal database was not used.
- All-feature/OCR test build passes after installing libclang and English Tesseract data. Earlier all-feature execution, before the core fixes, reported 232 passed, 2 failed, 16 ignored (includes core tests); this stage rebuilt but did not rerun those known-failing OCR tests.
- OCR failures: leaderboard distance parsing absorbs a preceding time-delta digit; the name-detection snapshot contradicts current output and existing unit-test expectations.
- Explicit ignored OCR checks report 6 missing-model failures and 2 misleading passes that swallow model-load errors. The code-provided detection-model download URL returns HTTP 403; image baselines are absent too. No expected outputs were regenerated.
- Clippy now runs and fails on existing lint errors (243 library / 393 library-test reports, overlapping).
- `rustfmt` is installed. Formatting check runs but fails on existing source/test formatting; the previous missing-rustfmt diagnosis is superseded.
- Syntax checks passed for 25 Python, 26 Bash, and 5 JavaScript files. ShellCheck reported 17 findings across 5 scripts.
- The public API responded with 200 events for `limit=200`; `limit=250` returned HTTP 400. The diagnostic script misleadingly reports the error object as one event and exits zero.
- Python OCR dependencies are missing from the probed uv-managed interpreter. Mutation/coverage tools remain unavailable; Tesseract/Leptonica development libraries are installed.
- Earlier documentation link/audit checks are historical evidence, not proof that every current command or claim is valid.

## Approved history migration and full import — 2026-10-30

The initial six-result import is superseded by the supported nullable-score model and full private export import. All original row IDs, notes, creation timestamps and confirmed scores were preserved; precise observations and source identity were added. The other 188 observations were inserted without fabricated scores. All 194 match the source export, non-history tables are unchanged, integrity/foreign-key checks pass, and repeat import makes no changes. A private SQLite backup and detailed receipt are recorded in the [implementation audit](docs/project-history/HISTORY_MODEL_2026-10-30.md).

The supported workflow is [measured ZwiftPower history](docs/howto/ZWIFTPOWER_EXPORT_STEPS.md). At the history-import checkpoint the estimator was still the route/category formula. The later personal integration now consumes measured history; ingestion itself is not prediction calibration. Future work must use equivalent event geometry and chronological holdout evaluation without future leakage.

## Immediate next action

P2 is complete. Retain the active estimator. The single recommended next correction is a fresh source-qualified history reconciliation to include the later supplied race, with a backed-up preview and separately approved apply; do not insert a chat-derived row or tune the model first. This is a data-coverage correction, not a promise of better predictions. Existing formatting/Clippy debt still blocks a clean all-gates release checkpoint. Imports, commit/publication and E1/E2/M1 remain separately controlled; no further model investigation is implied.

The normal-state client import is complete. Backup: `~/.local/share/zwift-race-finder/backups/races-before-client-import-20261030T075234091278Z.db`. The rebuilt CLI now uses the imported client route metadata. The selected archive hash is unchanged and installed game version is 1.124.0 (167761), but the installed manifest did not establish an active-cache link. See `docs/howto/CLIENT_ROUTE_DATA.md` for future refreshes. Client imports replace conflicting legacy records without deleting history. Fresh databases no longer seed routes, and unsupported import/scrape paths are retired. Remaining core issues include active-cache/version selection, multi-stage description parsing for unresolved IDs, overlapping score-range ambiguity, incomplete route coverage, the stale live-API test, empty-data accuracy passes, and lint failures. OCR needs separate parsing/snapshot fixes and a trustworthy model source/baselines. Do not regenerate expected outputs merely to make tests pass. Preserve credentials and personal data; their disposition remains deferred.

## Coaching project handoff

The owner requested a separate `~/git/zwift-race-coach` project. Local instructions, scope, task queue, and documentation are prepared there. Ecosystem/multi-instance references and the historical simulation catalogue were moved there with navigation repaired; route collection/import, source provenance, and production race selection remain here. The owner approved the coaching project's GitHub skill preview: private `jchidley/zwift-race-coach` now exists with Actions disabled and matching README/MIT/Apache template files locally and remotely. Its remaining local docs have not been committed or pushed. Continue coaching development from a new Pi session at that canonical root; do not stage or publish this dirty checkout as part of its setup.

## Retained uncertainty and boundaries

- Earlier empty-data and 13.3% MAPE regressions are historical. The corrected recorded-distance/precise-seconds category comparison reported 2.093 min MAE on six scored rows on 2026-11-02. It is small and does not qualify personal-model superiority; do not mix cohorts or metrics.
- Returning-rider eligibility/seeding claims and optional practical advice still need editorial/source review before publication.
- Legacy requirements/security/UOM recommendations remain historical and untriaged; they are not current acceptance evidence.
- OCR, Garmin, simulation, import workflows, and reference-repository support boundaries still need decisions and documentation.
- Historical `archive/` files remain read-only. Existing unpublished work is preserved. Privileged installation, credential changes, history rewriting, commit, and publication require separate approval.
