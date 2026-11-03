# Tasks

## Ownership and repository setup

- [x] Publish and verify the separate Fitness Bridge repository; its sensor/processing/radio backlog no longer belongs here.
- [x] Reconcile README, boundaries and current experiment guidance; retain historical bridge design and device evidence.
- [x] Recheck the private coach remote and disabled Actions; initialize local main tracking the existing origin/main history without changing working files.
The approved package is published and its remote contents/settings and local tracking were verified; see the [publication checkpoint and limits](STATUS.md#publication-verification--2026-11-03). Publication is no longer pending; coaching implementation and acceptance work below remain open.

## Standards adoption

- [x] Adopt S1–S4 as documentation only (D1, 2026-11-03): independent recorded review, explicit numerical semantics, narrowly selected examples, and qualified/private evidence. See the [contract](docs/explanation/FIRST_RIDE_REVIEW.md#adopted-standards--2026-11-03) and [adoption scope](STATUS.md#standards-adoption--2026-11-03).
- [x] Document D2: clarified S4 and adopted S5–S8, including research applicability limits and the private before/after workflow. See [D2 scope](STATUS.md#d2-adoption--2026-11-03). Documentation adoption is not implementation or physiological acceptance.
- [ ] When an approved implementation needs them, enforce explicit observation/parameter/evidence boundaries and focused positive/negative checks (E2). No physiological model framework, runtime or application gate selected.
- [ ] With the first justified coaching implementation, enforce S2/S3 through a validated input boundary and synthetic positive/negative regressions with independently calculated expectations (E1). No runtime or exact implementation files selected yet; existing metric-validation work below remains open.

No coaching application exists to migrate (M1/M2); do not refactor historical diagnostics into a coaching framework. Documentation adoption does not authorize commits or pushes.

## Selected workflow — next race, before and after

The owner will select the event in race-finder. Follow the [race-planning/review contract](docs/explanation/PACING_AND_RACE_EXPERIMENTS.md#race-planning-and-review-standards--d2); do not edit that sibling project implicitly.

- [ ] Receive an event link or supplied route/format snapshot and rider objective; confirm relevant rules, timing/lead-in and finish semantics.
- [ ] Prepare and preserve a dated private plan: likely selections and durations, conditional actions, recovery opportunities, fallback, evidence bases and unknowns. Use the supplied GoldenCheetah summary as qualified prior work, not a newly verified or calibrated capacity profile.
- [ ] If stronger claims need it, obtain an explicitly supplied durable private evidence location and verify only the relevant prior-work evidence. Do not rescan the profile automatically; keep predictive claims limited if evidence is unavailable.
- [ ] After the race, inspect the explicitly approved completed recording and compare it with the preserved plan; separate measured effort, observed tactics, adherence and uncertain explanations. Optional video or immediate notes may help, but are not prerequisites.
- [ ] Retain the private plan/review and minimal provenance outside Git; record only a safe, scoped acceptance summary. One result does not prove a strategy caused improvement.

Tool development, a maximal test and model calibration are not prerequisites for the first conditional plan. Numerical model work needs a separate justified task; do not change configured CP/FTP/W′ merely because a planning workflow uses them.

## Supporting ride-review implementation — deferred until justified

Contract: [FIRST_RIDE_REVIEW.md](docs/explanation/FIRST_RIDE_REVIEW.md).

Initial approved-path intake, read-only field/time inspection and exploratory recommendations have occurred; see [current state and evidence limits](STATUS.md#first-milestone-and-next-action). These are not a completed implementation or accepted first-review milestone.

- [ ] Select the smallest suitable parser/runtime and scope the local review command from intake findings; no placeholder application or simulator. Python/uv with `fitdecode` remains a candidate, not an adopted decision.
- [ ] Implement a reproducible read-only review command with data-quality limits, declared-policy effort summaries, inspectable time blocks/windows and provisional phase labels. This is proposed implementation work, not authorized by documentation reconciliation.
- [ ] Retain a durable private review and minimal private input/check provenance outside Git, with a safe acceptance summary here. The exploratory conversation is not sufficient durable evidence.
- [ ] Reproduce representative metrics and intervals independently against the maintained implementation. Prior exploratory agreement between decoders does not establish this gate.
- [ ] Add synthetic positive/negative regressions for missing/zero, irregular sampling, gaps, pauses, duplicate/out-of-order timestamps, conflicting populated fields and exact timing boundaries (E1).

Use only explicitly approved inputs; prior approval of one file does not authorize searching personal folders, fetching account data or selecting additional activities.

## Later controlled experiments

- [ ] Select one concrete strategy question and effort constraint after the recorded-review workflow is useful.
- [ ] Agree the smallest coach-to-Fitness-Bridge input/status contract only when a live experiment needs it; do not duplicate acquisition or profile encoders.
- [ ] Qualify one native Windows client input/telemetry path, then repeat a fixed baseline with recorded settings and timing boundaries.
- [ ] Compare bounded pacing strategies; separate measured outcomes from unverified fatigue/physics assumptions.
- [ ] Distinguish fixed replay benchmarks from responsive opponents and verify actual drafting effects.
- [ ] Add a second independent native Windows client only for a concrete unanswered question. Container runners, accelerated/headless execution and mixed-version clients remain deferred.

## Ownership references

- [Fitness Bridge](https://github.com/jchidley/fitness-bridge): sensors, source selection, transformations, synthetic measurement sources, BLE/ANT+ output and device qualification.
- Life Fitness: custom hardware, Pico firmware and physical control.
- Race-finder: independent event selection and route-data extraction/import. Consume explicit supplied snapshots; no finder repair or shared writable database is required here.
- [Historical reports](docs/README.md): retained evidence, not active bridge implementation tasks.
