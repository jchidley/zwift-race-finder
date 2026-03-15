# Zwift Simulation Tools for Automated Testing

This document lists tools that can simulate Bluetooth/ANT+ cycling devices for automated testing of Zwift applications.

## Device Simulation Tools

### 1. [Gymnasticon](https://github.com/ptx2/gymnasticon) (500+ stars)
- **Language**: JavaScript/Node.js
- **Purpose**: Bridge proprietary bikes (Peloton, etc.) to Zwift
- **Features**:
  - Simulates ANT+ and BLE power/cadence sensors
  - Modifiable for custom power profiles (Cat A/B/C/D riders)
  - Active development and community support
- **Testing Use**: Create repeatable test scenarios with specific power outputs

### 2. [FortiusANT](https://github.com/WouterJD/FortiusANT) (100+ stars)
- **Language**: Python
- **Purpose**: Connect old Tacx trainers to modern cycling apps
- **Features**:
  - Full ANT+ FE-C (Fitness Equipment Control) protocol
  - Grade simulation support
  - Can run headless for automation
  - Detailed logging capabilities
- **Testing Use**: Automated regression testing with grade changes

### 3. [Zwack](https://github.com/paixaop/zwack) (50+ stars)
- **Language**: JavaScript/Node.js
- **Purpose**: Pure Bluetooth LE sensor simulator
- **Features**:
  - Simulates FTMS (Fitness Machine Service)
  - Cycling Power Service
  - Heart Rate Service
  - Keyboard controls: w/s (power), a/d (cadence)
- **Testing Use**: Interactive testing of specific race scenarios

### 4. [openant](https://github.com/Tigge/openant) (300+ stars)
- **Language**: Python
- **Purpose**: ANT+ protocol library with simulators
- **Features**:
  - Complete ANT+ protocol implementation
  - Example device simulators included
  - Power, HR, speed, cadence simulation
  - Well-documented API
- **Testing Use**: Build custom test simulators for edge cases

### 5. [GoldenCheetah](https://github.com/GoldenCheetah/GoldenCheetah) (2000+ stars)
- **Language**: C++
- **Purpose**: Comprehensive cycling analytics platform
- **Features**:
  - ANT+ device simulation code
  - Reference power curve implementations
  - Extensive physiological models
- **Testing Use**: Reference implementation for power calculations

## Related Tools & APIs

### [Sauce4Zwift](https://github.com/SauceLLC/sauce4zwift) (400+ stars)
- **Purpose**: Real-time Zwift data access
- **API Endpoints**:
  - REST: `http://localhost:1080/api`
  - WebSocket: `ws://localhost:1080/api/ws/events`
- **Features**:
  - Live race data, gaps, positions
  - Headless mode for automation
  - Python client examples
- **Testing Use**: Validate predictions against live race data

### [zwift-offline](https://github.com/zoffline/zwift-offline) (912+ stars)
- **Purpose**: Offline Zwift server implementation
- **Features**:
  - Complete server protocol implementation
  - Reveals internal API structure
  - No subscription required for testing
- **Testing Use**: Test without active Zwift subscription

### [zwift-mobile-api](https://github.com/Ogadai/zwift-mobile-api) (109+ stars)
- **Purpose**: JavaScript client for Zwift's mobile API
- **Status**: ⚠️ Now requires Developer API access (restricted)
- **Features**:
  - Protocol documentation
  - Protobuf message decoding examples
- **Testing Use**: Understanding Zwift's data formats

## Testing Applications

### Automated Regression Testing
1. Use device simulators to create repeatable power profiles
2. Simulate different categories (A/B/C/D) with appropriate power outputs
3. Test edge cases: getting dropped, rejoining pack, sprint finishes
4. Validate duration predictions against simulated completions

### Test Scenarios
- **Steady State**: Constant power for entire race
- **Variable Power**: Simulate real race dynamics with surges
- **Getting Dropped**: High power start, then drop to solo pace
- **Sprint Finish**: Steady power with final sprint
- **Equipment Failure**: Power dropouts, sensor disconnections

### Implementation Example
```bash
# Using Zwack for simple testing
git clone https://github.com/paixaop/zwack.git
cd zwack
npm install
node server.js
# Use w/s keys to adjust power, a/d for cadence

# Using openant for automated testing
pip install openant
# Create custom script with specific power profile
```

## Future Integration Plans

1. **Automated Test Suite**
   - Create power profiles for each racing category
   - Run simulated races on all routes
   - Compare predicted vs actual times
   - Build regression test database

2. **Continuous Integration**
   - Nightly automated tests
   - Performance tracking dashboard
   - Alert on prediction accuracy degradation

3. **Machine Learning Dataset**
   - Generate thousands of simulated races
   - Various power profiles and race dynamics
   - Train improved prediction models

## Security & Compliance Note

When using these tools:
- Respect Zwift's Terms of Service
- Use only for testing and development
- Don't use for gaining unfair advantage in races
- Consider impact on other riders if testing in public events