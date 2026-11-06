# Local Zwift ecosystem references

Historical Coach evidence retained during consolidation. Current state and ownership
are in the root [status](../../STATUS.md) and [documentation map](../README.md). Dated claims below are not current acceptance.

Recorded 2026-10-30 at the owner's request. These Windows checkouts were inspected
read-only (READMEs, top-level license declarations, and selected implementation/docs).
No dependencies, drivers, containers, bots, or applications were installed or run.
These are research references, not adopted dependencies or validated integrations.

## Priority: netbrain/zwift for multiple client instances

- Repository: <https://github.com/netbrain/zwift>
- Local checkout: `C:\Users\jackc\git\zwift`
- Inspected revision: `d98cb5bf484847f249fad527424e8aec7fbc4171`
- Purpose: run the Zwift client on Linux in Docker/Podman containers using Wine.
  Its README explicitly identifies running multiple instances on one machine as
  the author's motivation; `docs/index.md` lists multi-instance support.

### What supports multiple instances

`docs/configuration/options.md` documents **`ZWIFT_RIDER`**, which selects a profile
without changing the host's `USER` variable. `src/zwift.sh` confirms:

- Container name: `zwift-$ZWIFT_RIDER` (line 448 at the inspected revision).
- Persistent volume: `zwift-$ZWIFT_RIDER` (line 451).
- Profile configuration: `~/.config/zwift/$ZWIFT_RIDER-config`.
- Profile graphics configuration: `~/.config/zwift/$ZWIFT_RIDER-graphics.txt`.

Distinct rider labels therefore provide distinct named containers and volumes.
This is a concrete basis for a multi-client test environment, not merely a README
claim. It does not prove concurrent login to one account is supported, or guarantee
isolation if common host log/activity/config paths are explicitly mounted.

Potential uses here: simultaneous client views for OCR comparison, controlled
multi-rider/offline experiments, and separate activity/log capture. These remain
proposals; synthetic results must not be presented as real-race accuracy evidence.

### Checks required before adoption

1. Validate one client, then two distinct profiles on the chosen Linux host. WSL,
   GPU acceleration, display integration, resource capacity, and container runtime
   compatibility have **not** been tested on this machine. The Windows checkout
   is a reference; any WSL build/run workspace should be under `~/git` on ext4.
2. Decide accounts and online/offline operation explicitly. Routing the container
   to zwift-offline is a separate, unverified integration; this project is not
   itself an offline server or an account/subscription bypass.
3. Keep credentials, profiles, activities, logs, and device control separated.
   Do not copy the upstream plaintext credential examples into this repository.
4. Validate device connectivity before combining these tools: the inspected
   `docs/getting-started/connecting-devices.md` says ANT+ is unsupported. It describes
   Companion bridging for BLE and direct-connect networking for compatible trainers.
   Consequently, the ANT+ output of vspeed/vpower is **not** an established input
   path for these containers.
5. Review the pinned launcher before execution: it can update its script/image,
   create persistent volumes, and remove old images. Do not run the README's
   remote install pipeline blindly. Installations, privileged runtime/driver
   changes, and account use need separate authorization.

Evidence: [profile documentation](https://github.com/netbrain/zwift/blob/d98cb5bf484847f249fad527424e8aec7fbc4171/docs/configuration/options.md),
[launcher](https://github.com/netbrain/zwift/blob/d98cb5bf484847f249fad527424e8aec7fbc4171/src/zwift.sh),
[device documentation](https://github.com/netbrain/zwift/blob/d98cb5bf484847f249fad527424e8aec7fbc4171/docs/getting-started/connecting-devices.md).

## Other local references

| Repository / Windows checkout | Inspected revision | Documented purpose and potential relevance |
|---|---|---|
| [oldnapalm/zoffline-bots](https://github.com/oldnapalm/zoffline-bots) — `C:\Users\jackc\git\zoffline-bots` | `f9578164784a6049458c960d53f515fbacac94c7` | Activity replays for zwift-offline bots; grouping/multipliers could supply repeatable background riders for offline experiments. Not autonomous riding logic or an authoritative route catalogue. |
| [oldnapalm/vspeed](https://github.com/oldnapalm/vspeed) — `C:\Users\jackc\git\vspeed` | `5465bf28128f391ee71caebc2486c1848d580d11` | Heart-rate-derived virtual running speed broadcast as an ANT+ stride-based speed/distance monitor. Potential sensor/UI testing input, not measured speed or race calibration data. |
| [oldnapalm/vpower](https://github.com/oldnapalm/vpower) — `C:\Users\jackc\git\vpower` | `33ee701e333ef29ca7249755af9d9f9103eb3eb1` | ANT+ virtual power derived from trainer speed/curves, with slider-controlled test bot, heart-rate/cadence variants, and Concept2 support. Potential controlled-input testing reference, not validated rider power. |

### Boundaries and license notes

- `zoffline-bots/check_bots.py` calls `os.remove` to delete unsuitable replay files;
  it is **not** a read-only validator. No top-level license file was found during
  this inspection; verify permissions before copying/redistributing replay data.
  Replay contents and suitability were not inspected.
- vspeed/vpower READMEs require two ANT+ sticks when sender and receiver share a
  computer, identify supported sticks, and warn of Cycplus incompatibility.
  Those are upstream claims, not hardware tests on this machine. Driver replacement
  via Zadig is a material system change and has not been authorized/performed.
- vspeed and vpower declare MIT at the top level; vpower includes a separate
  `LICENSE.PyRow`. netbrain/zwift's top-level license is the Unlicense. These labels
  do not license bundled third-party software, proprietary Zwift assets, or
  recorded activities; transitive licensing was not audited.
- Use virtual inputs in controlled/offline testing, not to manufacture competitive
  results or contaminate real-rider accuracy datasets.

## External route/event inputs

Finder owns its source selection, route collection/import and validation; this
project does not track or gate on that work. Consume an explicit supplied
route/event snapshot only when the selected coaching or experiment workflow
needs one. See [project boundaries](PROJECT_BOUNDARIES.md).
The larger [simulation-tools catalogue](SIMULATION_TOOLS.md) contains additional
ideas, but is not evidence that an end-to-end simulation environment exists.
