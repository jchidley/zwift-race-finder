# Repository and written-reference research — 2026-10-30

## Scope and evidence

Read-only inspection of the four newly cloned repositories, paginated GitHub
repository metadata for `jchidley` (public listing and authenticated owner listing),
selected relevant READMEs/source, and `~/git/mkdocs-material-test/docs/`.
Notes only: no dependencies, captures, devices, accounts for riding, servers,
containers or simulations were run; no remote writes were made.

GitHub references below use inspected default-branch commit IDs. Local references
describe the working files inspected alongside their checkout HEAD; local changes
were not exhaustively audited. README claims are not operational validation.
Private-project references are navigation only: no activities, recordings,
credentials, profiles or proprietary exports were copied. Review these references
before any future public publication.

## Most useful conclusions

1. **First ride review:** keep the approved activity + upcoming-event workflow.
   Race-finder owns route/event inputs; GoldenCheetah's training-zone documentation
   is a useful discovered reference, not a substitute for inspecting actual fields.
2. **Telemetry (corrected after follow-up):** do not recommend old packet
   monitors for the current encrypted client without demonstrated support.
   Zwift Activity Monitor explicitly discontinued support due to encryption;
   Sauce documents an encrypted watching client and REST/WebSocket API. See the
   [companion research](COMPANION_TOOLS_2026-10-30.md). Offline compatibility
   remains a separate, unverified question.
3. **Controlled inputs:** the oldnapalm copy already contains the reported
   ANT+ transmitter prototype. Inspect/adapt that before inventing another one.
   The cloned python-ant is legacy Python, not a proven current dependency.
4. **Alternative transport:** digital-garden pointers to BLE simulation and Direct
   Connect bridging deserve investigation because the previously inspected
   netbrain/zwift docs did not support ANT+. No bridge is validated here.
5. **Physical bike evidence:** the current Pico controller is a separate project
   with a supported host recorder and explicit calibration limitations.
   Consult its documented interfaces rather than opening a competing serial reader.

## Four new local checkouts

All paths are under `/home/jack/git/`.

| Repository / inspected HEAD | Evidence and relevance | Limits |
|---|---|---|
| [wemakewaves/PyRow](https://github.com/wemakewaves/PyRow/tree/69dd5ada5d748564f82dc3bb28e1e4e815df2952) — `PyRow/` | README documents Concept2 PM3/PM4 USB/CSAFE access: time, distance, stroke rate, watts, pace, heart rate and optional force plot. Useful rowing/input-protocol reference; existing ecosystem notes record vpower's separate PyRow notice. | README says incompletely tested, tested with Python 2.7.2; `pyrow.py` has Python-2 print syntax. PM5/current Python compatibility is not established. Not a Zwift transport by itself. README declares Simplified BSD. |
| [mch/python-ant](https://github.com/mch/python-ant/tree/02e045825434a17ffe113a82cc8191683223ea5f) — `python-ant/` | ANT/ANT+/ANT-FS over USB ANT stick. Tree includes `src/ant/plus/PowerMeterTx.py`, `SpeedTx.py`, heart rate, trainer, rower and FE-C-related modules. `setup.py` depends on pyserial and PyUSB. Relevant to the existing prototype. | Packaging advertises Python 2.6/2.7; current interpreter, USB driver and profile behavior untested. Module presence does not prove complete profile support. MIT/X11 declared. |
| [pyusb/pyusb](https://github.com/pyusb/pyusb/tree/3241c0e6a7debafd7597925e8403d9d3ea02ed93) — `pyusb/` | Current checkout README requires Python >=3.9 and a USB backend, recommending libusb 1.0. USB plumbing for device-specific integrations. | Not ANT+, BLE, CSAFE or Zwift protocol logic. A modern PyUSB does not automatically modernize its older consumers. Backend/permissions/USB passthrough remain separate checks. |
| [jeroni7100/zwift-packet-monitor](https://github.com/jeroni7100/zwift-packet-monitor/tree/1b5cb35b37831c81bd672bbcba0473e843237e98) — `zwift-packet-monitor/` | Node package `@zwfthcks/zwift-packet-monitor`, version 0.6.0. Source captures UDP 3022/TCP 3023 using `cap`, decodes protobuf and emits incoming/outgoing player states. Schema includes world time, distance, speed, cadenceUHz, heart rate and power. | Requires libpcap/Npcap and an appropriate capture interface/permissions. Current schema, units, encryption and topology compatibility untested. `npm test` explicitly fails with “no test specified”; live examples are not offline regression coverage. Package declares ISC; no top-level license file found in shallow inventory. |

### Packet-monitor compatibility concern

`ZwiftPacketMonitorSource.js` opens the filter
`udp port 3022 or tcp port 3023`. The inspected offline-server READMEs instead
document UDP 3024 and TCP 3025. This is a concrete reason **not** to assume
drop-in offline compatibility. Changing a filter alone may not handle packet
framing, encryption or schema differences.

Before adoption, compare an explicitly approved capture/server-side stream with
the selected client and server version. Establish timestamps, sampling, units,
loss and player identity. Restrict collection to required fields; chat and other
riders' states are additional privacy surfaces. Do not start a privileged live
capture just to test installation.

## Relevant repositories on jchidley's GitHub

### Direct references

| Repository / inspected commit | Findings and coaching role |
|---|---|
| [zwift-race-finder](https://github.com/jchidley/zwift-race-finder/tree/ad3e87c19010e5b33de873a148f5f590acfbf570) | Rust race-duration/event selection. README points to physics, tactics, architecture and data-import docs. Tree includes `docs/explanation/ZWIFT_PHYSICS.md`, `ZWIFT_PHYSICS_EQUATIONS.md`, `ZWIFT_RACING_TACTICS.md`, `docs/howto/ZWIFT_OFFLINE_INTEGRATION.md` and OCR utilities. These are pointers, not newly audited accuracy claims. Keep route extraction/import and production model ownership there. |
| [zwift-offline](https://github.com/jchidley/zwift-offline/tree/177419ca6618c242821db4d5cc68487ffcd0a3f4) | README identifies fatsbrown/zwift-offline as active upstream; GitHub's fork parent still identifies zoffline/zwift-offline. Documents partial offline server, optional multiplayer, ghosts, bots, RoboPacers and pelotons. Peloton behavior is an upstream description, not verified independent tactical agents or physics. GitHub metadata declares AGPL-3.0. |
| [zwift-offline-oldnapalm](https://github.com/jchidley/zwift-offline-oldnapalm/tree/25207af053d5876ce17fcdeb8837cb0612299fdb) | Independent history-preserving copy. Contains `standalone_power.py` and `PowerMeterTx.py`; selected implementation inspected below. Useful controlled-input baseline, not an approved runner. |
| [life_cycle](https://github.com/jchidley/life_cycle/tree/6e35171bff53c4cd9e314807f279c8cf99084ce4) — private | README owns historical Life Fitness 95Ri board evidence, measurements and reverse engineering. Points to the sibling replacement-interface investigation. Explicitly warns historical pin/interface claims are not a validated build procedure. Treat as provenance, not the current integration target. |
| [life-fitness-pico-controller](https://github.com/jchidley/life-fitness-pico-controller/tree/5d997e175d1d50467f2d40c5d346ed37c02739d1) — private | Current standalone 95Ri Pico/Embassy sensing/manual excitation and native host recording; C1 is a separate system. README points to `ARCHITECTURE.md`, `software/standalone-bike/HOST_SERVICE.md`, `RECOVERY.md` and supported-path matrix. Local canonical workspace is `~/git/life-fitness-cycle-controls-investigation`, not the GitHub repository name. Only README reviewed here: follow current owning-project docs before any actual integration. |
| [zwift-race-coach](https://github.com/jchidley/zwift-race-coach/tree/e9982b0eb5de6927b337376afe9572cb05ec2e0d) — private | Remote exists with the coaching description and a README. This contradicts the older local status statement that no remote repository exists. Local coach root is not a Git worktree. Remote existence does not establish who performed setup, settings, license completeness or local reconciliation; no initialization/pull/push attempted. |

### Existing power prototype: concrete findings

In the oldnapalm copy at the pinned revision:

- `standalone_power.py` imports `ant.core`, USB discovery and the sibling
  `PowerMeterTx`. Its UDP handling chooses increased/decreased/default power
  and calls `power_meter.update(power)`.
- It also includes offline-server networking, packet encoding/decoding and
  background threads. This is not a small side-effect-free library.
- `PowerMeterTx.py` emits ANT+ standard power-only page 0x10, including event
  count, cumulative/instantaneous power, and optional cadence. That is
  simulated sensor output, not a physical trainer's resistance controller.
- Transmitter identity uses the Windows `wmic cpu get ProcessorId` command
  with a fixed error fallback; `standalone_power.py` imports `win32api`.
  Portability and distinct sensor identity require review before multi-client use.
- Offline READMEs state two ANT sticks plus python-ant are required.
  This is a documented setup claim, not a test on this machine.
- The script adjusts a client's synthetic power while recording a pacer;
  subsequent ghost playback is still fixed. Do not confuse recording-time
  feedback with a replay opponent reacting to a new strategy.

Pico-controller README limits are also important: host owns the one USB
connection; recording is best effort, only synced history durable, replay does
not execute saved commands, and recorded acknowledgements are not proof of
physical PWM or calibrated power. INA averaged voltage is not waveform power,
and supply current is not winding current. Do not import its project-specific
operating permissions into this coaching task.

### Secondary reference, not a dependency recommendation

[heatpump-analysis](https://github.com/jchidley/heatpump-analysis/tree/8bab65336ab5362d0bc5444e23895be7a018e405)
has README-level patterns worth revisiting for time-window review, explicit gap
reporting, opt-in simulated data, holdout validation and reproducibility snapshots.
The useful analogy is evidence handling, not its thermal model or infrastructure.
Do not introduce Polars, a TSDB or its operating-state abstractions just for parity.

Other account repositories were screened by name/description; unrelated personal,
agent-state and business data were not investigated. This is a relevance screen,
not an exhaustive source audit of every repository.

## Written information in the digital garden

Local root: `/home/jack/git/mkdocs-material-test`; inspected HEAD:
`080513ebc5b620afd31b5642ad3b3c0512fde69e`.
Its `AGENTS.md` explicitly classifies it as historical experience, thematic
writing and curated references, not live implementation truth. No files there
were edited and no site build was needed.

### Main source: January 2026 curated links

[`docs/2026-01-21-Curated-Links---January-2026.md`](../../../mkdocs-material-test/docs/2026-01-21-Curated-Links---January-2026.md),
especially **Cycling/Fitness** and **Electronics**. The page explicitly says it
was generated from browser-history analysis with AI-generated summaries.
The following are discovered pointers; external implementations were **not**
inspected in this research pass:

| Pointer | Why revisit it | Qualification needed |
|---|---|---|
| [paixaop/zwack](https://github.com/paixaop/zwack) | Candidate BLE synthetic sensor output for controlled tests; potentially relevant where ANT+ is unavailable. | Verify current BLE profiles, adapter/OS support, advertising and actual client pairing. A BLE broadcaster is not automatically a bidirectional FTMS trainer. |
| [Theo-Marshall/SHIFTR-Hardware](https://github.com/Theo-Marshall/SHIFTR-Hardware) | BLE-to-Direct-Connect and virtual-shifting lead; potentially relevant to netbrain transport constraints. | Check firmware/software counterparts, supported profiles and hardware. Not proof it accepts arbitrary synthetic watts or works with our client layout. |
| [roethigj/MyBleTrainer](https://github.com/roethigj/MyBleTrainer) | BLE sensor-to-FTMS trainer lead. | Verify power model, resistance-control behavior and compatibility; modeled power is not a calibrated measurement. |
| [FTMS emulator](https://ftmsemu.github.io/) | Exercise-bike/nRF52 FTMS implementation lead. | Inspect actual source, license, service/control-point behavior and device assumptions before adopting. |
| [GoldenCheetah training zones](https://github.com/GoldenCheetah/GoldenCheetah/wiki/UG_Preferences_Athlete_Training-Zones) | Reference for rider-specific training-zone definitions in activity review. | Inspect definitions and required rider parameters. Zones do not establish fatigue, threshold or race tactics from incomplete telemetry. |
| [oldnapalm/vpower](https://github.com/oldnapalm/vpower) | Existing ANT+ virtual-power lead; corroborates the prior ecosystem inventory. | Already tracked; avoid a duplicate implementation. Validate inputs and trainer-specific assumptions. |
| [Berg0162/Simcline-V2](https://github.com/Berg0162/Simcline-V2) | Incline-actuation reference, lower priority than review/telemetry. | Physical motion is a separate hardware scope, not needed for the first coaching workflow. |

### Other written sources

- [`docs/Other/2022-03-25-Radio-and-Electronics.md`](../../../mkdocs-material-test/docs/Other/2022-03-25-Radio-and-Electronics.md),
  **General**, points to [Unbricking a $2,000 Bike With a $10 Raspberry Pi](https://ptx2.net/posts/unbricking-a-bike-with-a-raspberry-pi/).
  Useful historical bridge/sensor-design lead. Its **Simulation Software** section
  also emphasizes missing/parasitic model effects and real-world verification.
  External article not inspected.
- `docs/2026-02-10-One-Year-With-LLM-Coding-Agents.md:51` mentions the Life
  Fitness controller-board/Pico work and the need for agents to verify rather
  than guess. Found by text search; not a new hardware specification or a
  full article review.
- Wider text searches found no additional substantive coaching/pacing material
  beyond these references. This was keyword discovery, not a complete manual
  reading of the garden.

## Follow-up priorities (not authorization to execute)

1. Inspect one explicitly approved activity and define the useful review output.
2. Reconcile local/remote coach setup separately before publishing these notes.
3. For later experiments, choose one actual client/server/device topology and
   compare server-side telemetry with packet-monitor feasibility.
4. Inspect zwack, MyBleTrainer, SHIFTR and the FTMS emulator at pinned revisions
   before selecting a BLE/Direct-Connect path. Do not assume they compose.
5. Before reusing ANT code, resolve current Python support, hardware, identity,
   sender/receiver arrangement and netbrain's documented ANT+ gap.
6. Request any bike-host data through its owner-defined export/interface and
   explicit approved private paths; do not import recordings during research.

Related: [existing ecosystem inventory](ZWIFT_ECOSYSTEM.md),
[simulation catalogue](SIMULATION_TOOLS.md), and
[project boundaries](PROJECT_BOUNDARIES.md).
