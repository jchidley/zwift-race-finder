# Physical Stats and Duration Estimation

This reference distinguishes Zwift physics inputs from the inputs currently used by Zwift Race Finder. It intentionally contains no personal rider measurements.

## Zwift physics

Weight, height, power, equipment, gradient, rolling resistance, and drafting can all affect speed in Zwift. Weight is especially important on climbs; height contributes to aerodynamic drag; absolute power matters more on flatter terrain.

For domain background rather than application behaviour, see [Zwift Physics](../explanation/ZWIFT_PHYSICS.md) and [Zwift Physics Equations](../explanation/ZWIFT_PHYSICS_EQUATIONS.md).

## Current estimator inputs

The live finder does **not** directly use rider weight, height, or FTP. It uses:

- Racing Score to select event subgroup/course geometry, not personal pace;
- declared event time when available;
- earlier source-qualified ordinary-race results: measured time, total distance
  and canonical route identity for same-route pace;
- total course distance and known climbing for cross-course personal fitting
  when same-route support is insufficient.

Unavailable personal estimates remain unknown and are excluded from duration
matches. Unfamiliar routes can still use supported cross-course fitting.
The retained category-speed and climbing-penalty helpers are for separate callers
and legacy regression, not the finder's personal duration fallback. See
[Algorithms](ALGORITHMS.md) for precedence, chronology and support rules.

This is an empirical performance model, not an individual physics simulation.
Race outcomes also reflect drafting, field, pacing and motivation; they are not
pure measurements of physiological capacity.

## Removed unused settings

Unused physical-stat configuration, accessors and the manual updater were removed. Fresh databases no longer create `rider_stats`; existing tables and their data are left intact. Adding them to the formula would be a model change and requires new calibration evidence plus the accuracy regression described in [Algorithms](ALGORITHMS.md).

## Consequence for changes

Do not describe raw weight, height, or FTP as current prediction inputs. Adding physical inputs must establish an improvement on chronological held-out predictions over the existing personal history model. Racing Score remains a subgroup selector, not a pace or fitness feature.
