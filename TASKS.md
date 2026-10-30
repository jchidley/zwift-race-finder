# Tasks

`TASKS.md` is the canonical unresolved queue. Items remain distinct and open until current evidence verifies satisfaction or an owner decision marks them superseded.

## Test-first validation and cleanup

- [x] Install approved initial native prerequisites and run default Rust tests with isolated state: 170 passed, 7 ignored. See [test audit](docs/project-history/TEST_AUDIT_2026-10-29.md).
- [x] Install approved `libclang-dev` and `tesseract-ocr-eng`; all-feature build passes and tests run (232 passed, 2 failed, 16 ignored).
- [ ] Fix OCR leaderboard parsing when time delta and distance have no separating space; preserve the minimized failing case recorded in the test audit.
- [ ] Reconcile the name-detection snapshot with explicit unit-test expectations; do not blindly regenerate snapshots.
- [ ] Establish a verified OCR model source: the code-provided detection-model URL returns HTTP 403. Six explicit image tests fail due to missing models; image snapshots/golden data are also absent.
- [ ] Remove or replace the two ignored OCR engine tests that swallow all errors and return success; retain meaningful extraction smoke tests and add real result assertions where appropriate.
- [ ] Correct the ignored live-API test's stale endpoint; explicit execution returned HTTP 404 while the actual CLI fetched events successfully.
- [ ] Make the accuracy regression distinguish missing data from measured success. All 194 observations are now imported; the legacy check reports/excludes 188 unknown-score rows and retains 13.3% MAPE on six scored records. It still needs an explicit no-evaluable-data failure/skip policy and correct event geometry/precise-time evaluation before calibration claims.
- [x] Fix filter/display duration disagreement with one shared estimate; regression tests and live 30±15 / 60±15 checks pass. KRG declares 45 fixed minutes, which now displays correctly.
- [x] Distinguish unknown elevation from known zero; calculate lap/lead-in totals; import pinned route-ID-based metadata with provenance and preservation tests.
- [x] Back up the owner's normal database and apply the verified client export with `import_client_routes` (approved/applied 2026-10-30). Verified all 386 client records, unchanged non-route data/history, integrity, no foreign-key violations, and repeat-import idempotence. Private export: `~/.local/share/zwift-race-finder/client-routes-0862ca71d9b4.json`.
- [x] Client import reconciles all ten seed IDs against client metadata; the incorrect hardcoded Bell Lap fallback is removed. History is preserved and community imports cannot downgrade client records.
- [x] Remove misleading automatic route seeds; fresh databases are empty until explicit client import, with a CLI warning and lifecycle regression coverage.
- [x] Establish client-cache → extracted-client precedence, fractional climbing, availability metadata, and per-route provenance; verified the selected 386-route client export. The former community fallback is retired.
- [x] Retire bundled legacy/community route data, importers, website discovery, and guessed mapping/alias recipes; preserve existing database/history. See [retirement audit](docs/project-history/CLIENT_ONLY_ROUTES_2026-10-30.md).
- [ ] Verify active client-cache selection and refresh source coverage when the installed game changes. Rechecked archive SHA and installed version 1.124.0 (167761); installed manifest provided no active-cache link. Do not infer freshness from file time or public availability from internal definitions.
- [ ] Select the relevant stage's distance from multi-stage event descriptions rather than the first distance in the text; current shared resolver deliberately preserves the existing parser.
- [ ] Reassess subgroup selection against actual Racing Score range labels; the shared-resolver fix does not change existing eligibility matching.
- [x] Implement optional-score, precise, source-qualified performance history and a non-writing preview; migrate/import all approved observations with backed-up reconciliation, conflict/rollback tests, preserved six confirmations and repeat no-op verification. See [history audit](docs/project-history/HISTORY_MODEL_2026-10-30.md).
- [ ] Develop/evaluate a separate history-based estimator: equivalent event geometry first, comparable terrain/distance/format next, recency/uncertainty and explicit fallback. Use chronological splits without future leakage; do not switch production merely because ingestion succeeded.
- [ ] Resolve current Clippy failures; do not treat overlapping library/library-test lint totals as distinct defects.
- [ ] Resolve existing Rust formatting failures and ShellCheck findings without conflating them with test assertion failures.
- [ ] Fix `tools/utils/test_api_limits.sh` to check HTTP status and JSON type before reporting counts; the current diagnostic mislabels an HTTP 400 error as one event.
- [ ] Establish supported Python OCR dependencies/models and execute its smoke/comparison checks; distinguish these from assertion-based tests.
- [ ] Decide whether to implement real performance baselines later. Empty benchmark targets and inactive UOM migration scaffolding have been removed, not replaced by new coverage.
- [ ] Defer credential storage/rotation and personal-data decisions until functional testing, as requested by the owner. Do not publish the current credential-bearing artifacts.

## Parked documentation review

- [ ] Review and accept, revise, or discard the complete unpublished documentation rationalisation described in `STATUS.md`.
- [ ] Review `docs/howto/RETURN_TO_RACING.md`.
  - Re-check the official Zwift Racing Score FAQ and calculation page if their 13 September 2026 verification is no longer fresh enough for publication.
  - Decide whether to retain the practical 20–30 minute return-ride suggestion with a representative five-minute hard effort or limit the guide to official wording.
  - Keep the official eligibility minimum separate from optional advice for better initial seeding.
  - Keep historical requirements labelled as historical and retain primary links for future re-checks.
  - Refresh the verification date if source wording changes.
- [ ] Complete documentation acceptance after the test-first audit. Native prerequisite approval is tracked above; `rustfmt` is already installed but formatting fails. Installation, privilege elevation, commit, and publication are not implied.

## Duration accuracy

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
- [ ] Decide whether stored rider weight and FTP should remain in the schema when the estimator does not use them directly.
- [ ] Isolate CLI integration tests from the live API and normal user database.

## Code-truth decisions requiring resolution

These remain separate open questions from `docs/code-truth/DECISIONS.md`:

- [x] Enable SQLite foreign-key enforcement on application database connections; migration and history import verify constraints, and normal post-import `foreign_key_check` is clean.
- [ ] Revalidate the assumed ~200-event/~12-hour Zwift API limit when API behaviour is next reviewed.
- [ ] Decide whether `Config::default()` should retain personal-shaped weight and height defaults or use generic/absent values.
- [ ] Assess and remove or contain the `Box::leak` lifetime workaround in `src/estimation.rs` before reusing this code in a long-lived process.
- [ ] Choose the correct support destination for the unrelated `anthropics/claude-code` issue URL in `src/main.rs`.

## Untriaged retained requirements and recommendations

The exact legacy items remain in their original numbered/listed form; this section does not combine, reprioritise, or declare them obsolete.

- [ ] Reconcile **every numbered requirement, future enhancement, KPI, immediate priority, and known issue** in `docs/project-history/REQUIREMENTS_2025.md` against current code and an owner decision. Until then, each item whose satisfaction is not directly verified remains individually unresolved in that record.
- [ ] Reassess every unresolved finding and recommendation in `docs/project-history/SECURITY_AUDIT_2025.md` before making a current security claim. Its obsolete commands must not be executed; the record retains exact historical context.
- [ ] Reassess every stated gap and recommendation in `docs/project-history/UOM_INTEGRATION_TEST_COVERAGE_2025.md` if UOM or integration-test work resumes. Current test documentation does not by itself verify that each historical acceptance criterion was satisfied.

The Pi release audit reports no compatibility follow-up and therefore adds no open Pi task. Its evidence remains at `docs/reference/PI_RELEASE_AUDIT.md`.
