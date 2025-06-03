# ZwiftHacks Techniques Analysis

## Most Valuable Techniques for Zwift Race Finder

### 1. Hidden Event Tags (HIGH VALUE)
**Discovery**: Zwift HQ adds hidden tags to all events that aren't visible in the standard API response but can be used for filtering.

**Implementation Opportunity**: 
- Research these hidden tags in event data
- Use them to improve event categorization and filtering
- Could help identify special event series or recurring races

### 2. URL-Based Event Filtering (HIGH VALUE)
**Discovery**: ZwiftHacks uses URL parameters like `/app/events?filter=your_search_here` for direct linking to filtered views.

**Implementation Opportunity**:
- Add support for filter parameters in our CLI tool
- Enable saving and sharing of specific search configurations
- Could support patterns like: `zwift-race-finder --url-filter "mountain-routes"`

### 3. Route Completion Tracking (MEDIUM VALUE)
**Discovery**: ZwiftHacks tracks which routes users have completed and shows this in the event list.

**Implementation Opportunity**:
- Track which races the user has participated in
- Show new/unridden routes prominently
- Calculate route variety score

### 4. Event Description Parsing (HIGH VALUE - ALREADY IMPLEMENTING)
**Discovery**: ZwiftHacks likely parses event descriptions for additional metadata.

**Current Status**: We've already identified this need and added requirements FER-19.9.1-6

### 5. Direct Zwift Insider Links (MEDIUM VALUE)
**Discovery**: ZwiftHacks provides direct links to Zwift Insider's detailed route descriptions.

**Implementation Opportunity**:
- Add `--route-info` flag to show Zwift Insider URL
- Include in event output for users wanting more details
- Build mapping of route_id to Zwift Insider URLs

### 6. Animated Route Maps (LOW VALUE)
**Discovery**: ZwiftHacks includes animated route visualizations.

**Implementation Opportunity**: 
- Not suitable for CLI tool
- Could be valuable for future web interface

### 7. Personal Sync URLs (MEDIUM VALUE)
**Discovery**: Users can create personal URLs to sync preferences across devices.

**Implementation Opportunity**:
- Generate shareable configuration URLs
- Support importing settings from URL
- Useful for team/club setups

## Recommended Implementation Priority

1. **Immediate**: Hidden event tags research and implementation
2. **Next Sprint**: URL-based filtering support
3. **Future**: Route completion tracking and Zwift Insider integration
4. **Long Term**: Personal sync URLs for configuration sharing

## Attribution Requirements

Per ZwiftHacks terms (https://zwifthacks.com/about/):
- Non-commercial use only
- Must give credit when using their techniques or code
- Cannot use for money-making purposes

Our project is MIT/Apache licensed and non-commercial, so we comply with their terms.