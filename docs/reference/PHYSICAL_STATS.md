# Physical Stats and Duration Estimation

This reference distinguishes Zwift physics inputs from the inputs currently used by Zwift Race Finder. It intentionally contains no personal rider measurements.

## Zwift physics

Weight, height, power, equipment, gradient, rolling resistance, and drafting can all affect speed in Zwift. Weight is especially important on climbs; height contributes to aerodynamic drag; absolute power matters more on flatter terrain.

For domain background rather than application behaviour, see [Zwift Physics](../explanation/ZWIFT_PHYSICS.md) and [Zwift Physics Equations](../explanation/ZWIFT_PHYSICS_EQUATIONS.md).

## Current estimator inputs

The production duration estimator does **not** directly use rider weight, height, or FTP. It uses:

- Zwift Racing Score → empirical category speed;
- route or event distance;
- route elevation gain per kilometre;
- a category-specific climbing penalty on steeper routes;
- route-name difficulty only when elevation data is unavailable.

The category/elevation interaction is an aggregate proxy for climbing differences between rider groups. It is not an individual physics simulation.

## Removed unused settings

Unused physical-stat configuration, accessors and the manual updater were removed. Fresh databases no longer create `rider_stats`; existing tables and their data are left intact. Adding them to the formula would be a model change and requires new calibration evidence plus the accuracy regression described in [Algorithms](ALGORITHMS.md).

## Consequence for changes

Do not describe raw weight, height, or FTP as current prediction inputs. A future personalized model must establish that those inputs improve held-out predictions rather than duplicating effects already represented by Racing Score category and elevation.
