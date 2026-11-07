# Project Status

## Consolidation — 2026-11-06

The local integration preserves Finder through `a55d488` and Coach through
`b6bdccc` as parents of merge `aead11f`; neither history was squashed.
The combined workflow is find → plan → preserve predictions → review.
The working command, Rust package, estimator and application-state paths are
unchanged. Planning/review remain agent-assisted, not a delivered application.
Coach's [prior status and evidence limits](docs/reference/EVIDENCE_LIBRARY.md#coach-status-pre-consolidation)
remain historical; unfinished work is reconciled into the root task queue.
The shared-library consumer pilots are complete for their tested workflows;
new Finder/Coach publication proposals remain separate and unexecuted here.
Detailed Coach/run evidence is now retained in the FitnessData archive described
by the [evidence library index](docs/reference/EVIDENCE_LIBRARY.md). Existing
recording/export paths and frozen evidence remain unchanged. The current library
is `$HOME/FitnessData`, replacing the OneDrive plan; independent backup/restore
coverage is separate. See the [migration record](docs/project-history/CONSOLIDATION.md).

### Integration validation

Checked on 2026-11-06 against Finder `a55d488` and Coach `b6bdccc`, with
merge `aead11f` plus documentation/organization edits. Rust source, tests and
Cargo manifests were unchanged for these checks. Both source tips are ancestors;
the original bridge design and all imported diagnostic source bytes match Coach.
Scoped checks passed: 247 local Markdown links/anchors, five evaluator tests,
the Node helper and patch-generator syntax checks, and both radio-free uv-managed
Python checks. Default-feature release build and offline CLI help passed.

The full serial isolated suite **failed** the unchanged live-service verbose CLI
test (expected route/distance/category output absent). A supplementary run excluding
only that test passed **197 tests, 7 ignored**; this is not a full-suite pass.
The isolated empty-state legacy regression provides no accuracy qualification;
no formula changed and no populated-data accuracy claim is made.
Formatting still fails and Clippy reports 152 library errors, matching the prior
recorded debt. No tests, snapshots or gates were changed or suppressed.
Private command outputs and path mapping: investigation `20261106-consolidation`
under `~/.local/share/zwift-race-finder/investigations/`.

Integration review of `20379d4` found a stale separate-project statement in the
active pacing contract and one moved diagnostic build path. Both are corrected;
the historical Windows invocation is now explicitly labelled historical-only.
The reviewer verified all 22 unfinished Coach tasks were retained and confirmed
source, licence and original-design byte preservation. This was a context-isolated
same-family review, not cross-family independent verification.

### Evidence ownership — 2026-11-06

The owner clarified that the issue is storage ownership, not confidentiality:
original Git history stays unchanged. Reusable contracts, code, tests and concise
status belong here; detailed run/bench evidence and machine/session inventories
belong in FitnessData. The [evidence index](docs/reference/EVIDENCE_LIBRARY.md)
records the exact archive and receipt. It retains the original Coach bundle,
39-file checkpoint and pre-relocation combined reports. PowerShell 7 verified
59 artifact copies; independent mounted-byte verification matched all hashes
and the receipt. No originals, existing session indexes or older receipts changed.

Generic shared-library procedures now take explicit placeholder paths rather
than embedding a particular rider session. Detailed reports are removed from
the maintained tree after verified archival, not erased from Git history.
The earlier privacy-driven publication hold is superseded by this ownership
clarification; no history sanitization or rewriting is requested or performed.
The bounded Coach-history audit found no embedded credentials or raw activity
payloads; it is not comprehensive secret-detection or legal clearance.

### Repository lifecycle

GitHub rename is verified: the public repository is now
[`jchidley/zwift-racing`](https://github.com/jchidley/zwift-racing), and local
`origin` uses that URL. Package/executable/state names remain unchanged.
Current README, tutorial, Cargo homepage/repository metadata and bootstrap hints
use the new identity. The local checkout move to `~/git/zwift-racing` is the final
session handoff; frozen absolute paths remain untouched historical evidence.
Combined content was pushed to public `main` at `468de0e`; GitHub API readback
matched the local commit, preserving both original history tips. Coach retirement
commit `3aca613` marks its root entrypoints superseded; that commit was pushed
and GitHub API readback matched. The old Coach repository is verified **archived**,
still private, with description/homepage pointing to Zwift Racing. It is retained
read-only, not deleted. No force-push or history rewriting was used.
A fresh context-isolated same-family review independently matched all 59 archived
artifacts, the 39-file original checkpoint, all 18 pre-relocation documents and
the complete six-commit bundle. It found one omitted reusable comparison step;
that frozen-evidence reconciliation guidance is restored in the review procedure.

Documentation relocation checks on the tree based on `2bc701c`: 154 changed
Markdown links/anchors, all 58 maintained Bash example blocks, bootstrap Bash
syntax, five evaluator tests and `git diff --check` passed. Rust source/tests,
Cargo.lock, CI and archive are unchanged; only Cargo URL metadata changed.
These checks do not replace the previously qualified suite/build results above.

## Local evidence-library correction — 2026-11-06

The owner replaced the OneDrive plan with `$HOME/FitnessData`. Current README,
consumer procedures and library guidance now use that root; frozen receipts,
source paths, project directory names and historical transcripts remain intact.
Validation against repository `9dfa315` plus these documentation edits matched
all 250 migration entries (both inventory and receipt), 59 consolidation artifacts,
12 session-index hashes and 21 library navigation links. Saved evaluation from
the new root matched the frozen report byte-for-byte; isolated import preview
reported 1 new, 0 enriched, 196 unchanged and preserved its disposable DB hash.
The former OneDrive root was absent, so no full source-tree comparison was possible.
Private verification and consumer outputs: `fitness-data-path-review-20261106-aRSCpc`
under `~/.local/share/`.

Both unpublished proposals still have matching source hashes (Finder: 19 files;
Coach: 8); their destinations do not yet exist in the local library. No publication,
session-index edit, source retirement or recorder relocation was performed.
The installed home backup selects `/home/jack` with `--one-file-system`; the
library shares that filesystem. Snapshot inclusion and restore are not verified
by this configuration inspection. The owner subsequently clarified the recording
model: each system writes into its own local home-directory FitnessData library,
with optional rsync synchronization, not live cross-host filesystem access. The
library's `SYNCHRONIZATION.md` records options; Bridge and Life Fitness still need
recorder-specific output/completion configuration. No recorder or sync job was
changed. The archived Coach checkout is not an active owner.
See the [remaining library work](TASKS.md#local-evidence-library-follow-through).

## FitnessData follow-through — 2026-11-07

The approved pilot allowlists are now published locally: Finder has 19 artifacts,
Coach has 8, each with a new `publication-receipt.json` beneath its original
proposed relative destination. Source/destination hashes were verified before and
after copying. Root library navigation links the new runs; frozen session indexes,
original proposals and historical evidence remain unchanged.

A scoped restore from home snapshot `d9689e35` (2026-11-06 08:07 UTC) into fresh
ext4 staging matched all 250 migration files and 59 consolidation artifacts.
The scheduled-backup inspection found stale snapshots and runtime GPG
`No pinentry` failures; the interactive configured provider worked. A normal
existing home backup completed as `3a0451e2` (2026-11-07 11:35 UTC), and freshness
status passed. Restoring the entire current library into new ext4 staging matched
all 344 files and the exact file set, including new pilot publications and sync
guidance. This qualifies local recovery, not an external replica or future GPG
agent availability. No credentials, scheduler, retention or recovery settings changed.

Fitness Bridge has no maintained production recorder/output setting; its docs now
state the local-library boundary without inventing one. Only the two new doc hunks
were committed as `084be5b` and pushed; unrelated pre-existing changes were left
unstaged. Life Fitness's direct host and supported `bikectl` launcher defaults
are published as `c47b60e`, preserving explicit outputs and legacy/binary lifecycle
semantics. Current-source validation passed 334 tests (2 hardware tests ignored),
13 focused launcher tests, 5 focused host tests and strict Clippy. A test-only
platform-path improvement was followed by another 5-test host/Clippy pass.
Scoped formatting passed; package formatting still reports unchanged
`src/reporting_client.rs` debt. A fresh context-isolated same-family review found
no blocking defects; native Windows runtime was not exercised.

The newer upstream evidence-only commit was incorporated before push, with all
validated runtime/test source bytes preserved. Canonical Life Fitness checkout
fast-forward is blocked by pre-existing untracked files now tracked upstream;
its original files were preserved and the clean integration worktree retained.
No installed launcher or ongoing session was changed. Checkout reconciliation,
between-session client deployment and source/collector selection remain open.

All 250 exact migration originals still match inventory hashes. Five supplied
pilot evidence files retain old path references, so no originals or pilot sources
were retired. The private follow-through evidence is
`~/.local/share/fitness-data-followthrough-20261107-DFrbpxSb/`, including scoped
recovery, original-dependency audit and current-backup logs. No synchronization
job, account access, live acquisition or device operation was performed.

## Current focus

Use the personal finder for race selection while comparing recency/current-form
ideas against the preserved **`v0.3.0` baseline** (`05edb8a`). The owner accepts the
baseline as useful; that does not cancel improvement research or establish general
predictive accuracy. The active model has not been replaced by the experiment.

The workflow is **Racing Score → subgroup/course geometry → fixed time or personal
duration → time-window filter**. Unsupported durations stay unknown and are omitted;
unfamiliar courses can use cross-course personal fitting. Score is not personal pace.
[Algorithms](docs/reference/ALGORITHMS.md) owns the exact rules.

## Duration precision — 2026-11-05

The finder retains fractional minutes for inclusive duration-window filtering,
including fixed durations supplied in seconds. Only display rounds to whole
minutes. B0's fitting formula remains active and unchanged; R1/H1/RW1 are not
promoted. The saved-prediction evaluator reports raw signed/absolute errors and
uses measured seconds for metrics and window decisions. See the
[evaluation procedure](docs/howto/TESTING_GUIDE.md#evaluate-saved-personal-predictions).

The nine-result 90-day retrospective replay has unrounded MAE **128.633 seconds
for R1** and **130.538 seconds for RW1**: close on this small inspected sample,
not a clear RW1 advantage or independent validation. Frozen pre-race predictions
remain separate from retrospective geometry replays. Source whole-kilometre
distances remain an input limitation.

Validation on the working tree based on `0813baf`: **198 default-feature tests
passed, 7 ignored**, five evaluator tests passed, and the offline release CLI
build and help smoke check passed. Populated legacy MAE was **2.093 → 2.093 minutes**
on the same six-observation snapshot; this is not personal-model qualification.
Formatting still fails repository-wide; Clippy still reports 152 library errors.
No clean gate/CI claim. Private evidence: `20261105-duration-precision-URErMC7k`
under `~/.local/share/zwift-race-finder/investigations/`. Normal database unchanged.

## Baseline and experiment

R1 uses a 90-day half-life and requires three comparable same-route results within
180 days before preferring that branch. It is private and experimental, not active.
Fresh source reconciliation resolves the two later results and EVR's reported
route/distance. Separate private-copy replays give **2:42 → 2:19** mean displayed
error over the four recent races, but **2:13 → 2:12** over seven supported races
from 25 September through 4 November 2026. Selection is mixed; recent support is
sparse and one race-labelled activity lacks a supported result. The original
historical replay remains mixed and fails its predeclared improvement screen.

**There is not enough recent evidence for a strong signal either way.** Keep B0
active because it is useful, not because its weighting is proven optimal. Keep R1
and current-form research open; neither the small gain nor the mixed selection
results settle the question. The owner's impression of similar performances over
two years is plausible but has not been systematically tested or established as a
fitness measurement. See [Recency evidence and the open question](docs/explanation/RECENCY_EVIDENCE.md)
for the working interpretation, limitations and what could change the decision.

The [original baseline/experiment record](docs/project-history/RECENCY_BASELINE_2026-11-03.md)
and frozen evidence remain unchanged. Fresh results were applied only to private
analysis copies; normal history is unchanged and Stage 3/4 were not duplicated.
Any production import still requires the [backup/preview procedure](docs/howto/ZWIFTPOWER_EXPORT_STEPS.md).
Do not substitute activity durations or invent missing result identity.

## Validation and limitations

The versioned baseline recorded **195 tests passed, 7 ignored**, a passing offline
release build, 14 experiment tests and independent arithmetic matching the replay.
The retained legacy regression was **2.093 → 2.093 minutes MAE**, six known-score
observations; that is not personal-estimator qualification. These are baseline
results, not a claim that every later working tree has been tested.

Repository-wide formatting and Clippy remain known blockers; the last recorded
Clippy run for this cleanup reported 152 library errors (baseline 160). OCR model/snapshot/parse limitations remain
separate from default-feature success. No clean release/deployment claim is made.

## Current cleanup

The owner requested removal of code comments and historical documentation baggage.
Comments are removed with syntax-aware checks; command help is explicit metadata,
and interpreter/tool directives remain. Current documentation centres on the finder,
safe data ingestion and baseline-versus-experiment work. Superseded material remains
retrievable from `v0.3.0`; `archive/`, private data, fixtures and active model rules
are not removed. The cleanup is separate from the unchanged baseline tag.

Cleanup checks passed: **195 tests, 7 ignored**, focused personal/import contracts,
all-feature binary builds, six unchanged CLI help outputs, seven unchanged recent
prediction outputs, JavaScript exporter checks and 90 native PowerShell assertions.
Syntax/token/AST comparisons cover 113 code files. Formatting still fails; Clippy
still fails with the count above. No assertions, snapshots or gates were weakened.

Cleanup validation/provenance is retained privately as `20261103-cruft-cleanup` under
`~/.local/share/zwift-race-finder/investigations/`. It is separate from baseline
research evidence `20261103-recency-v030`. No normal-state import is included;
publication requires the owner's explicit instruction.

[Tasks](TASKS.md) owns remaining work, not this status record.
