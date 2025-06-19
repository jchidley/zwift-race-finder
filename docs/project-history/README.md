# 📜 Project History

This directory preserves the development journey of the Zwift Race Finder, showing how we achieved 23.6% prediction accuracy.

## 📁 Directory Structure

### `/accuracy-timeline/`
The journey from 92.8% to 23.6% error:
- Initial assumptions vs reality
- Key breakthroughs
- Algorithm evolution

### `/test-evolution/`
How our testing approach developed:
- From basic tests to mutation testing
- Coverage plans and iterations
- Lessons learned

### `/refactoring-plans/`
Code improvement initiatives:
- Completed refactoring efforts
- Design pattern evolution
- Modernization steps

### `/api-discoveries/`
Zwift API research history:
- Endpoint discoveries
- Integration challenges
- Workarounds developed

### `/coverage-plans/`
Testing coverage iterations:
- Different approaches tried
- Why we don't target 100%
- Pragmatic decisions

### `/migrations/`
Major technical migrations:
- UOM (Units of Measure) evaluation
- Database schema changes
- API version updates

### `/debugging/`
Significant debugging sessions:
- Error investigations
- Performance issues
- Algorithm corrections

## 🎯 Key Milestones

### The Accuracy Journey
1. **92.8% Error** - Initial category-based estimates
2. **31.2% Error** - Discovered we were comparing estimates to estimates
3. **25.1% Error** - Integrated real Strava times
4. **36.9% Error** - Added pack dynamics model
5. **25.7% Error** - Fixed route mapping errors
6. **23.6% Error** - Current state with proper calibration

### Major Discoveries
- ZwiftPower times were estimates, not actuals
- Pack dynamics are binary (with pack or dropped)
- Route IDs more reliable than names
- Multi-lap races need special handling
- Draft benefit crucial for accuracy

### Technical Evolution
- Started with simple HTTP requests
- Added SQLite for persistence
- Integrated Strava for real times
- Built regression test suite
- Implemented mutation testing

## 📊 Lessons Learned

### What Worked
- Empirical approach with real data
- Regression testing against actual races
- Simple algorithms over complex models
- Route ID as primary identifier

### What Didn't
- ZwiftPower scraping (data quality issues)
- Pure physics models (127% error)
- Category position tracking
- Complex ML approaches

### Key Insights
1. **Real data beats assumptions**
2. **Simple models can be accurate**
3. **Test quality matters more than quantity**
4. **User needs drive features**

## 🔍 Exploring History

### For Developers
- See how design decisions were made
- Understand why certain approaches were chosen
- Learn from past challenges

### For Contributors  
- Avoid repeating past mistakes
- Build on proven foundations
- Understand project philosophy

### For the Curious
- See how accuracy improved over time
- Understand the development process
- Learn about iterative improvement

## 📝 Notable Documents

- `ACCURACY_TIMELINE.md` - The full accuracy improvement story
- `DISCOVERIES_TIMELINE.md` - Key technical discoveries
- Various timestamped plans showing iteration

This history helps future development by preserving context and rationale.