# Versioned personal baseline and recency experiment — 2026-11-03

## Decision and scope

**Baseline B0 is versioned as `v0.3.0`** (Cargo `0.3.0`). It is the current
personal-only finder, accepted by the owner as useful for practical race choice
and approximate duration filtering. That acceptance does not end recency research
or imply universal accuracy, ±1-minute precision, or clean release gates.
No local version tags existed before this checkpoint. The minor version separates
the personal-only estimator from the earlier `0.2.0` category-based release.
The annotated tag is local only; no publication is authorized.

The active estimator source is unchanged from `ea787a9`. Version metadata and
documentation changed; no model, threshold, normal database or importer changed.
The tag identifies the baseline code/documentation; the exact evaluation data and
private scripts are retained outside Git, not embedded in the tag.

**R1, the one predeclared stronger-recency candidate, remains experimental.**
It improves the main recent-example comparison, while the broader replay is mixed
and does not pass its predeclared screening rule. Keep B0 active and retain R1 for
further comparison; do not cancel recency/current-form research because B0 is useful,
or retune R1 until these inspected examples look perfect. A separate current-form
model remains an open research option, not an adopted implementation or automatic
response to a mixed candidate. [Status](../../STATUS.md) and
[Tasks](../../TASKS.md) own subsequent work.

## Why this is not a repeat of the last-five experiment

The [earlier last-five experiment](PRACTICAL_ESTIMATOR_2026-11-02.md) multiplied a
neutral category-speed/terrain reference by a recent measured/reference ratio.
Its mixed historical results did not establish improvement and it worsened the
small known-score comparison. It did not compare stronger weighting in today's
personal same-route/distance-ascent estimator. Reusing its ratio unquestioningly
would confound the baseline and repeat an already-inspected exercise.

## Predeclared protocol

| Component | B0: active baseline | R1: private experimental copy |
|---|---|---|
| Recency half-life, both branches | 365 days | 90 days |
| Same-route branch | At least 3 comparable prior results, any age | At least 3 comparable prior results within 180 days; only those recent route results fit that branch |
| Otherwise | Global distance/ascent fit | Same global fit with stronger weighting |
| Global minimum support | 12 usable earlier races | Unchanged |
| Shape grid, pace bounds, chronology, source/rider rules | Existing production rules | Unchanged |

R1 has no hard global age cutoff. Sparse recent support can therefore still yield
an older-history-supported global estimate; it must not be described as an
independently established current-form measurement. Unfamiliar courses remain
eligible for global fitting. No category-speed substitute or physiological model
was introduced. See [Algorithms](../reference/ALGORITHMS.md) for baseline rules.

The protocol was saved and SHA-256-hashed before replay. It specified one candidate,
no parameter sweep, identical frozen data, strict earlier-UTC-date training, and
rounded-duration error, prediction availability and inclusive window selection.
Latest-20% means the last 20% of chronological targets including the entire boundary
day. Latest 90 days is relative to the latest stored eligible target, not the run date.

The screening rule required at least 10% paired latest-cohort MAE improvement,
no worse combined window inclusion/exclusion errors or lower latest coverage,
and no more than 5% worsening of all-paired MAE. This is an experimental screen,
not a newly adopted product accuracy requirement or automatic promotion rule.

## Main practical comparison: the four supplied races

After the initial historical replay, the owner clarified that the four recent
races are the main practical comparison and reaffirmed the baseline-plus-experiment
goal. No R1 settings changed. This is a documented scope clarification, not a
retroactive change to the earlier protocol or an independent holdout.

With the same reconstructed geometry inputs as the baseline replay, R1 reduces
mean displayed absolute error from **2:31 to 1:51** over the three non-EVR cases.
It improves two predictions and worsens one; the two 40–50-minute matches remain
matches. Using client lap geometry instead of the rounded source distance changes
one prediction by a minute in both models, without changing that direction.

The fourth case, EVR, remains conditional on unverified course geometry. The
candidate changes an underprediction to an overprediction under the tested course
assumption and would include it in the 75–105-minute window where B0 omits it.
Do not count that as a verified selection gain or include EVR in the aggregate
until its course is confirmed. Exact rider-level outputs remain private.

All 14 recent-case model/geometry predictions were independently reproduced.
Training remains strictly earlier-date supported history; later owner-reported
finishes are comparison outcomes, not fabricated training observations. This is
an experiment against the same frozen baseline, not an import or active-model change.

## Supporting chronological historical replay

The frozen history supplies 182 eligible ordinary-race targets; both models support
170. The other 12 are unavailable rather than category-filled. MAE below uses the
same supported targets for both models and whole-minute predictions against precise
measured finish minutes.

| Cohort | Targets | Supported B0 / R1 | B0 MAE, minutes | R1 MAE, minutes |
|---|---:|---:|---:|---:|
| All eligible history | 182 | 170 / 170 | 6.914 | 6.592 |
| Latest 20%, complete boundary day | 37 | 37 / 37 | 4.940 | 4.876 |
| Latest 90 days | 6 | 6 / 6 | 2.159 | 2.222 |

The latest-20% paired gain is about 1.3%, below the predeclared 10% screen.
Across the four predeclared windows (20–40, 40–50, 50–70, 75–105 minutes), summed
false inclusions plus exclusions change from 19 to 18 in the latest cohort but
from 3 to 6 in the latest-90-day cohort. These are sums across separate, sometimes
overlapping windows, not counts of distinct incorrectly selected races.

For the 40–50-minute window specifically (all eligible cohort targets, including
unknown durations as not selected):

| Cohort/model | Suitable shown | Unsuitable shown | Suitable missed |
|---|---:|---:|---:|
| Latest 37, B0 | 6 | 2 | 4 |
| Latest 37, R1 | 6 | 2 | 4 |
| Latest 90 days, B0 | 2 | 0 | 1 |
| Latest 90 days, R1 | 1 | 1 | 2 |

Recent support is sparse: the median target in the latest-90-day cohort has only
three eligible earlier races within 180 days. A separate form multiplier might
add complexity without sufficient recent evidence. A separate layer would need an explicit mechanism and evaluation protocol;
these mixed results alone do not select or rule out that future experiment.

## Evidence and limitations

- B0 executes the actual shipped Rust database adapter; R1 executes a private
  copied module with the declared weighting/route-age changes. The production
  source is not patched or made configurable for this experiment.
- All 364 historical Rust target/model cases agree with an independent arithmetic reference,
  including availability, raw predictions, rounded minutes, method and support.
- Fourteen private Rust tests pass: nine copied fitting/adapter tests (one stale
  same-route expectation updated to R1's declared policy), plus five explicit
  recency, stale-route, sparse-support and same-day/future-exclusion cases.
- Rebuilt baseline default suite: **195 passed, 7 ignored**. Focused personal and
  duration-consistency checks pass. Offline locked release CLI reports `0.3.0`.
- Retained legacy regression on the same populated copy: **2.093 → 2.093 minutes
  MAE**, six known-score observations. This does not qualify R1's personal accuracy.
- Normal database checksum is unchanged. No private observations were imported.
- Historical client metadata is retrospective; reported whole-kilometre event
  distances are not original live-event payloads. Outcomes and earlier holdouts
  have already been inspected; these results are exploratory, not independent
  prospective validation.
- Owner-reported later finishes were reconciled separately. Two are already in
  supported history; missing later observations were not manufactured or added
  to this replay. The conditional EVR course example is not a qualified evaluation
  target. Individual activities and measurements are not published here.
- Repository-wide formatting and Clippy remain previously recorded blockers;
  those unchanged global gates were not rerun or weakened. This tag is a baseline
  checkpoint, not a clean release/deployment qualification.

Private evidence ID: **`20261103-recency-v030`**, at
`~/.local/share/zwift-race-finder/investigations/20261103-recency-v030/`.
It retains the predeclared protocol/hash, frozen database, original estimator,
candidate patch, locked replay, tests, raw/annotated predictions, independent
reference, summaries, validation logs and checksums. The version tag and these
artifacts together identify the evaluated baseline. The related private IDs
`20261103-reported-races` and `20261103-four-race-predictions` retain the earlier
reconciliation and recent-example geometry qualifications.
