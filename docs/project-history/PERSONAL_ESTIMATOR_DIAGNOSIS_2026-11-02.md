# Personal-estimator diagnosis and event-resolution corrections — 2026-11-02

## Delivered and not delivered

Implemented corrections to the production event-resolution path: choose the
declared Racing Score subgroup, use its route, ignore zero laps, prefer its
explicit total distance, and share geometry between fixed/calculated output.
These are actual finder changes, not just offline metrics.

A failure-led personal course-model candidate was built and evaluated. It is
**not enabled**: it recovers short-window misses at the cost of other selections.
This work does **not** establish a generally better personal duration estimator.
Category speeds/difficulty and the production duration formula remain unchanged.

## Diagnosis

The previous profile prototype missed four suitable 40–50-minute performances:
three overestimates and one underestimate. Distance/ascent missed the same four,
so detailed profile geometry cannot be their sole cause.

Before the three overestimates, the preceding 90 days supplied only 9–22% of the
pace-fitting weight; performances lasting at least 60 minutes supplied 61–68%.
The time-MAE objective gives larger effective courses more pace-fitting weight.
This is mathematically intentional, but may be inappropriate for predicting
shorter events from that mixture. No individual observation dominated the fit.
The opposite-direction miss followed 168 days without recorded observations;
fitness loss is a hypothesis, not demonstrated by the available results.

Earlier same-route comparable-distance history exists for only one of the four
misses; both comparable results were over 51 minutes. A simple exact-route lookup
therefore does not obviously solve these misses.

### Corrected evidence qualifications

The retained source payload's `f_t` field was overlooked by earlier experiments.
It distinguishes 182 ordinary race-tagged rows, seven time-trial/race-tagged rows,
four ride-tagged rows and one ambiguous ride/race-tagged row. These are source
tags, not independently recovered historical API event formats. No database
classification or records were rewritten.

All 194 source distances are whole kilometres. They are reported event distances,
not proven exact course traversals; the export's rounding mechanism is unverified.
Precise measured finish seconds remain available. Generic lead-in identity,
unverified world scaling and historical finish cuts remain explicit limitations.
More profile bins cannot replace those missing facts.

## One bounded model change

Frozen before the new evaluation: retain the existing two-parameter profile
formula, parameter grids, one-year recency weighting, 12-observation minimum,
rounding and four time windows. Fit ordinary race-tagged targets using earlier
ordinary races within a 0.8–1.25 distance ratio. Fall back to same-format global
fit, then the original prototype if support is insufficient. Non-race predictions
are retained and reported separately. A format-only ablation separates the effect
of source tags from length locality. No parameter sweep or follow-on candidate.

The independent diagnostic agent suggested a 90-day half-life instead. That
alternative was **not evaluated**: length locality was already declared, and
changing both would obscure attribution and expand the inspected-holdout search.

### Results

Broader approximate-profile holdout, same 38 observations, 11 suitable for
40–50 minutes:

| Estimator | MAE min | Suitable shown | Unsuitable shown | Suitable missed |
|---|---:|---:|---:|---:|
| Original personal profile | 4.506 | 7 | 1 | 4 |
| Format-only fit | 4.453 | 7 | 1 | 4 |
| Comparable-length personal profile | 4.346 | 10 | 3 | 1 |
| Fixed neutral reference, NOT production | 5.053 | 10 | 2 | 1 |

This diagnoses a useful response, not a deployment win. The local model also
missed two formerly selected suitable 80–100-minute observations. On the stricter
eight-observation geometry-compatible holdout, MAE worsened 6.289 → 7.039 minutes
and an extra unsuitable event was selected in the 80–100-minute window.

Among the six score-known observations, local profile MAE improves 1.492 → 1.150
minutes versus production's 2.093, but the local model adds a false 40–50-minute
inclusion. It does not improve production filtering. The 36 ordinary-race-tagged
holdout rows and early/late segments are separately retained in private results.

All these historical outcomes have already been inspected. This is exploratory
chronological analysis, not a new blind validation. No default personal estimator
or speculative fallback was added to the finder.

## Production event-resolution fixes

Files: `src/event_analysis.rs`, `src/event_filtering.rs`, `src/models.rs`,
`src/event_display.rs`, `tests/duration_consistency_tests.rs`; `src/main.rs`
gains required fields in existing test literals and passes the score to selected-route logging.

- Match numeric `rangeAccessLabel` for score events, not a category letter
  anywhere in a title. Touching inclusive endpoints retain API order; a score
  outside all declared ranges has no estimate.
- Traditional groups prefer exact `subgroupLabel`, otherwise category tokens;
  exact D precedes the retained D-to-E fallback.
- Prefer the selected subgroup's route ID over the event route ID, including
  unsigned IDs above `i32::MAX`.
- Resolve geometry once: positive finite subgroup total distance, then positive
  subgroup laps plus known event lead-in, then positive finite event distance,
  then existing route/description/name fallbacks.
- Zero laps are unavailable, not a request to estimate just the lead-in.
- Event positive fixed duration retains precedence. Subgroup positive fixed
  minutes or API `durationInSeconds` now precede calculated duration.
- Fixed and calculated paths share distance/elevation; compact/verbose/filtering
  continue to consume one estimate. Subgroup highlighting uses the same selection.

### Captured live-feed evidence

Read-only public capture: 200 events, 178 cycling, 54 cycling races. At score195,
subgroup selection changes in 25 races compared with the former title/category
matcher. Five race records reach the old zero-lap branch. Reproducing that branch
and running the corrected resolver shows, for example:

- Reported 47.6 km: old lead-in-only distance 0.194 km / 0-minute calculation;
  corrected total distance 47.6 km / 90-minute estimate.
- Reported 15.0 km: old lead-in-only distance 1.852 km / 3-minute calculation;
  corrected total distance 15.0 km / 27-minute estimate.

These are input-resolution corrections, **not verified actual finish times**.
All 49 estimable captured races at each of scores195 and210 agree between
filtering, compact and verbose output; five selected positive subgroup distances
at each score are preserved exactly. Six races at each score have no matched
subgroup; one has usable event-level fallback under the traditional path. No
selected race subgroup route differs in this capture; four other cycling events
have differing selected routes, and their route metadata was verified against the
selected IDs. Eight traditional races/time trials with blank range labels remain
estimable. Differing-route duration/elevation/URL/logging also has synthetic coverage.

## Validation and private artifacts

- Fresh HOME/XDG, explicit Cargo/Rustup homes, offline Cargo, one test thread,
  `INSTA_UPDATE=no`: **182 passed, 0 failed, 7 ignored**. Some CLI tests still
  call the public API; the suite is not hermetic.
- Focused native regression on an isolated database copy:
  **2.093 → 2.093 minutes MAE**, six known-score observations, precise seconds and
  reported event distances. This unchanged formula regression does not replay
  historical API subgroup selection.
- Eight synthetic local-model tests passed. Parent independently recomputed
  366 new fits, checked chronology/support and all three 40–50 holdout tables.
- Private Rust replay compiles against the current library and checks captured
  API input, selected total distance and filter/compact/verbose agreement.
  Its first standalone offline resolution selected an uncached dependency;
  reusing the project lockfile fixed it without downloads.
- `cargo build --offline --release --bin zwift-race-finder` passed. The rebuilt
  release CLI also ran successfully against the public API for score195 and
  45±5 minutes using isolated copied state; normal state was not used.
- `git diff --check` passed. No snapshot regeneration.
- Normal database SHA-256 remains
  `c79e15e945c0d8a4d7cfe0ff915150bef6f8cc98157a2a492011fea367d608cf`.
  No imports, normal-state writes, archive edits, commits or publication.
- Fresh-context same-family review reproduced the local-model results and identified
  a blank-range eligibility regression plus stale verbose route metadata. Both were
  fixed and the reviewer confirmed resolution. Follow-up findings (a shared mutable
  test fixture and subgroup sub-minute display) were fixed with a unique route ID
  and one shared duration helper; parent reran the full suite and rebuilt release.
  This is not cross-family verification. Final small follow-ups were parent-verified,
  not reviewed in a third child pass.

Private source snapshots, protocol, local_model.py, test_local.py, local results
and predictions, verify_local.py, Rust replay helper and test/build logs are under
`~/.local/share/zwift-race-finder/investigations/20261102-personal-diagnosis/`.
No rider records, client profiles or live API payloads were added to Git.

## Remaining concrete limits

A deployable personal profile estimator still needs defensible event traversal
composition: subgroup route, actual lead-in identity, lap/finish distance and
physical scaling. The current application stores route totals, not an ordered
event-specific physical profile. The private approximate prototype is not silently
promoted into that role. The statistical problem also remains: recent/short-course
adaptation helps some selections but harms others. That is unresolved work, not
a request for the owner to supply more recordings by default.
