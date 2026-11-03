# Local bridge/bot comparison — 2026-10-30

## Current requirements and later local verification

The [consolidated design](../explanation/BRIDGE_AND_MULTI_RIDER_DESIGN.md) records
subsequent owner requirements: both ANT+/BLE, modified real inputs as well as
bots, SkiErg run/cycle output, and multiple independently controlled profiles.
It also records newly inspected Bleak/Bless/OpenANT building blocks.

**SmartSpin2k follow-up:** its checkout subsequently completed. Local HEAD
`df59de54a85459e1b155a6f1b54c29d159de8481` was verified clean; local source
confirms the FTMS/Dircon findings below. Local `docs/ride-testing.md` and
`scripts/erg_sim/` also document accelerated packet-integrated production
controller experiments and an optional real-BLE bench route. These are useful
simulation references, not a Zwift opponent engine or proof of RF/physical
validation. Earlier “upstream-only/incomplete checkout” statements below record
the initial inspection state, not the current state.

## Decision constraints and scope

Owner prefers local/offline tools and no payments beyond Zwift. Existing
`~/git/life-fitness-cycle-controls-investigation` owns physical resistance,
the native host, Pico USB and recording/dashboard. Do not replace that app
or open a second serial connection merely to provide Zwift inputs.

Compared local source/configuration, not installed behavior:

| Checkout | Origin | Inspected HEAD / package |
|---|---|---|
| `~/git/gymnasticon` | ptx2/gymnasticon | `e142f539c518d6d4ced3abdd587a825b918a9367`, 1.4.1 |
| `~/git/gymnasticonV2` | 4o4R/gymnasticonV2 | `b0976cffab7b6c2d10cc1c6f22b2f69a8f860348`, 3.0.1 |
| `~/git/qdomyos-zwift` | cagnulein/qdomyos-zwift | `1b2441edaf27a39b64bd497733285ba4be409160` |
| `~/git/zwift-offline-oldnapalm` | independent oldnapalm copy | `25207af053d5876ce17fcdeb8837cb0612299fdb` |
| `~/git/python-ant` | mch/python-ant | `02e045825434a17ffe113a82cc8191683223ea5f` |

The three main bridge checkouts were clean when checked. No directory named
`~/git/gymnasticonV2.git` exists. Authenticated paginated owner-repository
listing found no `jchidley` repository with Gymnasticon in its name.

No dependencies/builds/tests, sensor broadcasts, devices, services, drivers or
account sessions were started. Source inspection does not validate client,
OS/radio, offline-server compatibility or physical safety.

## Functional comparison

| Capability | Original Gymnasticon | GymnasticonV2 | QZ / qdomyos-zwift |
|---|---|---|---|
| Main role | Proprietary bike-to-standard sensor bridge | Expanded bike/sensor bridge and Pi appliance | Broad equipment bridge, virtual trainer, workout/recording app |
| BLE output | Cycling Power and Speed/Cadence | Cycling Power, Speed/Cadence, optional Heart Rate | Virtual bike can expose FTMS including control point |
| ANT+ | Bicycle-power output | Bicycle-power output, explicit/automatic selection | Platform/device-specific ANT integrations; not established here as equivalent generic Linux FE-C output |
| Direct Connect | Not found in inspected output | Not found in inspected output | Actual Dircon manager, TCP server, discovery and FTMS write processing |
| Bidirectional trainer requests | No FTMS trainer service in inspected output | No FTMS trainer service in output factory | FTMS target power, resistance and simulation/slope request processing |
| Simple programmable bot | UDP JSON watts/cadence | Same interface with robustness fixes | Fake bike plus MQTT control, but different semantics |
| Headless/local | CLI | CLI | `-no-gui` selects QCoreApplication |
| Physical 95Ri/Pico integration | Not implemented | IC5 profile is not 95Ri/Pico support | No integration with our existing host established |
| Runtime/build | Node >=12.16.1 declared; Babel build; old native BLE dependencies | Node >=14.21.3 <15 enforced; old native BLE stack | C++/Qt and platform dependencies; documented Linux source-build path |
| Source license | MIT | MIT | GPL-3.0 declared in prior metadata/README review |

### Gymnasticon: smallest useful interface

Both versions' `src/bikes/bot.js` accept UDP JSON such as
`{"power":180,"cadence":85}`. Fields are non-negative integers; updates can omit
one field. Both emit the retained values on a one-second timer.

V2 improves malformed JSON/null handling and socket/timer teardown. The original
logs parse errors but can then destructure an undefined value and throw.
V2 adds substantially more adapter/discovery/reconnection code, HR rebroadcast,
multi-adapter output, IC5/IC8 profiles and FTMS bike-data input parsing.

**Input is not output:** `gymnasticonV2/src/servers/ble/index.js:createServices`
constructs services 1818 (power), 1816 (CSC), optional 180d (HR), not 1826 (FTMS).
Receiving FTMS bike data does not make it a controllable FTMS trainer.

### Freshness and network exposure

- Both default bot control to **0.0.0.0:3000**. Inspected handlers have no
  authentication. Prefer a deliberately restricted same-host/network boundary.
- Neither bot expires the last UDP command. Its internal timer keeps creating
  fresh-looking stats even if the producer disappears. App-level bike-stats
  timeouts do not detect that upstream producer loss.
- A real integration needs producer timestamp/sequence and explicit stale-input
  behavior, without silently passing old rider watts as live measurement.
- Sensor output is not physical resistance authority. Do not equate a stop in
  one component with guaranteed physical release.

### V2 can change the supplied signal

`src/app/app.js:onBikeStats` scales power, applies MetricsProcessor smoothing,
and estimates speed from cadence when no native speed exists.
`src/util/power-smoother.js` defaults to:

`output = 0.8 × previous + 0.2 × new`

First positive sample seeds immediately; explicit zero resets immediately.
A 100→200 W step therefore initially broadcasts about 120 W, not 200 W.
A smoothing factor of zero gives immediate positive samples in this function.
Confirm actual configured path before experiments.

IC8's fallback power model in `src/util/power-estimator.js` is a generic
cadence/resistance polynomial, not a validated model of our 95Ri. External
speed/cadence sensor handlers in `src/app/app.js` still describe blending as
TODO/logging; sensor discovery is not evidence those values drive broadcasts.

### Complexity/deployment trade-off

Non-test source line counts at inspection (including comments): original ~2,062,
V2 ~10,311. V2 also adds deployment/images, hotplug/recovery and many configuration
options. These may benefit a standalone appliance but are not all required by our
host-owned telemetry workflow.

V2's checked-in `deploy/gymnasticon.service` removes original `User=pi` /
`Group=pi`, so that unit defaults to root, and retains network capabilities.
Do not execute its one-line installer or grant global Node capabilities based
on this review. Node 14 is obsolete; recent commits do not resolve that risk.

V2 includes additional software tests, with
`scripts/test-prelude.mjs` explicitly enabling dependency stubs. These are not
proof of real BLE/ANT+ pairing. No tests were executed here.

## QZ: important differences confirmed in local code

### Actual virtual trainer and network transport

- `src/main.cpp`: `-no-gui` selects `QCoreApplication`.
- `src/virtualdevices/virtualbike.cpp`: virtual bike exposes FTMS 1826,
  feature 2ACC, control point 2AD9 and related data. The selected configuration
  can instead use Dircon.
- `src/devices/dircon/dirconmanager.cpp`: builds virtual characteristics,
  including writable/indicating FTMS control point, and connects request signals.
- `src/devices/dircon/dirconprocessor.cpp`: actual TCP listener/discovery code;
  listener binds Any/AnyIPv4, so network exposure needs review.
- `src/characteristics/characteristicwriteprocessor2ad9.cpp`: handles target
  resistance, target power and indoor-bike simulation parameters and forwards
  requests to bike methods.

This is stronger source evidence than a README feature checkbox. It does **not**
prove our selected Zwift/container can discover/connect to it, or that QZ knows
our host's physical control contract. Dircon may avoid BLE/ANT radios at the
Zwift-facing boundary, but the chosen input path still needs defining.

### External control is richer, but not raw sample injection

`src/mqttpublisher.cpp:350–428` subscribes to general and device control topics.
It dispatches start/stop/pause, resistance, power, incline and bike cadence/gears.
Despite the filename, this is not just a telemetry publisher.

However:
- `bike::changePower` is a **target-power/control request**, with auto-resistance
  gating, offset and optional ERG adjustments; it is not a direct measured-watts
  setter.
- `bike::changeCadence` sets requested cadence, not necessarily broadcast cadence.
- `src/osc.cpp` binds UDP 9001 and handles `/QZ/Resistance`; its inspected handler
  does not implement an arbitrary power/cadence sample API.
- Do not treat MQTT/OSC presence as a ready-made input adapter for our host.
  MQTT broker access and network listeners need appropriate isolation.

### Fake bike is deliberately variable

`src/devices/fakebike/fakebike.cpp:update` runs on a 200 ms timer. With requested
power set, it generates approximately **target × [0.8,1.2]** and random cadence
between about **50 and 100 rpm** when target power is nonzero. It does not clear
the request on each update. Speed is modeled from power.

Therefore stock fake-bike mode is not equivalent to deterministic Gymnasticon
UDP sample forwarding. Fixed targets do not imply fixed output; seed/randomness,
metrics and stale-request behavior must be addressed before repeatability claims.
A complete runtime/control audit remains undone.

### Offline/no-extra-payment fit

Local source/build route is documented in `docs/10_Installation.md`, including
a no-GUI appliance path. Source-build availability makes it a candidate without
requiring a paid mobile app, but we have not built it or established a
fully-provisioned free binary for this machine. App-store pricing, cloud
integrations and source availability are separate questions.

The Qt/platform build is larger than a narrow bridge; avoid adopting QZ's whole
UI/workout/control system simply to publish watts.

## Expanded bike/run/row comparison

Follow-up requested local FortiusANT, Zwack and SmartSpin2k inspection.
FortiusANT and Zwack are clean local checkouts at the commits recorded below.
**SmartSpin2k local directory contains only .git, with no resolvable HEAD or
checked-out source.** Its origin is doudar/SmartSpin2k. No repair/fetch/checkout
was performed. SmartSpin findings use selected upstream files at pinned commit
`df59de54a85459e1b155a6f1b54c29d159de8481`, not local source.

### Sport and protocol matrix

“Not established” means not demonstrated in the inspected path, not proof that
no platform/plugin/version could implement it. Output profile support does not
prove that a particular fitness app accepts that profile.

| Tool | Bike/run/row role | BLE output confirmed or documented | ANT+ output | Return-control / network path | Best use |
|---|---|---|---|---|---|
| Gymnasticon | Cycling telemetry / bot | Cycling Power, CSC | Bicycle Power | UDP JSON input; no inspected FTMS output | Small programmable bike-sensor bridge |
| GymnasticonV2 | Cycling, HR/accessory aggregation | Cycling Power, CSC, HR | Bicycle Power | UDP input; multi-adapter/recovery machinery | Pi bridge if its added behavior is actually needed |
| QZ | Bike, treadmill, rower, elliptical and other equipment | Source-confirmed virtual FTMS bike/treadmill/rower; treadmill RSC option; configuration-dependent alternatives | Platform/device-specific integration, not generic parity established | Source-confirmed FTMS control and Dircon; MQTT requests, limited OSC | Broad multi-sport or network trainer reference |
| FortiusANT | Cycling; compatible older Tacx trainers or simulated trainer | FTMS indoor bike, HR; Node or Python Bless backend | FE-C, with other sensor-profile code | Actual power/grade requests; Node HTTP telemetry/command queue | Bidirectional trainer protocol reference |
| Zwack | Synthetic cycling and running | Cycling Power, RSC, partial indoor-bike FTMS | None found in inspected implementation | Library notification calls; FTMS forwarding disabled; no Dircon found | Lightweight BLE test fixtures after repair/qualification |
| SmartSpin2k | Cycling, physical resistance knob actuator | Upstream FTMS, cycling power/CSC/HR service source | No ANT+ path established in selected source | Upstream Dircon and ERG/stepper control | Embedded control/protocol reference, not desktop bridge |
| vpower | Bike virtual power; Concept2 row variant; HR/cadence variants | Not the documented output | Bicycle Power | Slider bot; curve/table/row inputs | ANT+ estimated or rowing-to-cycling-power bridge |
| vspeed | HR-derived virtual running speed | Not the documented output | Stride-based Speed/Distance | HR-to-speed mapping | Running-profile test input, not measured speed |
| Oldnapalm standalone_power + PowerMeterTx | Synthetic cycling input selected by server-observed terrain | None in inspected transmitter | Bicycle Power page 0x10 | Coupled offline-server feedback; Windows assumptions | Existing terrain-responsive input prototype |
| MyBleTrainer | Cycling sensor bridge + servo trainer | README FTMS | Not established | README app requests to physical servo | Historical DIY trainer architecture |
| SHIFTR | Cycling BLE/Direct-Connect bridge lead | Discovery-level evidence only | Not established | Virtual shifting/Direct-Connect lead | Separate source investigation needed |
| FTMS emulator | Embedded exercise-bike FTMS lead | Discovery-level evidence only | Not established | Uninspected | Separate source investigation needed |
| PyRow | Concept2 PM3/PM4 acquisition, not broadcast bridge | None by itself | None by itself | USB/CSAFE | Upstream rower-data acquisition layer |
| python-ant / PyUSB | Protocol/USB plumbing, not turnkey applications | None by themselves | python-ant has transmit/profile building blocks | Device/protocol APIs | Dependencies/reference only |

Running Speed/Cadence is not treadmill control. Bicycle Power is not ANT+ FE-C.
Cycling Power/CSC is not BLE FTMS. Rower FTMS is not automatically a native Zwift
rowing mode: a rower-to-cycling-power conversion changes sport semantics.
Do not silently substitute stroke rate for crank cadence or distance across sports.

### FortiusANT: local source goes beyond its README summary

Local `~/git/FortiusANT`, HEAD
`0eee91049ac7e0487ccdc77d5a351876347eb857`; source license GPLv3 with separate
third-party notices (e.g. LICENSE-antifier). No code executed.

Evidence:
- `pythoncode/FortiusAntCommand.py`: console operation and `-s` trainer
  simulation; `-b` selects Node BLE, `-bb` selects Python Bless.
- `pythoncode/FortiusAntBody.py:1484–1593`: receives FE-C basic resistance,
  target power and track resistance pages and calls trainer power/grade methods.
  Basic resistance is converted to grade: protocol support is not an identity
  mapping of raw physical resistance.
- `pythoncode/bleBless.py` and `bleBlessClass.py`: Python BLE FTMS server.
  This gives a **non-Node FTMS reference**, unlike Gymnasticon's old native stack.
  requirementsNoGUI lists Bless and other Python dependencies; compatibility
  would still need an approved uv-managed environment.
- `node/server.js`: Express listener on **9999**; POST `/ant` supplies
  telemetry to virtual trainer; GET `/ant` retrieves queued trainer requests.
  `node/virtual-trainer.js` constructs FTMS and shifts pending command messages.
  No authentication/bind-address restriction is shown in that small server.
  It is a reference, not a production-ready public API.
- Same-host ANT sender/receiver arrangements can need two sticks. BLE backend,
  USB/ANT access and selected platform must be validated independently.

**Simulator caveat:** `pythoncode/usbTrainer.py:1026–1125` implements
clsSimulatedTrainer, not a human physiological model. It moves power toward the
target, adds random power/cadence/HR variation and invents speed. When target power
is below 10 W it supplies fixed values including **246 W**, 113 rpm and 34.5 km/h.
That is demo behavior, emphatically not a safe “zero means stopped” test oracle.
It is neither deterministic replay nor a validated race/effort simulation.

### Zwack: source qualifies the advertised control support

Local `~/git/zwack`, HEAD
`0abecc5f049770d3ac37d4a3212abaf50794b994`, package 0.1.6; no code executed.

- Public class in `lib/zwack-ble-sensor.js` exposes `notifyCSP`,
  `notifyFTMS`, `notifyRSC`; BLE advertising profile selection comes from
  command-line `--variable` arguments even though it is exposed as a library.
- It imports `ftms/fitness-machine-service.js`, an **indoor-bike** service.
  Another treadmill-data implementation exists in the tree but is not the
  selected service in that public class. Running output is established via
  RSC; a filename alone does not prove a working FTMS treadmill endpoint.
- In `lib/ftms/fitness-machine-control-point-characteristic.js`, target-power
  and simulation/grade commands are parsed and acknowledged, but
  **`this.messages.push(message)` is commented out**. There is no demonstrated
  path to our controller. Wahoo-extension code likewise has commented target
  notification calls.
- README's “partial FTMS” warning is material: an accepted command does not
  mean a control action occurred. We should not promise ERG support from a
  pairing screen or successful acknowledgement.
- README warns cycling speed/cadence changes can be erratic/take about two seconds
  to stabilize. Example variability can be zeroed, but byte encoding, timing
  and full signal fidelity still need tests.
- `npm test` is an explicit failing placeholder. Native BLE/OS behavior untested.
  Windows is untested in README, which also says use a separate receiving host.
- Runtime manifest lists bleno/readline but source also directly imports
  debug/minimist. A clean dependency audit is needed; transitive availability
  must not be assumed as a supported install contract.

**License caution:** repository/package declares MIT, but FTMS files explicitly
say code came from FortiusANT, whose repository is GPLv3. That is a concrete
provenance/licensing review item, not a determination of infringement or proof
that the whole code can be copied under MIT. Preserve notices and verify rights
before redistribution.

### SmartSpin2k: upstream source, not a completed local checkout

Pinned upstream files inspected through GitHub's contents API:
- `src/BLE_Fitness_Machine_Service.cpp`: FTMS setup, data encoding and
  control processing; service registers a Dircon write handler.
- `src/DirConManager.cpp`: Wi-Fi/TCP client and characteristic subscription
  machinery. Thus **Direct Connect is implemented**, not just a marketing lead.
- `src/ERG_Mode.cpp`: target power/cadence and stepper-position control logic.
- `src/SensorCollector.cpp`: collected sensor values/real-versus-simulated flags.
- `platformio.ini`: ESP32/pioarduino/NimBLE/stepper dependencies.
- `LICENSE`: GNU GPL **version 2** text. Hardware has separate licensing.

It is a controller firmware and sensor aggregator, not a substitute desktop
server and not a generic bike/run/row bridge. It can be useful for learning how
to expose an FTMS/Dircon trainer while collecting BLE sensor data, but its stepper
and physical model must not be transplanted to our alternator-controlled Pico.
A claimed compatibility/calibration feature is not accuracy evidence for our bike.

Firmware can run locally on suitable hardware; obtaining/building hardware is
not zero-cost just because source is available. No purchase, flash, build or
physical test was performed.

### Rowing, running and lower-level paths

Re-read primary vpower/vspeed READMEs and pinned their remote HEADs:
- vpower `33ee701e333ef29ca7249755af9d9f9103eb3eb1`: speed-curve-to-power,
  slider bot, HR and cadence lookup variants, **Concept2 row variant via PyRow**.
  Documented output is ANT+ power, not BLE rower FTMS. README claims Python 3
  source execution and names python-ant; that does not modernize the separate
  cloned legacy python-ant automatically.
- vspeed `5465bf28128f391ee71caebc2486c1848d580d11`: HR-derived virtual speed
  broadcast as ANT+ stride-based speed/distance, not actual treadmill measurement.
- Both document compatible ANT sticks, Cycplus problems and two-stick same-host
  arrangements. Driver replacement instructions are not approval to run them.
- PyRow local source targets PM3/PM4 and contains Python-2 syntax. Modern Concept2
  monitor support is not established. PyUSB supplies USB transport, not fitness
  profile semantics.

QZ has source-confirmed **virtualrower.cpp** FTMS 1826/rower data 2AD1/control
point 2AD9, and **virtualtreadmill.cpp** FTMS treadmill data 2ACD plus RSC
configuration. That is the broadest inspected multi-sport output implementation.
Actual device model, selected virtual profile, mobile/desktop platform and target
app remain necessary compatibility checks. Automatic treadmill motion is a
different hazard/control scope from broadcasting speed.

### Selection by actual requirement

| Requirement | Best reference(s) from this set |
|---|---|
| Feed our existing measured bike watts/cadence into Zwift with minimal additional machinery | Original Gymnasticon or a narrow Zwack-style broadcaster; fix freshness, fidelity and dependencies first |
| Need network input to avoid BLE/ANT pass-through | QZ Dircon; SmartSpin Dircon as embedded protocol reference |
| Need ANT+ trainer **control**, not just power | FortiusANT FE-C |
| Need BLE FTMS without taking Node 14 forward | FortiusANT's Python Bless server; QZ's Qt implementation |
| Need bike + treadmill + rower virtual profiles | QZ |
| Need running speed/cadence synthetic fixtures | Zwack RSC; vspeed only if specifically testing HR-derived ANT+ stride output |
| Need Concept2 USB → cycling power | vpower row + PyRow as legacy reference; inspect actual monitor/units before selecting |
| Need physical resistance-knob actuation | SmartSpin2k, only for compatible hardware and separate hardware scope |
| Need deterministic pacing experiments | None should be accepted unchanged merely because it has a bot/simulate mode |
| Need native host integration without replacing our existing controller | A narrow explicit interface; no listed project currently proves this integration |

These are reference rankings, not adopted dependencies or passing compatibility
tests. Local implementations have no inherent need for a new coaching subscription,
but operating systems, provisioned dependencies, hardware and app-store binaries
have separate availability/cost constraints.

## Other bridge/bot references

Evidence is intentionally uneven: local source for the oldnapalm prototype;
selected remote README inspection for other repositories, not hardware tests.

| Tool / source | Relevant input/output | Fit and qualification |
|---|---|---|
| [Zwack](https://github.com/paixaop/zwack/tree/0abecc5f049770d3ac37d4a3212abaf50794b994) | BLE synthetic cycling power/cadence, running speed/cadence; partial FTMS; Wahoo-extension target-power test path | Small simulator/protocol reference. README says separate computer from receiving app; Windows untested. Check its code before expecting complete FTMS/ERG. MIT declared. |
| Local oldnapalm `standalone_power.py` + `PowerMeterTx.py` | Server-observed altitude change selects fixed power values; ANT+ page 0x10 power/cadence transmitter | Existing feedback prototype, but tightly coupled to offline server, Win32 imports and Windows CPU-based identity. Default/increased/decreased watts are 164/181/131, not physiological optimization. Two-stick setup described. |
| [mch/python-ant](https://github.com/mch/python-ant/tree/02e045825434a17ffe113a82cc8191683223ea5f) | Low-level ANT USB, power/other profile modules | Building block, not a complete modern bridge; Python 2.6/2.7 packaging claims need resolution. MIT/X11 declared. |
| [vpower](https://github.com/oldnapalm/vpower) | Virtual/estimated power and ANT+ test-input variants | Prior inspected ecosystem reference; not re-audited/pinned anew here. Trainer curves/HR-derived power are not our calibrated physical measurement. See existing ecosystem inventory. |
| [vspeed](https://github.com/oldnapalm/vspeed) | HR-derived virtual running speed via ANT+ | Not the cycling-control solution; prior ecosystem evidence only. |
| [FortiusANT](https://github.com/WouterJD/FortiusANT/tree/0eee91049ac7e0487ccdc77d5a351876347eb857) | Compatible older Tacx USB/ANT trainer bridge; README describes ANT/BLE app communication | Useful bidirectional protocol reference, not 95Ri support. Same-host ANT arrangement can need two sticks. GPLv3 declared. |
| [MyBleTrainer](https://github.com/roethigj/MyBleTrainer/tree/47f43d15156c67b81a7564b35b82ad6476dcfc15) | Sensor-to-FTMS trainer plus app callbacks to servo resistance | README explicitly says it is outdated and names Node 10. Useful architectural history, weak adoption candidate without source/runtime review. |
| [SmartSpin2k](https://github.com/doudar/SmartSpin2k/tree/df59de54a85459e1b155a6f1b54c29d159de8481) | Physical resistance-knob actuation, ERG/SIM behavior | Hardware/firmware reference, not a replacement for our existing Pico/controller. Claims of generic bike support are upstream descriptions. |
| [SHIFTR](https://github.com/Theo-Marshall/SHIFTR-Hardware) / [FTMS emulator](https://ftmsemu.github.io/) | Prior BLE/Direct-Connect and embedded FTMS leads | Discovery-level pointers only; not source-inspected in this comparison. |

Fixed ghosts and replay bots are a different category: they supply trajectories,
not a physical/input bridge or independently responding race tactics.

## Recommendation

1. **Telemetry-only input:** inspect the smallest Gymnasticon/Zwack broadcast
   path first. Preserve exact values, provenance and freshness; avoid smoothing
   unless explicitly selected.
2. **Network trainer endpoint:** QZ is the strongest inspected Direct Connect/
   FTMS reference. Establish a narrow host adapter and return-command semantics
   before choosing it as a dependency.
3. **Keep physical control in the existing native host.** Trainer power targets,
   grade and resistance levels are different quantities, not Pico duty values.
   An eventual adapter must honor that project's current control contract.
4. Do not adopt V2 wholesale just because it is newer, or use QZ fake-bike output
   as deterministic reference data.
5. All synthetic output is for controlled/offline testing, never public
   competitive rides.

A plausible division, not an implemented system:

```text
Existing bike host → timestamped measured/qualified metrics → bridge → Zwift
Existing bike host ← optional validated trainer request adapter ← FTMS/Dircon

Zwift → optional telemetry service → coaching/dashboard
```

Offline/headless Sauce is the separate telemetry question, investigated in
[the download review](SAUCE_DOWNLOAD_REVIEW_2026-10-30.md).
