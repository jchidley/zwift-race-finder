# Project Wisdom - Executive Summary

## Universal Development Insights

### 1. Data Quality > Algorithm Sophistication
Single data error degraded accuracy by 11.2%. The path from 92.8% to 25.7% error wasn't about better algorithms - it was about better data.

### 2. Test Reality, Not Assumptions
Tested estimates against estimates for weeks. Always validate test data comes from actual source of truth, not derived calculations.

### 3. Variance Can Be a Feature
Same conditions yielded 32-86 minute variations. High variance may be inherent to the domain, not a model flaw.

### 4. Binary States Often Dominate
Despite complex physics, outcomes were binary: with pack or dropped. Seek simple dominant patterns before complex models.

### 5. Use Stable Identifiers
Event names changed, route IDs didn't. Always identify and use the most persistent identifier in your data model.

### 6. Zero Means "Look Elsewhere"
APIs returning 0 often signal alternate data locations. Check descriptions, metadata, or related fields.

### 7. Build Fallback Chains
Multiple strategies (route→distance→description→defaults) ensure resilience. Design degradation paths for missing data.

### 8. Plan Before Debugging
When confused, resist random changes. Create systematic investigation plans for efficient problem solving.

### 9. Field Presence Reveals Types
Missing type fields? Use presence patterns (`rangeAccessLabel`) to differentiate data variants.

### 10. Browser DevTools for API Discovery
Official clients reveal API behavior. Check browser network tab before building blind.

### 11. Hierarchical Logs for AI Context
Large logs slow LLMs. Summary/Recent/Archive pattern achieves 13x reduction while preserving history.

### 12. 80% Match Rate is Success
Perfect data integration is impossible. Focus on sufficient quality for your use case.