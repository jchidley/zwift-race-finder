# Zwift Physics Equations and Sources

## Overview

This document provides a comprehensive reference for the physics equations used in Zwift, with proper attribution to their sources. It distinguishes between officially documented values, community reverse-engineered formulas, and academic research.

### Key Understanding

Zwift appears to use established mathematical models from cycling physics research (primarily Martin et al. 1998) but applies its own coefficients and assumptions. The exact implementation details are proprietary, leading the community to reverse-engineer behaviors through empirical testing. Since only power output can be changed during a race (height, weight, and equipment are fixed at start), understanding these relationships is crucial for race performance.

### What We Know vs What We Assume

**Confirmed by Zwift or Testing:**
- Rolling resistance values for different surfaces (Zwift Insider tests)
- Draft savings percentages (24.7-33%)
- Environmental constants (no wind, fixed air density)
- General physics relationships (power/weight on climbs, power/CdA on flats)

**Community Reverse-Engineered:**
- CdA formula with specific coefficients
- Equipment CdA base value (0.1647)
- Height/weight impact on aerodynamics

**Unknown/Assumed:**
- Exact implementation of physics equations
- All coefficient values in Zwift's code
- How different equipment CdA values are calculated
- Future changes to the physics engine

## Power Equation (Academic Foundation)

### Martin et al. (1998) Equation
The fundamental cycling power equation from "Validation of a Mathematical Model for Road Cycling Power":

```
P = M·g·v·cos(arctan(G))·Crr + M·g·v·sin(arctan(G)) + (1/2)ρ·CD·A·v³
```

Where:
- **P** = Power (watts)
- **M** = Mass of rider + bike (kg)
- **g** = Gravitational acceleration (9.81 m/s²)
- **v** = Velocity (m/s)
- **G** = Grade (slope percentage)
- **Crr** = Rolling resistance coefficient
- **ρ** = Air density (kg/m³)
- **CD** = Drag coefficient
- **A** = Frontal area (m²)

**Source**: Martin, J.C., Milliken, D.L., Cobb, J.E., McFadden, K.L., & Coggan, A.R. (1998). Journal of Applied Biomechanics, 14(3), 276-291.

### Power Components
- **Rolling resistance**: 10-20% of total power
- **Gravitational resistance**: 10-20% on climbs
- **Aerodynamic drag**: 56-96% of total power (largest component)

## Zwift-Specific Values

### CdA (Coefficient of Drag × Area)

#### Frontal Area Formula
```
A = 0.0276 × h^0.725 × m^0.425 + 0.1647
```

Where:
- **h** = Height in meters
- **m** = Mass in kilograms
- **0.1647** = Equipment CdA (bike + wheels)

#### Formula Origins
This formula appears to be based on the **Du Bois Body Surface Area (BSA)** formula:
- Standard Du Bois: `BSA = 0.007184 × Weight^0.425 × Height^0.725`
- The exponents (0.725, 0.425) match exactly
- The coefficient 0.0276 appears to be Zwift's scaling factor to convert BSA to frontal area

**Source**: Community reverse-engineered
- **Status**: NOT officially documented by Zwift
- **Discovery Method**: Systematic testing and data analysis by community members
- **Forum Discussions**:
  - [CdA dependency on height issue](https://forums.zwift.com/t/cda-dependency-on-height-issue/561927)
  - [How does watts/CdA work for TT in Zwift?](https://forums.zwift.com/t/how-does-watts-cda-work-for-tt-in-zwift/147046)
  - [CdA for tall riders](https://forums.zwift.com/t/cda-for-tall-riders/520042)
  - [Zwift TT tests - TrainerRoad](https://www.trainerroad.com/forum/t/zwift-tt-tests-take-2-cda-relationship-to-rider-weight-does-this-look-right/56932)

#### Height/Weight Controversy
The community has identified significant issues with this implementation:
- Taller riders face disproportionate aerodynamic penalties
- The relationship doesn't accurately reflect real-world physics
- Creates fairness concerns in competitive racing

### Rolling Resistance (Crr) Values

**Source**: Zwift Insider testing (https://zwiftinsider.com/crr/)

#### Confirmed Values
- **Road wheels on pavement**: Crr = 0.004
- **MTB wheels on pavement**: Crr = 0.009
- **Gravel wheels on dirt**: Crr = 0.018

#### Surface Penalties
- **November 2023 Update**: Road bikes get ~80W penalty on dirt surfaces
- **Dirt surfaces**: More than double the rolling resistance for road wheels

### Pack Dynamics

**Source**: Zwift Insider (https://zwiftinsider.com/road-bike-drafting-pd41/)

- **Draft benefit**: 24.7-33% power savings (position dependent)
- **"Sticky draft"**: Wattage windows for maintaining draft
- **Dynamic CdA**: 3% reduction during attacks (>20% power increase)

### Environmental Constants

**Status**: Officially implemented in Zwift

- **Air density**: Fixed at 1.225 kg/m³
- **Wind**: None (0 m/s always)
- **Temperature effects**: None
- **Altitude effects**: None on air density

## Speed Relationships

### Fundamental Relationships
- **On flats**: Speed ∝ ∛(Power/CdA)
- **On climbs**: Speed ∝ Power/Weight

### Category-Based Pack Speeds (Empirical)
Based on analysis of 151 races:
- **Cat A**: 37.5 km/h
- **Cat B**: 35.0 km/h
- **Cat C**: 32.5 km/h
- **Cat D**: 30.9 km/h

**Source**: Regression analysis of actual race data

## Empirical Testing Approach

Since Zwift's exact implementation is proprietary, the only way to understand the physics better is through empirical testing:

### Testing Variables
- **Height**: Can be adjusted in Zwift settings (though should match real life)
- **Weight**: Can be adjusted in Zwift settings (should be accurate for fair play)
- **Power**: The only variable changeable during a race
- **Equipment**: Fixed at race start (different bikes/wheels have different CdA values)

### Testing Methodology
1. **Controlled Time Trials**: Same power, different heights/weights
2. **Real-Time Monitoring**: Tools like Sauce4Zwift for live data
3. **Statistical Analysis**: Large sample sizes to account for draft variations
4. **Community Collaboration**: Shared data across multiple testers

## Fairness and Gameplay Implications

### The Height/Weight Dilemma
Zwift requires riders to use their real-world height and weight for fair competition. However, if the physics model doesn't accurately reflect reality, this creates unintended consequences:

1. **Height Penalty**: Taller riders are slower than shorter riders at the same W/kg
2. **Weight Effects**: Complex interactions between weight, CdA, and gradient
3. **Gaming the System**: Some riders may be tempted to misrepresent their dimensions

### Fixed vs Variable Factors
During a race, only power output can be changed:
- **Fixed at Start**: Height, weight, bike choice, wheel choice
- **Variable**: Power output (watts)
- **Implication**: Understanding the physics is crucial for equipment selection and pacing strategy

## Important Notes

1. **Community vs Official**: Most formulas are community-discovered, not officially documented
2. **Simplifications**: Zwift simplifies many real-world factors for gameplay balance
3. **Updates**: Values may change with game updates without notice
4. **Validation**: Community values validated through extensive empirical testing
5. **Proprietary Nature**: Zwift's exact implementation remains a trade secret

## References

### Academic Papers
- Martin et al. (1998): Base power equation
- Chung (2003): Virtual elevation method for CdA testing

### Community Resources
- Zwift Insider: Rolling resistance tests, draft analysis
- TrainerRoad Forums: CdA formula discussions
- Zwift Forums: Height/weight impact discussions

### Tools for Testing
- Sauce4Zwift: Real-time power/speed monitoring
- ZwiftPower: Historical race data analysis
- Virtual elevation method: CdA validation technique