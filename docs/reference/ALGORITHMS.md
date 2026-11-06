# Duration Estimation Algorithms

## Overview

Racing Score selects the relevant event subgroup. Its route and distance feed a
score-independent personal estimate from earlier measured race results when
usable history is available. Declared fixed durations remain authoritative;
otherwise insufficient history, geometry or unsupported formats produce unknown
duration, excluded from time-window matches. The CLI discloses the omitted count. Filtering and display consume the same resolved estimate.

## Personal duration estimates

`src/personal_estimation.rs` implements the live history path for ordinary races.
It uses measured seconds, reported total event distance and canonical client route
identity, without requiring or inferring historical Racing Scores. Time trials,
rides and unknown source formats are not ordinary-race training data.

The model uses earlier UTC dates only, before `min(today, event day)`:

1. At least three same-route observations with prior/target distance ratio in
   `[0.8, 1.25]`: recency-weighted median minutes/km × target distance. This is
   approximate distance scaling, not a reconstructed partial-route profile.
2. Otherwise at least twelve observations with known course climbing: fit
   `minutes = pace × (distance_km + h × ascent_km)`. A fixed grid
   `h = 0, 2, 4, 8, 16, 32, 64` and pace bounded to 1–4 min/km minimize
   recency-weighted time MAE. Pace is the
   feature-weighted median; exact shape ties retain the first/smallest grid value.
3. If neither is supported, return an explicit unavailability reason, not a category-speed duration. Unfamiliar courses still use step 2 when supported.

Both branches use a **365-day half-life with no hard age cutoff**. Same-route
support takes priority whenever its threshold is met; it is not compared against
newer cross-course support. Older volume can dominate newer observations. These
are current implementation choices, not an adopted optimal current-form policy.
The unresolved recency work belongs in [Tasks](../../TASKS.md#immediate-plan--recent-performance-relevance).

Resolved durations retain fractional minutes through filtering. Only presentation
rounds to whole minutes, so a displayed boundary value can fall just outside the
requested window. Both CLI and library filtering use the same inclusive raw-duration
comparison. Verbose output reports
method, sample count and descriptive fitted residual error. That error is **not**
a confidence interval. The model uses course distance/climbing totals, not exact
ordered gradients. Whole-kilometre historical distances and retrospective route
metadata remain qualifications. This user-requested integration is not a claim
that personal prediction has demonstrated superior time-window selection.
For the preserved baseline, candidate policy and measured comparisons, see the
[baseline/recency record](../project-history/RECENCY_BASELINE_2026-11-03.md).

## Estimation Priority

`event_filtering::resolve_event()` returns `Result<EventEstimate, EstimateUnavailable>`. A successful estimate carries minutes, available distance/elevation, fixed status and optional personal method/support; an error carries the unavailability reason. `estimate_event()` is the `Option` convenience adapter. `event_matches_duration()` compares those minutes to the target with an inclusive tolerance. Both compact and verbose display use the same resolver; display does not add a separate surface multiplier or extra lead-in. An unestimable event is not a match.

The resolver selects a subgroup before calculating geometry. For Racing Score events,
it matches the numeric `rangeAccessLabel` against the supplied score rather than
searching the event title for a category letter. Ranges are inclusive; touching
endpoints retain API order, not an inferred unique eligibility decision. A score
outside all declared ranges has no estimate. Blank/whitespace-only range labels
are not restrictions; malformed nonblank labels remain restricted rather than
silently turning into traditional events. Traditional groups use exact
`subgroupLabel` when present, otherwise category tokens in the name; D-to-E
fallback follows an exact D match. Score is not passed into personal fitting and does not supply a substitute speed.

Duration precedence is positive event minutes, positive event seconds, then the
selected subgroup's positive minutes or seconds. Seconds are converted to fractional minutes without truncation; any positive
declared duration, including one below a minute, remains authoritative. Subgroup detail display uses the same duration helper. Declared time overrides a calculated estimate. For example, 2,700 seconds
remains 45 fixed minutes even when route geometry would predict something else.

Fixed and calculated durations share one geometry policy:

1. **Selected subgroup total distance**, if positive and finite.
2. **Selected subgroup positive laps**, using lap distance × laps plus one known
   event lead-in. Zero laps are unavailable, not a lead-in-only event.
3. **Event total distance**, if positive and finite.
4. **Known route fallback**, including the existing historical multi-lap lookup
   and one known event lead-in.
5. **Racing Score description distance**, then the existing name-distance guess.

All route calculations prefer `subgroup.route_id` over `event.route_id`. An explicit
total distance never receives another lead-in. Known route metadata supplies geometry to the personal model; unknown routes do
not receive a name/category-speed duration.
Distance/elevation provenance remains independent of whether duration is fixed.

Description and name fallbacks are estimates, not verified route facts. In particular, the existing description parser takes the first matching distance and does not select the relevant stage from multi-stage text; shared display makes this visible but does not resolve that data-quality issue.

### Lead-in Distance Handling

- **Multi-lap geometry (step 2)**: Lead-in added once by the shared resolver in `event_filtering.rs`
- **Route-only geometry (step 4)**: The shared resolver adds one lead-in; the standalone `estimate_duration_from_route_id` helper also adds one for its own callers.
- **Explicit event/subgroup distance**: Treated as total distance; no extra lead-in is added by filtering or display.
- **Other fallbacks**: Lead-in is not added. Historical DB multi-lap metadata uses lap distance × laps plus one lead-in, then estimates that total, rather than multiplying a single-lap time and repeating its lead-in.

### Elevation knowledge and totals

Route and event lead-in climbing are optional: unknown is not zero. SQLite knowledge flags preserve legacy positive values while treating legacy zeros as unknown. Explicitly stored zero is known flat. The primary source is selected installed-client route metadata, then previously extracted client XML for missing IDs. These are the only supported route metadata inputs; see [client-only import](../howto/CLIENT_ROUTE_DATA.md). Client imports reconcile legacy metadata by numeric ID, atomically preserve history, and persist source provenance. Community imports and website scraping are retired; existing database rows remain available until explicitly updated. No route dataset is bundled.

- **Lap-based totals:** fractional lap elevation × laps + fractional event lead-in elevation once, rounded only for display. Both must be known; otherwise the total displays as unknown. Missing lead-in distance is also preserved as unknown rather than assumed zero. Route-only/lap distance estimates require a known lead-in distance.
- **Distance-limited totals:** proportional lap climbing for (total distance − lead-in distance), plus known lead-in climbing. Without a full elevation profile this is approximate and displays with a `~` prefix. Distances shorter than the lead-in do not get invented climbing.
- **Personal duration inputs:** use available total climbing for the cross-course fit. Unknown climbing is not zero; same-route pace may still be supported without it. No category or name-based speed is substituted.
- **Fixed durations:** climbing/distance metadata never overrides declared time.

### Route ID Alias Resolution

Zwift uses different internal route IDs for the same physical route depending on context (event-only vs free-ride). The `route_aliases` table maps these alternative IDs to canonical DB route IDs. `Database::get_route()` checks aliases transparently: direct lookup → alias lookup → None. See [Database Reference](DATABASE.md).

## Validation and calibration

Evaluate personal changes chronologically against the versioned baseline, using
identical input snapshots and only earlier observations for each target. Report
unrounded predicted seconds against exact measured finish seconds. Report signed
error (prediction minus measured), absolute error, MAE and mean signed error in
seconds, duration-window inclusions/exclusions and prediction availability. Window
decisions also use unrounded predictions and measured seconds; displayed rounding
must not enter these calculations. Round only report presentation, not stored inputs
or errors. Whole-kilometre source distances remain an independent input limitation.
The [prediction evaluator](../howto/TESTING_GUIDE.md#evaluate-saved-personal-predictions)
implements this policy for saved chronological replay outputs.
Preserve measured seconds and event geometry; do not tune on inspected races and
call the same outcomes independent validation. Fitted residuals are not forecast
intervals. The [baseline/experiment record](../project-history/RECENCY_BASELINE_2026-11-03.md)
owns results; [Tasks](../../TASKS.md) owns the next experiment.

The standalone category-speed helpers in `category.rs`, `duration_estimation.rs`
and `estimation.rs` remain for compatibility callers and the legacy accuracy
regression. They do not supply the active finder's pace or unknown-duration
fallback. Their retained calibration history is available in Git at `v0.3.0`.

Any formula change also requires the same approved populated-data legacy regression
before/after, separately from personal evaluation. That test reports MAE and MAPE,
asserts MAPE below 30%, and can pass on zero comparisons; zero usable data means
unqualified accuracy. See the [testing procedure](../howto/TESTING_GUIDE.md#run-the-accuracy-regression).
Time trials require declared time; ordinary-race fitting does not support them.
