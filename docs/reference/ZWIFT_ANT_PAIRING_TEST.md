# Windows Zwift ANT pairing test — 2026-10-30

Historical Coach evidence retained during consolidation. Current state and ownership
are in the root [status](../../STATUS.md) and [documentation map](../README.md). Dated claims below are not current acceptance.

## Verified result

The installed Gymnasticon 1.4.0 on gym successfully drove native Windows Zwift's
**pre-ride Paired Devices screen** over ANT+. Both Power Source and Cadence showed
**Powermeter 11234**, ANT icons, and green CONNECTED status.

Three captured windows were directly inspected:

| Capture timestamp (Windows UTC, after save) | Power | Cadence |
|---|---:|---:|
| 22:37:01.433 | 0 W | 0 rpm |
| 22:37:05.434 | 150 W | 85 rpm |
| 22:37:16.052 | 0 W | 0 rpm |

A bounded Windows PowerShell UDP producer sent complete 150/85 samples at roughly
500 ms intervals for eight seconds. Heartbeats stopped at 22:37:09.552; the zero
capture was saved roughly 6.5 seconds later. **No zero command was sent between
heartbeat cessation and that capture.** Explicit cleanup zero followed at
22:37:16.061. Thus the observed zero was not caused by the cleanup command.
This verifies the displayed return to zero, not an exact Zwift display-latency
bound or a client-side 3-second guarantee. See the separate
[watchdog timing/limitations](GYMNASTICON_WATCHDOG.md).

The foreground-guarded test checked Zwift before each heartbeat and capture,
would abort/reset on focus loss, and captured only its window. Positive and zero
screens both retained the expected sensor identity and connected indicators.
No OK/Ride/event action was taken during the pulse. Resistance, Controls and
Heart Rate remained unpaired; no physical resistance commands were sent.
Final independent gym logs confirmed 0 W / 0 rpm, active service, MainPID 2665,
NRestarts=0. Zwift was left on Paired Devices.

## Startup and unsuccessful observation attempts

The earlier startup block resolved without a repair: the owner reported that
Zwift had finished starting, and the process subsequently reported responsive.
Initially the owner had paired ANT+ while in a shared world. Only zero was
inspected there; no positive bot command was sent until the owner returned to
home and the pre-ride pairing screen was visually confirmed.

An initial positive pulse had a Windows/WSL command-bridge timeout before its
observer started. Source logs confirmed expiry but no client screenshot existed.
A subsequent attempt captured a browser covering Zwift; those images are not
client-reading evidence. The final guarded-window run above resolved that
observation gap. Do not describe the failed observations as passed client tests.

## Pi Zero 2 W retest after reboot — passed

Following the hardware move, the initial Zero 2 boot reported no ANT stick.
After the owner's reboot, strict SSH at 10.0.1.61 verified short uptime, boot ID
b9c901a2-b9d3-495c-9dce-419e339b2943, USB ANTUSB-m 0fcf:1009, and Gymnasticon's
ANT+ stick opened message. PID 552 was active with NRestarts=1 after an initial
Bluetooth ENODEV failure; output zero and throttled=0x0.

An initial mixed ANT-power/BLE-cadence attempt aborted at the foreground guard
before positive output, and a later native Windows launch hit a WSL vsock timeout.
Neither is a passed pulse test. Before the successful retry, direct inspection
showed that the owner had selected **ANT+ Powermeter 11234 for both fields**.
No mixed-transport reception claim is made.

The successful foreground-guarded producer sent 150/85 UDP samples directly to
10.0.1.61:3000 for eight seconds at roughly 500 ms intervals. All three window
captures were inspected:

| Windows UTC timestamp after capture save | ANT power | ANT cadence |
|---|---:|---:|
| 23:37:51.110 | 0 W | 0 rpm |
| 23:37:55.032 | 150 W | 85 rpm |
| 23:38:06.006 | 0 W | 0 rpm |

Both fields retained ANT icons, Powermeter 11234 identity and CONNECTED status.
Heartbeats stopped at 23:37:59.665; explicit cleanup zero was sent only after the
zero capture, at 23:38:06.015. Thus the zero followed command silence, not cleanup.
This verifies functional return to zero by the later snapshot, not a precise
3-second client display deadline. No ride/event action was taken; Zwift was left
on Paired Devices with both ANT fields at zero.

This establishes the basic cycling ANT pairing/pulse/expiry baseline on the
Zero 2 W as well as the earlier Pi 3 A+. [BLE was separately verified on the
Zero 2](ZWIFT_BLE_PAIRING_TEST.md); simultaneous reception/parity, cold-boot
repeatability, full profile conformance and sustained reliability remain open.
The Bluetooth startup readiness race was not repaired.

Private Windows Temp captures: `zwift-zero2-ant-baseline.png`,
`zwift-zero2-ant-positive.png`, `zwift-zero2-ant-expired.png`.
Private raw output: `zwift-zero2-ant-retry.json` under the existing bench directory.
No recordings/screenshots were copied into Git.

## Evidence and scope

Successful images remain outside Git in Windows user Temp:

- `zwift-ant-baseline-final.png`
- `zwift-ant-positive-final.png`
- `zwift-ant-expired-final.png`

Raw command outputs, including unsuccessful observations, are saved privately
under `~/.local/share/zwift-race-coach/bench-ant-2026-10-30/` with the
`zwift-pairing-` prefix. This note contains summarized evidence, not recordings
or rider-profile exports. Full-desktop diagnostic images also remain private
in Windows Temp, not in this repository.

This establishes one **cycling ANT+ client pairing and displayed-value baseline**.
It does not qualify BLE, FTMS/resistance control, running profiles, real sensors,
multiple riders, in-world physics, cold-boot reliability or public competition.
The next independent transport check is BLE reception/parity; SkiErg remains
parked until the equipment is available.
