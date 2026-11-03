# First recorded-ride review

## Deliverable

Produce a useful private review of one approved completed activity. Add a pacing plan when an upcoming event/route and sufficient rider context are supplied. No live bridge, simulator, account login or cloud service is required.

## Inputs to request

1. An explicit path to one activity outside this checkout, with permission to read it. FIT, TCX, GPX or a structured export are candidates; the extension does not guarantee usable fields.
2. The rider's question or objective, if available: pacing consistency, a difficult climb, repeated surges, recovery or another specific concern.
3. For a pre-race plan: a supplied event/route description or snapshot, expected duration/timing format and relevant effort guidance. Preserve numeric route IDs and lead-in semantics when supplied. A reliable recent threshold/effort estimate is useful but must not be fabricated from one ride.

Do not search personal directories for an activity, download account data, or assume files used by another project are approved here. A ride review can proceed without the event details; label the pre-race plan pending.

## Read-only intake

- Identify the actual format, decoder provenance and available record/session/lap fields before selecting the smallest implementation.
- Record timestamp coverage, units, missing values, duplicate/out-of-order records, pauses, sampling intervals and elapsed versus moving time where supported.
- Inspect power, cadence, heart rate, speed, altitude and position independently. Missing power means no power-based analysis; HR alone does not recover watts.
- Keep raw bytes unchanged. Keep input paths, file hashes, private metrics, coordinates and the generated report outside Git. Use synthetic fixtures for published tests.

## Report

1. **Data quality:** present/missing fields, coverage, exclusions, gaps and the time basis used.
2. **Observed effort:** supported time-weighted summaries and selected sustained efforts/surges/recovery intervals, with exact interval boundaries and units.
3. **Interpretation:** what the observations support, plausible alternative explanations and what cannot be concluded.
4. **Practical takeaways:** a few concrete pacing actions tied to evidence, not an invented fatigue model or universal prescription.
5. **Optional event plan:** start/lead-in, terrain-specific effort, recovery opportunities and finish strategy, with explicit rider-capacity and event assumptions. If numerical targets lack support, give qualitative guidance or request the missing evidence.

Examine the effort timeline and relevant intervals before making recommendations from whole-ride averages. Warm-up, main effort, easier riding, renewed effort and final recovery can have different distributions; do not let their combined average alone justify a general pacing or cadence recommendation. Phase labels inferred from power/HR remain provisional interpretations, distinct from rider-confirmed purpose; the recording alone does not establish whether an earlier warm-up was excluded.

Do not equate observed peaks with maximum capacity, identify drafting from speed alone, infer tactical intent, or present a single ride as physiological model validation.

## Acceptance

- The report works with the actual approved fields; unsupported sections are explicitly omitted rather than populated with guessed values.
- Time-weighted metrics account for irregular sampling. Declare gap/interpolation and paused-time policies; do not silently fill missing measurements with zero or carry values across long gaps.
- Duration/effort windows have explicit start/end inclusion rules and adequate coverage. Show sample exclusions and reconcile totals with the selected time basis; do not silently force agreement with device summaries.
- At least representative reported metrics and intervals are independently recomputed from the same declared policy. Synthetic regressions cover missing/zero values, irregular timestamps, gaps and boundary cases once an implementation is selected.
- Every recommendation is traceable to observations or clearly labelled supplied assumptions. State uncertainty rather than promising faster races.
- No personal data or generated private report appears in the Git publication set.

## Adopted standards — 2026-11-03

The owner approved S1–S4 in documentation tranche D1, and clarified S4 and added S5–S8 in documentation tranche D2 on 2026-11-03. These are documented requirements, not evidence of a working implementation or passing checks. This contract owns S1–S4 and S6–S7; the [race-planning and review contract](PACING_AND_RACE_EXPERIMENTS.md#race-planning-and-review-standards--d2) owns S5 and S8.

- **S1 — Independent recorded review:** the activity review is useful without an event plan, bridge, live client or finder changes. Retain the deliverable and ownership boundaries above.
- **S2 — Explicit numerical semantics:** every reported metric uses declared time, coverage, units and missing-data policies. Apply the checklist below before implementation.
- **S3 — Selected examples, limited transfer:** select maintained first-party examples with matching tests for named properties. The historical [BotInput helper](../../scripts/gymnasticon_bot_input.cjs) and [tests](../../scripts/test_gymnasticon_bot_input.cjs) demonstrate complete-sample validation, an injected clock and exact boundary assertions. They are not a coaching architecture: do not copy radio ranges, the expiry threshold or expired-to-zero semantics into activity analysis. No coaching example exists yet; select the first justified metric implementation with its tests rather than creating a demonstration scaffold.
- **S4 — Evidence and privacy:** distinguish a requirement, an inspected check, an executed result and an accepted outcome. Keep private inputs/reports outside Git. Retained acceptance claims need a date, source checkpoint with relevant dirty-state qualification, named check/result and material limits; retain private supporting evidence only in durable private storage with a safe reference. Ignore rules are a backstop, not proof that a publication set is free of private data. Every consequential recommendation must identify its basis: observed performance, rider-supplied context, configured value, model estimate, research finding or tactical hypothesis. For research, retain source/date or version, inspected access level (full text or abstract), study type and relevant population/context, and what does and does not transfer. Do not require a citation for every routine sentence. A supplied prior-work summary is not newly verified evidence.

### S6 — Demonstrated performance is not calibrated capacity

Keep observed power-duration bests, configured CP/FTP/W′, fitted estimates and prescribed targets distinct. A numerical comparison retains the observation period/source, measurement comparability, duration and calculation policy, whether maximal intent is known, and fresh versus prior-work context where known. Unknown context stays unknown; it is not an excuse to fabricate a maximum.

Do not convert a best effort into a repeatable allowance, cross-ride consistency into within-race repeatability, low late power into proven fatigue, or rider-reported HR experience into a measured physiological threshold. CP (critical power) and FTP (functional threshold power) are related concepts, not interchangeable measurements. Qualitative planning can proceed while maximum capacity remains uncertain.

When implemented, represent observations and model parameters explicitly rather than as one unqualified capacity number. Requirement-derived negative tests should reject or clearly label unsupported conversions; they must not claim to mechanically establish physiological truth.

### S7 — Physiological models are optional, qualified tools

A model may inform a decision only within its documented assumptions and qualification limits. Mathematical correctness and individual predictive validity are separate claims. Before relying on CP/W′ balance or another physiological model, record:

- formulation/version, units, parameter values and provenance;
- fitting data and method, intended use and applicability limits;
- relevant validation and sensitivity to uncertain parameters.

Use demonstrated performance and explicit uncertainty by default when a model is unqualified. A configured W′ (the model's finite-work-above-CP parameter) is not an authoritative remaining-capacity gauge. Do not require a new maximal test merely to begin a conditional race plan. Exploratory plots are allowed when labelled and must not silently become prescriptions.

For implemented models, check calculations with independent expected values, units and boundaries. Separately assess individual usefulness against suitable efforts not merely reused to fit the model. Do not claim that a mathematically correct implementation validates recovery kinetics or predicts an individual's failure time. [Research applicability notes](PACING_AND_RACE_EXPERIMENTS.md#research-basis-and-transfer-limits) explain why these qualifications matter.

No first-party physiological-model or race-plan implementation exists. Keep the BotInput example limited to its named testing properties; select the first justified coaching calculation and its matching tests when a real planning question requires code, not a demonstration model framework.

### Implementation checklist

For the selected activity and first metrics:

1. Declare timestamp ordering, duplicate handling, elapsed/moving time and pause semantics. Preserve units, fractional precision and missing versus zero; reject or explicitly exclude unusable records rather than silently coercing them.
2. Declare sample weighting/interpolation, gap treatment, required coverage and interval endpoint inclusion. Select thresholds from the inspected data and intended metric, not an undocumented universal default.
3. Keep decoding/validation separate from metric calculation through the smallest suitable API. Calculations must consume explicitly validated records and the declared timing/coverage policy, not bypass those semantics through raw decoder fields. Select concrete types and files only after intake.
4. Independently calculate representative expected values and exclusions under that policy. Synthetic positive and negative regressions must cover genuine zero, missing power, irregular timestamps, duplicates/out-of-order records, gaps, pauses and exact interval boundaries. In particular, absent power must not become a zero-power observation or a power-based recommendation.
5. Preserve conflicting timer/elapsed summaries and timestamp spans explicitly rather than forcing agreement. Declare which basis each result uses and record discrepancies and limits.
6. Resolve repeated decoded field names explicitly. A null duplicate must not overwrite a populated value; retain source identity where fields are distinct, and reject or report conflicting populated values rather than silently choosing the last one. Add focused synthetic regressions for these cases.

Mechanical enforcement (E1, and D2's separately scoped E2) remains pending: implement the validated boundary and focused regressions with the first justified coaching code after parser/runtime selection. Migration (M1) is not required now because no coaching application exists; historical diagnostics remain historical.

## Implementation decision

Select language, parser and report generation as an explicit implementation decision informed by actual approved-input inspection, not by the mere existence of an export extension. Consult [STATUS.md](../../STATUS.md) for intake progress and current candidates, and [TASKS.md](../../TASKS.md) for pending selection and acceptance work. An exploratory review does not by itself select a runtime or complete implementation acceptance. Hand off supported observations and uncertainty to the [before-and-after race workflow](PACING_AND_RACE_EXPERIMENTS.md#s8--preserve-the-plan-before-evaluating-the-outcome); a reusable tool is not a prerequisite for making that plan. Build only the smallest repeatable workflow justified by the approved example. Existing historical Python/Node/C diagnostics do not select the coaching runtime.

The owner approved the phase-aware interpretation, provisional phase labels, conflicting-time preservation and repeated-field rules above during the 2026-11-03 focused documentation reconciliation. They extend the documented contract only; mechanical enforcement remains pending.
