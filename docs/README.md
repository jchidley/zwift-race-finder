# Documentation

## Purpose and design

- [First ride review](explanation/FIRST_RIDE_REVIEW.md): private input selection, field inspection, report and acceptance criteria. Start here.
- [Coach integration and multi-rider experiments](explanation/BRIDGE_AND_MULTI_RIDER_DESIGN.md): coach-owned strategy/telemetry work and the Fitness Bridge boundary.
- [Fitness Bridge](https://github.com/jchidley/fitness-bridge): authoritative sensor/processing/BLE/ANT+ design and implementation backlog.

- [Pacing and race experiments](explanation/PACING_AND_RACE_EXPERIMENTS.md): practical first milestone, experimental roles, limitations, and evidence needs.
- [Project boundaries and migration](reference/PROJECT_BOUNDARIES.md): which project owns which work and where these notes came from.

## Setup evidence

- [GitHub setup preview](reference/GITHUB_SETUP_PREVIEW.md): target, visibility, original approved template writes, content hashes, and completed repository bootstrap.

## Historical reference

- [Status history through 2026-10-31](reference/STATUS_HISTORY_2026-10-31.md): earlier setup, hardware and pairing chronology; superseded state is historical.
- [Original bridge/multi-rider design — 2026-10-30](explanation/BRIDGE_AND_MULTI_RIDER_DESIGN_2026-10-30.md): byte-preserved source of the Fitness Bridge handoff; not an active implementation plan.

- [Earlier session handoff — 2026-10-30](reference/SESSION_HANDOFF_2026-10-30.md): retained source citations and historical findings. Not a coach backlog, required onboarding or dependency on finder repairs; finder takes care of itself.

## References

- [Historical diagnostic scripts](../scripts/README.md): effects, upstream attribution, radio-free checks and publication-time key removal.

These reports retain dated findings and earlier candidate rankings, not current
implementation tasks. Native Windows is now the live-client baseline; device
qualification and bridge implementation belong to Fitness Bridge.

- [Windows Zwift BLE pairing test](reference/ZWIFT_BLE_PAIRING_TEST.md): power/cadence pulse, return to zero and basic sequential parity with ANT+.

- [Windows Zwift ANT pairing test](reference/ZWIFT_ANT_PAIRING_TEST.md): verified device identity, displayed power/cadence and return to zero on the pre-ride screen.

- [Gymnasticon startup zero and receive-side watchdog](reference/GYMNASTICON_WATCHDOG.md): installed change, tests, rollback and the sender-age timing limitation.

- [Native Windows ANT receive bench](reference/ANT_WINDOWS_RECEIVE_TEST.md): real Pi-to-Windows radio reception, controlled zero/power/cadence steps, counter checks, observed gaps and freshness limitations.

- [Gymnasticon device](reference/GYMNASTICON_DEVICE.md): inspected card, approved first-run/network/key changes, live boot/SSH/service verification, and remaining freshness/startup limitations.

- [ANT USB capabilities](reference/ANT_USB_CAPABILITIES.md): successful direct-USB version/channel/network queries on both NUCs, exact effects and reproducible diagnostic source.

- [QZ SkiErg audit and WYK82Q test plan](reference/QZ_SKIERG_AUDIT_AND_TEST_PLAN.md): live Debian host checks, source-level FTMS/RSC findings, scaling and freshness caveats, and gated Windows-client test procedure.

- [Bridge/bot comparison — 2026-10-30](reference/BRIDGE_COMPARISON_2026-10-30.md): bike/run/row matrix; local Gymnasticon, V2, QZ, FortiusANT and Zwack source; pinned upstream SmartSpin2k; bot fidelity, exact-input/freshness and physical-control boundaries.
- [Sauce actual download review — 2026-10-30](reference/SAUCE_DOWNLOAD_REVIEW_2026-10-30.md): headless source, stable archive inspection, disabled offline-server switch, payment/licensing and runtime qualification limits.

- [Companion applications and services — 2026-10-30](reference/COMPANION_TOOLS_2026-10-30.md): web-researched analysis, training, pacing, Sauce API, Gymnasticon/QZ bridges and simulator shortlist; includes encryption corrections.

- [Repository and written-reference research — 2026-10-30](reference/REPOSITORY_RESEARCH_2026-10-30.md): new USB/ANT/telemetry checkouts, relevant jchidley GitHub projects, digital-garden pointers, and evidence limits.

- [Local Zwift ecosystem](reference/ZWIFT_ECOSYSTEM.md): inspected local checkouts and pinned revisions; earlier netbrain/zwift prioritization is superseded by the native-Windows baseline.
- [Simulation tools](reference/SIMULATION_TOOLS.md): inherited research catalogue, not a tested integration or an approved implementation plan.

Current state is in [STATUS.md](../STATUS.md); unfinished work is in [TASKS.md](../TASKS.md).
