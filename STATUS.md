# Status

## Current state — 2026-11-03

This project owns recorded-ride reviews, practical pacing plans and higher-level controlled strategy experiments. One explicitly supplied private FIT activity has been inspected read-only and an exploratory ride review produced in the project conversation. No maintained coaching application, intake/report command, synthetic coaching regression suite or durable private report exists yet. The full first-review milestone is not accepted.

[Fitness Bridge](https://github.com/jchidley/fitness-bridge) now owns sensor acquisition, processing and BLE/ANT+ output. Its private repository was published at `2b12f24f3d7b2c5fd5e56d8c6cbdcbdf2baaab01`, with all 60 approved files and Actions disabled independently verified. That is a design/reference baseline, not an implemented bridge. Life Fitness retains hardware/Pico/physical-control ownership.

## Coach repository

- Canonical root: ~/git/zwift-race-coach.
- Existing private remote: https://github.com/jchidley/zwift-race-coach.
- The 2026-10-31 authenticated recheck confirmed main at `e9982b0eb5de6927b337376afe9572cb05ec2e0d`, with only README.md and the two licence files and Actions disabled. Remote contents/settings have not been rechecked during this documentation repair.
- Local main tracks origin/main; the approved package was committed and pushed at `78a7bb224443dcd9220860793c687bf58fc43447`, preserving the bootstrap history. The verification receipt below records that publication checkpoint.
- The owner-approved complete package is published: README, .gitignore, AGENTS, STATUS, TASKS and all current docs/scripts. GitHub API verification matched all 38 file path/mode/blob entries at the publication checkpoint. Existing licence files and README template sections are unchanged.

Preparation and publication evidence is kept outside the repository at ~/.local/share/zwift-race-coach-setup/. The earlier approved bootstrap is documented in [GITHUB_SETUP_PREVIEW.md](docs/reference/GITHUB_SETUP_PREVIEW.md).

## First milestone and next action

[First ride-review contract](docs/explanation/FIRST_RIDE_REVIEW.md): inspect one approved private activity, report supported effort patterns and data quality, then add an event-specific pacing plan when event/rider context is available.

The activity-path prerequisite was supplied and used for read-only inspection; no personal directories were searched and no activity was copied into the checkout. Recorded review does not depend on Fitness Bridge implementation or live telemetry.

**Selected next workflow:** the owner will use the independent race-finder project to select a suitable event, then prepare and review that race here. The immediate missing input is an event link or supplied route/format snapshot with a rider objective. Preserve a dated private plan, then compare it with the approved completed recording under the [before-and-after standards](docs/explanation/PACING_AND_RACE_EXPERIMENTS.md#race-planning-and-review-standards--d2). Supplied rider evidence can support a conditional plan without a simulator, a new maximal test or a reusable command.

The earlier recommendation to select a runtime/build a review command first is superseded by this concrete race workflow, not completion of that implementation. Python/uv with `fitdecode` remains a candidate, not an adopted runtime. Reproducible code, synthetic regressions and durable private review evidence remain unfinished acceptance work; implement only a bounded calculation when a real planning question justifies it.

### Exploratory intake and review — 2026-11-03

Session observations: `fitdecode` 0.11.0 completed strict decoding/CRC checks on the explicitly approved private input. Garmin FIT SDK 21.218.0 independently decoded it with CRC checking and matched representative timestamp and declared-policy power/HR/cadence calculations; selected phase/window calculations were also recomputed. These were one-off commands, not maintained project tests. The review distinguished a main effort, easier riding, renewed effort and final recovery; those labels combine trace interpretation with supplied rider context, not automatic proof of intent.

Provenance/limits: local HEAD `e9982b0eb5de6927b337376afe9572cb05ec2e0d`, with modified README and untracked instructions, state/queue, docs, scripts and ignore file; the exploratory commands were not committed source. This is a safe summary of session observations only: no immutable private-input identifier, command transcript or detailed report has been retained as durable private evidence. It does not establish reproducible acceptance or general coaching correctness. Private paths, activity timestamps, hashes, coordinates, rider metrics and HR context are intentionally omitted from Git. Retain suitable private evidence before relying on these observations for milestone acceptance.

Focused documentation reconciliation on 2026-11-03 preserved the existing README/AGENTS/status/queue/contract roles and historical records. At the same HEAD and dirty-state scope above, six documents were aligned: README, AGENTS, STATUS, TASKS, the first-review contract and pacing explanation. Scoped content review confirmed obsolete intake blockers were removed while implementation, regression, private-evidence and publication requirements remained open. A read-only Node check passed for 43 local links/anchors, whitespace across all six files and known stale-intake statements; `git diff --check` passed for tracked changes only. Essential newly linked status/queue destinations were inspected. These results support documentation consistency only; no application or diagnostic tests were run, and no private evidence was created by this repair.

### Supplied prior work and research — 2026-11-03

The owner supplied an earlier GoldenCheetah assessment describing a recent multi-ride power-duration baseline and independent agreement with underlying samples. This is a prior-work summary, not a profile inspection performed or newly verified in this session. No profile paths, rider power/HR values, activity details or fitted parameters are copied into Git. Durable private evidence has not been located or revalidated; use an explicitly supplied evidence location if stronger reliance is needed, not an implicit rescan of the profile. Demonstrated efforts, configured parameters and repeatability remain distinct under S6/S7.

Focused sports-science research informed D2; [source access levels and transfer limits](docs/explanation/PACING_AND_RACE_EXPERIMENTS.md#research-basis-and-transfer-limits) are retained in the pacing guide. This qualifies the reasoning approach, not individual physiology, race targets or a chosen model. A first conditional plan can use the supplied baseline with that qualification; stronger predictive claims require stronger evidence.

## Historical evidence

The Gymnasticon and ANT/BLE reports remain here as historical device/output evidence. The [original combined bridge design](docs/explanation/BRIDGE_AND_MULTI_RIDER_DESIGN_2026-10-30.md) is preserved byte-for-byte; the [current integration note](docs/explanation/BRIDGE_AND_MULTI_RIDER_DESIGN.md) assigns only strategy/telemetry coordination here. Native Windows is the selected first live-client baseline; earlier container priorities are superseded.

[Status history](docs/reference/STATUS_HISTORY_2026-10-31.md) retains the earlier setup, device and pairing chronology. Historical source results do not validate the unimplemented Rust bridge or coaching models.

Next actions are in [TASKS.md](TASKS.md). No hardware operations, installs, account changes or live experiments were performed during this handoff.

## Publication verification — 2026-11-03

Checked checkpoint: `78a7bb224443dcd9220860793c687bf58fc43447`, with a clean working tree and local origin/main matching HEAD immediately after push. This is the committed 36-file approved change set on the existing bootstrap; the complete tree contains 38 files including the two unchanged licences.

Named checks/results: GitHub Git-ref API main matched that full commit; recursive Git-tree API returned an untruncated tree whose 38 file path/mode/blob entries exactly matched `git ls-tree -r HEAD`. Repository API confirmed private visibility and default main; Actions-permissions API confirmed disabled Actions. Local tracking and `git status --porcelain` checks passed. Final staged package/link checks passed for the exact allowed file set, 147 local links/anchors and `git diff --cached --check`; no ignored cache/private-input path was staged. Diagnostic source blobs are those recorded in the preparation table below.

This verifies publication contents/settings at the named checkpoint, not hardware behavior, coaching correctness or model calibration. This receipt is recorded in a follow-up documentation-only commit; it does not retroactively change the checked source or rerun diagnostic tests. Private ride data and reports remain outside Git. The two historical sibling-workspace research links remain nonportable.

## Publication preparation — 2026-11-03

Approved target: private `jchidley/zwift-race-coach`, `origin/main`. Read-only GitHub preflight confirmed private visibility, default main, Actions disabled and remote main at bootstrap `e9982b0eb5de6927b337376afe9572cb05ec2e0d`. No settings were changed. Candidate scope is 36 changed/new files plus the two unchanged licence files; .ruff_cache, private inputs/reports, generated candidates and runtime artifacts are excluded.

Publication-safety review inspected the complete candidate file inventory, diagnostic sources/tests and attribution, and sensitive-topic excerpts in historical notes. A bounded text scan for private keys, common token forms, credential URLs, literal secret assignments and raw activity payload patterns found no matching payloads; the sole key-related match was the receiver's environment parser, manually checked. This is a scoped review, not a comprehensive secret-detection guarantee. The retained historical design SHA-256 matches its recorded handoff value, and licence files are unchanged. A read-only Markdown check resolved 146 local links/anchors in 27 documents: 144 in-repository and two explicitly local-workspace research pointers to the sibling digital garden. Those two pointers are not portable GitHub links and were not used as publication prerequisites.

Using existing Node v24.21.0 and uv-managed Python 3.13.7, these checks passed on the prepared local sources (bootstrap HEAD above, dirty README and untracked package):

| Check | Result | Checked Git blob(s) |
|---|---|---|
| `uv run --offline --no-project --managed-python python scripts/ant_power_receive.py --self-test` | PASS: framing, fragmentation, checksum rejection, measurement decoding and zero/unavailable | receiver `df23e562498355b9c11fb0c89e78bf13d15c3f72` |
| `uv run --offline --no-project --managed-python python scripts/test_ant_receiver_config.py` | PASS: synthetic key parsing, value-free errors and startup rejection before USB | test `02175c49e8f5b43f60b8e346c532f9eb50f446dd`; receiver as above |
| `node scripts/test_gymnasticon_bot_input.cjs` | PASS: startup, boundaries, expiry, invalid/partial input, refresh and recovery | helper `2a440824bd798f206bacfc511090d3cf1f1af56e`; test `26ea5a3a37ce497101b07f37271177d210165c59` |
| `node --check scripts/patch_gymnasticon_bot.cjs` | PASS: syntax only | generator `1feb823a7f6cb815e6f5d637bab770ad6b22c85a` |

No hardware, USB/radio, adapter-candidate integration or C build was exercised. No candidate was supplied or generated, and no runtime was installed. These are historical diagnostic regressions, not coaching acceptance or general safety qualification. The fresh dated results above supersede reliance on the unqualified older diagnostic passes below, not their hardware limitations.

## Publication-safety checks

The historical ANT receiver source requires an explicitly supplied network key and validates it before USB access. The earlier handoff reported passes for its parser/configuration checks, the Gymnasticon helper tests and the patch-generator syntax check. Those passes lack a checked-source checkpoint/dirty-state qualification and individual dated results in this record; treat them as historical reports, not verified current acceptance. The diagnostic source/tests and documented command path were inspected during the 2026-11-03 standards review, but no diagnostic tests were rerun. See [diagnostic notes](scripts/README.md). This establishes no new hardware result or coaching-runtime selection.

## D2 adoption — 2026-11-03

The owner approved documentation-only D2: retain S1–S3, clarify S4 evidence applicability, and adopt S5 objective-led pacing, S6 evidence/capacity separation, S7 optional qualified physiological models and S8 preserved pre-race/post-race comparison. The first-review and pacing contracts own these rules; README/AGENTS route readers to them. STATUS/TASKS now put the selected race workflow ahead of optional tool development and acknowledge supplied prior work without claiming re-verification.

Scope/checkpoint: HEAD `e9982b0eb5de6927b337376afe9572cb05ec2e0d` with pre-existing modified README and untracked instructions, state/queue, docs, scripts and ignore file. D2 changes only README, AGENTS, STATUS, TASKS and the first-review/pacing contracts. It does not choose a runtime/model, change private data, implement E2 enforcement or migrate an application (M2). Existing E1 implementation/regression and publication requirements remain open. No new physiological acceptance is claimed.

Validation on 2026-11-03 at the checkpoint/dirty state above: a read-only Node documentation check passed for all six scoped files, 52 local links/anchors, whitespace, standards routing and queue-priority assertions. `git diff --check` passed for tracked changes; the Node check also covered untracked documents. Essential new local destinations were read, and a before/after content comparison confirmed that removed wording was superseded or preserved in the new owners; historical material and existing numerical/privacy requirements remain. Source-access qualifications describe the earlier research, not a new retrieval or comprehensive review. These results support documentation consistency only, not implemented enforcement, tactical efficacy or individual physiological validity. No application/diagnostic tests, installs, private-data operations, commits or pushes were performed for D2.

## Standards adoption — 2026-11-03

The owner approved D1 (documentation only): S1–S4 are recorded in the [first ride-review contract](docs/explanation/FIRST_RIDE_REVIEW.md). README now makes event planning optional and describes the first read-only intake result; AGENTS routes coaching changes to the contract, selected testing-technique example and change-specific validation. TASKS distinguishes documented adoption from pending enforcement.

Scope/checkpoint: local HEAD `e9982b0eb5de6927b337376afe9572cb05ec2e0d`, with pre-existing modified README and untracked instructions, state/queue, docs, scripts and ignore file. This tranche changes only README, AGENTS, the first-review contract, STATUS and TASKS; it does not publish that working tree. Validation on 2026-11-03: scoped content review found the standalone-review wording, S1–S4 and pending enforcement consistent; essential newly linked instruction/example destinations were inspected. A read-only Node check passed for all 30 local links/anchors and trailing whitespace across the five changed documents; `git diff --check` passed for tracked changes (it does not cover untracked files). These are documentation checks only. They do not establish coaching correctness, diagnostic passes, remote state or general agent compliance. No application code, runtime, tests or hardware operations were added or executed. E1 enforcement is deferred until approved activity intake and implementation selection; no M1 application migration is currently needed. At that documentation-only adoption checkpoint, the activity-path and publication blockers remained open; the subsequent intake above supersedes only the activity-path blocker.
