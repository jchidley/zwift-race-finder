# Race-finder session handoff — 2026-10-30

## Historical reference — not an active coach work plan

The owner subsequently directed that **finder take care of itself**. Retain this
record for provenance, but do not turn its route/OCR/importer/accuracy findings or
suggested integrations into coach tasks or prerequisites. Finder's implementation,
validation and backlog remain its own. Current scope is in the [bridge design](../explanation/BRIDGE_AND_MULTI_RIDER_DESIGN.md)
and [coach tasks](../../TASKS.md). Any route/event snapshot needed here is an
explicit input, not a requirement to inspect, repair or integrate finder.

## Scope and evidence rules

At the owner's request, screened the ten pre-existing JSONL session logs attached to the exact working directory `/home/jack/git/zwift-race-finder`, then inspected relevant messages and selected tool results. This is a focused coaching handoff, not a full forensic audit or a rerun of historical experiments. Legacy sessions attached to other directory names were not included. The helper session launched for this review adds no independent experimental evidence.

Session root: `~/.pi/agent/sessions/--home-jack-git-zwift-race-finder--/`.
Message IDs below identify entries inside those logs. Some child-session filenames contain a different UUID from their session header; use the recorded filename and header ID together. No complete transcripts, credentials, personal rider measurements, or activity payloads were copied into this project.

## 1. Decisions to retain

- The owner's goal is practical help to **pace and race better**, not merely a simulator. The independent coaching project and documentation move were explicitly requested. Source: main session **M**, user messages `945c3e11` and `8298ad9b`.
- Route metadata priority is **selected installed-client cache → extracted client XML for missing IDs → community fallback**. This is an owner decision, not just an agent suggestion: M/user `e93ba6c6`. Preserve whole-record precedence; do not blend missing fields across snapshots silently.
- The finder owns event selection and route collection/import. The coach owns ride review, pacing assistance, and experiment design. Keep those responsibilities and their mutable databases separate.
- The user supplied another session's account of `standalone_power.py`, `PowerMeterTx.py`, and fixed ghost playback in M/user `945c3e11`. That remains **quoted inspection evidence**, not a controller or replay system exercised in this project. See [the experiment design](../explanation/PACING_AND_RACE_EXPERIMENTS.md).

## 2. Existing route input worth reusing

The finder now has a client collector and importer; the coach should not start by building a duplicate route scraper.

Sibling-project entrypoints, relative to the coach root:

- `../zwift-race-finder/scripts/export_client_routes.ps1`
- `../zwift-race-finder/src/bin/import_client_routes.rs`
- `../zwift-race-finder/docs/howto/CLIENT_ROUTE_DATA.md`
- `../zwift-race-finder/docs/reference/DATABASE.md`

The normalized collector format has `schema_version: 1` and a `routes` array:

| Fields | Meaning / constraint |
|---|---|
| `route_id` | Unsigned numeric identity; signed client hashes are normalized to `u32`. Do not join by name. |
| `distance_km`, `elevation_m` | Base route distance in km and climbing in fractional metres. Not an entire multi-lap event total. |
| `lead_in_distance_km`, `lead_in_elevation_m` | Event lead-in, separate from the base route. Missing is not zero. |
| `lead_in_distance_free_ride_km`, `lead_in_elevation_free_ride_m` | Free-ride lead-in metadata, not interchangeable with event lead-in. |
| `lead_in_distance_meetups_km`, `lead_in_elevation_meetups_m` | Meetup lead-in metadata. |
| `event_only`, `sport_type`, `supported_laps` | Nullable source strings; preserve their representation rather than assuming all are Boolean flags. |
| `source` | Source kind, path, entry/file, and SHA-256. Retain these with any derived plan or result. |
| `surface` | Currently defaults to `road`; it is **not an extracted terrain fact**. |

The selected archive contained 386 IDs across 13 worlds. It covered all 353 IDs in the community reference plus 33 more; some additional definitions were running/internal routes. All 171 extracted Watopia IDs were already present, so the real export gained no extra IDs from the fallback. Synthetic tests exercise that fallback separately. Counts describe the inspected snapshot, not universal coverage or current public availability.

Private export locator, recorded in M: `~/.local/share/zwift-race-finder/client-routes-0862ca71d9b4.json`. It is intentionally outside Git. Do not copy proprietary exports or the normal rider database into this repository. The selected archive's use by the currently running game has not been established; freshness/active-cache selection remains an open check.

Evidence: M/assistant `6936ba52`; source XML tool result `b3ea2042`; collector report **C**/`1e046313`. M/tool `3d6b33c3` records the approved normal-database import: 386 verified client records, one preserved unmatched route, unchanged non-route tables, integrity OK, no foreign-key violations. That import is already complete; do not repeat it as an onboarding prerequisite.

## 3. Timing, precision, and identity traps

### Declared time is not a lap-derived prediction

The KRG event declared 2,700 seconds (45 minutes). An earlier display independently estimated 53 minutes from laps, while filtering used the declared 45 minutes. The fix made filtering and display share a resolver. Independent distance/climbing metadata was retained without overriding declared time.

For coaching, preserve whether the event target is fixed time or distance/laps. A lap count alone is not evidence that a time-limited event should be treated as a fixed-distance finish. Define event start, route start, lead-in, and finish boundaries before comparing rides.

Evidence: M/`91a39470`, `ad25caef`, `c7f3b35e`; intermediate review **D**/`a40c15b8`. That review's metadata-loss finding was addressed later; it is not an unresolved current defect merely because the older report exists.

### Aggregate fractional climbing before rounding

Downtown Dolphin (`947394567`) has client climbing of 16.6176948547 m per lap and an event lead-in with zero climbing. Fifteen laps total approximately 249.2654 m, displayed as **249 m**. The earlier **255 m** figure multiplied already-rounded community input (17 m × 15); it is superseded, not an alternative client measurement. The 249 m correction was checked through calculation/regression coverage, not a later live KRG capture; KRG had left the sampled upcoming events.

For exact lap totals, add lap climbing × laps plus event lead-in climbing **once**. Do not scale the lead-in by the lap's average gradient. Distance-limited proportional climbing is an estimate, not a measured elevation profile. Checkpoint coordinates still need verified units/axes/segmentation before becoming a reliable profile.

### IDs can expose misleading names

Client data identifies `1258415487` as Glasgow Reverse, not the legacy seed's Bell Lap label. The Bell Lap is `2875658892`. Client imports reconciled the old seed records and the incorrect hardcoded fallback was removed. Never infer identity from a familiar route name or an old seed record.

Evidence: M/source tool `b3ea2042`, conclusions `6936ba52` and `44c670aa`.

### Prefer the identified route over generic multi-stage prose

The description parser can select the first distance in a multi-stage description, even when it belongs to another stage. Identifying California Dreamin' (`966890401`) from client data avoided that fallback in the captured Stage 4 examples: 21.2 km / 106 m / estimated 39 minutes. This does not establish that every stage/subgroup selection issue is solved.

Evidence: M/live tool `3772d8c6` and conclusion `df2cbed8`.

## 4. Evidence gates for coaching integrations

### Passing tests are not measured prediction accuracy

The accuracy regression repeatedly returned success after reporting **no races with mapped routes**. Before/after MAE was unavailable, not zero, unchanged, or improved. Require nonempty approved benchmarks, mapping coverage, and explicit excluded cases before making a coaching accuracy claim. Category-average duration estimates are not a personalized fatigue or race-physics model.

Evidence: M/tool results `940bc860`, `56862317`, `efd883a0`, `d08517f5`; final qualification `44c670aa`. These are finder tests, not tests of a coaching application.

### OCR is an optional, incompletely validated telemetry path

Recorded October 29 testing found a distance parser absorbing an adjacent time-delta digit (451.3 km became 1451.3 km), a name-snapshot conflict, missing model/image baselines, and ignored checks that reported success after swallowing model-load failures. The detection-model URL returned HTTP 403 then; this review did not retry it or establish its current availability.

Do not make screenshots/OCR the assumed reliable observation channel. Prefer structured activity or client telemetry when available, and require independent field/sequence validation before using any extraction to control effort. Source: M/tool `187bdd6e`, conclusions `2cbce4e5` and `d642397d`; audit **A**/`c0a61177`. Default-feature success and later all-feature compilation do not erase the earlier runtime failures.

### Existing import tools are not automatically safe input adapters

Audit **B**/`3bf60611` reported that ZwiftPower importer variants target different tables, and the development variant deletes race results and substitutes placeholder route IDs. It also identified stale import paths. Reinspect any proposed adapter; do not run inherited import scripts against the owner's normal data merely to obtain coaching inputs. Prefer read-only intake of one approved file and an explicit schema/field report.

The source repository's privacy issues are another reason not to bulk-copy datasets, configuration, or history into the coach. This handoff deliberately omits those payloads.

## 5. Corrections and qualifications that must not be lost

- The first elevation scout (**E**/`20eda245`) incorrectly reported that zwift-data lacked event lead-ins. Parent inspection found the tuple mapping and type definitions: M/`745fde70`. Do not repeat the superseded claim.
- Reviews **R** and **S** were code/data inspections, not executed simulations or independent physical validation. Their no-findings conclusions are not evidence of pacing-strategy efficacy.
- The September session's final record (**L**/`64caf161`) explicitly left practical return-ride advice versus official wording unresolved. Treat event eligibility/seeding rules separately from training prescriptions; recheck dated official guidance before using it. This review did not establish a validated return-to-racing prescription.
- The October 8 review (**F**) failed before producing substantive findings. It supplies no coaching evidence.

## 6. Local Git discrepancy: record it, do not repair it implicitly

The private GitHub repository exists with `main`, Actions disabled, and the README plus two licence files. The local coach directory has no `.git`; instructions and other documents remain local/unpublished. This is the intentionally unfinished linking/publication step, not evidence that GitHub creation failed.

Remote settings/content were verified in M/tool `100b26b1` (2026-10-30T08:28:42Z). This review rechecked only the local `.git` absence and did not reauthenticate, initialize Git, add a remote, commit, or push. The user's explicit note is M/user `5251dd8a`. Continue from the canonical coach project session and preserve remote bootstrap history when that separate step is authorized. See [setup evidence](GITHUB_SETUP_PREVIEW.md) and [current status](../../STATUS.md).

## Source index

All filenames below are relative to the session root above. The table lists session-header IDs, not assumed filename IDs.

| Label | Session header ID | JSONL file | Scope used |
|---|---|---|---|
| M | `01a18965-4241-774f-95cf-638636e455c5` | `2026-10-29T20-41-17-121Z_01a18965-4241-774f-95cf-638636e455c5.jsonl` | Main implementation, validation, owner decisions, setup, and later corrections. |
| A | `01a18967-0dcf-7416-8ef1-e3d3c4521d90` | `2026-10-29T20-43-13-829Z_9c82ea0f-1369-4910-9a55-07d0ae9266c2-6f199dde-3351403a-cb58.jsonl` | Rust/OCR test-boundary audit; inspection only. |
| B | `01a18967-0df2-7199-b0e2-633687f72557` | `2026-10-29T20-43-13-943Z_fa525b57-abcb-40aa-b3bb-680b163c4c4b-c10da272-0eb84209-e630.jsonl` | Importer and surrounding-tool audit; inspection only. |
| D | `01a1898d-66ba-7078-8bcf-b631fbf86cd1` | `2026-10-29T21-25-05-876Z_7e203407-352a-4938-97ea-d02954aa5245-744839f2-d46898f1-6521.jsonl` | Intermediate duration review, later fixes distinguished. |
| E | `01a189c6-fd1e-729e-9f56-97181db1cfd1` | `2026-10-29T22-28-00-965Z_d724b3c0-191d-4e16-b0ef-57c9f671b221-b08b85a5-81f25d6b-2069.jsonl` | Community-source scout, including corrected lead-in claim. |
| R | `01a189d1-fbde-723b-aa37-0463042d552b` | `2026-10-29T22-40-00-625Z_e773aefb-de03-4724-bceb-cb9c7c6fad5c-1a5ad15e-f9e657dd-6685.jsonl` | Elevation code/data review, message `47aeb9cc`; no tests run. |
| C | `01a189fb-8756-7242-a341-c913c5d112dd` | `2026-10-29T23-25-23-086Z_ea8ea93e-656d-431f-9a42-2df7ac1c81be-ce61d846-8343ef05-d428.jsonl` | Collector implementation and reported source/test validation. |
| S | `01a18a03-4e36-71b1-86cb-89a1b7e8082b` | `2026-10-29T23-33-53-157Z_cd7566a1-2a1f-4c60-ac2c-bf6af0a97a6e-a2ebc9a7-de0295e8-0491.jsonl` | Client-import Rust review, message `5ee9213c`; no tests run. |
| L | `01a09ca6-ff84-776b-ae86-29ed8f32b922` | `2026-09-13T21-23-05-222Z_01a09ca6-ff84-776b-ae86-29ed8f32b922.jsonl` | Final unresolved-advice qualification; other maintenance history omitted. |
| F | `01a11b87-7f84-771b-99bd-309e03131f72` | `2026-10-08T12-40-25-746Z_e92e7632-3bdd-424d-a00f-a568fca9f00c-d092260d-f9b26e64-5550.jsonl` | Task `4dcca2e9`, failed assistant turn `e487bba1`; no usable review. |
