# Open Tasks

The current objective is one find–plan–review workflow, with a useful personal
race selector plus bounded comparisons
against **`v0.3.0`**. [Status](STATUS.md) owns current results; the
[baseline/experiment record](docs/project-history/RECENCY_BASELINE_2026-11-03.md)
owns the original frozen B0/R1 comparison. The
[recency evidence note](docs/explanation/RECENCY_EVIDENCE.md) records the expanded
comparison and open interpretation: recent evidence is insufficient for a strong
signal either way. R1 stays experimental; retaining B0 does not close research.

## Immediate plan — recent-performance relevance

- [ ] Resolve the remaining source-coverage gap: a race-labelled activity in the
  expanded period is absent from the supported results export. The two previously
  missing later results and EVR geometry are now reconciled in private copies;
  normal history is unchanged. Preserve exact seconds and avoid duplicate results.
- [ ] Record unchanged B0/R1 predictions before new races, using identical geometry
  and earlier history. Accumulate source-qualified outcomes without tuning to the
  seven inspected results. Keep frozen and refreshed-data runs separate; report
  geometry qualifications and missing-source coverage independently of model
  unavailability. Do not treat inconclusive evidence as rejection or validation of
  recency weighting, current-form adjustment or B0's existing policy.
- [ ] Before another recency/current-form candidate, specify its mechanism and
  evaluation protocol. Address a concrete limitation rather than tune inspected
  races to perfection. Preserve cross-course estimates, source/rider boundaries,
  honest sparse support and unknown durations. A separate current-form adjustment
  remains open; useful baseline acceptance does not cancel research.
- [ ] Keep error, duration-window inclusion/exclusion and prediction availability
  visible in comparisons. Do not promote R1 automatically or add a category fallback.

## Correctness and validation

- [ ] Require populated accuracy qualification to distinguish zero usable samples
  from measured success. Keep the legacy category regression separate from personal
  chronological evaluation. Any future acceptance threshold must identify units
  and cohort; older percentage wording remains unresolved, not silently redefined.
- [ ] Resolve explicit CLI duration/tolerance/days values equal to defaults being
  replaced by configuration preferences.
- [ ] Isolate config/cache tests that mutate process-wide cwd/environment. Reduce
  live-service dependence in CLI tests without weakening production-adapter tests.
- [ ] Consider one immutable estimate per event if independent filter/display reads
  cause a demonstrated discrepancy; it is not implemented today.
- [ ] Select the applicable stage distance in multi-stage descriptions rather than
  taking the first matching distance.
- [ ] Resolve repository-wide formatting/Clippy debt without suppressing gates.
- [ ] Review tracked credential-bearing/personal-data artifacts before any publication;
  no credential rotation or Git-history rewriting is authorized by cleanup.

## Component limits outside the immediate model work

- Client-cache freshness is not established by file time; verify selected sources
  when updating the installed game's route data.
- Optional OCR has unresolved parsing/snapshot/model prerequisites and ignored tests.
  Do not replace golden data or swallow failures to claim support. It is not a
  prerequisite for race selection; coaching review does not require it; sensor acquisition and control remain external.

Superseded task inventories, 2025 requirements and old audits are retained in Git
at `v0.3.0`, not a parallel active backlog. Their removal from the working tree does
not certify all historical findings fixed or turn untriaged ideas into requirements.

## Planning and review work inherited from Coach

### Standards adoption

- [x] Adopt S1–S4 as documentation only (D1, 2026-11-03): independent recorded review, explicit numerical semantics, narrowly selected examples, and qualified/private evidence. See the [contract](docs/explanation/FIRST_RIDE_REVIEW.md#adopted-standards--2026-11-03) and [adoption scope](docs/reference/EVIDENCE_LIBRARY.md#coach-status-pre-consolidation).
- [x] Document D2: clarified S4 and adopted S5–S8, including research applicability limits and the private before/after workflow. See [D2 scope](docs/reference/EVIDENCE_LIBRARY.md#coach-status-pre-consolidation). Documentation adoption is not implementation or physiological acceptance.
- [ ] When an approved implementation needs them, enforce explicit observation/parameter/evidence boundaries and focused positive/negative checks (E2). No physiological model framework, runtime or application gate selected.
- [ ] With the first justified coaching implementation, enforce S2/S3 through a validated input boundary and synthetic positive/negative regressions with independently calculated expectations (E1). No runtime or exact implementation files selected yet; existing metric-validation work below remains open.

No coaching application exists to migrate (M1/M2); do not refactor historical diagnostics into a coaching framework. Implementation and publication still require their applicable task scope.

### Selected workflow — next race, before and after

Select the event with the existing finder. Follow the [race-planning/review contract](docs/explanation/PACING_AND_RACE_EXPERIMENTS.md#race-planning-and-review-standards--d2).

- [ ] Receive an event link or supplied route/format snapshot and rider objective; confirm relevant rules, timing/lead-in and finish semantics.
- [ ] Prepare and preserve a dated private plan: likely selections and durations, conditional actions, recovery opportunities, fallback, evidence bases and unknowns. Use the supplied GoldenCheetah summary as qualified prior work, not a newly verified or calibrated capacity profile.
- [ ] If stronger claims need it, obtain an explicitly supplied durable private evidence location and verify only the relevant prior-work evidence. Do not rescan the profile automatically; keep predictive claims limited if evidence is unavailable.
- [ ] Prepare the [four-source recording plan](docs/explanation/BRIDGE_AND_MULTI_RIDER_DESIGN.md#race-recording-integration): identify existing bike/Sauce recorder outputs and private destinations, approved preflight, open-ended explicit start/stop and post-race timing alignment; arrange explicit Zwift FIT and ZwiftPower inputs. Confirm present Sauce readiness/entitlement, not merely historical success. Do not build new acquisition here.
- [x] Approve and install the open-ended Sauce launcher/recorder, preserve verified backups, and pass installed non-login preflight and 132 offline Windows recorder tests; see [installation evidence and limits](docs/reference/EVIDENCE_LIBRARY.md#coach-status-pre-consolidation).
- [ ] Separately qualify actual terminal start/stop and live race coverage; installed synthetic checks do not establish those results.
- [ ] On approved supplied inputs, verify rider/event identity, inventory fields and per-source coverage, then align bike telemetry, Zwift FIT and Sauce clocks with held-out checks. Retain raw timestamps, offset/drift uncertainty and gaps; do not reuse the old fixed offset blindly.
- [ ] After the race, compare the preserved plan with supported effort/tactical evidence and the supplied dated ZwiftPower result/status. Reconcile boundaries and disagreements; separate measurements, adherence and uncertain explanations. Missing sources narrow claims but do not block FIT-only review; optional video or immediate notes may help.
- [ ] Retain the private plan/review and minimal provenance outside Git; record only a safe, scoped acceptance summary. One result does not prove a strategy caused improvement.

Tool development, a maximal test and model calibration are not prerequisites for the first conditional plan. Numerical model work needs a separate justified task; do not change configured CP/FTP/W′ merely because a planning workflow uses them.

### Shared-library consumption

- [x] Document and test the explicit-path, agent-assisted FIT/ZwiftPower procedure against the approved November 5 library session, with a new local review and private provenance; see [pilot scope and limits](docs/reference/EVIDENCE_LIBRARY.md#coach-status-pre-consolidation).
- [x] Publish the separate approved Coach pilot allowlist without overwriting the
  frozen review or changing the shared session manifest; see the
  [local-library follow-through](#local-evidence-library-follow-through).

This pilot does not implement a maintained review command, select the application runtime or close the synthetic-validation gates below.

### Local evidence-library follow-through

- [x] Verify current home-backup inclusion and recovery (2026-11-07): snapshot
  `3a0451e2` restored all 344 current FitnessData files into fresh ext4 staging,
  with matching SHA-256 and exact file set. This qualifies local recovery only,
  not an independent external replica. No OneDrive sync/pinning work is required.
- [x] Publish the approved Finder (19-file) and Coach (8-file) pilot allowlists
  under the current local library root, verify source/destination hashes and retain
  separate receipts (2026-11-07). Root library navigation links the new runs;
  frozen session indexes and original proposals remain unchanged.
- [x] Inspect Fitness Bridge recording support: its production bridge has no
  maintained recorder/output-path option to configure (2026-11-07). Do not build
  one implicitly or treat synthetic fixtures/private trial evidence as a recorder.
- [ ] Select actual synchronization source/collector hosts, native library paths
  and machine/session ownership before transferring private recordings or enabling
  scheduling. The library's `SYNCHRONIZATION.md` supplies manual rsync options;
  no host selection or sync job is implied.
- [x] Publish Life Fitness direct-host and `bikectl` native-home defaults with
  existing legacy/binary lifecycle semantics (`c47b60e`, 2026-11-07), including
  isolated-path tests and local recording/sync guidance.
- [ ] Reconcile the canonical Life Fitness checkout's pre-existing untracked files
  with newer upstream before between-session client rebuild/reinstallation. The
  fast-forward refused safely; originals and the clean tested worktree are retained.
  Do not delete/move evidence or interrupt an ongoing ride implicitly. Preserve
  intentional Git recordings and read-only `life_cycle` source evidence.
- [x] Audit the exact migration originals and supplied pilot dependencies
  (2026-11-07): all 250 originals still match inventory hashes; five pilot evidence
  files retain historical path references. Preserve these sources and frozen paths.
  This bounded audit does not prove every consumer portable; no deletion or source
  retirement is approved, and the obsolete Coach checkout stays archived.

### Supporting ride-review implementation — deferred until justified

Contract: [FIRST_RIDE_REVIEW.md](docs/explanation/FIRST_RIDE_REVIEW.md).

Initial approved-path intake, read-only field/time inspection and exploratory recommendations have occurred; see [current state and evidence limits](docs/reference/EVIDENCE_LIBRARY.md#coach-status-pre-consolidation). These are not a completed implementation or accepted first-review milestone.

- [ ] Select the smallest suitable parser/runtime and scope the local review command from intake findings; no placeholder application or simulator. Python/uv with `fitdecode` remains a candidate, not an adopted decision.
- [ ] Implement a reproducible read-only review command with data-quality limits, declared-policy effort summaries, inspectable time blocks/windows and provisional phase labels. This is proposed implementation work, not authorized by documentation reconciliation.
- [ ] Retain full first-review acceptance evidence outside Git, with a safe summary here. The shared-library pilot now has a durable bounded review and input/check provenance, but maintained implementation, synthetic regressions and full acceptance remain incomplete; the earlier exploratory conversation alone is still insufficient.
- [ ] Reproduce representative metrics and intervals independently against the maintained implementation. Prior exploratory agreement between decoders does not establish this gate.
- [ ] Add synthetic positive/negative regressions for missing/zero, irregular sampling, gaps, pauses, duplicate/out-of-order timestamps, conflicting populated fields and exact timing boundaries (E1).

Use only explicitly approved inputs; prior approval of one file does not authorize searching personal folders, fetching account data or selecting additional activities.

### Later controlled experiments

- [ ] Select one concrete strategy question and effort constraint after the recorded-review workflow is useful.
- [ ] Agree the smallest coach-to-Fitness-Bridge input/status contract only when a live experiment needs it; do not duplicate acquisition or profile encoders.
- [ ] Qualify one native Windows client input/telemetry path, then repeat a fixed baseline with recorded settings and timing boundaries.
- [ ] Compare bounded pacing strategies; separate measured outcomes from unverified fatigue/physics assumptions.
- [ ] Distinguish fixed replay benchmarks from responsive opponents and verify actual drafting effects.
- [ ] Add a second independent native Windows client only for a concrete unanswered question. Container runners, accelerated/headless execution and mixed-version clients remain deferred.
