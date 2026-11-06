# Pacing assistance and controlled race experiments

## Goal and first milestone

Help the owner ride better, rather than make building a simulator a prerequisite for useful coaching. Start with one explicitly approved completed activity, a field-availability report and an evidence-based ride review. Add a pacing plan only when upcoming-event details and adequate rider context are supplied; it does not block the review.

A ride can show what the rider did; it does not alone establish maximum sustainable power, fatigue capacity, drafting state, or tactical intent. Reports must distinguish observed effort patterns from explanations that require additional evidence. Keep private inputs and generated reports outside Git.

## Current workflow and integration boundary

The [first ride-review contract](FIRST_RIDE_REVIEW.md) defines the workflow and acceptance requirements. [STATUS.md](../../STATUS.md) owns current progress and evidence limits; [TASKS.md](../../TASKS.md) owns unfinished work.
[Fitness Bridge](https://github.com/jchidley/fitness-bridge) owns sensor acquisition,
processing and BLE/ANT+ output; Life Fitness owns hardware and physical control.
The [coach integration design](BRIDGE_AND_MULTI_RIDER_DESIGN.md) defines our later
strategy/telemetry responsibilities. Native Windows is the first live-client
baseline; container/multi-instance work is deferred. The architecture below
retains earlier experimental context, not a prerequisite for recorded review.

## Race-planning and review standards — D2

The owner adopted S5 and S8 as documentation requirements on 2026-11-03. [S4, S6 and S7](FIRST_RIDE_REVIEW.md#adopted-standards--2026-11-03) govern evidence provenance, capacity claims and model qualification. These standards do not select a runtime/model, implement an application or establish physiological validation.

### S5 — Declare the objective before prescribing pacing

State whether the objective is fastest solo time, staying with a sustainable group, placing, or a training outcome. Record the supplied event format and relevant rules, including whether drafting applies. Advice must address that objective: an efficient solo power schedule is not automatically optimal bunch-race tactics.

For group racing, consider the tactical benefit of an effort, its duration/cost and the next recovery opportunity. Neither maximum effort on every climb nor never exceeding FTP is a universal rule. Research-derived guidance is a conditional starting point, not a fixed wattage prescription or proof of an optimal strategy. No opponent simulator or optimisation engine is required.

Initially enforce this through a short objective/event-context section and human review of tactical reasoning. Numerical tests cannot establish whether a tactical decision was wise.

### S8 — Preserve the plan before evaluating the outcome

Keep a dated pre-race plan outside Git and preserve its original expectations before reviewing the result. Subsequent amendments are distinguishable from the original; do not rewrite it with hindsight. The plan contains:

- objective and supplied event/route snapshot, including timing/lead-in and finish semantics;
- likely selection points and approximate effort durations;
- conditional actions, expected recovery opportunities and a fallback if the intended group is unsustainable;
- the basis for consequential recommendations, assumptions and unresolved questions;
- the [recording integration plan](BRIDGE_AND_MULTI_RIDER_DESIGN.md#race-recording-integration): bike telemetry, Zwift FIT, Sauce and ZwiftPower roles, readiness, coverage and alignment, with absent sources explicitly noted.

After the race, compare the original plan with the explicitly approved recording and any optional video or immediate rider notes. Separate what the rider produced, what happened tactically where evidence exists, whether the planned action was followed, plausible explanations and what remains unknown. Separate the race finish from cooldown and any extra efforts.

A FIT file alone does not establish group position, drafting status or why a gap opened. Video and notes can improve interpretation but are not prerequisites. One race cannot prove a strategy caused a better result. Initially use paired private plan/review documents; later preserve plan identity and separate observations from interpretations in any implemented workflow.

The observable result is an auditable comparison: each consequential planned action has an identifiable basis, and its post-race assessment names the evidence and limits. It need not produce a faster finish to be a useful review.

### Research basis and transfer limits

Sources below were inspected during focused research on 2026-11-03, not a systematic review. These references support the safeguards and reasoning approach, not a selected algorithm or individual model validation. No proprietary text, private rider profile or reference archive is retained.

| Source / inspected access | Evidence and applicability | Do not transfer |
|---|---|---|
| [Atkinson, Peacock & Passfield (2007), variable versus constant power](https://doi.org/10.1080/02640410600944709); abstract via Europe PMC | Mathematical solo time-trial modelling with hypothetical riders supports considering more power uphill/into headwinds and less downhill/with tailwinds. | Predicted savings or tolerated power variation are not established for this rider, a bunch race or Zwift physics. |
| [Poole et al. (2016), critical power review](https://doi.org/10.1249/MSS.0000000000000939); abstract via Europe PMC | Physiological review supports CP as an intensity-domain boundary and CP/W′ as a power-duration framework. | CP is not indefinitely sustainable power; the review does not validate this rider's parameters or equate CP and FTP. |
| [Skiba & Clarke (2021), W′ balance considerations](https://doi.org/10.1123/ijspp.2021-0205); abstract via Europe PMC | Review reports mixed predictive results and distinct assumptions in integral/differential formulations. | Do not treat all formulations as equivalent or a W′ balance plot as a measured fuel gauge. |
| [Caen, Boone & Lievens, W′ recovery limitations](https://www.jsc-journal.com/index.php/JSC/article/view/743); published abstract, publication date not established in this pass | Research synthesis identifies recovery-intensity, preceding-effort and individual-fitness effects and limitations of predictive recovery models. | No universal recovery time or individual reconstitution rate is established. |
| [Babault et al. (2020), fixed-gear criterium pacing/positioning](https://pmc.ncbi.nlm.nih.gov/articles/PMC7739637/); full text via Europe PMC | Observational study of an elite fixed-gear criterium associates race position with finishing position and describes variable pacing. | Association is not causation; real-world cornering/technical demands and this field do not map directly to Zwift. |
| [GoldenCheetah science explanations](https://www.goldencheetah.org/); relevant power-duration, W′ balance and NP/xPower sections inspected, unversioned page | First-party explanation of available analytical concepts and model uses. | Explanatory simplifications are not an individual physiological assessment, sequential energy-system switches or universal recovery schedules. Tool availability is not calibration. |

Supporting numerical implementation remains separately scoped E2 work: explicit evidence/parameter boundaries and focused positive/negative tests only when justified by an approved task. There is no model framework, runtime or new application gate selected here, and no coaching application to migrate (M2). Keep live client experiments below distinct from this ordinary human-ridden race-planning/review workflow; reviewing a real race does not require telemetry integration or repeating a controlled baseline.

## Experimental architecture

The owner supplied a summary from another inspection session. The architecture and prototype observations below are retained as handoff evidence, **not reverified implementation facts in this project**.

| Component | Proposed role |
|---|---|
| Zwift client | Compute the controlled rider's movement using game physics; render and emit telemetry. |
| zwift-offline | Session/profile/event support, rider-state exchange, and recorded ghost playback. Not a replacement headless physics engine. |
| Strategy controller | Observe progress/terrain/opponents, choose target power within realistic effort constraints, and log decisions/results. Additional development. |
| Virtual sensor/input adapter | Feed target power/cadence into the client through a verified compatible path. |
| Previous rides | Observed effort evidence, baseline input traces, or fixed replay benchmarks when their format supports it. |

The supplied session identified `PowerMeterTx.py` and `standalone_power.py` in `C:\Users\jackc\git\zwift-offline-oldnapalm` as a partial ANT+ terrain-responsive prototype. It reportedly uses three power levels, not a fatigue-aware racing strategy. Reinspect those files and verify hardware/client compatibility before adopting them.

The separately inspected netbrain/zwift documentation says ANT+ is unsupported. Its multi-instance support therefore does not establish compatibility with that prototype. See [the ecosystem evidence](../reference/ZWIFT_ECOSYSTEM.md).

## Two distinct questions

1. **Pacing against a fixed benchmark:** one controlled client can compare constrained power strategies with an earlier outcome or recorded trajectory.
2. **Tactics against responsive opponents:** requires independently controlled opponents or an interactive simulator. Replaying an activity does not make its rider chase, react, or change its recorded trajectory.

Replaying a **power trace** feeds recorded inputs into a new simulation, which can produce a different result. Replaying a **trajectory** presents recorded outcomes; it is not a resimulation from watts. Whether a ghost setup affects drafting must be tested, not inferred from visibility.

## Inputs and route data

- FIT: inspect actual fields and sampling; power/cadence/position are not guaranteed.
- Native zoffline ghost: potentially supplies internal road IDs and road positions for playback; inspect schema and provenance.
- Activity protobuf: may contain activity metadata and embedded FIT; inspect rather than promise conversion.
- GPX/summary exports: may support location/summary analysis but not power-strategy reconstruction.

Client route definitions provide numeric identity, distance, lead-ins, and course structure. Extracted checkpoints may support route progress and elevation profiles after coordinate units and axis/segment semantics are verified. They do not supply the complete aerodynamic, drafting, cornering, or rolling-resistance model.

The finder is this combined project’s race-selection component and owns route collection/import through the existing database modules. Recorded review does not require finder changes or repairs; any actual finder changes use their applicable validation gates. When a concrete review task needs route/event data, preserve an explicit snapshot with provenance and timing semantics. Review calculations must not write application SQL or commit proprietary assets.

## Meaningful trial protocol

Hold rider attributes/equipment, route, lead-in, start conditions, opponent setup, game/source versions, and timing boundaries constant. Define an effort constraint before comparing strategies; unrestricted watts makes the experiment meaningless. Capture finish time/gap, effort spent, peak demands, and the assumptions behind any fatigue estimate.

Suggested experimental progression, after the coaching workflow:

1. One client and verified input/telemetry loop.
2. Repeat a baseline to quantify variability.
3. Compare a few constrained strategies, such as steady effort versus terrain-responsive pacing.
4. Add fixed replay benchmarks and verify their actual drafting behavior.
5. Only then consider responsive opponents and multiple clients.

Do not assume faster-than-real-time or headless execution. A lightweight independent simulator could later support broader search, but would require its own calibration and cannot be presented as Zwift's actual physics.

## Integration evidence gates

Validate this project's actual inputs and outputs, not another project's backlog:

- Preserve source identity, fractional quantities and distinct event/free-ride/meetup lead-ins when supplied data includes them.
- Keep declared event time separate from lap-derived distance and finish assumptions. A fixed-time race does not become fixed-distance just because laps are present.
- Require nonempty approved benchmark inputs and explicit exclusions before claiming coaching or simulation accuracy.
- Prefer structured inputs. Any optional OCR or inherited importer needs its own scoped qualification before use; it is not a prerequisite or an assigned repair task.
- Begin activity intake read-only with explicit private paths.

The earlier [session handoff](../reference/SESSION_HANDOFF_2026-10-30.md) is historical reference only, not an implementation dependency or active checklist.

## Scope safeguards

No container launches, driver changes, account setup, autonomous riding, personal-data conversion, or model calibration are authorized merely by creating this project. Keep controlled trials separate from public competitive events and real-race accuracy evidence.
