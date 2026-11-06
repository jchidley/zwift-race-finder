# Project boundaries

## Current ownership

| Area | Owner |
|---|---|
| Race selection, duration estimation, route/history imports | Combined Zwift Racing project, existing Finder modules |
| Pacing plans, preserved predictions, recorded-ride review | Combined Zwift Racing project, agent-assisted contracts |
| Sensor acquisition, source selection, transformations, BLE/ANT+ output | Fitness Bridge |
| Hardware, firmware, physical resistance/control and actuator safety | Life Fitness |
| Recordings, exports, rider-specific reports, detailed run/bench evidence | Private FitnessData library |

Recorded review must work without a live bridge, client or hardware project.
Preserve explicit event/route snapshots, numeric identities, provenance and timing
semantics. Review calculations do not bypass the existing SQL/import owners or
write directly into the active application database.

Historical device reports and local operational inventories are retained in the
[evidence library](EVIDENCE_LIBRARY.md), not active implementation requirements.
Generic protocol/research references may remain here with their qualifications.

## Licensing and third-party boundaries

The project uses MIT OR Apache-2.0. Imported Coach notices remain in
[coach-licenses](../project-history/coach-licenses/); the historical Gymnasticon
anchors retain their [upstream MIT notice](../../tools/historical-coach/LICENSE-GYMNASTICON).
Project licensing does not override third-party code/data, personal-data or
proprietary Zwift asset restrictions. No live device, account, installation or
experiment permission follows from a reference document.
