# AI-Assisted Development: Building Zwift Race Finder with Claude Code

This document captures the approach, lessons, and insights from building a real-world application using AI assistance without traditional coding.

## The Team

### Human: Jack (Product Owner/Manager)
- **Domain Expertise**: Active Zwift racer who understands the problem space
- **Technical Background**: 40+ years IT experience (retired professional)
- **Role**: Define problems, provide direction, validate solutions, catch assumptions

### AI: Claude Code (Developer)
- **Technical Skills**: Implements code, integrates APIs, handles debugging
- **Transparency**: Shows reasoning and decision-making process
- **Role**: Write code, explain approaches, flag assumptions, iterate on feedback

## The Management Model

Think of it as managing a very willing and enthusiastic employee who:
- Never gets tired or frustrated
- Always explains their thinking
- Sometimes makes reasonable but wrong assumptions
- Needs clear direction and context

## Key Success Factors

### 1. Clear Problem Definition
**Started with**: "I know when and how long I want to race, but not which races will actually take that long"
**Not**: "Build me a Zwift app"

This clarity guided every decision.

### 2. Domain Knowledge is Crucial
Understanding Zwift racing meant I could:
- Spot when 100km races finishing in 60 minutes didn't make sense
- Know that draft benefit matters (~30% speed increase)
- Recognize that different categories race different distances
- Understand why elevation profiles matter

### 3. Technical Experience Helps (But Coding Isn't Required)
40 years of IT experience meant:
- Knowing when SQLite beats JSON files
- Understanding API authentication patterns
- Recognizing when to pivot approaches
- Asking the right debugging questions

### 4. Transparency Enables Quality Control
Claude showing its reasoning allowed me to:
- Catch wrong assumptions early
- Spot data/description mismatches
- Understand why certain approaches were chosen
- Redirect when heading the wrong direction

## Development Process

### 1. Initial Attempt (92.8% Error)
- Started with ZwiftPower data
- Discovered "actual times" were estimates
- Lesson: Always validate your data

### 2. Strava Integration (31.2% Error)
- Pivoted to get real race times
- Required learning OAuth flows
- Lesson: Don't be afraid to change approach

### 3. Multi-Lap Fix (25.1% Error)
- Event names misleading, API had better data
- Required understanding event_sub_groups
- Lesson: Data structure matters more than descriptions

## Practical Tips

### DO:
- **Test with real data frequently** - Assumptions will be wrong
- **Keep good documentation** - Track decisions and discoveries
- **Question surprising results** - 100km in 60 minutes?
- **Use your expertise** - Domain knowledge is your superpower
- **Think system-wide** - Consider the full workflow

### DON'T:
- **Accept magic numbers** - Ask why 25 km/h?
- **Ignore your instincts** - If something seems wrong, investigate
- **Skip testing** - Real data reveals real problems
- **Fear pivoting** - Better approaches emerge with learning

## Results That Matter

- **25.1% prediction accuracy** - Better than many v1.0 products
- **151 real races analyzed** - Data-driven, not theoretical
- **Multiple pivots** - Adapted as we learned
- **Practical tool** - Actually solves the original problem

## The Bottom Line

You don't need to be a coder to build software with AI. You need:
1. A clear problem to solve
2. Domain knowledge about that problem
3. Ability to manage and direct AI (like any employee)
4. Willingness to test and iterate

The combination of human expertise and AI capability is powerful. The human provides wisdom, context, and quality control. The AI provides implementation, integration, and transparency.

This isn't about learning to code - it's about learning to direct AI effectively to solve real problems.