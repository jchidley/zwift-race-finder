# Tasks

`TASKS.md` is the canonical unresolved queue. Items remain distinct and open until current evidence verifies satisfaction or an owner decision marks them superseded.

## Immediate plan — recent-performance relevance

Preserve **`v0.3.0`** as the usable active baseline while continuing bounded
research. Owner acceptance for practical choices does not cancel the improvement
objective. [Status](STATUS.md) owns current evidence and blockers; the
[baseline/experiment report](docs/project-history/RECENCY_BASELINE_2026-11-03.md)
owns this completed comparison. No production recency change is adopted.

- [x] Reconcile the four supplied finishes with stored/source history and recordings.
  Two already exist; preserve exact source seconds and the corroborated event date.
  The other two remain absent. Detailed reconciliation stays private.
- [x] Preserve a versioned baseline, frozen data, inputs and performance evidence.
  Local annotated `v0.3.0` identifies the active model; private evaluation artifacts
  remain outside Git. This is not a push or clean-release qualification.
- [x] Review the prior last-five experiment and predeclare one stronger-recency
  candidate for the current personal estimator: R1 uses a 90-day half-life and
  requires three comparable same-route observations within 180 days before that
  branch takes priority. No global hard cutoff or relaxed minimum support.
- [x] Compare chronologically on identical frozen data: duration MAE, coverage and
  four duration windows. Independent arithmetic matches all 364 Rust cases;
  The supporting historical screen fails; the main recent-example comparison
  improves 2:31 → 1:51 across three non-EVR cases. R1 is not promoted. Do not retune on the same
  inspected outcomes or describe them as an independent holdout.
- [ ] **Qualify missing later observations and refresh supported history:** obtain
  source-qualified finish/event identity and verify the EVR route; back up,
  preview and reconcile a fresh supported export. Do not bulk-import FIT/Strava
  activities, infer provenance or add the existing results twice.
- [ ] **Before another model candidate:** identify a specific selection failure
  or adaptation limitation beyond this small/mixed recency result, preferably
  with new source-qualified outcomes. Recent support is sparse; a separate
  current-form adjustment remains open but needs an explicit mechanism/protocol.
  Keep R1 available for comparison; useful baseline acceptance does not cancel research.
  Preserve cross-course prediction, source/rider boundaries and earlier-only dates.
- [ ] **Predictive qualification remains open:** use new evaluation evidence before
  claiming general improvement. Keep functional use acceptance, retrospective
  comparisons and release gates separate. Whole-minute output is not ±1-minute
  precision. Existing inspected comparisons are not fresh validation data.
- [ ] **Release-gate preparation remains incomplete:** repository-wide formatting
  and Clippy failed previously (165 → 160 library errors). Do not weaken gates;
  no unrelated cleanup, push or deployment is authorized by the baseline tag.

## Separate proposals — not automatically approved

- **E1:** Explicit populated-data accuracy qualification that cannot pass with
  zero usable comparisons; keep ordinary isolated tests independent of private data.
- **E2:** Config tests in isolated subprocesses instead of process-global cwd mutation.
- **M1:** Pass one immutable estimate through filtering/rendering; defer until justified.
- **CLI precedence:** Explicit duration/tolerance/days equal to built-in defaults
  can be replaced by config preferences. This is documented, not repaired by D1.
- **Metric decision:** Resolve the older below-20% “mean absolute error” wording;
  MAE in minutes and MAPE are different. Do not silently adopt the test's 30% MAPE guard.

## Retained test-first validation and cleanup backlog

The items below retain their individual dispositions. They are not the execution
order for the recency investigation; OCR and general cleanup are deferred unless needed for a concrete
finder defect. Historical test counts are dated evidence, not current passes.


- [x] Install approved initial native prerequisites and run default Rust tests with isolated state: 170 passed, 7 ignored. See [test audit](docs/project-history/TEST_AUDIT_2026-10-29.md).
- [x] Install approved `libclang-dev` and `tesseract-ocr-eng`; all-feature build passes and tests run (232 passed, 2 failed, 16 ignored).
- [ ] Fix OCR leaderboard parsing when time delta and distance have no separating space; preserve the minimized failing case recorded in the test audit.
- [ ] Reconcile the name-detection snapshot with explicit unit-test expectations; do not blindly regenerate snapshots.
- [ ] Establish a verified OCR model source: the code-provided detection-model URL returns HTTP 403. Six explicit image tests fail due to missing models; image snapshots/golden data are also absent.
- [ ] Remove or replace the two ignored OCR engine tests that swallow all errors and return success; retain meaningful extraction smoke tests and add real result assertions where appropriate.
- [ ] Correct the ignored live-API test's stale endpoint; explicit execution returned HTTP 404 while the actual CLI fetched events successfully.
- [ ] Make the accuracy regression distinguish missing data from measured success (E1, separate approval). Recorded event distance and precise seconds are already implemented; the corrected six-score comparison reported 2.093 min MAE on 2026-11-02. Unknown-score rows remain valid personal history but cannot qualify the category comparison. Zero usable comparisons still pass.
- [x] Fix filter/display duration disagreement with one shared estimate; regression tests and live 30±15 / 60±15 checks pass. KRG declares 45 fixed minutes, which now displays correctly.
- [x] Distinguish unknown elevation from known zero; calculate lap/lead-in totals; import pinned route-ID-based metadata with provenance and preservation tests.
- [x] Back up the owner's normal database and apply the verified client export with `import_client_routes` (approved/applied 2026-10-30). Verified all 386 client records, unchanged non-route data/history, integrity, no foreign-key violations, and repeat-import idempotence. Private export: `~/.local/share/zwift-race-finder/client-routes-0862ca71d9b4.json`.
- [x] Client import reconciles all ten seed IDs against client metadata; the incorrect hardcoded Bell Lap fallback is removed. History is preserved and community imports cannot downgrade client records.
- [x] Remove misleading automatic route seeds; fresh databases are empty until explicit client import, with a CLI warning and lifecycle regression coverage.
- [x] Establish client-cache → extracted-client precedence, fractional climbing, availability metadata, and per-route provenance; verified the selected 386-route client export. The former community fallback is retired.
- [x] Retire bundled legacy/community route data, importers, website discovery, and guessed mapping/alias recipes; preserve existing database/history. See [retirement audit](docs/project-history/CLIENT_ONLY_ROUTES_2026-10-30.md).
- [ ] Verify active client-cache selection and refresh source coverage when the installed game changes. Rechecked archive SHA and installed version 1.124.0 (167761); installed manifest provided no active-cache link. Do not infer freshness from file time or public availability from internal definitions.
- [ ] Select the relevant stage's distance from multi-stage event descriptions rather than the first distance in the text; current shared resolver deliberately preserves the existing parser.
- [x] Resolve Racing Score subgroups by numeric range and traditional groups by exact label/category token; prefer subgroup route/distance, ignore zero laps, and share fixed/calculated geometry. Synthetic regressions and captured live-feed replay pass; touching numeric endpoints retain API order, not a proven unique eligibility rule. See [diagnosis](docs/project-history/PERSONAL_ESTIMATOR_DIAGNOSIS_2026-11-02.md).
- [x] Implement optional-score, precise, source-qualified performance history and a non-writing preview; migrate/import all approved observations with backed-up reconciliation, conflict/rollback tests, preserved six confirmations and repeat no-op verification. See [history audit](docs/project-history/HISTORY_MODEL_2026-10-30.md).
- [x] Implement the explicitly requested live history-based estimator: score selects subgroup; earlier same-route pace or personal distance/ascent fit supplies duration. Subsequent personal-only policy exposes unknown reasons instead of category fallback. Shipped-code chronological replay and isolated tests pass. [Implementation](docs/project-history/LIVE_PERSONAL_ESTIMATOR_2026-11-02.md).
- [ ] Establish improved personal prediction/selection accuracy beyond the mixed already-inspected historical results. Live integration is experimental, not evidence that this accuracy objective is satisfied.
- [ ] Resolve current Clippy failures; do not treat overlapping library/library-test lint totals as distinct defects.
- [ ] Resolve existing Rust formatting failures and ShellCheck findings without conflating them with test assertion failures.
- [ ] Fix `tools/utils/test_api_limits.sh` to check HTTP status and JSON type before reporting counts; the current diagnostic mislabels an HTTP 400 error as one event.
- [ ] Establish supported Python OCR dependencies/models and execute its smoke/comparison checks; distinguish these from assertion-based tests.
- [ ] Decide whether to implement real performance baselines later. Empty benchmark targets and inactive UOM migration scaffolding have been removed, not replaced by new coverage.
- [ ] Defer credential storage/rotation and personal-data decisions until functional testing, as requested by the owner. Do not publish the current credential-bearing artifacts.

## Parked documentation review

- [x] Publish the approved v0.2.0 checkpoint, including the documentation rationalisation, as `3e13a78`; earlier personal activity exports are untracked and privately backed up.
- [ ] Review `docs/howto/RETURN_TO_RACING.md`.
  - Re-check the official Zwift Racing Score FAQ and calculation page if their 13 September 2026 verification is no longer fresh enough for publication.
  - Decide whether to retain the practical 20–30 minute return-ride suggestion with a representative five-minute hard effort or limit the guide to official wording.
  - Keep the official eligibility minimum separate from optional advice for better initial seeding.
  - Keep historical requirements labelled as historical and retain primary links for future re-checks.
  - Refresh the verification date if source wording changes.
- [ ] Complete documentation acceptance after the test-first audit. Native prerequisite approval is tracked above; `rustfmt` is already installed but formatting fails. Installation, privilege elevation, commit, and publication are not implied.

## Deferred duration research and optional telemetry

The 31 October FIT/Sauce session reported aligned recordings and grouped/solo
segments; this is measurement evidence, not a validated drafting forecast. The
requested open-ended recorder and stream-stop diagnostics need reconciliation
with their private implementation before claiming completion. They are deferred,
not prerequisites for useful race selection; do not ask for new recordings by default.
Detailed pacing/strategy simulation belongs in the separate coaching project.

- [ ] Use solo rather than draft-inclusive speeds for time trials.
- [ ] Assess a race-field-size factor for draft consistency and variance.
- [ ] Evaluate gradient-profile data instead of only elevation gain per kilometre.
- [x] Count event lead-in climbing/distance once for lap/route-only estimates; distinguish unknown lead-in climbing and label proportional distance totals.
- [ ] Obtain elevation profiles for more accurate partial-route/event-distance estimates.

## Route and event data

- [ ] Import verified mappings for remaining event series such as EVO CC, Sydkysten, and Tofu Tornado. Evidence: `archive/consolidated-originals/ROUTE_MAPPING_RESEARCH.md`.
- [x] Superseded community enrichment with client-only metadata: fresh isolated import now contains exactly 386 client records, no seeds. Unresolved coverage remains explicit.
- [ ] Parse lap counts from event descriptions where subgroup data is absent.
- [x] Retire completion marking/filtering and description-analysis/discovery commands at owner request. Also remove completion progress/indicators and manual result recording. Preserve existing stored data; history ingestion uses the source-qualified importer.
- [ ] Evaluate undocumented event tags for useful filters. Evidence: `archive/consolidated-originals/ZWIFTHACKS_TECHNIQUES.md`.

## Coaching project handoff

Coaching, pacing experiments, and the deferred netbrain/zwift multi-instance evaluation now belong to the independent sibling `../zwift-race-coach` project and its `TASKS.md`. Route-data collection/import and production event-duration estimation remain here.

## Operational limitations

- [ ] Add API-region failover or verify that the single `us-or-rly101` endpoint is sufficient.
- [x] Remove unused physical-stat settings/accessors/updater and fresh table creation; preserve existing stored values.
- [ ] Isolate remaining CLI integration tests from the live API. Subprocess HOME/XDG state is now isolated, and adapter tests use typed loopback mocks.

## Code-truth decisions requiring resolution

These remain separate open questions from `docs/code-truth/DECISIONS.md`:

- [x] Enable SQLite foreign-key enforcement on application database connections; migration and history import verify constraints, and normal post-import `foreign_key_check` is clean.
- [ ] Revalidate the assumed ~200-event/~12-hour Zwift API limit when API behaviour is next reviewed.
- [x] Remove unused physical-stat defaults and environment overrides; preserve existing legacy database data.
- [ ] Assess and remove or contain the `Box::leak` lifetime workaround in `src/estimation.rs` before reusing this code in a long-lived process.
- [ ] Choose the correct support destination for the unrelated `anthropics/claude-code` issue URL in `src/main.rs`.

## Untriaged retained requirements and recommendations

The exact legacy items remain in their original numbered/listed form; this section does not combine, reprioritise, or declare them obsolete.

- [ ] Reconcile **every numbered requirement, future enhancement, KPI, immediate priority, and known issue** in `docs/project-history/REQUIREMENTS_2025.md` against current code and an owner decision. Until then, each item whose satisfaction is not directly verified remains individually unresolved in that record.
- [ ] Reassess every unresolved finding and recommendation in `docs/project-history/SECURITY_AUDIT_2025.md` before making a current security claim. Its obsolete commands must not be executed; the record retains exact historical context.
- [ ] Reassess every stated gap and recommendation in `docs/project-history/UOM_INTEGRATION_TEST_COVERAGE_2025.md` if UOM or integration-test work resumes. Current test documentation does not by itself verify that each historical acceptance criterion was satisfied.

The Pi release audit reports no compatibility follow-up and therefore adds no open Pi task. Its evidence remains at `docs/reference/PI_RELEASE_AUDIT.md`.

## Practical duration estimates — 2026-11-02

- [ ] Before adopting the **disabled profile-based candidate**, resolve event-profile
  composition (lead-in identity, physical scaling and finish cut) and short/long-course
  fitting tradeoffs. This does not block the already-active distance/ascent and
  same-route personal estimator. The comparable-length candidate remains disabled; see
  [diagnosis](docs/project-history/PERSONAL_ESTIMATOR_DIAGNOSIS_2026-11-02.md).
- [ ] Strengthen the production comparison with source-qualified historical starting
  scores where available. They are not required for personal fitting; never infer
  scores for existing records.
- [ ] Reassess calibration only with new evidence: the last-five pace adjustment
  failed the dated/known-score comparison; do not tune another multiplier on the
  same holdout and call it independent validation.
- Whole-minute rounding and the recorded-distance/precise-time regression correction
  are implemented. See [evaluation](docs/project-history/PRACTICAL_ESTIMATOR_2026-11-02.md).
