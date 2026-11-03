# Live history-based duration estimates — 2026-11-02

## Scope and authority

The owner explicitly requested the live flow: Racing Score selects the event
subgroup; its actual course/distance is resolved; earlier personal performances
estimate duration; that estimate drives the requested time-window filter.
This is implementation of that request, not a claim that earlier exploratory
experiments proved improved accuracy. Prior negative and mixed results remain
recorded in PERSONAL_ESTIMATOR_DIAGNOSIS_2026-11-02.md.

## Declared model

- Fixed-duration events retain declared time.
- Personal fitting uses earlier source-qualified ordinary races with measured
  finish seconds and positive reported event distances. Missing historical scores
  do not exclude these performances or get inferred.
- Recency weighting has a one-year half-life. Only dates strictly earlier than
  the earlier of today and the target event date may train a prediction.
- With at least three same-canonical-route results within a 0.8–1.25 distance
  ratio, use their weighted median measured minutes/km times target distance.
  This scaling is approximate; it does not reconstruct partial-route climbing.
- Otherwise, with at least twelve usable races with course climbing, fit
  `minutes = pace * (distance_km + h * ascent_km)`. Select `h` from
  `[0,2,4,8,16,32,64]` by recency-weighted time MAE; weighted-median pace is bounded
  to 1–4 min/km. Racing Score is not an input to either personal calculation.
- Unknown/insufficient history, unavailable geometry, and unsupported event
  formats retain an explicit category-model fallback. TT and ride performance
  are not silently treated as ordinary race training.
- Descriptive training residuals are not calibrated confidence intervals or
  guarantees that a new race will finish within those limits.

## Evidence boundaries

Reported historical distances are whole kilometres; actual finish seconds are
precise. Current route metadata is retrospective. The live model uses distance
and climbing totals, not an exact section-by-section event traversal. Missing
lead-in identity and physical scaling do not get fabricated. The previously
rejected comparable-length profile fit is not this production model.

The chronological backtest must call the shipped Rust event estimator, not a
separate approximation. Its historical outcomes have already been inspected;
results are exploratory and must not be presented as independent validation.
The existing accuracy regression remains a category-formula baseline; its
before/after MAE alone cannot establish performance of the new history path.

## Implementation and verification

Implemented and active by default for supported ordinary races:
- `src/personal_estimation.rs`: pure estimator and source-qualified DB adapter.
- `src/event_filtering.rs`: personal estimate after subgroup/geometry resolution,
  before category fallback; fixed-duration path skips history.
- `src/event_display.rs`: compact Basis column and verbose method/sample/error
  details. Personal estimates are labelled experimental; fallback reason is
  visible in compact output too.
- `src/lib.rs`, `tests/personal_estimation_tests.rs`: library surface and
  isolated end-to-end synthetic history fixture.

The DB adapter accepts supported ZwiftPower provenance (decimal rider/event IDs,
bounded result identity, 64-hex source hash), checks ordinary-race source tokens,
and rejects pooling multiple eligible riders. Explicit stored event type is
authoritative. Unknown/mixed source formats are excluded. Historical reported
distance supplies proportional ascent even if stored laps disagree. Target
lap-derived geometry still adds its known lead-in once.

### Checks

- Full default suite in fresh HOME/XDG, explicit Cargo/Rustup homes, offline
  dependencies, single thread, no snapshot updates: **192 passed, 7 ignored**.
  Some CLI tests call the public API; this is not a hermetic suite.
- The integration fixture proves that imported measured history changes the
  duration filter, compact minutes and verbose minutes together; covers absent
  scores, canonical aliases, thresholds, chronology, fixed duration, unknown
  climbing, unsupported formats/sources, mixed riders and query-error fallback.
- A reviewer-identified weak contradictory-laps test was strengthened with twelve
  varying nonzero-ascent routes and a known 43.2-minute expected prediction.
  The affected integration target passed again after that test-only change.
- Legacy category-formula regression on copied history:
  **2.093 → 2.093 minutes MAE**, n=6. This is the unchanged fallback baseline,
  not the personal-estimator result.
- Shipped Rust `estimate_event()` replay: 182 ordinary race observations, with
  170 history estimates (18 same-route, 152 distance/ascent) and 12 warm-up
  category fallbacks. Filter/compact/verbose consistency checked per observation.
  An independently written Python arithmetic reference matches **all170** personal
  predictions after nearest-minute rounding.
- Initial replay exposed same-route pace incorrectly clamped as though it were
  flat pace, and inconsistent historical lap versus distance-based ascent.
  Both were corrected before final replay; no hyperparameters were tuned against
  the resulting evaluation metrics.
- Fresh-context same-family review found and verified fixes to compact provenance,
  experimental labels and supported-source checks; final verdict no integration
  blocker. It did not independently rerun parent validation and is not cross-family
  verification.
- Release build passed. A live release CLI run with isolated copied history,
  score195 and 45±5 minutes returned **26 matching races** with history basis
  visible. This count is feed/time dependent, not a permanent expectation.
- Normal database SHA-256 unchanged:
  `c79e15e945c0d8a4d7cfe0ff915150bef6f8cc98157a2a492011fea367d608cf`.
  No normal-state imports/writes, archive edits, commits or publication.

### Actual shipped-estimator historical results

The dated holdout retains the earlier fixed cutoff2025-03-30; after ordinary-race
format selection it contains37 observations, not the prior approximate-profile
experiment's38. Cohorts must not be interchanged.

| Paired cohort | n | Personal/live-path MAE | Comparator MAE |
|---|---:|---:|---:|
| All ordinary races, including12 fallback warmups |182|7.072 min|7.195 min fixed C-neutral reference |
| Dated ordinary-race holdout |37|4.940 min|5.197 min fixed C-neutral reference |
| Genuinely score-known observations |6|2.159 min|2.093 min category production formula |

The fixed C-neutral reference is **not** inferred historical score or reconstructed
production. On the37-row holdout,40–50-minute selection is TP/FP/FN **6/2/4**
for the personal model versus **9/2/1** for neutral. The other fixed windows and
all-row counts are retained privately. On six genuinely scored rows, both personal
and category select two suitable40–50-minute performances, include none unsuitable,
and miss one suitable performance. No selection-superiority claim is supported.
The owner requested the personal workflow after discussing mixed earlier evidence;
this is that implementation, with explicit experimental qualification—not a claim
the original accuracy-improvement objective has been established.

Private protocol, readonly database snapshot, shipped-code Rust replay, independent
reference, row-level predictions, aggregate summaries and test/build logs live under
`~/.local/share/zwift-race-finder/investigations/20261102-live-personal/`.
No rider-level observations or client exports are in Git.
