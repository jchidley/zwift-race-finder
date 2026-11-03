# Practical race-finder estimates — 2026-11-02

## Decision

Keep the category-speed/terrain model. Do not introduce a simulator, equipment
configuration, drafting bonus, or unvalidated personal multiplier. Implement
nearest-minute rounding and correct the historical accuracy regression's inputs.
No credentials, rider-level records, exports or client-cache data are added to Git.

## Evaluation

A single candidate used the median measured/neutral-duration ratio from the last
five eligible observations on strictly earlier UTC dates. The neutral reference
uses the existing terrain calculation and 33 km/h; it is NOT an inferred historical
category. No same-day or future outcomes train a prediction. Five warmup records
leave 189 walk-forward predictions out of 194 observations; all have measured
seconds and recorded event distances. Missing historical scores remain NULL.

A latest-20%-by-date holdout contains 39 observations. Rolling calibration may use
earlier holdout outcomes for later predictions, never future or same-day outcomes.
No outcome-based exclusions were made. Distances receive no duplicate lead-in.

| Rounded comparison | Reference MAE | Personal MAE | Interpretation |
|---|---:|---:|---|
| All-history walk-forward, n=189 | 7.05 min | 7.35 min | Neutral, NOT production baseline |
| Dated holdout, n=39 | 4.94 min | 4.50 min | Small neutral-reference gain; not stable in later half |
| Known-score production comparison, n=6 | 2.09 min | 2.59 min | Personal candidate worse |

The holdout's day-cluster bootstrap interval for the rounded MAE gain is
approximately -0.74 to +1.64 minutes, including no improvement. On known-score rows,
the personal candidate lost four of six comparisons. A predeclared frozen-factor
sensitivity was not promoted based on these same holdout labels. No personal
adjustment was enabled.

Current selected client metadata is retrospective, not proven available/unchanged
at historical race dates. Original live API payloads are unavailable: this checks
the distance/category/terrain formula, not every historical resolver decision.
Two lap-distance discrepancies were retained without silently correcting them.
The existing model has historical calibration overlap; this is not a claim of
newly independent validation of all its fixed parameters.

## Implemented changes

- `src/duration_estimation.rs`, `src/estimation.rs`: round final calculated minutes,
  positive half-minute ties upward. Speeds, difficulty, lead-ins and fixed-duration
  handling remain unchanged. The shared resolver still drives filter and display.
- Boundary regressions cover below/at/above half-minute and very short estimates.
  An integration check verifies 45.75 -> 46 in compact/verbose output and filtering.
- `src/regression_test.rs`: prefer stored total event distance and measured seconds;
  retain legacy fallbacks, report MAE in minutes and precise-input coverage.
- Algorithm/reference and current-work documentation describe the limits honestly.

### Before/after measurements

The initially unchanged legacy regression used route/name geometry and whole-minute
observations: MAE 6.167 -> 5.833 min, MAPE 13.3% -> 12.4%, n=6. This exposed a test
input problem: stored event distances/precise times were being ignored.

With recorded event geometry and precise times, the comparable production-formula
values are **1.93 -> 2.09 minutes MAE**, **4.32% -> 4.67% MAPE**, truncation versus
rounding. Both have six of six predictions within five minutes and one within one
minute. The corrected native Rust focused regression reports **2.093 min / 4.7%**,
with six precise times and six recorded distances. Do not attribute better metrics
from corrected test inputs to a production-model improvement. Rounding expresses
the requested resolution; it does not establish improved forecast accuracy.

## Validation and boundaries

- Fresh HOME/XDG, cleared environment, explicit Cargo/Rustup homes, `INSTA_UPDATE=no`,
  one thread: default suite 173 passed, 7 ignored. Some CLI tests call the public API.
- Release main CLI build passed offline.
- A private schema-only synthetic fixture deliberately supplied conflicting event
  name distance/legacy minutes; the native regression used recorded distance and
  precise seconds, yielding the expected 0.008-minute MAE.
- Seven synthetic evaluation tests passed. Parent independently recomputed all 194
  training selections/personal predictions and six paired MAEs from private rows.
  The evaluator also checked 200 historical and 36 synthetic cases against frozen
  Rust formula code. Its snapshot noticed the parent's concurrent rounding change;
  both floor and rounded results are explicitly retained.
- Scoped read-only rounding review found no issues. Review itself did not run tests.
- An initial broader library run incorrectly used the personal-history copy and hit
  a route-only consistency diagnostic on multi-lap history. Accuracy runs use that
  isolated copy; the complete suite was rerun with fresh state. No tests were masked.
- Normal database checksum remained unchanged. Existing unrelated local cleanup was
  preserved. No archive edits, database imports, commits or publication.

Private scripts, row-level results, protocol and reports are under
`~/.local/share/zwift-race-finder/investigations/20261102-practical-estimator/`;
parent checks and before/after logs are under the sibling
`20261102-estimator-validation/`. These artifacts are not in Git.
