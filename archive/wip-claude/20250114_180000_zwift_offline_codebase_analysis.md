# Zwift-Offline Codebase Analysis and Documentation

*Generated: 2025-01-14 18:00:00*

This document provides a comprehensive analysis of the zwift-offline project codebase, detailing its components, functionality, and architecture for human reference.

## Table of Contents

1. [Project Overview](#project-overview)
2. [Core Components](#core-components)
3. [Functionality Groups](#functionality-groups)
4. [Technical Architecture](#technical-architecture)
5. [File Structure Analysis](#file-structure-analysis)
6. [Integration with Zwift Race Finder](#integration-with-zwift-race-finder)

## Project Overview

**Zwift-offline** (also known as "zoffline") is an open-source implementation of a partial Zwift server that enables running Zwift in offline mode. It intercepts Zwift client requests and provides local server functionality, allowing users to:

- Ride without an internet connection
- Have full control over their fitness data
- Create custom events and routes
- Run bots and ghost riders
- Integrate with external services (Strava, Garmin, etc.)

## Core Components

### 1. Main Server Applications

#### zwift_offline.py
The primary Flask-based web server handling HTTP/HTTPS endpoints:
- **Authentication**: User registration, login, password reset
- **Profile Management**: Create, update, and retrieve user profiles
- **Activity Tracking**: Store and manage ride data
- **External Integration**: Strava, Garmin Connect, Intervals.icu APIs
- **Database Operations**: SQLAlchemy ORM for persistent storage
- **Web Interface**: Serves HTML templates for user interaction

Key endpoints include:
- `/auth/realms/zwift/protocol/openid-connect/token` - Authentication
- `/api/profiles/{id}` - Profile operations
- `/api/activities` - Activity management
- `/relay/worlds` - World/route information

#### standalone.py
The game server component handling real-time communication:
- **TCP Server** (port 3025): Handles Zwift protocol messages
- **UDP Server** (port 3024): Processes real-time telemetry
- **Bot Management**: Controls AI riders and ghosts
- **Multiplayer Support**: Manages multiple concurrent riders
- **CDN Proxy**: Optional content delivery proxy

### 2. Network Infrastructure

#### fake_dns.py
A DNS server that redirects Zwift domains to the local server:
- Intercepts requests to `*.zwift.com`
- Redirects to localhost or configured IP
- Allows Zwift client to connect to local server

#### SSL/TLS Support
- Self-signed certificates for HTTPS endpoints
- Located in `/ssl/` directory
- Enables secure communication with Zwift client

### 3. Data Layer

#### Protocol Buffers (`/protobuf/`)
Google Protocol Buffers matching Zwift's communication protocol:

**Core Messages:**
- `profile.proto` - Player profiles and attributes
- `activity.proto` - Ride/workout data structures
- `world.proto` - World and route definitions
- `udp_node_msgs.proto` - Real-time telemetry
- `tcp_node_msgs.proto` - TCP protocol messages

**Game Features:**
- `events.proto` - Event definitions and scheduling
- `segment_result.proto` - Segment/KOM timing
- `goal.proto` - Achievement tracking
- `variants.proto` - Equipment and customization

#### Database (SQLite)
Storage location: `/storage/zwift-offline.db`
- User accounts and authentication
- Activity history
- Profile data
- Integration tokens

#### File Storage (`/storage/`)
- `profile.bin` - Binary profile data
- Activity files (.fit format)
- Ghost rider data
- User-specific configurations

## Functionality Groups

### 1. Simulation and Physics

**World Simulation:**
- Real-time position updates
- Draft/pack dynamics calculation
- Power-to-speed conversions
- Gradient effects
- Surface type handling

**Bot System:**
- Configurable AI riders (RoboPacers)
- Power output profiles
- Route following logic
- Pack behavior simulation

### 2. User Management and Authentication

**Local Authentication:**
- User registration with email
- Password hashing and storage
- Session management (JWT tokens)
- Optional password reset via email

**Profile Management:**
- Avatar customization
- Equipment selection
- Achievement tracking
- Ride statistics

### 3. External Service Integration

#### Strava Integration
- OAuth2 authentication flow (`/scripts/strava_auth.py`)
- Automatic activity upload
- Activity data formatting
- Token refresh handling

#### Garmin Connect
- Authentication with MFA support (`/scripts/garmin_auth.py`)
- Activity sync
- .fit file upload

#### Discord Bridge (`discord_bot.py`)
- Real-time chat relay
- Rider status updates
- Command interface
- Webhook integration

### 4. Content and Asset Management

**CDN Structure (`/cdn/`):**
- Game assets and updates
- Map schedules
- Streaming content configuration
- Version management

**Data Files (`/data/`):**
- `events.txt` - Event definitions
- `climbs.txt` - Climb categorization
- `start_lines.txt` - Route start positions
- `game_dictionary.txt` - Localization
- `economy_config.txt` - In-game economy

### 5. Utilities and Tools

**Scripts Directory (`/scripts/`):**

**Setup and Configuration:**
- `configure_client.bat` - Windows client configuration
- `launch.bat` - Server launcher for Windows
- `disable_zoffline.bat` - Revert to online mode

**Data Management:**
- `get_profile.py` - Export profile data
- `upload_activity.py` - Manual activity upload
- `find_equip.py` - Equipment ID lookup
- `bot_editor.py` - Configure bot behavior

**Integration Helpers:**
- `strava_auth.py` - Strava OAuth flow
- `garmin_auth.py` - Garmin authentication
- `login_to_json.py` - Convert login data

**Asset Updates:**
- `get_gameassets.py` - Download latest assets
- `get_events.py` - Fetch event schedule
- `gen_schedule.py` - Generate custom schedules

## Technical Architecture

### 1. Network Architecture

```
Zwift Client
    |
    v
fake_dns.py (DNS redirection)
    |
    v
zwift_offline.py (HTTPS/443, HTTP/80)
    |
    v
standalone.py
    ├── TCP Server (3025) - Game protocol
    └── UDP Server (3024) - Telemetry
```

### 2. Data Flow

```
User Input → Zwift Client → Network Layer → Server Components
                                                |
                                                v
                                        Protocol Buffers
                                                |
                                                v
                                        Business Logic
                                                |
                                                v
                                    Database / File Storage
```

### 3. Key Design Patterns

**Modular Architecture:**
- Clear separation of concerns
- Pluggable components
- Protocol abstraction

**Event-Driven:**
- Real-time message processing
- Asynchronous operations
- State management

**Service Integration:**
- OAuth2 flows
- REST API clients
- Webhook handlers

## File Structure Analysis

### Critical Files

**Configuration:**
- `/storage/secret-key.txt` - Server secret key
- `/storage/credentials-key.bin` - Encrypted credentials
- `docker-compose.yml` - Docker deployment config

**Documentation:**
- `README.md` - Setup and usage guide
- `RUNNING_ZWIFT_OFFLINE.md` - Detailed running instructions
- `CHANGELOG` - Version history

**Entry Points:**
- `zwift_offline.py` - Web server main
- `standalone.py` - Game server main
- `fake_dns.py` - DNS server main

### Data Directories

**Static Content:**
- `/cdn/` - Game assets and updates
- `/data/` - Game configuration files
- `/ssl/` - SSL certificates

**Dynamic Storage:**
- `/storage/` - User data and activities
- Database files
- Temporary files

## Integration with Zwift Race Finder

The zwift-offline project is included in zwift-race-finder for several purposes:

### 1. OCR Calibration Testing
- Provides controlled environment for OCR development
- Enables consistent UI state for calibration
- Allows automated testing scenarios

### 2. Telemetry Access
- UDP packet monitoring for ground truth data
- Real-time metrics for validation
- Bot control for reproducible tests

### 3. Development Benefits
- No internet requirement for testing
- Full control over race scenarios
- Ability to pause/resume for debugging
- Custom event creation for edge cases

### 4. Future Integration Possibilities
- Automated race simulation
- Performance prediction validation
- Training plan execution
- Real-time coaching development

## Security Considerations

**Authentication:**
- Password hashing with salt
- JWT token expiration
- Session management

**Network:**
- SSL/TLS encryption
- Certificate validation
- DNS spoofing (intentional)

**Data Privacy:**
- Local storage only
- No cloud dependencies
- User-controlled sharing

## Limitations and Considerations

1. **Partial Implementation**: Not all Zwift features are supported
2. **Protocol Changes**: May break with Zwift updates
3. **Legal**: Users responsible for compliance with Zwift ToS
4. **Performance**: Single-threaded components may limit scalability
5. **Maintenance**: Requires updates when Zwift protocol changes

## Conclusion

Zwift-offline is a sophisticated reverse-engineering project that provides valuable functionality for:
- Offline training
- Development and testing
- Data sovereignty
- Custom experiences

Its modular architecture and comprehensive feature set make it an excellent tool for understanding Zwift's internals and developing related applications like the Zwift Race Finder's OCR capabilities.