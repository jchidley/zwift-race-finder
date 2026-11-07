# Review an approved shared-library session

## Scope

Review is agent-assisted, not a maintained application command. Follow the
[first-review contract](../explanation/FIRST_RIDE_REVIEW.md) and
[pacing/review standards](../explanation/PACING_AND_RACE_EXPERIMENTS.md#race-planning-and-review-standards--d2).
There is no maintained intake/report executable, selected coaching runtime or
coaching regression suite. Do not promote a frozen script into one.

The FitnessData root README owns the central directory convention. Supply its
machine-local path explicitly: `$HOME/FitnessData` (`/home/jack/FitnessData`) on
this machine, not OneDrive. No cross-project `FITNESS_DATA_ROOT` setting is
implemented. The [evidence library index](../reference/EVIDENCE_LIBRARY.md)
locates the completed original pilot and its run-specific calculation evidence.

## Procedure

1. Confirm the explicit approved activity/result paths, session index and review
   question. Do not search personal folders or fetch account data implicitly.
2. Read the session index as navigation, not a machine-readable API. Verify
   selected input hashes against its evidence records before analysis; stop on
   missing bytes or mismatches. An old migration receipt is not a current hash.
3. Create a fresh local output directory on Linux/WSL ext4, outside Git. Read
   approved inputs directly from the library, or use verified local copies where
   processing needs stable offline inputs. Keep caches/intermediates local.
4. Inspect actual fields, units, timing, gaps, missing versus zero, duplicate
   names and timer metadata before selecting the smallest justified calculation.
5. Retain calculation source, decoder versions, exact input identities, declared
   weighting/coverage/interval policies and independent arithmetic checks in
   private run evidence. Do not execute historical analysis just because it exists.
6. Produce a new local review. Preserve activity-versus-race timing ambiguity,
   result precision/status, and observation-versus-capacity/tactical distinctions.
   Missing Sauce, bike telemetry or a plan narrows claims; it does not block a
   useful FIT/results review. Where prior evidence overlaps, compare deterministic
   results with the frozen evidence and explain scope/policy differences rather
   than forcing equality. Recheck input hashes after processing.
7. Propose an exact new library destination and finalized artifact list before
   publication. Do not overwrite prior evidence or edit a shared session index
   implicitly. Verify destination hashes and retain original local evidence.

## Explicit input selection

From the combined checkout in Bash, use the current local library and replace
the activity/result placeholders with the approved selections. This creates only
a fresh local working directory; it is not a review or a decoder invocation:

```bash
library="$HOME/FitnessData"
fit="$library/<approved-activity-relative-path>"
results="$library/<approved-results-relative-path>"
work="$HOME/.local/share/zwift-race-finder/reviews"
mkdir -p "$work"
out=$(mktemp -d "$work/run-XXXXXXXX")
sha256sum "$fit" "$results"
```

Compare hashes with the selected evidence records, then supply `$fit`, `$results`
and `$out` to the reviewing agent. Choose any parser only after field inspection;
use uv-managed Python if Python is selected. No generic invocation of a historical
private calculation is supported here.

## Boundaries and remaining work

Fitness Bridge owns acquisition, transformations and BLE/ANT+ output; Life Fitness
owns hardware/control. No live recorder location, device or account is changed by
this procedure. Existing Coach-named evidence directories remain valid; do not
rename frozen paths to match the combined repository name.

Completed reviews belong in the library; working state remains local. Local
readability and checksum agreement do not establish independent backup/restore
coverage, race boundaries, calibration or physiological validity. Reusable tooling
and synthetic numerical regressions remain in [TASKS.md](../../TASKS.md#supporting-ride-review-implementation--deferred-until-justified).
