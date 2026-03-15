# Zwift Physics

How Zwift models cycling physics, where it simplifies reality, and what the academic research says.

## The Foundation: Martin et al. (1998)

Zwift's physics derive from the standard cycling power equation:

```
P = M·g·v·cos(arctan(G))·Crr + M·g·v·sin(arctan(G)) + (1/2)·ρ·CD·A·v³
```

| Symbol | Meaning | Zwift treatment |
|--------|---------|-----------------|
| P | Power (watts) | Measured from trainer |
| M | Rider + bike mass (kg) | User-entered weight + bike |
| g | Gravity (9.81 m/s²) | Standard |
| v | Velocity (m/s) | Calculated |
| G | Gradient (%) | From route data; **halved on descents** |
| Crr | Rolling resistance | Fixed per surface type |
| ρ | Air density (kg/m³) | Fixed at 1.225 — no altitude/temperature effects |
| CD·A | Drag coefficient × area | Simplified formula per equipment |

## Zwift's Simplifications

### What's Removed
- **Wind** — no crosswinds, headwinds, or echelons
- **Temperature and altitude** — air density is constant
- **Bearing friction and drivetrain losses** — not modelled
- **Position changes** — CdA is fixed per equipment choice
- **Fatigue** — repeated efforts don't compound

### What's Modified
- **Descent gradients halved** — an 8% descent feels like 4%
- **Braking removed in races** — groups stay together on descents
- **Binary draft** — full benefit or nothing (no gradual falloff)
- **Sticky draft** — wattage windows keep you attached

## Aerodynamics (CdA)

### Frontal Area Formula (Community Reverse-Engineered)

```
A = 0.0276 × h^0.725 × m^0.425 + 0.1647
```

- **h** = height (metres), **m** = mass (kg)
- **0.1647** = equipment CdA (bike + wheels)
- Exponents match the Du Bois body surface area formula
- **Not officially documented** — discovered through systematic community testing

**Controversy**: Taller riders face disproportionate aerodynamic penalties that may not reflect reality.

### Speed Relationships
- **Flats**: Speed ∝ ∛(Power / CdA) — aerodynamics dominate
- **Climbs**: Speed ∝ Power / Weight — w/kg is king

## Rolling Resistance (Crr)

Source: [Zwift Insider testing](https://zwiftinsider.com/crr/)

| Surface | Crr | Notes |
|---------|-----|-------|
| Road wheels on pavement | 0.004 | Standard |
| MTB wheels on pavement | 0.009 | 2.25× penalty |
| Gravel wheels on dirt | 0.018 | 4.5× penalty |
| Road bike on dirt | ~0.004 + 80W penalty | November 2023 update |

## Draft Physics

### Zwift's Model vs Real World

| Aspect | Real world | Zwift |
|--------|-----------|-------|
| Behind one rider | ~30% savings | 25% savings |
| Optimal peloton position | 90–95% drag reduction | ~35% maximum |
| Draft falloff | Gradual with distance | Binary (in or out) |
| Crosswind effect | Echelons form | No crosswinds |
| Uphill draft | Reduced but present | Still 10–15% |

### Academic Research on Real Pelotons

**Blocken et al. (2018)** — CFD study of 121-rider peloton:
- Riders in rows 12–14 experience only 5–10% of solo rider drag
- Nearly 3 billion computational cells used
- Optimal position provides 90–95% drag reduction

**Olds (Sports Engineering)** — Group velocity modelling:
- Speed increases rapidly up to 5–6 riders, then gradually to ~20
- Diminishing returns beyond 20 riders
- Real-world optimal breakaway: 5–7 riders

**Swiss Side wind tunnel** — Distance effects:
- 10 cm gap: 65% drag reduction
- 2.64 m gap: 48% reduction
- 10 m gap: 23% reduction
- Benefits measurable up to 50 m

**Sports Engineering (2021)** — Uphill drafting:
- 7.5% gradient at 6 m/s: 7% power savings
- 7.5% gradient at 8 m/s: 12% power savings

### Zwift's "Blob Effect"
Academic models don't predict the blob effect because it emerges from Zwift-specific mechanics:
- Continuous churn (no fatigue for front riders)
- No team blocking
- Perfect efficiency in position changes
- Result: 20+ rider groups go 2–3 km/h faster than equivalent real groups

## Environmental Constants

| Parameter | Value | Notes |
|-----------|-------|-------|
| Air density | 1.225 kg/m³ | Fixed, no variation |
| Wind speed | 0 m/s | Always |
| Gravity | 9.81 m/s² | Standard |
| Temperature effects | None | Not modelled |
| Altitude effects on air | None | Not modelled |

## Category-Based Pack Speeds (Empirical)

Calibrated from 151 real races:

| Category | Score Range | Pack Speed (km/h) |
|----------|-------------|-------------------|
| E | 0–99 | 28.0 |
| D | 100–199 | 30.9 |
| C | 200–299 | 33.0 |
| B | 300–399 | 37.0 |
| A | 400–599 | 42.0 |
| A++ | 600+ | 45.0 |

These speeds already include average draft benefit — they're empirical, not derived from physics models.

## What Matters for Performance

### Weight (kg)
- w/kg is the primary determinant of climbing speed
- Heavier riders have an advantage on flats (higher absolute power)
- At 86 kg vs 70 kg typical, major climb disadvantage

### Height (m)
- Increases frontal area → more drag
- Taller riders get less benefit from the draft
- Affects flats and descents more than climbs

### The Only Variable in a Race
Once a race starts, you control exactly **one thing**: power output. Height, weight, equipment, and physics are all fixed. Understanding the physics lets you spend those watts wisely.

## References

1. Martin, J.C., et al. (1998). "Validation of a Mathematical Model for Road Cycling Power." Journal of Applied Biomechanics, 14(3), 276–291.
2. Blocken, B., et al. (2018). "Aerodynamic drag in cycling pelotons." Journal of Wind Engineering and Industrial Aerodynamics.
3. Olds, T. (2018). "Optimizing the breakaway position in cycle races." Sports Engineering.
4. Swiss Side (2023). Wind tunnel testing of cycling aerodynamics and drafting effects.
5. Zwift Insider testing articles on rolling resistance, pack dynamics, and draft measurements.
6. Zwift community forum discussions on CdA formula and height/weight effects.
