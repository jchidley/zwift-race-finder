# Review an approved shared-library session

## Scope and prerequisites

Coach's supported workflow is an **agent-assisted, read-only review**, not an
application command. Follow the [first-review contract](../explanation/FIRST_RIDE_REVIEW.md)
and [before/after standards](../explanation/PACING_AND_RACE_EXPERIMENTS.md#race-planning-and-review-standards--d2).
There is no maintained executable intake/report pipeline or coaching regression
suite. Do not promote a frozen analysis script into one.

The private library is at `C:\Users\jackc\OneDrive\FitnessData`, accessible here
as `/mnt/c/Users/jackc/OneDrive/FitnessData`. **Its root `README.md` owns the
directory convention**, inventory and copy-receipt interpretation. Read it first;
this page describes only Coach's use. Local access does not verify cloud sync,
another machine's availability or independent backup.

The combined project owns reviews, pacing plans, experiment evidence and retained Sauce review
evidence. Existing `projects/zwift-race-coach/` and local Coach-named paths remain
valid evidence locations; do not rename or republish them as part of consolidation. It reads approved shared FIT activities
and ZwiftPower exports in place, plus explicitly approved source snapshots as
needed. Do not maintain another Coach copy of shared FIT/results. Acquisition,
sensor processing and hardware/control remain outside Coach.

Use Bash from the combined project root (currently `~/git/zwift-race-finder`;
planned canonical root after handoff: `~/git/zwift-racing`). Keep processing, dependency caches,
intermediates and draft reports on WSL ext4, outside Git; for example
`~/.local/share/zwift-race-coach/`. Do not move live recording locations.
Only completed, approved review artifacts belong in the library's Coach area.
No `FITNESS_DATA_ROOT` support is added: explicit paths and a shell-local
`library` variable suffice.

## Explicit-path procedure

1. Read the approved session index and follow its links manually. The manifest
   is navigation, not a machine-readable API. Confirm input approval and the
   review question; do not search for other rides.
2. Select the required FIT and result files. Verify SHA-256 against the manifest
   **before analysis**; stop on mismatch or unavailable bytes. The root inventory
   and receipt describe migration, not present availability.
3. Create a fresh local output directory. Supply each input path and that output
   directory explicitly to the reviewing agent. Inspect actual fields, units,
   timestamps, gaps, missing/zero values, duplicate names and timer metadata.
   Read library bytes directly; buffering a FIT in local memory is fine.
4. Perform only the justified one-off calculations. Retain their source,
   decoder versions, input paths/hashes, policies and check results privately.
   Independently recompute representative values. Do not execute or redirect a
   frozen script merely because it exists.
5. Write a new local `review.md` and supporting provenance/check evidence.
   Preserve interval endpoints, missing-data and weighting policies, activity
   versus race timing, result precision/status limits, and observations versus
   capacity/tactical interpretation. Missing Sauce, bike logs or a pre-race plan
   narrow claims; they do not block a FIT/results review.
6. Compare overlapping deterministic results with frozen evidence, naming
   differences in scope/policy rather than forcing equality. Hash inputs again
   after processing.
7. Before publication, propose the **exact new destination and artifact list**,
   then obtain confirmation. Do not overwrite a prior review, update the shared
   manifest or copy private scripts/intermediates into OneDrive implicitly.

### Tested November 5 pilot

Read `shared/sessions/2026-11-05-tour-of-watopia.md` in the library. From the
canonical checkout in Bash, this selects the approved inputs and creates only a
fresh local directory:

```bash
library=/mnt/c/Users/jackc/OneDrive/FitnessData
fit="$library/shared/activities/zwift/2026/2026-11-05-15-07-40.fit"
results="$library/shared/race-results/zwiftpower/2026/zwiftpower-history-2026-11-05T16-45-51-851Z.json"
prior="$library/projects/zwift-race-coach/reviews/2026-11-05-sauce-fit"
work="$HOME/.local/share/zwift-race-coach/shared-library-pilot-2026-11-05"
mkdir -p "$work"
out=$(mktemp -d "$work/run-XXXXXX")
sha256sum "$fit" "$results" "$prior/alignment.json" "$prior/zwiftpower-match.json"
```

Compare these hashes with the session table, not merely with each other.
For a new review, supply `$fit`, `$results` and `$out` to the agent and follow
steps 3–6. Read the prior review as historical evidence, not as established race
boundaries.

The pilot's retained private calculation can be reproduced **on this machine**
with the following invocation after the selection above. It needs the private
source at `$work/check_inputs.py` and existing cached uv-managed Python,
`fitdecode==0.11.0` and `garmin-fit-sdk==21.218.0`. It is a bounded one-session
evidence calculation, not shipped Coach tooling or an installation command.
It requires an empty local output directory, checks the four manifest hashes
it uses and writes `intake.json`; a human/agent writes the review separately.

```bash
uv run --offline --no-project --managed-python \
  --with fitdecode==0.11.0 --with garmin-fit-sdk==21.218.0 \
  python "$work/check_inputs.py" \
  --fit "$fit" --zwiftpower "$results" \
  --prior-alignment "$prior/alignment.json" \
  --prior-match "$prior/zwiftpower-match.json" --output "$out"
```

See [the scoped pilot result](../project-history/COACH_STATUS_PRE_CONSOLIDATION.md#shared-library-pilot--2026-11-05)
for the retained evidence location and limitations. No Downloads or live-capture
path is needed for this procedure.

## Frozen paths and remaining tooling gap

The copied November 5 `inspect_alignment.py` explicitly identifies itself as
one-off analysis. It hard-codes a Downloads FIT path, a Windows live Sauce session
directory, and output beside its own source; it also reads the historical stop
file. The frozen alignment/result evidence retains original source paths.
Leave all of these unchanged. Do not run that script inside the library.

The maintained review procedure previously required an explicitly supplied
activity path but had no executable path defaults to change. This integration
therefore changes documentation only. A reusable review command, validated
numerical boundary and synthetic regressions remain unfinished work in
[TASKS.md](../../TASKS.md#supporting-ride-review-implementation--deferred-until-justified).
The private pilot source does not close those implementation gates or reproduce
the Sauce offset fit.
