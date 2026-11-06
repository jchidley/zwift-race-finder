# Project Status

## Consolidation — 2026-11-06

The local integration preserves Finder through `a55d488` and Coach through
`b6bdccc` as parents of merge `aead11f`; neither history was squashed.
The combined workflow is find → plan → preserve predictions → review.
The working command, Rust package, estimator and application-state paths are
unchanged. Planning/review remain agent-assisted, not a delivered application.
Coach's [prior status and evidence limits](docs/project-history/COACH_STATUS_PRE_CONSOLIDATION.md)
remain historical; unfinished work is reconciled into the root task queue.
The shared-library consumer pilots are complete for their tested workflows;
new Finder/Coach publication proposals remain separate and unexecuted here.
No OneDrive or frozen evidence paths are moved. Cloud sync/offline retention
has not been rechecked. See the [migration record](docs/project-history/CONSOLIDATION.md).

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

### Publication hold

The owner approved public consolidation including Coach history. A subsequent
reachable-history audit covered all six Coach commits through `b6bdccc`, 50
unique text blobs (427,778 bytes). No embedded credentials, private keys, raw
activity files or bulk proprietary exports were found within that bounded audit.
A parent spot-check confirmed real session/activity filenames and timestamps in
the shared-library procedure and private-network/device inventory in historical
reports. These occur in retained ancestry: a tip-only edit cannot remove them.
Public push/rename and Coach retirement are paused pending explicit disclosure
approval or a separately authorized sanitization plan. No history rewrite is
performed. Audit limits include unavailable specialist secret scanners and no
legal clearance of small client-derived factual excerpts. Finder's existing
history was not part of this Coach-specific audit.

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
