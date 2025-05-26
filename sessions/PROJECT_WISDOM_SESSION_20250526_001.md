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

### 2025-05-26: Zwift API Returns Zero Distance for Races
Insight: The Zwift API returns `distance_in_meters: 0.0` for most race events, breaking duration estimation
Impact: This explains why "no races found" - we were filtering out all races with unknown routes because we couldn't estimate their duration without distance data

### 2025-05-26: Fallback Estimation Chain is Critical
Insight: Need multiple fallback strategies: route_id → distance → event name → subgroups
Impact: Makes the tool resilient to incomplete API data - if one method fails, try the next

### 2025-05-26: Default Duration Range Too Narrow
Insight: Default 90-150 minute range misses most races (crits are 40min, Three Village Loop is 77min)
Impact: Users need guidance on using tolerance parameter for better results (-t 60 catches most races)

### 2025-05-26: Stop and Plan When Confused
Insight: When a "fix" stops working mysteriously, resist the urge to add more debug prints - create a systematic investigation plan first
Impact: Saves time by avoiding random changes and ensures thorough understanding of the problem

### 2025-05-26: Racing Score vs Traditional Categories Discovery
Insight: Zwift has two mutually exclusive event systems - traditional A/B/C/D categories and Racing Score ranges (0-650)
Impact: Our filtering logic was excluding all Racing Score events because they have distanceInMeters: 0 in the API

### 2025-05-26: Distance Data Location for Racing Score Events
Insight: Racing Score events return 0 for distance in API but embed it in description text ("Distance: 23.5 km")
Impact: We need to parse description text for these events instead of relying on distanceInMeters field

### 2025-05-26: Route ID Without Route Data
Insight: Events have routeId (e.g., 3379779247) but no route details in API response - webpage must have separate route database
Impact: Our local route database strategy is correct; we just need to map more route IDs

### 2025-05-26: Finding Route Data via WhatsonZwift Search
Insight: Use site search `site:https://whatsonzwift.com [Event Name]` to find route details when mapping unknown routes
Impact: Reliable method for finding route distance/elevation data - e.g., found Three Village Loop details this way

### 2025-05-26: API Design Patterns - Zero Means "Look Elsewhere"
Insight: Some APIs use 0 or null values as signals to check other fields - not actual zero values
Impact: Changed our filtering logic to accept 0 distance and check other data sources (description, route DB)

### 2025-05-26: Multiple Event Type Detection via Field Presence
Insight: Presence of specific fields (like rangeAccessLabel) can identify event variants when type field is missing
Impact: Successfully differentiated Racing Score from Traditional events, enabling proper handling of each

### 2025-05-26: Browser DevTools for API Reverse Engineering
Insight: When documentation is lacking, browser developer tools can reveal API behavior and data structures
Impact: Discovered the Racing Score event pattern and API endpoint structure in minutes vs hours of guessing

### 2025-05-26: Hierarchical Log Management for LLM Context Efficiency
Insight: Large log files (66KB+) slow LLM loading - hierarchical structure (Summary/Recent/Archives) reduces context to <5KB
Impact: 13x reduction in loaded context while preserving all historical data - pattern applicable to any growing log file