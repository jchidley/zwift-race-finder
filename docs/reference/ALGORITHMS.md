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
   recency-weighted time MAE. Recency has a 365-day half-life. Pace is the
   feature-weighted median; exact shape ties retain the first/smallest grid value.
3. If neither is supported, return an explicit unavailability reason, not a category-speed duration. Unfamiliar courses still use step 2 when supported.

Calculated personal output rounds once to whole minutes. Verbose output reports
method, sample count and descriptive fitted residual error. That error is **not**
a confidence interval. The model uses course distance/climbing totals, not exact
ordered gradients. Whole-kilometre historical distances and retrospective route
metadata remain qualifications. This user-requested integration is not a claim
that personal prediction has demonstrated superior time-window selection.
See [implementation and verification](../project-history/LIVE_PERSONAL_ESTIMATOR_2026-11-02.md).

## Retained Legacy Algorithm: Category Speed Model

These helpers remain for standalone callers and the historical regression. The
finder no longer calls them to fill unsupported personal estimates.

```
duration_minutes = round((distance_km / (category_speed × difficulty_multiplier)) × 60)
```

Calculated durations are rounded once to the nearest whole minute; positive half-minute ties round up. The shared event estimate is used for both filtering and display. The retained legacy category speeds and difficulty multipliers are unchanged. Fixed-duration handling is unchanged. Whole-minute presentation does not imply accuracy within one minute.

### Category Speeds (km/h)

Empirical averages from real races. Already include draft benefit.

| Category | Score Range | Speed (km/h) | Source |
|----------|-------------|--------------|--------|
| E | 0–99 | 28.0 | Estimated ~10% slower than Cat D |
| D | 100–199 | 30.9 | Calibrated from 151 real races |
| C | 200–299 | 33.0 | Scaled from Cat D |
| B | 300–399 | 37.0 | Scaled from Cat D |
| A | 400–599 | 42.0 | Scaled from Cat D |
| A++ | 600+ | 45.0 | Estimated ~7% faster than Cat A |

Score 600+ maps to "A++" (45.0 km/h). The detailed category function splits this further: 590–649 = "A+", 650+ = "A++".

### Route Difficulty Multiplier

Multiplied with category speed. Values >1.0 mean faster (flat), <1.0 mean slower (hilly).

**Elevation-based** (`get_route_difficulty_multiplier_from_gain_and_category`):

Uses piecewise linear interpolation across 9 breakpoints for smooth transitions, with a category-aware penalty on steep climbs.

| Elevation/km | Base Multiplier | Terrain Example |
|-------------|-----------------|-----------------|
| 0 m/km | 1.10 | Tempus Fugit (~1 m/km) |
| 5 m/km | 1.05 | Most Watopia routes |
| 10 m/km | 1.00 | Rolling (transition zone) |
| 15 m/km | 0.93 | Rolling hills |
| 20 m/km | 0.85 | Mountain 8 (~21 m/km) |
| 30 m/km | 0.70 | Very hilly |
| 40 m/km | 0.55 | Mountain climbs |
| 60 m/km | 0.45 | Road to Sky (~60 m/km) |
| 100 m/km | 0.30 | Theoretical maximum |

Values between breakpoints are linearly interpolated.

**Category climbing penalty** (applied above 15 m/km):

On flat terrain, all categories ride near their empirical category speed (ratio ≈ 1.0). On steep climbs, lower categories are disproportionately slower because w/kg matters more than raw watts. This was measured from race data: Cat D achieves 48% of its flat speed on >20 m/km climbs, while Cat C achieves 54%.

| Category | Climbing Factor | Effect |
|----------|----------------|--------|
| A / A++ | 1.00 | No penalty |
| B | 0.97 | Minor penalty |
| C | 0.93 | Moderate penalty |
| D | 0.85 | Significant penalty |
| E | 0.75 | Large penalty |

The final multiplier on a climb is `base_multiplier × category_factor`. For example, Road to Sky (60 m/km) for Cat D: `0.45 × 0.85 = 0.38`, giving an effective speed of `30.9 × 0.38 = 11.7 km/h` — close to the observed 9.6 km/h from actual race data.

**Name-based fallback** (`get_route_difficulty_multiplier`) — used only when no elevation data is available:

| Route name contains | Multiplier |
|---------------------|------------|
| "alpe", "ventoux" | 0.7 |
| "epic", "mountain" | 0.8 |
| "flat", "tempus" | 1.1 |
| (anything else) | 1.0 |

### Which Multiplier Is Used Where

| Function | Multiplier Used | When Called |
|----------|----------------|------------|
| `estimate_duration_from_route_id` | Elevation + category | Route known, no distance override |
| `estimate_duration_with_distance` | Elevation + category | Route known, distance provided (multi-lap) |
| `estimate_duration_for_category` | Name-based only | No route lookup possible (fallback) |
| `event_filtering::estimate_event` | Resolves the ordered fallback below | Shared by duration filtering, compact rows, and verbose duration output |

### Estimation Priority

`event_filtering::estimate_event()` resolves one `EventEstimate` (minutes, available distance/elevation, and fixed-versus-estimated status). `event_matches_duration()` compares those minutes to the target with an inclusive tolerance. Both compact and verbose display use the same resolver; display does not add a separate surface multiplier or extra lead-in. An unestimable event is not a match.

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
selected subgroup's positive minutes or seconds. Seconds use integer division by
60; a result below one whole minute is unavailable and does not mask a calculated
estimate. Subgroup detail display uses the same duration helper. Declared time overrides a calculated estimate. For example, 2,700 seconds
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

The accuracy regression reports **mean absolute percentage error (MAPE)** and
**mean absolute error in minutes (MAE)**. These are different metrics.

The 2026-11-02 evaluation used recorded total event distances and precise finish
seconds. Six observations have known historical Racing Scores:

| Formula output | MAE, minutes | MAPE | Within 5 minutes |
|---|---:|---:|---:|
| Previous truncation | 1.93 | 4.32% | 6/6 |
| Nearest-minute rounding | 2.09 | 4.67% | 6/6 |
| Tested personal last-five adjustment, rounded | 2.59 | 5.39% | 6/6 |

Nearest-minute rounding is a presentation-resolution decision, not a demonstrated
accuracy improvement. This small cohort does not establish accuracy within one
minute or reconstruct the original live API event resolver.

The focused regression now prefers recorded event distance over event-name guesses
and measured seconds over legacy whole-minute observations. The previous regression
ignored these fields: its 6.167 → 5.833 minute MAE and 13.3% → 12.4% MAPE rounding
comparison therefore used different geometry/targets, not evidence that production
predictions improved by correcting the test. Legacy rows without the new fields
retain fallback handling, and the regression reports precise-time/distance counts.

All 194 measured observations were retained. The 188 without historical Racing
Score cannot support a genuine score-dependent production comparison, but were
used in the score-independent personal-pace experiment. That candidate was rejected:
its small dated-holdout gain was not robust and it worsened the known-score result.
See [the practical evaluation](../project-history/PRACTICAL_ESTIMATOR_2026-11-02.md).

Run `cargo test --lib test_race_predictions_accuracy -- --nocapture` against an
isolated database copy. Run the rest of the suite with fresh HOME/XDG state;
some route-consistency diagnostics assume synthetic rather than personal history.
Older 125-race and percentage-labelled “MAE” claims are historical calibration
claims, not the current verified test result.

A personal pace adjustment must use only earlier observations, preserve unknown
historical scores, and be evaluated using comparable event distances/lead-ins.
Report out-of-sample MAE in minutes and within-window counts. Do not select an
adjustment using the same later results presented as its validation. Existing
category speeds already include drafting; adding a second generic draft bonus
would double-count that benefit. Physics simulation and exact equipment are not
prerequisites for useful race selection.

## Special Cases

| Case | Handling |
|------|----------|
| Fixed duration events | Duration used directly from event data (no estimation) |
| Time trials | Declared duration only; ordinary-race fitting is unsupported |
| Multi-lap | Route distance × laps + lead-in |
| Racing Score events | `distanceInMeters: 0` in API; parse from description |
| Unknown routes | Personal support required; no category/name-speed duration |
| Category E | Used for subgroup selection, not personal pace |
