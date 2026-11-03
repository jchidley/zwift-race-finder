# Project Status

`STATUS.md` is the canonical current-work record. Durable facts belong in
`docs/reference/`; unresolved work belongs in `TASKS.md`.

## Current focus

**Versioned usable baseline plus recency research — 2026-11-03.** The owner
accepts the current estimator as useful for race choice/filtering and explicitly
wants continued comparison against a preserved baseline. Baseline **`v0.3.0`**
(Cargo `0.3.0`) is a local annotated checkpoint, not a published clean release.
The active model is unchanged from `ea787a9`.

One predeclared stronger-recency candidate has now been tested, not promoted:
90-day half-life with same-route support restricted to the preceding 180 days.
The four supplied races are the main practical comparison: over the three
non-EVR cases, displayed MAE improves **2:31 → 1:51**; EVR remains conditional on
unverified course geometry. Broader replay is supporting evidence: MAE changes
6.914 → 6.592 minutes over all 170 supported targets, 4.940 → 4.876 on the latest
37, and 2.159 → 2.222 on the latest six stored targets. That historical screen
fails; these are different cohorts, not a contradiction of the recent examples.
See [baseline and experiment report](docs/project-history/RECENCY_BASELINE_2026-11-03.md)
for protocol, selection counts, validation and private evidence. No retuning or
current-form layer has been adopted. [Tasks](TASKS.md#immediate-plan--recent-performance-relevance)
owns remaining work.

The implemented finder uses **score → subgroup/course → fixed time or personal
same-route/cross-course estimate → time-window filter**. Unavailable durations
are excluded and disclosed; no category speed is substituted. Unfamiliar courses
can use supported cross-course fitting. The 365-day half-life, same-route priority
and support thresholds are unchanged. They are implementation choices, not
permanently adopted requirements. Whole-minute output does not promise ±1-minute
accuracy. [Algorithms](docs/reference/ALGORITHMS.md) owns implemented rules.

### Evidence and ingestion are different prerequisites

- The four owner-reported race finishes have been reconciled with history/export
  and decoded recordings. Two already exist with matching precise source times;
  one recalled date differs from all corroborating sources. Do not duplicate
  those observations or overwrite precise seconds with whole-second reports.
- Two later results remain absent from supported history. Their finish times are
  owner-reported; official result provenance and the EVR route remain unresolved.
  Recordings include post-race riding and are not importer-compatible race rows.
- A current-program replay of the supplied examples is retained privately under
  `20261103-four-race-predictions`. Geometry is reconstructed; the EVR case is
  conditional and not a confirmed event prediction. It is not a fresh holdout.
- **Supported ingestion still needs a fresh source-qualified export.** Follow the
  [measured-history procedure](docs/howto/ZWIFTPOWER_EXPORT_STEPS.md) for a backed-up,
  reconciled import. Read-only analysis/experiments are not blocked by that export.

### Model decision still open

Keep `v0.3.0` active. The private R1 experiment is complete and not promoted; it
is not a production configuration option. Retain R1 as an experimental comparison and diagnose specific
recency/selection failures or collect new source-qualified outcomes before selecting
another candidate. A separate current-form model remains open, with an explicit mechanism/protocol
needed before another experiment; it is neither cancelled nor an automatic layer.
Sparse recent support and already-inspected holdouts limit what this replay proves.
Preserve chronology, rider/source boundaries and unfamiliar-course prediction.

Coach phase-aware evidence rules inform this investigation; the sibling coach is
not a maintained capacity model to adopt or a finder prerequisite. Coaching/pacing
remains in that separate project; sensor/recorder work is not required here.

## Verified implementation checkpoint

**Personal-only duration policy implemented.** Completed P1/P2 acceptance remains
valid within its dated scope; removal of category fallback followed that run.
Fitting parameters did not change.

- Versioned baseline isolated default suite rerun: **195 passed, 7 ignored**. Focused checks
  cover fixed time, personal same-route/cross-course estimates, unknown exclusions,
  omission counts, subgroup eligibility and filter/display agreement.
- Retained legacy formula regression: **2.093 → 2.093 minutes MAE**, six
  score-known observations on the same populated copy. Not personal accuracy proof.
- Offline locked release build and scoped Rust formatting passed. Live personal
  matches and empty-state unknown omissions were checked separately; they are
  functional observations, not an immutable event replay or accuracy qualification.
- **Release gates remain blocked:** repository-wide formatting fails; final Clippy
  reported 160 library errors versus 165 before the bounded correction. No gate
  suppression or unrelated global formatting was applied. Not release-ready.
- Improved personal prediction and time-window selection remain unproven.
  Existing inspected holdouts are not fresh independent validation data.

### P2 acceptance evidence

The earlier unchanged-model acceptance run recorded **192 passed, 7 ignored**,
a passing release build and compact/verbose agreement on live matches. It still
included category fallback, superseded by the personal-only policy above.
Controlled recency probes showed 12 year-old 60-minute races plus three recent
40-minute races predicting 60 minutes; three old plus three recent predicted 40.
This establishes weighting behaviour, not adequate adaptation to current form.
The copied history had sparse recent support; base recency share is not the same
as target-specific fit influence. Data reconciliation precedes policy selection.

### Durable private evidence

Evidence ID: **`20261103-docs-reconciliation`**, at
`~/.local/share/zwift-race-finder/investigations/20261103-docs-reconciliation/`.
`PROVENANCE.md` describes source checkpoints, limits and original temporary paths;
`SHA256SUMS` covers retained logs, probes, manifests and private database copies.
These were copied during documentation repair, **not newly executed tests**.
Source/test/Cargo files present in the commit-organization manifest were
hash-checked against the retained checkpoint before this repair. Earlier P2
source reconstruction is limited; do not claim exact replay of every prior run.
Original FIT recordings/decoded samples are not in this archive; it does not
corroborate an official finish. No private data or raw logs were added to Git.

## Working tree and history

Baseline: **`v0.3.0`**, on `main`, including version metadata and the focused
documentation reconciliation. The estimator source is unchanged from `ea787a9`.
Four earlier local commits preserve the completed work: `bc1c6df` personal finder, `7e6cc05` unsafe importer retirement,
`b264747` OCR cleanup, and `ea787a9` documentation alignment. They are not pushed;
the local `origin/main` ref remains `3e13a78` (no remote freshness check).
The owner authorized the local baseline commit/tag. No estimator source, normal
database import, coach change or publication is included. Research artifacts and
private data remain outside Git; the R1 candidate is not shipped in this tag.

Superseded status details, test counts and system-setup observations remain
identifiable with `git show ea787a9:STATUS.md`; they are not current next actions.
Dated implementation/investigation records remain unchanged:

- [History foundation](docs/project-history/HISTORY_MODEL_2026-10-30.md)
- [Client-only metadata](docs/project-history/CLIENT_ONLY_ROUTES_2026-10-30.md)
- [Cleanup audit](docs/project-history/OBSOLETE_CODE_CLEANUP_2026-10-30.md)
- [Practical estimator and last-five experiment](docs/project-history/PRACTICAL_ESTIMATOR_2026-11-02.md)
- [Geometry diagnosis and disabled profile experiment](docs/project-history/PERSONAL_ESTIMATOR_DIAGNOSIS_2026-11-02.md)
- [Live personal integration](docs/project-history/LIVE_PERSONAL_ESTIMATOR_2026-11-02.md)

Their earlier category-fallback and unpublished-work descriptions are dated facts,
not current policy. E1 populated-data qualification, E2 config-test isolation and
M1 immutable once-per-event estimates remain separate proposals in `TASKS.md`.
