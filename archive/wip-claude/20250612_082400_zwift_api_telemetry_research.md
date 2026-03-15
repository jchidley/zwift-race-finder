# Zwift API Data Availability Research for OCR Calibration

*Research Date: 2025-06-12*

## Executive Summary

This research investigates Zwift's API data availability during races to understand what telemetry data could serve as ground truth for calibrating and validating OCR extraction. The findings reveal limited official API access but several viable unofficial approaches for obtaining real-time and post-race telemetry data.

## Current Codebase Analysis

### Existing API Integrations

The zwift-race-finder codebase currently has:

1. **Official Zwift Events API**: Used in `src/main.rs` for fetching upcoming events
   - Endpoint: `https://us-or-rly101.zwift.com/api/public/events/upcoming`
   - Data: Event metadata, routes, timing, categories
   - Limitation: No real-time telemetry or live race data

2. **ZwiftPower Integration**: Used for fetching rider stats
   - Endpoint: `https://zwiftpower.com/profile.php`
   - Data: Racing Score, category, historical performance
   - Limitation: Requires session authentication, no live race data

3. **Strava Integration**: Partial implementation in secure storage
   - OAuth tokens stored for future use
   - Potential for post-race data extraction

### No Existing Real-time Data Handling

The codebase shows no evidence of:
- UDP packet monitoring
- WebSocket connections
- Real-time telemetry capture
- Live race data processing

## Zwift's Official APIs

### Current Status (2025)

1. **Developer API Access**: Restricted to special developer accounts
   - Contact: developers@zwift.com
   - Not available to hobby developers
   - Transitioned to new model in July 2024

2. **Training Connections API**: For training platforms only
   - Partners: TrainerRoad, JOIN, TriDot
   - Not available to individual developers

3. **Public Events API**: Limited to event listings
   - No telemetry data
   - No live race information
   - No participant data during events

### Rate Limits and Authentication

- Current code handles rate limiting (HTTP 429)
- No authentication required for public events API
- ZwiftPower requires session-based authentication

## Real-time Data Sources

### 1. UDP Packet Monitoring (Port 3022)

**Technical Implementation**:
- Zwift uses UDP port 3022 for telemetry
- Protocol Buffers (protobuf) payload format
- Requires packet capture privileges
- Real-time data during rides/races

**Available Data Fields**:
- Current power (watts)
- Speed (km/h)
- Heart rate (bpm)
- Distance (meters)
- Current gradient (%)
- Cadence (rpm)
- Elapsed time
- Position coordinates (virtual)

**Existing Tools**:
- `@zwfthcks/zwift-packet-monitor` (Node.js)
- `ZwiftPacketMonitor` (C#/.NET)
- `ZwiftTelemetryBrowserSource` (streaming overlay)

**Implementation Requirements**:
- Elevated privileges for packet capture
- libpcap/Npcap installation
- Network adapter access
- Protobuf parsing capabilities

### 2. TCP Port 3023 Monitoring

**Additional Data**:
- Complementary to UDP stream
- Also uses protobuf format
- May contain additional state information

### 3. Zwift Companion App Data

**Limitations**:
- No documented public API
- Official WebSocket endpoints not available
- Requires same WiFi network as Zwift client
- Real-time data not officially exposed

## ZwiftPower Live Data

### Current Status

**Limitations**:
- No public API for live race data
- Previous live monitoring feature discontinued (2022)
- Requires login for any data access
- Post-race results primarily available

**Data Availability**:
- Provisional results immediately after races
- Final results may be delayed for manual review
- Historical race data accessible via web scraping

**Existing Tools**:
- `zwift-scrape` (Python/Selenium) for post-race data
- Screen scraping required for automated access

## Alternative Data Sources

### 1. Strava Integration

**Live Segments API**:
- Requires special licensing from Strava
- Real-time segment comparison data
- GPS coordinate alignment
- Not openly available to all developers

**Post-Race Data**:
- Automatic sync from Zwift to Strava
- Comprehensive activity data including:
  - Power curve analysis
  - Speed/distance streams
  - Elevation profiles
  - Heart rate data
  - GPS coordinates (virtual)

### 2. Direct Sensor Broadcasting

**BLE/ANT+ Sensors**:
- Direct connection to power meters
- Heart rate monitors
- Cadence sensors
- Independent of Zwift's data processing

**Limitations**:
- Requires additional hardware setup
- No gradient/route information
- No race context data

### 3. FIT File Export

**Post-Race Data**:
- Complete activity files
- All telemetry streams
- Zwift-specific data fields
- Available immediately after ride completion

## OCR Calibration Strategy

### Recommended Approach

1. **Real-time Validation** (UDP Packet Monitoring):
   - Implement Rust-based packet capture
   - Parse protobuf telemetry streams
   - Compare with OCR extractions in real-time
   - Build confidence scoring system

2. **Post-Race Ground Truth** (Strava/FIT Files):
   - Automatic export of completed activities
   - Comprehensive data validation
   - Historical accuracy analysis
   - Regression testing data

3. **Hybrid Approach**:
   - Use UDP data for real-time feedback
   - Validate against post-race exports
   - Build statistical models for accuracy prediction

### Technical Implementation Plan

```rust
// Proposed module structure
mod telemetry {
    mod udp_monitor;      // Port 3022 packet capture
    mod protobuf_parser;  // Zwift protobuf decoding
    mod strava_client;    // Post-race data fetching
    mod calibration;      // OCR vs telemetry comparison
}
```

### Data Overlap with OCR Targets

**High Priority** (Direct OCR equivalents):
- Power (watts) - exact match
- Speed (km/h) - exact match  
- Distance (meters/km) - exact match
- Heart rate (bpm) - exact match
- Gradient (%) - exact match

**Medium Priority** (Calculated/derived):
- Lap times - can be derived from telemetry
- Elevation gained - calculated from gradient
- Average speeds - calculated from streams

**Low Priority** (OCR-specific):
- Rider rankings - not available in telemetry
- Race positions - requires race context
- Visual elements - UI-specific data

### Auto-calibration Possibilities

1. **Dynamic Thresholds**:
   - Adjust OCR confidence based on telemetry validation
   - Learn from successful extractions
   - Adapt to different visual conditions

2. **Error Detection**:
   - Flag OCR results that differ significantly from telemetry
   - Automatic retry with different OCR parameters
   - Real-time accuracy feedback

3. **Performance Optimization**:
   - Focus OCR processing on high-confidence regions
   - Skip frames where telemetry shows no changes
   - Optimize for specific data fields based on accuracy

## Security and Legal Considerations

### Terms of Service

- UDP packet monitoring: Gray area, no explicit prohibition
- API access: Restricted to official channels
- Data usage: Personal use likely acceptable
- Redistribution: Potential ToS violations

### Privacy Concerns

- Packet capture affects all network traffic
- Other riders' data may be captured
- Local network only recommended
- No data sharing without consent

## Recommended Next Steps

1. **Phase 1**: Implement UDP packet monitoring
   - Create Rust packet capture module
   - Implement protobuf parsing
   - Basic telemetry extraction

2. **Phase 2**: OCR validation integration
   - Real-time comparison framework
   - Confidence scoring system
   - Error detection and reporting

3. **Phase 3**: Post-race validation
   - Strava API integration
   - FIT file processing
   - Historical accuracy analysis

4. **Phase 4**: Auto-calibration
   - Dynamic threshold adjustment
   - Performance optimization
   - User feedback integration

## Conclusion

While Zwift's official APIs provide limited telemetry access, UDP packet monitoring offers a viable path for real-time data extraction. Combined with post-race validation through Strava/FIT files, this approach can provide comprehensive ground truth data for OCR calibration and validation.

The technical feasibility is high, with existing community tools demonstrating successful implementation. The main challenges are technical (packet capture setup) rather than fundamental limitations.

**Primary Recommendation**: Implement UDP packet monitoring as the core real-time data source, with Strava integration for post-race validation and historical analysis.