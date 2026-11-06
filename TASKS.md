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
- [ ] Prepare the [four-source recording plan](docs/explanation/BRIDGE_AND_MULTI_RIDER_DESIGN.md#race-recording-integration): identify existing bike/Sauce recorder outputs and private destinations, approved preflight, open-ended explicit start/stop and post-race timing alignment; arrange explicit Zwift FIT and ZwiftPower inputs. Confirm present Sauce readiness/entitlement, not merely historical success. Do not build new acquisition here.
- [x] Approve and install the open-ended Sauce launcher/recorder, preserve verified backups, and pass installed non-login preflight and 132 offline Windows recorder tests; see [installation evidence and limits](STATUS.md#open-ended-sauce-candidate--2026-11-05).
- [ ] Separately qualify actual terminal start/stop and live race coverage; installed synthetic checks do not establish those results.
- [ ] On approved supplied inputs, verify rider/event identity, inventory fields and per-source coverage, then align bike telemetry, Zwift FIT and Sauce clocks with held-out checks. Retain raw timestamps, offset/drift uncertainty and gaps; do not reuse the old fixed offset blindly.
- [ ] After the race, compare the preserved plan with supported effort/tactical evidence and the supplied dated ZwiftPower result/status. Reconcile boundaries and disagreements; separate measurements, adherence and uncertain explanations. Missing sources narrow claims but do not block FIT-only review; optional video or immediate notes may help.
- [ ] Retain the private plan/review and minimal provenance outside Git; record only a safe, scoped acceptance summary. One result does not prove a strategy caused improvement.

Tool development, a maximal test and model calibration are not prerequisites for the first conditional plan. Numerical model work needs a separate justified task; do not change configured CP/FTP/W′ merely because a planning workflow uses them.

## Shared-library consumption

- [x] Document and test the explicit-path, agent-assisted FIT/ZwiftPower procedure against the approved November 5 library session, with a new local review and private provenance; see [pilot scope and limits](STATUS.md#shared-library-pilot--2026-11-05).
- [ ] If publication is wanted, confirm the exact new Coach library destination and finalized artifact list first. Do not overwrite the frozen review or update the shared manifest implicitly.

This pilot does not implement a maintained review command, select the application runtime or close the synthetic-validation gates below.

## Supporting ride-review implementation — deferred until justified

Contract: [FIRST_RIDE_REVIEW.md](docs/explanation/FIRST_RIDE_REVIEW.md).

Initial approved-path intake, read-only field/time inspection and exploratory recommendations have occurred; see [current state and evidence limits](STATUS.md#first-milestone-and-next-action). These are not a completed implementation or accepted first-review milestone.

- [ ] Select the smallest suitable parser/runtime and scope the local review command from intake findings; no placeholder application or simulator. Python/uv with `fitdecode` remains a candidate, not an adopted decision.
- [ ] Implement a reproducible read-only review command with data-quality limits, declared-policy effort summaries, inspectable time blocks/windows and provisional phase labels. This is proposed implementation work, not authorized by documentation reconciliation.
- [ ] Retain full first-review acceptance evidence outside Git, with a safe summary here. The shared-library pilot now has a durable bounded review and input/check provenance, but maintained implementation, synthetic regressions and full acceptance remain incomplete; the earlier exploratory conversation alone is still insufficient.
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
