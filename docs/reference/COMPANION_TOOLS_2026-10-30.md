# Companion applications and services — 2026-10-30

Historical Coach evidence retained during consolidation. Current state and ownership
are in the root [status](../../STATUS.md) and [documentation map](../README.md). Dated claims below are not current acceptance.

## Research scope

Owner explicitly requested the web-search skill. Used
`~/git/agent-skills/skills/web-search/web-search.mjs` for discovery, including
official product/support pages; read selected GitHub READMEs and metadata with
the GitHub API. Search providers were Brave, Tavily, Exa and Spider via the managed
launcher (some Brave requests fell back after rate limiting).

Evidence levels:
- **README inspected:** selected primary repository documentation was read.
- **Official search evidence:** official page URL and search extract inspected,
  not a full-page/source audit.
- Neither establishes operational compatibility, safety, numerical accuracy,
  service entitlement or maintenance quality. No installation, sensor output,
  account connection, ride-data upload or device operation was performed.
- Websites and default branches change. Prices and subscription requirements were
  not systematically checked. Avoid assuming that an old “apps tested” list
  proves a third-party service still exists.

## Owner constraints and later verification

Owner prefers offline/local tools and no payments beyond Zwift. GoldenCheetah is
the primary analysis candidate; cloud training services below are catalogue
references, not selected dependencies. TrainerRoad is excluded. The official
Sauce packaged product explicitly requires Patreon; its newer GPL source is a
separate source-build question. See [actual Sauce download/headless/offline
review](EVIDENCE_LIBRARY.md#sauce-download-review-2026-10-30) and the [expanded local bridge
comparison](EVIDENCE_LIBRARY.md#bridge-comparison-2026-10-30) before adoption.

## Research shortlist (not adopted dependencies)

| Need | First candidates | Why |
|---|---|---|
| Private/local ride review and rider-capacity modelling | GoldenCheetah | Open-source desktop analysis; power-duration/critical-power/W′ analysis is relevant to constrained pacing. |
| Training calendar, workout construction and convenient Zwift exchange | Intervals.icu | Official announcement documents planned-workout uploads and completed-activity downloads; builder advertises ZWO/FIT/MRC/ERG. |
| Course-specific variable-power pacing | Best Bike Split | Explicit Zwift route analysis and race-plan-to-Zwift workout support. |
| Live race context and programmable telemetry | Sauce for Zwift | Inspected README documents encrypted watching client, overlay widgets and REST/WebSocket API. |
| Real-bike sensor bridge and programmable synthetic power | Gymnasticon | Inspected README documents BLE/ANT+ rebroadcast and bot power/cadence controlled over UDP. |
| Broader equipment bridging, possibly Direct Connect | QZ / qdomyos-zwift | Inspected README documents supported equipment classes, Zwift compatibility and Wahoo Dircon output. |
| Small BLE simulator/protocol reference | Zwack | Explicit synthetic power/cadence/speed and partial FTMS support. |
| Baseline client/device troubleshooting | Zwiftalizer | Log-file analysis of graphics, sensor and network behavior, rather than physiological coaching. |

These serve different layers. None is verified here as a responsive multi-opponent
Zwift race simulator. Do not mistake a structured workout or route-time predictor
for tactical race simulation.

## Analysis, planning and coaching

### GoldenCheetah

- Official: <https://www.goldencheetah.org/>
- Source/documentation: <https://github.com/GoldenCheetah/GoldenCheetah>
- Training-zone reference already in the digital garden:
  <https://github.com/GoldenCheetah/GoldenCheetah/wiki/UG_Preferences_Athlete_Training-Zones>
- **Official search evidence:** critical-power chart, W′ balance, workout editor,
  ride/interval comparisons and metric trends. GitHub HEAD observed:
  `03e1666e7af1d781d8ae675b7b24adc902c4b481`; implementation not audited.

**Fit:** strongest local analytical reference before writing our own power-duration
or effort/recovery visualizations. Inspect approved FIT/activity fields and export
options first. CP is not automatically FTP; W′ balance is a model, not a direct
measurement of fatigue. A model needs representative rider data and disclosed
assumptions. No claim of direct current Zwift cloud sync is made here.

### Intervals.icu

- <https://intervals.icu/>
- Official integration announcement:
  <https://forum.intervals.icu/t/intervals-icu-and-zwift-integration-is-live/81764>
- Workout builder:
  <https://www.intervals.icu/features/workout-builder/>
- Zwift announcement, including BBS:
  <https://forums.zwift.com/t/intervals-icu-and-best-bike-split-now-connected-with-zwift/641651>

**Official search evidence:** uploads planned workouts to Zwift, downloads
completed activities after account authorization; builder supports structured
targets and ZWO/FIT/MRC/ERG import/export.
**Fit:** practical training diary and workout delivery, and a reference for
coach-generated workouts. Cloud account linking/upload is a separate action.
Not evidence of live race control or zwift-offline support.

### Best Bike Split

- <https://www.bestbikesplit.com/>
- Zwift routes: <https://www.bestbikesplit.com/zwift-routes>
- Official export guide:
  <https://support.bestbikesplit.com/article/75/how-do-i-load-my-best-bike-split-race-plan-into-the-zwift-application>
- Race-course rehearsal:
  <https://www.bestbikesplit.com/blog/offseason-triathlon-cycling-training-with-best-bike-split>

**Official search evidence:** physics-based variable-power course planning and
predicted times; Zwift course analysis; race plans can become Zwift workouts.
**Fit:** closest ready-made comparison for our pre-race pacing output.
Validate rider parameters, route provenance/lead-in and event timing.
Do not interpret a predicted solo course time as a validated bunch-race result,
or workout rehearsal as reactive opponent simulation. Exact route coverage,
plan limits and subscription entitlement not audited.

### Managed training alternatives

| Tool | Official evidence link | Documented role / limits |
|---|---|---|
| TrainerRoad | <https://www.trainerroad.com/zwift-integration> and <https://support.trainerroad.com/hc/en-us/articles/33803500565403-Syncing-Zwift-with-TrainerRoad> | Daily scheduled indoor cycling workouts sync into Zwift; official support excludes ramp/FTP tests. Adaptive training rather than tactical race simulation. FTP settings require attention; do not assume all account parameters sync. |
| JOIN | <https://help.join.cc/hc/en-150/articles/29817821247121-How-to-Export-Workouts-to-Zwift> and <https://help.join.cc/hc/en-150/articles/29817874646417-How-to-Import-Activities-Directly-from-Zwift-into-JOIN> | Official guides describe direct workout export and completed-activity import. Useful managed-plan option, not a sensor-input library. |
| TrainerDay | <https://trainerday.com/blog/using-trainerday-workouts-in-other-platforms> | Official blog documents workout-file download and paid calendar/send-to options. ZWO/file workflow is the evidence here; a direct Zwift cloud integration was not established. |
| What's on Zwift | <https://whatsonzwift.com/workouts> and <https://whatsonzwift.com/worlds> | Workout/training-plan catalogue and route/segment browsing. Useful human discovery, not authoritative route import; race-finder retains data ownership. |

All four entries are official search evidence, not full account/UI trials.
Do not subscribe or authorize accounts as a research side effect.

## Live telemetry and diagnostics

### Sauce for Zwift — more than an overlay

- Verified repository: <https://github.com/SauceLLC/sauce4zwift>
- Inspected README; observed HEAD:
  `bb53edfc5050150cd669e56fc60f6d5726a534cd`.
- README says that since July 2022 it uses a full encrypted game client that
  watches the normal account and requires a second login (free account described).
- Documents nearby riders, gaps, draft-related metrics, rolling/peak statistics,
  maps and overlay customization.
- **Programmatic hook:** built-in web server, configurable default port 1080,
  REST directory `/api` and WebSocket event endpoint `/api/ws/events`.
  This is a concrete integration lead; inspect actual schemas and data rights.
- Documents a Sauce headless web-server mode. This does **not** establish that
  the Zwift game client can run headlessly, accelerated or unattended.
- Verify access control/listen addresses before exposing its web server.
  Avoid credentials in command arguments or logs.
- Current account/subscription requirements and zwift-offline compatibility
  need explicit inspection; not established by this pass.

**Correction to earlier conversation:** the previously supplied Sauce website URL
was not verified. Use the verified repository above for navigation.

### Zwift Activity Monitor — exclude from current-client recommendations

- <https://github.com/ruffk/ZwiftActivityMonitor>
- README inspected at observed HEAD
  `a3576fe04a8107acae0b26fb79399081c8d239c7`:
  **“This project is no longer supported due to Zwift switching to packet level
  encryption! Try Sauce for Zwift.”**
- GitHub not being marked archived does not mean an application is supported.
- Similarly, our newly cloned `zwift-packet-monitor` is only a historical/
  protocol research reference unless current encrypted-protocol support is
  independently established. Offline compatibility is a separate question.

### Zwiftalizer

- Search verified the classic site: <https://classic.zwiftalizer.com/>
- Explanatory sources:
  <https://zwiftinsider.com/zwiftalizer-2/>
  and <https://zwiftinsider.com/zwiftalizer-tutorial-and-analysis/>
- Search evidence: log-file graphics/framerate, sensor and network diagnostics.
  Useful to establish repeatability before pacing comparisons.
- Check log contents and local/cloud processing behavior before any approved
  upload. Not a live rider-power API or a coaching engine.

## Input bridges, emulators and physical-control references

### Gymnasticon

- Original: <https://github.com/ptx2/gymnasticon>
- README inspected, HEAD `e142f539c518d6d4ced3abdd587a825b918a9367`.
  GitHub reported last push in 2022; not marked archived.
- Connects to supported proprietary bike consoles and rebroadcasts standard BLE
  power/cadence, optionally ANT+ with USB stick. README lists Flywheel, Peloton,
  Schwinn IC4/IC8/Bowflex C6 and Keiser; LifeFitness IC5 only “probably works.”
  **That does not establish Life Fitness 95Ri or custom Pico support.**
- **Particularly useful:** CLI documents `--bike bot`, `--bot-power`,
  `--bot-cadence`, `--bot-host`, `--bot-port` for UDP power/cadence control.
  Candidate application-controlled test input without modifying an offline server.
- BLE client/server multi-role hardware or separate adapters can be necessary.
  A power-meter broadcast does not imply controllable trainer/FTMS resistance.
- MIT declaration inspected.

#### GymnasticonV2: newer fork, not automatically safer

- <https://github.com/4o4R/gymnasticonV2>
- README inspected, HEAD `b0976cffab7b6c2d10cc1c6f22b2f69a8f860348`;
  metadata showed an October 2026 push.
- Documents modern Pi images, dual adapters, BLE/ANT+ rebroadcast and bot mode.
- Still explicitly pins **Node 14.21.3**, an obsolete runtime, for native
  stack/legacy hardware compatibility. Installation examples enable services
  and capabilities. Do not run one-line installers or globally grant raw-network
  capability to Node based on a README. Audit source/runtime and select an
  isolated supported topology first.

### QZ / qdomyos-zwift

- <https://github.com/cagnulein/qdomyos-zwift>
- README inspected, observed HEAD:
  `1b2441edaf27a39b64bd497733285ba4be409160`.
  GPL-3.0 declared; metadata showed a current push, not an operational endorsement.
- Broad bike/treadmill/elliptical/rower bridge. README documents Zwift compatibility,
  equipment-dependent resistance control, ZWO workouts, and **Wahoo Dircon** to
  send data over Wi-Fi. MQTT and OSC also appear in its feature table.
- **Fit:** strongest broad equipment/Direct-Connect lead to inspect against the
  netbrain ANT+ limitation. Device-specific paths must be checked individually.
- Includes estimated wattage from HR/cadence for some bikes: distinguish that
  explicitly from calibrated power. Software cannot make a mechanically
  uncontrollable bike support automatic resistance.
- Current app-store pricing/device support not audited.

### Zwack

- <https://github.com/paixaop/zwack>
- README inspected, HEAD `0abecc5f049770d3ac37d4a3212abaf50794b994`.
- Synthetic cycling power/cadence, running speed/cadence, and **partial FTMS**.
  Keyboard-adjustable simulated values. Describes a target-power test path
  using a Wahoo extension, not a claim of complete FTMS behavior.
- README requires simulator and receiving fitness application on different
  computers; Windows is not tested there. BLE/native dependency review required.
- MIT declared. Useful protocol test fixture/reference, not calibrated power.

### Hardware-specific leads (original search evidence)

**Later source follow-up:** [expanded bridge comparison](EVIDENCE_LIBRARY.md#bridge-comparison-2026-10-30)
now includes local FortiusANT and Zwack source, QZ multi-sport profiles and pinned
upstream SmartSpin2k FTMS/Dircon/ERG source. In particular, Zwack's FTMS command
forwarding is disabled in the inspected implementation. The table below preserves
the initial discovery summary, not the newest evidence level.

| Tool | Link | Role and boundary |
|---|---|---|
| FortiusANT | <https://github.com/WouterJD/FortiusANT> | Converts compatible older USB/ANT Tacx trainers for modern training apps. Relevant protocol/control reference, not generic Life Fitness support. |
| SmartSpin2k | <https://github.com/doudar/SmartSpin2k> | Adds physical resistance actuation to compatible spin bikes for smart-trainer behavior. Hardware project, not a software-only bridge. Mechanical fit/control safety are separately scoped. |
| SHIFTR Hardware | <https://github.com/Theo-Marshall/SHIFTR-Hardware> | Prior digital-garden BLE-to-Direct-Connect/virtual-shifting lead; not further inspected in this web pass. |
| MyBleTrainer | <https://github.com/roethigj/MyBleTrainer> | Prior digital-garden sensor-to-FTMS lead; not further inspected here. |
| FTMS emulator | <https://ftmsemu.github.io/> | Prior digital-garden exercise-bike emulator lead; not further inspected here. |

## What “simulate” means in this shortlist

Keep four distinct jobs separate:

1. **Capacity/effort models:** GoldenCheetah and training services help describe a
   rider; inferred capacity is not a direct fatigue measurement.
2. **Course/pacing prediction:** BBS models a route and can produce a workout.
   That does not reproduce real bunch tactics or reactive competitors.
3. **Sensor/transport simulation:** Gymnasticon bot and Zwack provide synthetic
   inputs; Zwift supplies its own client-side movement/physics.
4. **Controlled race environment:** zwift-offline + verified client inputs and
   telemetry remains a separate integration. Recorded ghosts are fixed; merely
   calling them bots does not establish independent tactical control.

Cloud workout integrations target the official Zwift service, not automatically
zwift-offline. BLE/ANT+ bridges, Direct Connect and account APIs are different
interfaces, not interchangeable components.

## Smallest useful next investigations

1. Compare GoldenCheetah and Intervals.icu on the desired review output before
   implementing equivalent analytics. Use only an explicitly approved activity.
2. If useful without extra payment, inspect a BBS pacing example against an explicit supplied course/event scenario; no finder changes or integration required.
3. Inspect Sauce's API schemas for required live fields, account boundaries and
   whether an offline mode is actually feasible.
4. Compare **Gymnasticon bot over BLE** versus **QZ Dircon** for the selected
   one-client topology. Pick one transport, not a universal adapter framework.
5. Establish baseline with synthetic inputs only in controlled/offline testing;
   never transmit synthetic competitive power into public races.
6. Consider physical resistance integration only in the owning bike-controller
   project, with its current interfaces and explicit task scope.

See also [repository/digital-garden notes](EVIDENCE_LIBRARY.md#repository-research-2026-10-30)
and [existing ecosystem inventory](EVIDENCE_LIBRARY.md#zwift-ecosystem).
