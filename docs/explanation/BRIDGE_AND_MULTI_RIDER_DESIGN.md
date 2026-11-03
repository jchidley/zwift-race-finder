# Coach integration and controlled multi-rider experiments

## Ownership after the Fitness Bridge handoff

[Fitness Bridge](https://github.com/jchidley/fitness-bridge) now owns acquisition, source selection, filtering/transformation, synthetic measurement sources and BLE/ANT+ output. Its Rust/Linux design and implementation backlog are authoritative there. The published repository is a design/reference baseline, not a working bridge.

This project owns ride reviews, pacing plans, effort constraints, higher-level strategy and experiment coordination. Life Fitness owns custom hardware, Pico firmware and physical resistance. Race-finder remains independent; route/event data enters coaching as an explicit supplied snapshot, not a shared writable database.

The [first recorded-ride review](FIRST_RIDE_REVIEW.md) needs no bridge, sensor pairing or live client. Do not make the hardware backlog a prerequisite.

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
