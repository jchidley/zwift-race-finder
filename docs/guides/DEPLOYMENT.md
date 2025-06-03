# Zwift Race Finder - Deployment Guide

This guide covers deployment and production usage of Zwift Race Finder.

## Production Installation

### Quick Install (Recommended)
```bash
# Clone, build, and install in one step
git clone https://github.com/jchidley/zwift-race-finder.git
cd zwift-race-finder
./install.sh
```

This will:
1. Build the release version with optimizations
2. Install to `~/.local/bin/zwift-race-finder`
3. Show basic usage examples

### Manual Installation
```bash
# Build release version
cargo build --release

# Copy to local bin
cp target/release/zwift-race-finder ~/.local/bin/

# Ensure ~/.local/bin is in PATH
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## First Run

### Basic Usage (No Config Needed)
```bash
# Find races for next 24 hours with default settings
zwift-race-finder

# The tool will show what it found:
# "Found: 91 group rides, 52 races, 33 group workouts, 5 time trials"

# Common searches:
zwift-race-finder -d 30 -t 30    # 20-60 minute races
zwift-race-finder -d 90 -t 30    # 60-120 minute races
zwift-race-finder -e tt           # Time trials only
```

### With Personal Config (Optional)
```bash
# Create config from template
cp config.example.toml ~/.config/zwift-race-finder/config.toml

# Edit with your Zwift Racing Score
editor ~/.config/zwift-race-finder/config.toml

# Tool will auto-load config from this location
zwift-race-finder
```

## Production Features

### Performance
- **API Calls**: Cached for 5 minutes to reduce load
- **Database**: SQLite with indexes for fast queries
- **Binary Size**: ~10MB standalone executable
- **Memory Usage**: <50MB typical
- **Response Time**: <2 seconds for most queries

### Reliability
- **Error Handling**: Graceful degradation on API failures
- **Offline Mode**: Works with cached data when API unavailable
- **Data Persistence**: All data stored in `~/.local/share/zwift-race-finder/`

### Accuracy (16.1% Mean Error)
- **Flat Routes**: ~12% error
- **Rolling Routes**: ~16% error
- **Hilly Routes**: ~20% error
- **Multi-lap Races**: Fixed from 533% to ~16% error

## Monitoring & Maintenance

### Check Database Health
```bash
# Database location
sqlite3 ~/.local/share/zwift-race-finder/races.db "PRAGMA integrity_check;"

# Database size
du -h ~/.local/share/zwift-race-finder/races.db

# Route count
sqlite3 ~/.local/share/zwift-race-finder/races.db "SELECT COUNT(*) FROM routes;"
```

### Update Route Data
```bash
# Apply latest route mappings
cd zwift-race-finder
git pull
./apply_route_mappings.sh

# Check for unknown routes
zwift-race-finder --show-unknown-routes
```

### Backup Data
```bash
# Backup database
cp ~/.local/share/zwift-race-finder/races.db ~/zwift-races-backup-$(date +%Y%m%d).db

# Backup config
cp ~/.config/zwift-race-finder/config.toml ~/zwift-config-backup-$(date +%Y%m%d).toml
```

## Troubleshooting

### No Events Found
1. Check API is accessible: `curl -s https://us-or-rly101.zwift.com/api/public/events | head`
2. Try wider tolerance: `-t 60` for ±60 minutes
3. Try all event types: `-e all`
4. Remember API limit: only ~12 hours of events available

### Wrong Predictions
1. Record actual times: `--record-result "route_id,minutes,event_name"`
2. Update rider stats: `./update_rider_stats.sh 86.0`
3. Check if multi-lap race in `multi_lap_events` table
4. Report via GitHub if consistently wrong

### Performance Issues
1. Check disk space: `df -h ~/.local/share/`
2. Vacuum database: `sqlite3 ~/.local/share/zwift-race-finder/races.db "VACUUM;"`
3. Clear old logs if any exist
4. Rebuild with latest version

## Integration

### Shell Aliases
Add to `~/.bashrc` or `~/.zshrc`:
```bash
# Quick race searches
alias zr='zwift-race-finder'
alias zr30='zwift-race-finder -d 30 -t 30'
alias zr60='zwift-race-finder -d 60 -t 30'
alias zr90='zwift-race-finder -d 90 -t 30'
alias zrtt='zwift-race-finder -e tt'
```

### Cron Jobs
Schedule regular updates:
```bash
# Update route mappings weekly
0 3 * * 0 cd ~/zwift-race-finder && git pull && ./apply_route_mappings.sh

# Backup database monthly
0 2 1 * * cp ~/.local/share/zwift-race-finder/races.db ~/backups/zwift-races-$(date +\%Y\%m).db
```

## Security Best Practices

1. **Never share** your config.toml if it contains personal data
2. **Use environment variables** for any API tokens:
   ```bash
   export ZWIFT_SCORE=195
   zwift-race-finder -s $ZWIFT_SCORE
   ```
3. **Regular updates**: `git pull` for security fixes
4. **Check permissions**: `ls -la ~/.config/zwift-race-finder/`

## Support

- **Issues**: https://github.com/jchidley/zwift-race-finder/issues
- **Feedback**: See FEEDBACK.md
- **Updates**: Watch the GitHub repo for new releases