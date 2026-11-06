# Zwift Race Coach

## Scope

Build practical ride review and pre-race pacing assistance first. The sibling `fitness-bridge` owns sensor acquisition, processing and BLE/ANT+ output; keep its implementation backlog there. Life Fitness owns custom hardware, Pico firmware and physical control. Add controlled experiments only after the baseline input/telemetry path is validated. The sibling `zwift-race-finder` owns event selection and route-data extraction/import; do not edit it implicitly from a coaching task.

## Working directory

Run Pi and project commands from the canonical `~/git/zwift-race-coach` root. Keep other repositories and their sessions separate. No implementation runtime or build/test commands have been selected yet; do not manufacture a scaffold or passing test count.

## Design and change-specific validation

Before substantive coaching changes, read the [first ride-review contract](docs/explanation/FIRST_RIDE_REVIEW.md), including its adopted standards and implementation checklist. No coaching implementation or application gate exists yet. Consult [STATUS.md](STATUS.md) and [TASKS.md](TASKS.md) for intake progress and runtime-selection work; do not assume every task starts by obtaining the first activity again.

For pacing recommendations and before/after reviews, also read the [race-planning and review standards](docs/explanation/PACING_AND_RACE_EXPERIMENTS.md#race-planning-and-review-standards--d2). They own objective selection and preservation of the pre-race plan; the first-review contract owns evidence/capacity distinctions and physiological-model qualification. Do not make tool development or model calibration a prerequisite for a conditional plan.

The selected local testing-technique example is [BotInput](scripts/gymnasticon_bot_input.cjs) with [its tests](scripts/test_gymnasticon_bot_input.cjs): complete-sample validation, an injected clock and exact boundary assertions. Do not inherit its radio ranges, expiry policy or expired-to-zero behavior for activity analysis. Select the first justified coaching metric and matching tests as the maintained coaching example when implemented.

For historical receiver/configuration changes, use the two radio-free Python checks in [scripts/README.md](scripts/README.md); for helper changes, use its Node helper test; for patch-generator changes, use its syntax check. Inspect effects before running commands, use existing runtimes, and report anything not executed. Adapter integration additionally needs an explicitly supplied candidate; do not generate or deploy one implicitly. These checks are not coaching or hardware acceptance.

Documentation-only changes require scoped diff/consistency review and inspection of essential local link destinations, not diagnostic tests. For retained acceptance claims, record the date, checked source checkpoint and relevant dirty state, named check/result and material limits in `STATUS.md` or linked durable evidence. Keep private evidence outside Git; do not rely on temporary output alone.

## Data and evidence

- Never commit rider activities, profiles, credentials, proprietary client exports, or generated recordings. Use explicit private input paths and synthetic test fixtures.
- Inspect an approved activity's actual fields before promising reconstruction or conversion. FIT is not automatically a native Zwift ghost.
- Distinguish observations, inferred capacity, and unverified fatigue/physics assumptions. Fixed replay opponents do not react to strategy changes.
- Preserve numeric route IDs, units, fractional precision, null/zero distinctions, source provenance, timing boundaries, and event lead-in semantics.
- Validate device/input compatibility and client telemetry before strategy comparisons. Do not assume headless, accelerated, multi-client, or offline-server compatibility.
- Do not run virtual sensor output against public competitive events. Device/driver changes, account use, external writes, and destructive actions require appropriate approval.

## Ownership

Read `STATUS.md` and `TASKS.md` before continuing setup or implementation. `README.md` defines the workflow and boundaries; `STATUS.md` records verified state; `TASKS.md` holds unresolved work; `docs/` holds evidence and design decisions. Follow machine-wide instructions, including uv-managed Python if Python is selected. Retain the owner's approved dual-license template; it does not grant rights to third-party code or data.
