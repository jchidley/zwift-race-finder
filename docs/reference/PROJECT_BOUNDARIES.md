# Project boundaries and documentation migration

Historical Coach evidence retained during consolidation. Current state and ownership
are in the root [status](../../STATUS.md) and [documentation map](../README.md). Dated claims below are not current acceptance.

## Ownership

| Project | Owns |
|---|---|
| zwift-race-coach | Ride reviews, pacing plans, effort-constraint research, higher-level strategy and controlled experiment coordination. |
| [fitness-bridge](https://github.com/jchidley/fitness-bridge) | Sensor acquisition, source selection, filtering/transformation, synthetic measurement sources, BLE/ANT+ output and their qualification. |
| Life Fitness controller project | Custom hardware, Pico firmware, physical resistance and actuator safety. |
| zwift-race-finder | Event selection, production duration estimation, client route collection/import, and route-data provenance. |
| Existing zwift-offline checkouts | Offline session infrastructure and upstream controller/replay implementations; inspect before reuse. |
| netbrain/zwift | Candidate Linux container runner; keep its checkout and lifecycle independent. |
| vpower / vspeed / zoffline-bots | Reference implementations/data with separate licenses and operational boundaries. |

**Finder takes care of itself.** Its implementation, backlog, repairs and validation are not assigned to this project or prerequisites for coaching. Do not initiate finder work from a coach task.

If a selected workflow needs external route/event data, consume an explicit supplied snapshot and preserve its semantics/provenance. An existing finder export is an optional input, not a required integration. Use small versioned files rather than shared libraries or cross-project database writes.

## Moved from race-finder on 2026-10-30

The owner requested an independent coaching project and relocation of relevant notes:

- `docs/reference/ZWIFT_ECOSYSTEM.md`: inspected local repositories, pinned revisions, multi-instance evidence, device limitations, and licensing qualifications.
- `docs/reference/SIMULATION_TOOLS.md`: older simulation research, preserved with its unverified/historical qualifications.
- The deferred netbrain/zwift evaluation task now belongs to this project's `TASKS.md`.

Race-finder's documentation index retains a local-workspace handoff link. Its route collector/importer, client-source documentation, application database, licenses, audit history, and unrelated unpublished changes remain there. No private rides, database, client route export, credentials, or third-party code were moved.

From this project root, local route-data references are:

- `../zwift-race-finder/docs/howto/CLIENT_ROUTE_DATA.md`
- `../zwift-race-finder/docs/reference/ROUTE_DATA_EXTRACTION.md`
- `../zwift-race-finder/docs/reference/ALGORITHMS.md`

These are sibling-workspace paths, not promises that unpublished documents exist on GitHub.

## Fitness Bridge handoff — 2026-10-31

The private Fitness Bridge repository was published and independently verified at commit `2b12f24f3d7b2c5fd5e56d8c6cbdcbdf2baaab01`, with Actions disabled and all 60 approved files. Its canonical root is ~/git/fitness-bridge; keep implementation and sessions separate. Publication is not completion of the bridge executable.

The coach's active bridge implementation backlog is replaced by an ownership pointer. Historical device reports and diagnostic scripts stay here, not as maintained bridge components. The [original combined design](../explanation/BRIDGE_AND_MULTI_RIDER_DESIGN_2026-10-30.md) remains byte-identical; its [former path](../explanation/BRIDGE_AND_MULTI_RIDER_DESIGN.md) now describes coach integration. Fitness Bridge's historical extraction hashes identify the original source bytes, not future revisions at the same pathname.

Recorded-activity analysis can proceed independently. Later live experiments consume qualified bridge interfaces; they do not duplicate protocol implementation or physical-control ownership here.

## Licensing

The owner requested the Rust-style choice of MIT **or** Apache-2.0 using the GitHub skill and `jchidley/license_template`, not a hand-written substitute or an AND requirement. Template preview/application status is recorded in `STATUS.md`. Project licensing does not supersede third-party code, replay-data, personal-data, or proprietary Zwift asset restrictions.
