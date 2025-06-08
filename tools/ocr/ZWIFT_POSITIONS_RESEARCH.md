# Zwift Rider Positions Research Summary

## Key Finding: Visual Only, No Aerodynamic Impact

**IMPORTANT**: Regular rider positions in Zwift are **purely visual** and do NOT affect aerodynamics or speed!

### Actual Zwift Physics Model
- Aerodynamics in Zwift are determined ONLY by:
  - Rider height
  - Rider weight
  - These create a "body surface area" calculation
- Position changes (hoods/drops/standing) have NO effect on CdA or speed
- This is completely different from real-world cycling

### Visual Position Changes

#### 1. **Hoods vs Drops** (Automatic)
- **On the drops**: Speed ≥ 32-33 km/h AND not drafting
- **On the hoods**: All other situations (slower speed OR drafting)
- This is a visual cue to show drafting status
- NO speed/power difference between positions

#### 2. **Standing vs Seated** (Cadence-based)
- **Standing**: Cadence between 31-72 RPM on climbs ≥3% gradient
- **Seated**: Cadence ≤30 RPM or >72 RPM, or gradient <3%
- Again, purely visual - NO aerodynamic impact

#### 3. **Supertuck** (Special Case - DOES affect speed!)
- The ONLY position that actually changes aerodynamics
- Activates automatically when:
  - Steep downhill (typically -3% or steeper)
  - High speed
  - Not pedaling (0 watts)
  - Not drafting (must be "in the wind")
  - Not on TT or MTB bike
- Provides ~25% drag reduction (similar to Aero power-up)
- Can stack with Aero power-up for even more benefit

### Real-World CdA Values (for comparison)
- Upright position: 0.30-0.50
- On the drops: 0.25-0.30
- TT position: 0.20-0.25
- Pro TT riders: <0.20

### Implications for Our OCR Tool

Given this research, our pose detection should be updated:

1. **Regular Positions** (no speed impact):
   - `seated_hoods` - Normal seated, hands on hoods
   - `seated_drops` - Normal seated, hands on drops (≥33km/h, not drafting)
   - `standing` - Out of saddle (31-72 RPM on climbs)
   - `seated_climbing` - Seated on climb (≤30 or >72 RPM)

2. **Special Positions** (affect speed):
   - `supertuck` - Descending position (25% drag reduction)

3. **Power-ups** (separate from position):
   - Track these independently as they stack with positions

### Correcting Our Initial Understanding

Our initial classification was incorrect:
- ❌ "normal_tuck" having HIGH drag - This appears to be wrong
- ❌ Different drag levels for different positions - Only supertuck matters
- ✅ Visual positions are just animations for realism
- ✅ Only supertuck actually changes game physics

### Visual Cues Purpose
1. **Drafting indicator**: Sitting up (hoods) = you're drafting
2. **Effort indicator**: Standing = high effort/low cadence climbing
3. **Realism**: Makes the game feel more like real cycling

### Special Cases
- **Tron bike**: No position changes at all (always same visual)
- **TT bikes**: Cannot supertuck
- **MTB bikes**: Cannot supertuck

## Recommendations

For our OCR tool, we should:
1. Still detect positions for data analysis and realism
2. But NOT use them for speed/power calculations
3. Only apply aerodynamic effects for supertuck position
4. Track power-ups separately as they DO affect speed

## Sources
- Zwift Insider articles on rider positions, supertuck, and drafting
- Zwift Forums discussions on aerodynamics
- Science4Performance analysis of Zwift physics model