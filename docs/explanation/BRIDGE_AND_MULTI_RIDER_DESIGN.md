# Coach integration and controlled multi-rider experiments

## Ownership after the Fitness Bridge handoff

[Fitness Bridge](https://github.com/jchidley/fitness-bridge) now owns acquisition, source selection, filtering/transformation, synthetic measurement sources and BLE/ANT+ output. Its Rust/Linux design and implementation backlog are authoritative there. The published repository is a design/reference baseline, not a working bridge.

This project owns ride reviews, pacing plans, effort constraints, higher-level strategy and experiment coordination. Life Fitness owns custom hardware, Pico firmware and physical resistance. Race-finder remains independent; route/event data enters coaching as an explicit supplied snapshot, not a shared writable database.

The [first recorded-ride review](FIRST_RIDE_REVIEW.md) needs no bridge, sensor pairing or live client. Do not make the hardware backlog a prerequisite.

## Race recording integration

For the next human-ridden race, plan a private evidence bundle joining **bike telemetry, Zwift FIT, Sauce for Zwift and ZwiftPower**. This is recording/review integration, not synthetic sensor output or a new bike controller. Use existing recorders where available; inspect their actual outputs before promising fields or conversion. The first FIT-only review remains useful if another source is unavailable.

### Source roles and limits

| Source | Contribution to the review | Limits to preserve |
|---|---|---|
| Bike telemetry | Available upstream measurements, device/source health and, if actually logged, resistance/control state. Helps investigate a dropout or disagreement before data reaches Zwift. | Inspect the existing recorder, fields, units, timestamps and measurement versus transformation provenance. A command is not measured resistance, transmitted watts or confirmed client reception. Do not assume a particular bike log already exists. |
| Zwift FIT | Client-recorded effort and activity timeline: available power, cadence, HR, distance and altitude; laps/session summaries where present. | Activity time is not necessarily race time. Inspect actual fields; do not assume an explicit draft field or race-finish marker. |
| Sauce for Zwift | Live client context, including the tested self-rider draft stream, alongside overlapping effort/movement fields. | Preserve draft values as source-reported until semantics/units are qualified; do not label them measured watts saved. Nearby-rider/group-position capture is not established by the self-rider tests. |
| ZwiftPower | Supplied event/category result, placing, result time and applicable classification/status, plus summaries where available. | Record retrieval time and provisional/final state; category results, exclusions and timing can differ from an on-screen finish. It is not a continuous tactical trace. Do not assume access, eligibility or every field. |

These are complementary views, not four independent measurements: FIT and Sauce can reflect the same Zwift state, and bike telemetry may share the sensor feeding Zwift. Preserve each original instead of averaging disagreements away. Fitness Bridge owns sensor acquisition/processing/output; Life Fitness owns hardware/control. Coach consumes explicitly approved recordings and coordinates their interpretation. No sibling implementation changes are implied.

### Before the race

1. Alongside the [dated private pacing plan](PACING_AND_RACE_EXPERIMENTS.md#s8--preserve-the-plan-before-evaluating-the-outcome), identify which existing recorder supplies each source, its version/configuration and private destination. Confirm readiness separately from historical success; the [recovered Sauce evidence](../../STATUS.md#recovered-sauce-evidence--2026-11-04) is not a current launch instruction.
2. Plan a brief preflight with the existing bike/Sauce recording path: verify files actually accumulate usable samples and a timestamped health/status record. Start before the event lead-in and explicitly stop after the finish and short post-finish buffer. The selected Sauce workflow is open-ended start/stop, with no recording-duration cutoff or automatic race-finish detection; a bounded API startup timeout is separate. Leave the recording terminal open and account for growing disk use. Match actual race start/finish afterwards from FIT and reported result timing; record early termination or restart rather than overwriting the first session. Launches, device changes and account use retain their approval requirements.
3. In a private bundle manifest, associate the rider and event/subgroup/category, numeric route ID, laps/distance, lead-in and expected start/finish rules. Keep supplied IDs and source versions unchanged. Identify each approved input by private path/hash, source and recorder session; note missing sources explicitly. Record device/source selection and any configured transformation relevant to comparing bike and client watts.
4. Record time bases: original timestamps and timezone, source/device/game time, local receive time and clock synchronisation information where available. A receive timestamp includes transport/buffering delay; an unsynchronised device clock is not UTC. Avoid creating an artificial effort solely to align clocks during a race.
5. Plan how the Zwift FIT and permitted ZwiftPower result will be supplied after racing. Prefer explicit files or a supplied result snapshot/link; a link does not itself authorize authenticated fetching or scraping. No additional subscription, installation, credential retrieval or network-exposure change is authorized by this plan. Confirm current Sauce entitlement/cost and safe exposure before any launch; do not assume the old build or account setup is still usable.

### After the race: align, then compare

- Preserve raw inputs unchanged outside Git, with the original plan, a dated results snapshot and immediate rider notes where supplied. Do not search personal folders or download account data implicitly.
- Verify rider/event/session identity before joining. Inspect each source's fields, units, missing/zero values, duplicate/out-of-order samples, pauses, gaps, resets and coverage. Show a per-source coverage/exclusion table; a clean individual capture does not remove a gap between captures.
- Declare an event-relative timeline with separately evidenced warm-up, lead-in, race start, finish and cooldown boundaries. Use supplied result timing only with its documented basis; do not equate login time, first FIT sample, distance zero or recorder stop with the race boundary.
- Estimate clock offset and, where needed, drift from shared observations/anchors over overlapping coverage. Check alignment on held-out intervals and around relevant surges or transitions; report residual error and unresolved ambiguity. Re-establish mappings after resets/context changes. The historical quarter-second Sauce/FIT adjustment is not a universal correction. Preserve original timestamps and mappings; do not shift data just to force a desired conclusion.
- Use declared interval boundaries, resampling and missing-data policies. Do not bridge long gaps, treat missing draft as zero or interpret absent bike logs as confirmed sensor delivery. Compare upstream bike measurements with client watts/cadence over valid overlap; investigate source selection, transformations and latency before asserting a device fault. FIT/Sauce agreement supports consistency, not independent sensor calibration.
- Compare each consequential planned action with measured effort, available Sauce context, rider notes/video and the supplied ZwiftPower outcome. Distinguish observed draft from inferred shelter/group position and neither from tactical intent or causation. Reconcile any result-time/placing disagreement using event rules and result status; retain unresolved differences rather than silently choosing one source.

The private deliverable is one review with source provenance, coverage/alignment limits, a plan-versus-outcome timeline, and a few supported changes for the next race. If a source is absent, narrow the conclusions rather than block useful coaching. No general ingestion service, shared database, new runtime or physiological model is selected by this plan. Any later implementation needs the [ride-review validation boundary and tests](FIRST_RIDE_REVIEW.md#implementation-checklist).

## Later coach-to-bridge integration

When a concrete experiment requires live output, agree the smallest versioned interface with Fitness Bridge. Coach owns the intended effort schedule or policy; bridge owns interpretation of its measurement-source contract and profile encoding. Record:

- command/source identity, units, intended time/validity and any acceptance receipt;
- original versus transformed/generated measurements, transformation parameters and available timestamps;
- source/connection status, expiry and which timing/age facts are measured or inferred;
- intended commands, actual transmitted observations and independently observed client telemetry.

No particular API schema or endpoint is implemented or selected here. Sending a prescription is not proof of transmission, client reception or physical actuation. Do not bypass the bridge's validated input boundaries or add a competing sensor backend here.

## Coach-owned progression

1. Deliver useful recorded-ride reviews and practical pacing plans.
2. For a separately selected experiment, validate one input/telemetry path and repeat a baseline with explicit effort and timing constraints.
3. Compare a small number of bounded pacing strategies before adding opponents.
4. Test fixed replay benchmarks separately from responsive opponents; verify drafting rather than inferring it from visibility.
5. Add a second independently controlled client only when needed to answer a concrete question.

Use the owner's known-working native Windows clients first. One client per laptop is the initial multi-client candidate; netbrain/zwift and same-machine multi-instance work remain deferred. Headless, accelerated, cross-version and offline-server compatibility are unqualified.

Hold relevant rider/equipment settings, route identity, lead-in, start conditions, client/server versions and finish rules constant. Preserve fractional units and distinguish measured outcomes from fatigue/physics assumptions. See [pacing and experiment guidance](PACING_AND_RACE_EXPERIMENTS.md).

## Historical design and device evidence

The [2026-10-30 combined bridge/multi-rider design](BRIDGE_AND_MULTI_RIDER_DESIGN_2026-10-30.md) is preserved byte-for-byte as historical evidence. Its host choices, language candidates, monitor uncertainty and bridge-first sequence are superseded by the new project's decisions, not active coach tasks.

Snapshot SHA-256: `2cac5da57b83815998253071223a2752e67dd6b1b02d3e71c2b4085b5bf2c679`. This is the source hash recorded by Fitness Bridge's handoff provenance; the former pathname now holds this ownership/integration note.

The [documentation index](../README.md) retains the Gymnasticon, ANT/BLE, repository and client research reports. These document earlier tests, not validation of the unimplemented Rust bridge. Personal traces, raw captures and generated experiment recordings remain outside Git. Device/account operations and experiments retain their separate approval boundaries.
