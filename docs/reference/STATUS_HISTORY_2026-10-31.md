# Historical status through 2026-10-31

Historical Coach evidence retained during consolidation. Current state and ownership
are in the root [status](../../STATUS.md) and [documentation map](../README.md). Dated claims below are not current acceptance.

This preserves the earlier chronology. For current state and ownership, see [STATUS.md](../project-history/COACH_STATUS_PRE_CONSOLIDATION.md). Historical “current” and “next session” statements below describe their original checkpoints.

## Local setup — 2026-10-30

The canonical project directory is `~/git/zwift-race-coach`. It contains project instructions, a README, dual-license files, privacy-oriented ignore rules, documentation, and tasks. Ecosystem/multi-instance references and the simulation catalogue were moved out of the sibling race-finder project; navigation was repaired. The owner's supplied experiment summary is retained with explicit qualification limits.

No application/runtime scaffold, dependencies, application tests, personal-data imports, simulations, or container installations exist yet. No private records, client assets, or third-party code were copied. Reference tools remain separate repositories.

## GitHub and licensing — approved and applied

Repository: https://github.com/jchidley/zwift-race-coach

- Private, branch `main`, GitHub Actions disabled.
- The owner approved the preview. The GitHub skill regenerated its plan, and an exact comparison confirmed it matched before confirmation was supplied.
- Created `README.md`, `LICENSE-MIT`, and `LICENSE-APACHE` using the owner's `jchidley/license_template` sections and official SPDX texts. Applied identical files locally.
- Independent authenticated reads verified every planned repository setting, Actions policy, the three remote files, and exact file contents; local copies match.
- Verified remote head: `e9982b0eb5de6927b337376afe9572cb05ec2e0d` at 2026-10-30T08:28:42Z.
- [Setup evidence and content hashes](GITHUB_SETUP_PREVIEW.md) record the operation. Do not rerun repository creation: the repository now exists.

The remote currently contains only the README and two license files. The moved documentation, instructions, ignore rules, and task/status files are still local and unpublished. No local Git initialization, staging, commit, remote linking, or push has occurred. The skill created the three bootstrap commits remotely through GitHub's API.

## Read-only repository research — 2026-10-30

Inspected the new PyRow, python-ant, PyUSB and zwift-packet-monitor checkouts, relevant repositories on the authenticated owner's GitHub listing, and written pointers in `~/git/mkdocs-material-test`. Saved [pinned findings and follow-up leads](REPOSITORY_RESEARCH_2026-10-30.md). No implementation, installs, device operations, personal-data imports or remote writes were performed.

**Setup discrepancy:** GitHub now has private `jchidley/zwift-race-coach` (observed HEAD `e9982b0eb5de6927b337376afe9572cb05ec2e0d`), while this local root still has no `.git`. The earlier setup narrative above is historical, not current proof of remote absence. Creation provenance, remote settings/license completeness, and local/remote reconciliation were not established or changed in this research task.

## Session documentation review — 2026-10-30

Screened ten pre-existing logs attached to the race-finder working directory and inspected coaching-relevant messages/tool results. Added [the session handoff](SESSION_HANDOFF_2026-10-30.md), including route-export reuse, fixed-duration/precision lessons, OCR/import/accuracy limitations, and corrections to superseded claims. The quoted external controller/ghost summary remains qualified, not runtime validation. No personal activity payloads or credentials were copied, and no tests or experiments were rerun for this documentation review.

Reconfirmed that this directory has no `.git`. The already-created private remote and the absence of a local Git checkout are separate facts; this review did not change either Git setup or GitHub state. New notes remain local and unpublished.

## Bridge and multi-rider requirements consolidation — 2026-10-30

Added [the current design handoff](../explanation/BRIDGE_AND_MULTI_RIDER_DESIGN_2026-10-30.md):
offline/local and no-extra-payment preference; Assioma, SkiErg and Forerunner 265;
both ANT+ and BLE; real-input scaling and synthetic bots; running-first SkiErg
output with cycling retained; separate physical control and multi-client
coordination; evidence limits, open questions and staged acceptance checks.

Confirmed SmartSpin2k is now locally checked out and clean at
`df59de54a85459e1b155a6f1b54c29d159de8481`, matching the earlier upstream
inspection. Its local documentation/source includes FTMS/Dircon and accelerated
packet-integrated controller simulation tooling; this is not Zwift race physics
or a verified radio/physical bench. Newly inspected Bleak, Bless, OpenANT and
zoffline-bots checkouts provide implementation references, not selected runtime
dependencies. See the design and expanded comparison for exact revisions.

Only project documentation/tasks were changed. No implementation, dependency
installation, device session, account login, application test or experiment was
performed. Monitor model and device topology remain unresolved.

## Scope clarification — finder independence

The owner directed that finder take care of itself. Active coach tasks/design now
exclude finder repairs, validation and mandatory integration. Route/event data,
if needed by a concrete workflow, is an explicit supplied input. The earlier
session handoff remains historical evidence only, not a coach backlog or gate.
No sibling-project files were changed.

## Native Windows clients and Debian bridge preparation

Owner confirms both Windows laptops run Zwift and support external screens;
use them as the client baseline rather than requiring a container runner.
Live SSH checks confirmed WYK82Q and WYKH32J on Debian 13.7 with 7.7 GiB RAM
and kernel-visible Bluetooth controllers. WYK82Q is the preferred bridge candidate.
Bluetooth services were inactive; no ANT+ stick was identified attached.

Completed [QZ source audit and bounded test plan](QZ_SKIERG_AUDIT_AND_TEST_PLAN.md).
The default SkiErg FTMS treadmill-data path has a device-type mismatch; RSC is
a candidate but passes strokes as cadence. No source-expiry guarantee was found.
No installs, scans, sensor connections, advertising or client launches occurred.
Monitor identification remains the prerequisite to choosing the actual input test.

## Updated owner hardware inventory

Recorded Silver's Ryzen 7 / 8 GB / 1 TB SSD specification and the NUCs'
i5-4250U / 8 GB configurations (WYK82Q 120 GB mSATA; WYKH32J 500 GB BX100).
The owner confirms Silver's installed Windows works fine and runs Zwift.
The recovery-USB WinRE record describes a separate inspection, not its normal
operating state. No recovery or normal-boot prerequisite is assigned here. See the [hardware/test note](QZ_SKIERG_AUDIT_AND_TEST_PLAN.md).
No new live checks or machine changes accompanied this inventory update.

## ANT USB query — completed

Owner attached an ANT stick to each NUC and approved query tooling, elevated
USB access, driver detachment/restoration and reset. Both queries succeeded:
WYK82Q USB-m (0fcf:1009), AJK2.07RAF, 8 channels / 8 networks;
WYKH32J USBStick2 (0fcf:1008), AJK1.04RAF, 8 channels / 3 networks.
Installed libusb runtime on WYK82Q and libusb development headers on WYKH32J;
no upgrades or permanent driver/permission changes. Added a small C diagnostic,
not an application scaffold or runtime selection. Both original drivers restored
and independently verified. No ANT channels opened or sensor data transmitted.
[Evidence, packages, probe source and limits](ANT_USB_CAPABILITIES.md).
Earlier no-dependency/no-device-operation statements describe prior stages.

## Gymnasticon card — prepared and boot-verified

Owner booted the prepared card as gym on a Pi 3 A+ at 10.0.1.148. First-run
completed, SSH works as jack, and Gymnasticon 1.4.0 is running at 0 W on UDP 3000.
Both C2 and C1_ASUS configurations were prepared using their own credentials;
service account was reconciled with first-run's pi→jack rename. Initial Bluetooth
ENODEV failures caused two automatic restarts before successful startup.
Cadence is undefined; source expiry, radio outputs and client pairing remain
unqualified. No bot commands were sent. [Device evidence and limits](GYMNASTICON_DEVICE.md).
Oldgarage's earlier erroneous C1_ASUS key copy remains pending correction;
its C2/SSH worked before the same hardware was switched to gym.

## Native Windows ANT receive bench — passed with limits

Used the owner's newly attached Windows USBStick2 and its existing driver with
a bounded native Windows Python/PyUSB receiver. Gym's sensor 11234 was received:
37 zero pages in the baseline, then 100 pages showing 0/0 → 150 W/80 rpm →
250 W/95 rpm → 0/0. Ninety consecutive same-value accumulated-power checks
matched; four event-count gaps occurred. No rollover or Zwift pairing tested.
Positive values persisted during command silence, confirming the freshness gap.
Bot returned to zero and receiver resources were released; no new service restarts.
[Exact setup, diagnostic and evidence](ANT_WINDOWS_RECEIVE_TEST.md).
No Windows driver changes or USB/IP attachment were needed; a temporary uv-managed
PyUSB dependency was used. BLE remains untested.

## Gymnasticon receive-side watchdog — installed and tested

Gym's installed 1.4.0 bot now starts at explicit 0 W / 0 rpm and expires complete
UDP samples after 2500 ms using a monotonic clock, with 100 ms watchdog polling.
Malformed/partial/out-of-range input does not refresh the deadline. Installed with
private backups and a scoped service restart; startup zero and active service
verified. Radio-free tests passed on its actual Node 12 runtime.

Windows independently received 110 pages covering invalid traffic, heartbeat
maintenance, silence, recovery and zero. Gym's local acceptance-to-zero timings
were 2.520, 2.593 and 2.553 s. One Windows-send-to-zero case was **3.745 s**;
receive-side expiry is not an end-to-end producer-age guarantee. No BLE, frozen-
host, process-crash or cold-boot guarantee is claimed. Final output is zero.
[Implementation, evidence, backup/rollback and limits](GYMNASTICON_WATCHDOG.md).
This supersedes the earlier undefined-cadence/no-expiry observations for the
currently installed service; the separate 1.4.1 checkout remains unchanged.

## Zwift pairing attempt — blocked at client startup

Launched the installed Windows Zwift launcher for a pairing-screen-only check.
ZwiftApp opened but remained on its blue splash screen across visual checks;
Windows reported Responding=False during the bounded startup check, although
CPU time increased. Root cause and whether startup would eventually complete
were not established. The pairing screen was not reached, no devices were paired,
no positive bot commands were sent, and no ride/event was entered. Gym remained
active at 0 W / 0 rpm with NRestarts=0. No driver/settings/login changes or forced
process termination were performed; Zwift was left running. Desktop captures
remain in Windows user Temp, outside this repository. Resume pairing after the
client reaches its device screen; this attempt is not pairing compatibility proof.

## Zwift ANT pairing — verified after startup completed

The owner reported startup completion; Zwift was subsequently responsive. After
the owner left the shared world, directly inspected pre-ride Paired Devices
captures showed ANT+ Powermeter 11234 connected for both power and cadence:
0 W/0 rpm → 150 W/85 rpm → 0 W/0 rpm. The final zero was observed after producer
silence and before explicit cleanup zero. This is not an exact client-latency
measurement. No ride/event was started during positive output. Final gym output
zero, service active, NRestarts=0. Zwift left on Paired Devices.
[Evidence and unsuccessful-observation limits](ZWIFT_ANT_PAIRING_TEST.md).
BLE remains untested; SkiErg qualification is parked until equipment is available.

## Zwift BLE pairing — verified separately from ANT+

Owner switched Power Source and Cadence to Gymnasticon over BLE. Directly inspected
pre-ride captures showed both connected, 0/0 → 150 W/85 rpm → 0/0 after command
silence and before explicit cleanup zero. No ride started; final source output
zero, service active, NRestarts=0. BLE selected on the left-open pairing screen.
[BLE evidence](ZWIFT_BLE_PAIRING_TEST.md).
This supersedes earlier BLE-untested statements for the one-client cycling pairing
baseline only. Basic sequential displayed-value parity with ANT+ is verified;
simultaneous reception, detailed wire/profile behavior, long runs and reconnect
remain unqualified. No new implementation, driver or service change was needed.

## Hardware move — BLE zero baseline observed, network unresolved

After the approved shutdown, the owner reported gym running on new hardware.
Direct inspection of Zwift's pre-ride pairing screen showed Gymnasticon over BLE
connected for both power and cadence, displaying 0 W / 0 rpm. No positive
prescription was sent after this move. Windows/WSL DNS and the router lease still
pointed gym to 10.0.1.148 (the previously observed Pi 3 A+ MAC); strict SSH returned
No route to host. This does not establish the new board's model, address, service
state or watchdog-file integrity. Repeat the pulse/expiry check once its control
endpoint is reachable. The earlier complete ANT/BLE tests apply to the pre-move
hardware. Capture remains private in Windows Temp: zwift-new-gym-ble.png.

## Pi Zero 2 W — hardware and BLE retest verified

Strict SSH to the owner-provided 10.0.1.61 with HostKeyAlias=gym verified Raspberry
Pi Zero 2 Rev 1.0 and the unchanged watchdog/config. A direct native Windows UDP
pulse produced independently inspected pre-ride BLE readings 0/0 → 150 W/85 rpm →
0/0, with zero captured before cleanup. No ride started. [Retest evidence](ZWIFT_BLE_PAIRING_TEST.md).
SSH was intermittently unreachable from WSL, then succeeded on retry; cause not
isolated. At ten minutes uptime service PID 583 remained active, NRestarts=2,
output zero and throttled=0x0. Boot errors were ENODEV and Bluetooth poweredOff;
startup readiness remains unresolved. Boot reported no ANT+ stick found: ANT
output on this new board is not qualified. This supersedes the unresolved model/
IP/pulse status in the preceding hardware-move note. No network/service settings
were changed, and the stale gym DNS name was not rewritten.

## Zero 2 reboot — ANT initialization and discovery restored

Direct SSH and a router-jump attempt could not deliver the requested reboot;
router forwarding was prohibited and no settings were changed. The owner then
reported the board back up. Strict SSH verified roughly one minute uptime and
boot ID b9c901a2-b9d3-495c-9dce-419e339b2943. USB now enumerates ANTUSB-m 0fcf:1009;
Gymnasticon's current boot reports ANT+ stick opened. Service PID 552 active,
NRestarts=1 after initial Bluetooth ENODEV, output zero, throttled=0x0.

Direct Zwift pre-ride inspection showed Powermeter 11234 discovered and selected
for ANT power, with BLE Gymnasticon cadence still connected. The attempted mixed
ANT-power/BLE-cadence pulse was stopped by the foreground guard before positive
commands or captures, so no new positive-output or mixed-transport parity result
is claimed. Earlier no-ANT-on-Zero-2 observations are superseded for enumeration,
backend initialization and device discovery only. Complete the pulse with Zwift
kept foreground to qualify live readings. Captures remain in Windows Temp.

## Zero 2 ANT pulse/expiry retry — passed

After the foreground abort and a Windows/WSL launch timeout, the successful retry
used ANT Powermeter 11234 for **both** power and cadence (not the earlier mixed
ANT/BLE selection). Directly inspected pre-ride captures showed 0/0 → 150 W/85 rpm
→ 0/0; the zero capture preceded cleanup zero. No ride was started; Zwift left on
Paired Devices with ANT at zero. [Zero 2 retest evidence](ZWIFT_ANT_PAIRING_TEST.md).
This supersedes the pending-positive-test status above. Both ANT and BLE cycling
baselines have now passed separately on the Zero 2 W, not simultaneously and not
as a sustained/repeated-cold-boot qualification. Startup readiness remains open.

## Next session

Restart Pi from `~/git/zwift-race-coach` before project Git/build/implementation work. Do not continue development by changing directory inside the race-finder session.

Review scoped initialization and publication of this project's local documents. Preserve the existing remote `main` history and avoid force-pushes or unrelated race-finder files. The approved operation covered repository creation/settings and three template files, not publication of the remaining local project files.
