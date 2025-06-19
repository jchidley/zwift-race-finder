# Zwift Race Finder - User Feedback

Thank you for using Zwift Race Finder! Your feedback helps improve prediction accuracy and user experience.

## How to Provide Feedback

### 1. GitHub Issues (Preferred)
Report issues or suggestions at: https://github.com/jchidley/zwift-race-finder/issues

Include:
- Your Zwift Racing Score or category
- The command you ran
- What happened vs what you expected
- Any error messages

### 2. Accuracy Feedback
Help improve predictions by recording your actual race times:

```bash
# After completing a race, record the result:
zwift-race-finder --record-result "route_id,actual_minutes,event_name"

# Example:
zwift-race-finder --record-result "3379779247,22,Stage 4: Makuri May"
```

### 3. Route Mapping
Found an unknown route? Help by:
1. Running `zwift-race-finder --show-unknown-routes`
2. Looking up the route on ZwiftHacks.com or Zwift Insider
3. Creating an issue with the route details

## Common Issues & Solutions

### "No matching events found"
- Most races are 20-30 minutes, try: `zwift-race-finder -d 30 -t 30`
- Time trials are less common, try: `zwift-race-finder -e all`
- API only shows ~12 hours of events (200 event limit)

### Inaccurate predictions
- Record your actual times to improve calibration
- Check if it's a multi-lap race (we've fixed most of these)
- Report consistently wrong predictions via GitHub

### Unknown routes
- New routes need manual mapping
- Use `--show-unknown-routes` to see what needs mapping
- Contribute route data via GitHub issues

## Current Accuracy: 16.1%
We've exceeded our <20% target! Help us get even better by sharing your race results.

## Privacy
All feedback is voluntary. Race results are stored locally and never shared without your consent.