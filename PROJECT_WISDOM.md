# PROJECT_WISDOM.md

## Zwift Race Finder - Key Discoveries

### 2025-05-25: Accuracy Journey Reveals Data Quality Importance
Insight: The path from 92.8% to 25.7% error wasn't about better algorithms - it was about better data
Impact: Single route mapping error (EVO CC) caused 11.2% accuracy degradation. Data quality > algorithm sophistication

### 2025-05-25: Racing Variance is a Feature, Not a Bug
Insight: Same rider, same route can vary 32-86 minutes based on pack dynamics
Impact: Trying to achieve <20% error is futile - the variance is inherent to bicycle racing

### 2025-05-25: Binary States Dominate Zwift Racing
Insight: You're either with the pack (30.9 km/h) or dropped and solo (23.8 km/h) - no middle ground
Impact: Complex physics models fail because Zwift racing is about draft, not watts

### 2025-05-25: Test What You Ship, Not What You Think You Ship
Insight: We were testing estimates against estimates for weeks without realizing it
Impact: Integration with Strava for real race times revealed our entire baseline was wrong

### 2025-05-25: Route IDs > Event Names
Insight: Event names change but route IDs are stable - always use the most stable identifier
Impact: Reduced unknown routes from 50+ to <10 by focusing on route_id mapping

### 2025-05-26: SQLite Correlated Subqueries are a Trap
Insight: SQLite's UPDATE statement limitations force creative solutions - temp tables > complex subqueries
Impact: Hours of debugging could be avoided by using simpler, more SQLite-friendly patterns

### 2025-05-26: 80% Match Rate is Excellent, Not a Problem
Insight: Not all races will be in both systems - 80% matching between ZwiftPower and Strava is actually great
Impact: Stop trying to match everything; focus on having enough good data for accurate predictions