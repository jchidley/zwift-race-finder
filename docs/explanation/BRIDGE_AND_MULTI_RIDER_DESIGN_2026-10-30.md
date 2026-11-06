# Local fitness bridge and multi-rider experiments

Requirements and design handoff, consolidated 2026-10-30.
**No implementation/runtime has been selected or built.** This note records the
owner's requirements, proposed smallest architecture, evidence and open decisions.
It does not authorize installs, account use, device operations or experiments.

## Owner requirements

- Prefer **offline/local tools** and the smallest complete solution.
- **No additional paid services beyond Zwift.** TrainerRoad is excluded.
  GoldenCheetah is the primary existing local-analysis candidate; Intervals.icu
  offers cloud/calendar/sync convenience but is not a required dependency.
- Support **both ANT+ and BLE**, independently selectable and, where hardware
  permits, simultaneously. They are requirements, not an either/or selection.
  “Bluetooth” here means BLE fitness profiles, not Bluetooth Classic.
- Accept **real inputs**, optionally modified by an explicit transformation such
  as a scaling factor. Direct pairing alone is therefore insufficient.
- Accept **synthetic/bot inputs** through the same output mechanism.
- Support Concept2 **SkiErg → running and cycling initially**, with running the
  priority. Owner may later settle on one. Prefer one selected sport per session
  initially, not simultaneous run/cycle advertising.
- Eventually operate **multiple Zwift instances with separate rider profiles and
  independently controlled outputs**, to investigate hills, pacing and drafting.
  Different client versions are a possible later experiment, not a baseline.
- Reuse the existing bike resistance application. An overlay is optional, not a
  prerequisite; a telemetry service may feed the existing dashboard.

These requirements do not displace the [first useful coaching workflow](PACING_AND_RACE_EXPERIMENTS.md):
inspect an approved activity and produce a useful review/pacing plan without
making a simulator a prerequisite.

## Owner equipment and existing systems

| Item | Intended use / qualification |
|---|---|
| Assioma pedals | Real cycling power/cadence source via ANT+ or BLE. Exact model, firmware and concurrency not inspected. Direct-to-Zwift pairing provides a baseline; bridge path enables intentional modification or fan-out. |
| Concept2 SkiErg | Real skiing measurements, converted explicitly for a running or cycling avatar. Owner says monitor family is shared with the rower; **PM model/firmware remains unconfirmed**. Do not record PM5 as established hardware. |
| Garmin Forerunner 265 | Garmin documents Virtual Run transmission of pace, HR or cadence to apps including Zwift. Actual pairing not tested. Ordinary outdoor activity recording can feed local review. |
| ANT+ and BLE equipment | Available protocols, not proof of a particular dongle count, adapter role support or host passthrough configuration. |
| Existing controller | `~/git/life-fitness-cycle-controls-investigation`, GitHub `life-fitness-pico-controller`. Owns Pico USB, physical resistance, native recording and dashboard. |
| Zwift Companion | Optional BLE connection bridge for compatible sensors. Not a generic ANT+-to-BLE converter or arbitrary sport/profile translator. Offline-server compatibility unverified. |

If outdoor running means **live GPS-derived motion driving a Zwift avatar** rather
than recording an activity for later review, that is a separate unresolved
requirement. The watch's Virtual Run documentation does not establish that relay.
Official reference:
<https://www8.garmin.com/manuals/webhelp/GUID-F41EAFB3-6CC9-42DE-9C6C-9E358DBB0671/EN-US/GUID-9F45EF2C-D6D5-4583-B4C6-A386743B650A.html>.

## Selected host/client direction

Use the owner's known-working native Windows Zwift laptops and directly attached
external screens. One client per laptop is the first two-client candidate;
container/GPU-passthrough and phone external-display work are deferred.
Updated owner inventory identifies Silver as a Ryzen 7 Surface Laptop 4 with
8 GB RAM and a 1 TB SSD. The owner confirms its installed Windows works fine
and it runs Zwift. Recovery-USB observations describe a separate inspection,
not its normal operating state or a prerequisite to using it.
WYK82Q is the preferred separate Debian bridge candidate after live SSH checks.
WYKH32J is also running Debian but has other investigation peripherals attached.
Neither host's BLE multi-role operation or ANT+ output has been qualified.
See the [host evidence, QZ audit and test gates](../reference/QZ_SKIERG_AUDIT_AND_TEST_PLAN.md).
The SkiErg monitor remains unconfirmed; do not assume QZ's default treadmill
path works just because it constructs a virtual treadmill.

## Proposed smallest architecture

Separate acquisition, transformation, transport and experiment coordination.
Start with direct functions/modules, not a plugin framework or generic rules engine.

```text
Real sensor / approved input trace ─┐
                                  ├─ original samples + source timestamps
Explicit synthetic bot input ──────┘
       │
       ├─ per-rider source selection and explicit transformation
       │       ├─ unchanged baseline
       │       ├─ scaling
       │       └─ selected SkiErg sport mapping
       │
       └─ per-rider output
               ├─ BLE sensor identity
               └─ ANT+ sensor identity
                         │
                         └─ intended Zwift client/profile

Client/server telemetry → experiment recorder → optional policy feedback

Existing resistance host → Pico → physical bike resistance
```

### Four responsibilities

1. **Acquisition:** read real samples or explicit synthetic/recorded inputs.
   Both input protocols are useful, but do not implement every source/transport
   combination before inspecting the actual equipment.
2. **Per-rider control/transformation:** select a source and output values.
   Multiple riders may share one real source with distinct transformations.
3. **Transport:** encode each rider's output into BLE and/or ANT+ profiles.
4. **Experiment coordination:** client/profile assignment, start conditions,
   policy schedule/feedback, synchronization and result recording.

A real source should be acquired once where feasible, then distributed to rider
channels. Keep source-specific decoding separate from sport conversion and wire
encoding. No hidden path should bypass source freshness or provenance handling.

## Output profiles and boundaries

| Session mode | BLE | ANT+ |
|---|---|---|
| Cycling sensor | Cycling Power, with the required cadence representation | Bicycle Power |
| Running sensor | Running Speed and Cadence | Stride-Based Speed and Distance |
| Optional later trainer control | FTMS | FE-C |

The first two rows are the initial bridge scope. FTMS/FE-C **return control** is
not required merely to publish power or move an avatar. Receiving a grade/power
request is not the same as safely controlling physical resistance.

One transformed stream should feed both transport backends. Each encoder owns its
own units, counters, rollover and transmission schedule. Missing cadence must
remain unavailable rather than become invented cadence; preserve zero versus
missing. Quantization and different update rates must be disclosed and tested.

BLE/ANT+ outputs need distinct identities for separate rider channels. Verify
pairing rather than assuming profile separation isolates sensors. Prevent
self-discovery/feedback into the bridge's own input. Hardware channel limits,
BLE central/peripheral concurrency and multiple virtual peripherals are not
established. Separate adapters/hosts may be required.

## Transformations and signal integrity

Initially support only the concrete cases:

- **Pass-through:** preserve measurement values, subject to explicit wire encoding.
- **Scaling:** e.g. output power = measured power × chosen factor.
- **Sport mapping:** SkiErg → running speed/cadence or cycling power.
- **Synthetic prescription:** explicit bot values or approved input replay.

Preserve original and transformed values, source identity, units, timestamps and
transformation parameters. A scale factor is not “calibration” without reference
evidence. Do not silently add smoothing, offsets, randomization or fallback
power estimation. Treat live parameter changes as recorded events.

Define the allowed factor/range and rounding from actual requirements. Validate
finite values and protocol representability; fail clearly rather than wrap or
silently clamp an experiment to a different value. A stopped source stays stopped.
A stale source must stop positive output according to an explicit, tested timeout.
Distinguish producer sample time from transport tick time; repeating an old sample
must not make it fresh. Define disconnect/reconnect and process-exit behavior.

### SkiErg-specific mapping

- Cycling watts remain **skiing power presented through a cycling profile**;
  they do not establish cycling capacity or equivalent effort.
- Running speed may initially use the monitor's ski pace/speed as a virtual
  mapping, after confirming units and how it behaves in Zwift. It is not measured
  human running speed.
- Preserve SkiErg stroke rate as stroke rate. Any conversion into running steps
  or crank cadence must be explicit, not a silent renaming.
- Do not assume physical resistance can be controlled on the SkiErg.

Local QZ evidence is particularly relevant:
`src/devices/bluetooth.cpp` recognizes names beginning PM5 and ending SKI;
`src/devices/concept2skierg/concept2skierg.cpp` parses Concept2 notifications,
including watts/speed, and its inspected normal virtual-device path constructs a
**virtualtreadmill**. This is not proof our monitor matches, nor that cycling output
for that SkiErg path works without changes. Evaluate existing behavior first.

### Randomization is not ban protection

Some inspected simulators add variability. Their implementation is evidence of
variation, not proof of author intent, Zwift detection rules or ban avoidance.
Do not add noise to disguise synthetic or modified activity as human measurement.
Use deterministic bot output by default; optional seeded, logged noise may test
robustness. Sensor noise, smoothing, physiological response and random jitter
are distinct mechanisms.

## Bots and multiple clients

Keep these categories separate:

| Category | What it does | What it does not establish |
|---|---|---|
| Fixed ghost/replay | Replays a recorded trajectory, useful repeatable background | Reactive tactics or a fresh physics simulation from watts |
| Input replay | Replays power/cadence into a client | Same outcome under changed world/drafting conditions |
| Open-loop bot | Follows a prescribed watt/speed schedule | Response to current hills or opponents |
| Closed-loop bot | Uses observed position/gradient/gaps to select input | A validated human fatigue model or competitive legitimacy |

A sensor broadcast does not create another rider. Independently moving client
riders need independently controlled instances/profiles or a separately verified
server-side mechanism. Multiple virtual BLE identities on one adapter are not
assumed feasible.

Start with one pinned game version, then two isolated profiles with separate
state/output/recording. Different client versions add compatibility and physics
variables and should be deliberately selected later. Native Windows on the two known-working laptops is the baseline. netbrain/zwift
is deferred as an optional runner, **not installed/validated here**. Its documented ANT+
limitation means host-side ANT support alone does not establish an input path to
its containers. Test BLE/Companion/Dircon topology explicitly.

### Meaningful experiments

- **Hills/pacing:** compare open-loop constant/scheduled effort with bounded
  gradient/position feedback only after telemetry is validated.
- **Drafting:** riders must inhabit a shared world/server with appropriate
  positions. Test whether that configuration applies drafting; visibility or
  a supplied “draft” metric alone is not complete physics evidence.
- **Responsive tactics:** each interactive opponent needs state and a controller.
  Fixed replay collections cannot provide this merely by being called bots.

Hold rider attributes, equipment, route IDs, lead-in, start position/speed, world,
client/server versions and finish/timing rules constant where appropriate.
Record intended commands, actual transmitted values, observed telemetry,
timestamps, losses and decisions. Synchronize clocks/boundaries without assuming
different transport clocks are identical. Separate warm-up/rollout from measured
trial time. Define realistic effort limits before comparing strategies.
Seed stochastic scenarios and retain individual outcomes, not only averages.

Replay media/data and private profiles stay outside Git. Race-finder takes care
of itself: no finder repair, validation or integration is required for this work.
If an experiment needs supplied route/event data, preserve numeric IDs, units,
precision, source provenance and event timing semantics. Do not share writable
project databases or duplicate route collection as part of a bridge task.

## Implementation candidates, not a language decision

| Need | Candidate/reference | Evidence and limitation |
|---|---|---|
| BLE receive | Bleak | Local source/README; generic GATT client, not Concept2 decoder |
| BLE transmit | Bless | Local source/examples; generic GATT server, not complete fitness profiles |
| ANT receive/transmit | OpenANT | Modern Python package and generic TX APIs; profile-specific output work remains |
| SkiErg parsing/running behavior | QZ | Dedicated local SkiErg path plus multi-sport virtual profiles |
| Small programmable bike source | Gymnasticon | UDP watts/cadence; needs producer timeout and malformed-input qualification |
| FTMS/FE-C and Python server reference | FortiusANT | Real return-control paths; simulation is deliberately nonphysical/random |
| Small BLE fixture reference | Zwack | Partial FTMS; command forwarding commented out; licensing provenance review needed |
| Embedded FTMS/Dircon and simulation reference | SmartSpin2k | Local source confirmed; physical stepper model is not our Pico system |
| Fixed background riders | zoffline-bots | Replay activities; check_bots.py deletes files; rights/data review needed |

Local newly inspected library revisions:
- Bleak: `c7ca1b04b00b0bda5f98ccf9493e24784cfe869d`; Python >=3.10 declared.
- Bless: `55a1549742f02d160d40a34284b524c497e02420`; generic service/characteristic
  creation, writes and notifications. Metadata declares bleak>=1.1.1, so do not
  infer a usable environment merely from its older python_requires minimum.
- OpenANT: `52c545ced290793e506671873bb707fd398a20c3`; Python >=3.8.
  `examples/broadcast_send.py` demonstrates stride speed/distance TX;
  `devices/power_meter.py` is a receiver/parser, not turnkey power-meter TX.
  Generic TX support does not supply validated cycling/FE-C transmitter behavior.
- zoffline-bots: `f9578164784a6049458c960d53f515fbacac94c7`.

These four checkouts were clean at inspection; only documentation/source was read.
No dependency environments or device behavior were tested. A Python solution
would use uv-managed Python. Do not add MQTT, databases, plugins or a universal
adapter framework unless the first concrete workflow needs them.

### Telemetry service versus overlay

Sauce can run headless with a REST/WebSocket API; that is independent of overlays
and physical resistance. Official stable archive inspection found a disabled
zOffline host switch, not a demonstrated supported offline deployment.
The official packaged product requires Patreon; newer GPL source and the stable
binary's license metadata differ. Do not select it as a no-extra-payment dependency
without resolving the actual build/rights/requirements.
See [download evidence](../reference/SAUCE_DOWNLOAD_REVIEW_2026-10-30.md).
Old packet monitors cannot be assumed to handle current encryption.
Structured server/client telemetry should be evaluated for the selected topology.

## Small, testable progression

1. **Identify:** confirm monitor, host OS/client placement, ANT sticks and BLE
   adapters; inspect approved device fields with separately authorized access.
2. **One real path:** acquire SkiErg data; establish a direct Assioma baseline.
   Confirm units, timing, zero/missing and source loss without modifying equipment.
3. **One output:** evaluate running first, with original/transformed recording.
   Use QZ as an existing behavioral reference before writing equivalent code.
4. **Second transport and sport:** establish both ANT+/BLE outputs, then cycling;
   check parity, counters/rollover, source timeout and reconnection. Advertise
   only the selected sport per session initially.
5. **Transformations:** verify pass-through and scaling with synthetic fixtures;
   include zero, missing, invalid, stale and representability boundaries.
6. **One bot:** prescribe deterministic values; prove actual client input and
   telemetry match the intended policy. Do not inherit demo randomization.
7. **Two clients:** unique outputs/profiles, shared controlled world, measured
   resource use, reliable pairing and repeatable synchronized starts.
8. **Experiments:** repeated constant-power baseline, constrained pacing/hills,
   verified drafting, then responsive opponents. Broaden versions/rider count
   only for a concrete unanswered question.

This progression is a proposed acceptance sequence, not a build authorization,
a completed test suite or a promise that all combinations work.

## Open decisions

- SkiErg monitor model/firmware and accessible USB/BLE/ANT profiles.
- Input/output host placement, available dongles and simultaneous role limits.
- Running mapping/cadence semantics and later cycling mapping.
- Desired outdoor-running behavior: recorded review or live virtual-avatar relay.
- First concrete operating workflow: existing QZ versus narrow new bridge;
  implementation language and packaging remain undecided.
- Initial bot count, fixed versus responsive opponents, telemetry interface,
  offline-server/client compatibility and profile/account isolation.
- Explicit source-loss deadline, transform bounds and experiment effort model.

## Boundaries and evidence

No virtual/modified sensor output in public competitive events. Technical
pairing does not establish permission for converted-sport activity on the official
service. Do not claim calibrated watts, physiological equivalence, drafting,
real-race accuracy, headless/accelerated game clients or compatibility without
appropriate evidence. Account use, installs, network/service changes and
physical control retain their separate approval boundaries.

No new application, test harness, dependency, personal-data import or experiment
was implemented by writing this note. Supporting references:
[bridge comparison](../reference/BRIDGE_COMPARISON_2026-10-30.md),
[companion research](../reference/COMPANION_TOOLS_2026-10-30.md),
[ecosystem evidence](../reference/ZWIFT_ECOSYSTEM.md),
[project boundaries](../reference/PROJECT_BOUNDARIES.md).
