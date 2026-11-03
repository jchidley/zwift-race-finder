# QZ SkiErg audit and WYK82Q test plan

## Result

QZ is a useful PM5 SkiErg decoder/reference, but **not yet a qualified bridge
for this setup**. Source inspection found a default FTMS treadmill-data type
mismatch, stroke-rate-as-running-cadence semantics and no demonstrated
source-loss deadline. Do not install a large application merely on the
assumption that the existing SkiErg path is complete.

No source changes, builds, dependency installs, radio scans, advertising,
device connections or Zwift sessions were performed during this audit.

## Evidence and scope

Local QZ checkout: `~/git/qdomyos-zwift`, clean at
`1b2441edaf27a39b64bd497733285ba4be409160`.
Paths below are relative to that checkout. Findings are static inspection,
not executed tests or claims about other releases.

Owner-confirmed client arrangement: native Windows Zwift on the current laptop
or Silver Surface Laptop 4, with directly attached external screens. Both are
known by the owner to run Zwift. Start with one; later use one client on each
before attempting same-machine multi-client operation. Phone external-display
operation is deferred. Public racing uses genuine, unmodified rider measurements;
converted, scaled and bot signals stay in controlled experiments.

## Updated hardware inventory — owner supplied

These specifications were supplied after the live bridge-host checks below.
They are inventory evidence, not a new hardware or boot inspection. Canonical
machine identities, serials and recovery details remain in `~/tools/bootstrap/`;
this project records only role-relevant specifications.

| Machine | Supplied specification | Supplied operating state / qualification |
|---|---|---|
| Silver Surface Laptop 4 | AMD Ryzen 7 Microsoft Surface Edition; 8 GB RAM; Fanxiang S700 1 TB SSD | Owner confirms installed Windows works fine and runs Zwift. Recovery-USB WinRE observations describe a recovery inspection, not an operational limitation. |
| WYK82Q | Core i5-4250U; 8 GB RAM; 120 GB mSATA SSD | Debian 13.7, kernel 6.12.113+deb13-amd64; consistent with our live OS check |
| WYKH32J | Core i5-4250U; 8 GB RAM; Crucial BX100 500 GB SSD | Debian 13.7, kernel 6.12.111+deb13-amd64; consistent with our live OS check |

Physical disk capacities above differ from root filesystem sizes/free space
below; they are not contradictory. Other partitions retain their existing roles.
Silver is an owner-confirmed working Windows/Zwift client, available for the
planned client role. No recovery prerequisite is assigned. Historical recovery
inspection details remain in the tools workspace.

## Bridge hosts: live read-only check

Both hosts were reached over SSH. WYK82Q used the saved machine SSH configuration
explicitly after the ordinary alias failed to resolve; its saved Ed25519 pin
matched the documented pin. No SSH configuration was changed.

| Observation | WYK82Q | WYKH32J |
|---|---|---|
| Hostname | wyk82q-deb | wykh32j-deb |
| OS | Debian 13.7 (trixie) | Debian 13.7 (trixie) |
| Kernel | 6.12.113+deb13-amd64 | 6.12.111+deb13-amd64 |
| RAM / available at check | 7.7 / 7.3 GiB | 7.7 / 6.9 GiB |
| Root free | 31 GiB | 139 GiB |
| Bluetooth USB identity | 8087:07dc | 8087:07dc |
| Kernel controller | hci0 | hci0 |
| Bluetooth service | inactive | inactive |
| Userspace | BlueZ query returned no package information | bluez not installed; libbluetooth3 installed |
| ANT+ stick | None identified in attached USB list | None identified in attached USB list |

WYK82Q is the preferred first candidate because it is lightly loaded.
WYKH32J has a Raspberry Pi Debug Probe and an investigation peripheral attached;
do not disturb those. No sensor, serial or USB endpoint was opened.

Subsequently, the owner attached an ANT stick to each host and authorized direct
USB capability queries. [Both succeeded](ANT_USB_CAPABILITIES.md): WYK82Q USB-m
reports 8 channels / 8 networks; WYKH32J USBStick2 reports 8 channels / 3 networks.
The table above retains the earlier pre-attachment state. libusb prerequisites
were installed for that later diagnostic; Bluetooth preparation remains pending.

Presence of hci0 does not prove LE advertising, concurrent central/peripheral
operation, fitness profile compatibility or stable radio performance.

Inventory authority remains `~/tools/bootstrap/refind-recovery/README.md` and
its machine profiles. Do not copy recovery procedures, keys or private machine
snapshots here. No boot repair, repartitioning, firmware change or recovery-USB
operation is needed for this trial.

## QZ input path

### Detection and subscription

- `src/devices/bluetooth.cpp:2398`: advertised name must start with PM5 and
  end with SKI, case-insensitive, with the general discovery filter satisfied.
- `src/devices/concept2skierg/concept2skierg.h`: derives from `rower`.
  `src/devices/rower.cpp` reports device type `ROWING`.
- `concept2skierg.cpp:270–355`: subscribes to Concept2 multiplexed
  characteristic `ce060080-43e5-11e4-916c-0800200c9a66`.
  Notification/indication subscription writes the CCCD.
- This is a PM5-specific path, not proof that the owner's unconfirmed monitor
  exposes that characteristic, name or data layout. No USB/ANT SkiErg path
  was established by this inspection.

### Values and transformations

`concept2skierg.cpp:147–242` handles multiplexed records:

| Record | Behavior |
|---|---|
| 0x31 | Distance in decimetres → km; rowing state controls active/inactive; drag factor becomes resistance metric |
| 0x32 | Speed in 0.001 m/s → km/h by ×0.0036; stroke rate becomes Cadence; HR 0xff is ignored |
| 0x33 | Calories |
| 0x35 | Stroke count |
| 0x36 | Stroke power in watts while active |

Speed is therefore **ski speed presented as virtual running speed**, not a
calibrated human running-speed estimate. Watts are skiing watts. Neither establishes
cycling capacity or physiological equivalence.

Importantly, assignment is not necessarily pass-through:
`src/metric.cpp:24–88,298` applies settings on assignment.

- Positive speed: speed × `speed_gain` + `speed_offset`.
- Positive power: gain/offset, subject to a device-type-specific gain rule.
  For ROWING, gain is applied only if configured gain ≤5.
- Power is then capped by `watt_max`.
- Defaults in `src/qzsettings.h`: speed/watt gains 1, offsets 0,
  watt cap 9999 W.
- These settings are read on updates; record all settings and changes.
- The inspected immediate-value path stores the assigned value rather than
  replacing it with a rolling average. Historical-average metrics also exist;
  this is not a guarantee about every application path.

The inspected RSC encoder uses current speed/cadence directly. Do not assume a
setting called power gain changes running speed.

### Stop and signal loss

When a 0x31 record reports inactive, speed and cadence are set to zero.
The stored watt-reset line is commented out, despite a log claiming watts are
zeroed. `watts()` returns zero when cadence is zero, so published/accessed power
and retained internal watt state are distinct.

The input update timer runs every 200 ms. Disconnection emits a signal and
returns; the controller attempts reconnection. The inspected class updates
`lastRefreshCharacteristicChanged` but does not use it to expire old samples.
No explicit stale-source clearing was found in the inspected input/provider path.

**An inactive-state packet is not a source-loss watchdog.** Test missing
notifications and disconnection separately. Stop the trial if positive output
continues after source loss; a new output tick is not a fresh source sample.

## QZ running output: important mismatch

`concept2skierg.cpp:389` constructs `virtualtreadmill(this, noHeartService)`.
The object passed into it is still a ROWING device.

### Default FTMS mode

- `run_cadence_sensor` defaults false; virtual treadmill selects FTMS treadmill
  output (`src/virtualdevices/virtualtreadmill.cpp:1150–1180`).
- `src/characteristics/characteristicnotifier2acd.cpp` emits Treadmill Data
  only for device types TREADMILL or ELLIPTICAL, returning CN_INVALID otherwise.
- Therefore the inspected SkiErg object does **not** produce that normal
  treadmill-data notification through this encoder. Creating/advertising
  a virtual treadmill alone does not prove useful FTMS running telemetry.

### Alternative RSC mode

- Setting `run_cadence_sensor=true` selects Running Speed and Cadence output.
- `src/characteristics/characteristicnotifier2a53.cpp` has no corresponding
  device-type rejection.
- It encodes speed as uint16(speed_kmh / 3.6 ×256), truncating to wire resolution;
  distance as km ×10000 (0.1 m units); cadence as a byte.
- Cadence is copied from the input metric: here **SkiErg strokes/min**, not
  verified running steps/min. Do not silently call it measured running cadence.
- The normal virtual provider timer is 1 s; race mode uses 100 ms.
- RSC speed output is a candidate for an isolated test, not proof that Zwift
  accepts this complete service/device configuration or that cadence is meaningful.

No confirmed cycling-mode switch was found in the SkiErg creation path.
QZ also does not establish the required Linux ANT+ transmitter path merely
because it contains Android ANT integrations. Both protocols remain requirements.

## Control safety

The SkiErg class contains FTMS resistance-write code, but does not establish
that the SkiErg supports physical resistance control. Its characteristic loop
filters to the Concept2 multiplexed characteristic before the FTMS control-point
check, so a usable control endpoint is not demonstrated by that code.

The CLI advertises `-no-write-resistance`, but the inspected SkiErg update
branch does not visibly guard its resistance request with that flag.
Do not rely on the flag alone as proof of a read-only physical-control boundary.

For initial tests, no workouts, target resistance, grade-following or return
control are permitted. The existing controller remains the exclusive Pico owner.

## Installation assessment

- Repository LICENSE contains GPLv3; keep QZ separate. This project's
  MIT OR Apache-2.0 licence does not relicense QZ code.
- `docs/10_Installation.md` documents Linux source builds with qmake/make
  and many Qt libraries; `-no-gui` selects QCoreApplication in `src/main.cpp`.
- `.github/workflows/main.yml` has an x86 Linux build producing
  `linux-desktop-binary` using Ubuntu 24.04 and Qt 5.15.2.
  This is evidence of a build recipe, not a currently downloadable,
  dependency-complete or Debian-compatible binary.
- The Pi binary instructions are architecture-specific and reference a
  time-bound Actions artifact. Do not use that artifact on an x86 NUC.
- Source build is a no-extra-payment route; Debian 13 compatibility and exact
  dependencies remain untested. No suitable ready-to-run NUC binary was verified.
- Upstream instructions include broad upgrade/package wildcards and running
  QZ with sudo. Do not copy those wholesale. Preview exact dependencies and
  establish least-privilege Bluetooth access before installation/launch.

Given the default-path mismatch and incomplete freshness behavior, a narrow
input probe may be a better first device test than installing all of QZ.
Do not decide between a QZ fix and a small bridge until the actual monitor
and its notifications are confirmed.

## Bounded next test plan — not executed

### Gate 1: owner/device identification

Confirm monitor model/firmware at the SkiErg, nearby WYK82Q placement and selected
Windows client. If not a matching PM5, reassess the input path; do not force
the QZ PM5 assumptions.

### Gate 2: approve exact host preparation

Prepare an exact package/dependency preview for WYK82Q. Installing BlueZ,
starting its service, powering radios and building/installing QZ each need
appropriate approval. Do not reboot, alter networking, use the recovery USB,
change device drivers or disturb other attached equipment.

A passive controller-capability query after preparation must establish LE and
advertising support. Then separately authorize bounded discovery/connection
and notification subscription to the identified monitor. No public-event output.

### Gate 3: input-only inspection

Acquire actual fields/units and packet timing. Record a short idle → easy strokes
→ stop sequence, then monitor disconnect/reconnect. Keep captures in an explicit
private path outside Git; use synthetic fixtures for later committed tests.
Do not enable virtual advertising yet. Check the monitor against parsed values,
including active/inactive transitions and absent/stale fields.

### Gate 4: controlled BLE output qualification

Only after input validation, choose QZ RSC mode versus a narrow output probe.
Use an explicitly controlled/offline client environment, not public competition.
Record settings, packet timestamps, original values and encoded output.
Keep speed gain 1 and offsets 0 initially, and disclose stroke cadence semantics.
Do not invent a step-rate conversion just to satisfy a UI.

1. Inspect advertised services and actual RSC measurements with an independent
   receiver before claiming client compatibility.
2. Verify intended Windows client pairing, speed agreement within wire/display
   resolution and measured latency.
3. Test zero, stop, notification silence and source disconnection separately.
4. Proposed acceptance deadline: no positive output more than 3 s after the last
   valid speed sample. Confirm against measured monitor cadence before adoption;
   do not relax it silently. Disconnect output if this fails.
5. Reconnect without retaining stale positive motion.
6. Test receive/transmit concurrency. If one radio cannot do both, use a spare
   compatible adapter rather than assuming multi-role support.

The acceptance deadline is a proposed requirement, not current QZ behavior.
Have an immediate output-disconnect mechanism for the operator.

### Gate 5: expand only after BLE proof

Identify and attach an ANT+ stick with approval, qualify stride output with the
same semantics/freshness checks, then simultaneous ANT+/BLE parity. Cycling,
scaling, synthetic inputs and a second Windows client follow separately.
Do not let a successful BLE demo count as completion of the dual-protocol bridge.

## Next decision

**Current blocker:** monitor model/firmware. Useful completed preparation:
source audit, host selection and explicit test gates. Next authorized operational
step should name the exact WYK82Q installation/service effects and whether it
includes a device scan/connection. No broad install or radio authorization is
implied by this plan.
