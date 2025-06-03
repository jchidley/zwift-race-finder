# Project: Zwift Race Finder
Updated: 2025-06-03

## Current State
Status: Production ready with refined table UI
Target: Research simulation models and performance modeling
Latest: Table format optimized for compactness and clarity

## Essential Context
- Core zwift-race-finder: 16.1% accuracy, production ready
- Table output: ✅ Compact format with elevation data
- Filter statistics: ✅ Shows why events filtered with actionable fixes
- Advanced features defined: simulation, video analysis, live telemetry
- Key insight: GPL licensing requires clean-room implementations

## Recent Sessions

### 2025-06-03 Afternoon: Table Format Refinements
1. **Compact Table** ✅ COMPLETED
   - Columns: Event | Time | Distance | Elev | Duration
   - 24h time format (13:30 vs 1:30 PM)
   - No lap indicators in distance
   - Elevation gain in meters
   - Auto day separators for multi-day views

2. **Space Optimization** ✅ COMPLETED
   - Removed Route ✓ column (redundant)
   - Compressed duration (1h32m vs 1h 32m)
   - Day only shown via separators
   - ~20% reduction in table width

### 2025-06-03 Morning: Table Output Implementation
1. **Table Format** ✅ COMPLETED
   - Default table view, --verbose for details
   - Filter statistics with actionable fixes
   - Works with all event types

### 2025-01-06 Evening: Requirements Expansion
1. **Performance Modeling** (FER-19.16-17)
   - GoldenCheetah GPL prevents integration
   - Must reimplement from published papers
   - Focus: CP models, TSS/CTL/ATL metrics

2. **Simulation & Analysis** (FER-20.8-12)
   - Monte Carlo race simulations
   - Video analysis with OBS Studio
   - Live telemetry via AI/OCR
   - Research open-source models (MIT/Apache)

## Next Priorities
1. **Research Simulation Models**: Find MIT/Apache licensed cycling simulators
2. **Study CP Models**: From academic papers for performance prediction
3. **Proof of Concept**: Video analysis on race recordings
4. **Route Discovery**: Continue mapping unknown routes (538x Downtown Dolphin!)

## Development Approach
Continue AI-assisted development:
- Clear requirements before implementation
- Test with real Zwift data
- Iterate based on results
- Maintain compliance awareness

## If Blocked
- Review REQUIREMENTS.md Section 21 for live telemetry specs
- Check sessions/SESSION_20250106_210000.md for session details
- Remember: Screen capture only, no game interaction

## Project Structure
```
zwift-race-finder/        # Core duration prediction tool
├── src/                  # Rust application
├── tools/               # Import and utility scripts
├── sql/                 # Database scripts
└── docs/                # Documentation

zwift-live-telemetry/    # Future companion tool (not yet created)
├── capture/             # Screen capture modules
├── ocr/                 # Text extraction
└── streaming/           # Live data output
```