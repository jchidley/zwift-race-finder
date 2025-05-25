# Security Audit Report - Zwift Race Finder

## Summary
Overall security assessment: **MEDIUM RISK** - Some issues need addressing before public release.

## Findings

### 🔴 HIGH Priority Issues

1. **Personal Data in Multiple Files**
   - **Issue**: ZwiftPower profile ID (1106548) hardcoded in multiple scripts
   - **Files**: `scrape_zwiftpower.sh`, `export_zwiftpower_logged_in.sh`, `src/main.rs`
   - **Fix**: Run `sanitize_personal_data.sh` to replace with placeholders

2. **Session IDs in Code**
   - **Issue**: Session ID hardcoded in `src/main.rs` line 365
   - **Risk**: Could be used to access your ZwiftPower account
   - **Fix**: Remove or use environment variable

3. **Personal Race Data**
   - **Files**: `zwiftpower_results.json`, `zwiftpower_page_structure.json`
   - **Risk**: Contains your complete race history
   - **Fix**: Already in .gitignore, ensure not committed

### 🟡 MEDIUM Priority Issues

1. **Windows Username in Paths**
   - **Issue**: `/mnt/c/Users/YOUR_USERNAME/` in multiple scripts
   - **Files**: Various import scripts
   - **Fix**: Replace with generic path or environment variable

2. **Email Address**
   - **File**: `ZWIFT_API_LOG.md`
   - **Issue**: Contains `your.email@example.com`
   - **Fix**: Remove or replace with example email

3. **Browser Automation Scripts**
   - **Files**: `scrape_zwiftpower.py`, various JS files
   - **Risk**: Could be misused for scraping
   - **Mitigation**: Add usage disclaimer in README

### 🟢 LOW Priority / Good Practices

1. **SQL Injection Protection**
   - **Status**: ✅ GOOD - Uses parameterized queries
   - **Example**: `params![route_id, event_name, event_type]`

2. **API Security**
   - **Status**: ✅ GOOD - Only uses public Zwift API
   - **URL**: `https://us-or-rly101.zwift.com/api/public/events/upcoming`

3. **File Permissions**
   - **Status**: ✅ GOOD - Scripts use proper error handling
   - **Example**: `set -euo pipefail` in bash scripts

4. **Database Location**
   - **Status**: ✅ GOOD - Uses user's local data directory
   - **Path**: `~/.local/share/zwift-race-finder/races.db`

### 🔍 Additional Observations

1. **No Hardcoded Credentials**
   - No API keys, passwords, or tokens found in code
   - ZwiftPower access relies on browser-based extraction

2. **Safe External Dependencies**
   - All Cargo dependencies are well-known, maintained libraries
   - No suspicious or unmaintained dependencies

3. **Local-Only Data Storage**
   - All data stored locally in SQLite
   - No cloud services or external data transmission

## Recommendations

### Before Making Public:

1. **Run Sanitization Script**
   ```bash
   ./sanitize_personal_data.sh
   ```

2. **Verify Clean State**
   ```bash
   # Check no personal data in git
   git grep -i "1106548\|jackc\|rechung"
   
   # Ensure personal files not tracked
   git status --ignored
   ```

3. **Add Security Notice to README**
   ```markdown
   ## Security Notice
   - Never commit your ZwiftPower session IDs or profile IDs
   - Keep your `config.json` file private
   - The browser extraction scripts should only be used on your own profile
   ```

4. **Consider Adding**
   - `.env.example` file for configuration template
   - Rate limiting notice for API usage
   - Clear documentation about data privacy

### Good Security Practices Already in Place:

- ✅ Parameterized SQL queries prevent injection
- ✅ Proper error handling in scripts
- ✅ Local-only data storage
- ✅ No credential storage in code
- ✅ Public API usage only
- ✅ Comprehensive .gitignore file

## Conclusion

After running the sanitization script and addressing the high-priority issues, this repository will be safe to make public. The codebase follows good security practices for a personal project, with proper SQL handling and no credential storage.

## Session 2025-01-05: Repository Sanitization and Secure Configuration

### Problems Solved
- Created comprehensive system to sanitize personal data before making repository public
- Implemented multiple secure configuration options that survive repository updates
- Migrated from JSON to TOML configuration for better readability
- Integrated Bitwarden for secure secret management

### Key Discoveries
- Personal data was scattered across multiple files (profile IDs, session tokens, file paths)
- Configuration needed separation between secrets and non-secret settings
- Users need zero-friction way to restore personal config after sanitization

### Solutions Implemented

1. **Sanitization System**:
   - `sanitize_personal_data.sh` - Replaces all personal identifiers with placeholders
   - `check_secrets.sh` - Standalone security scanner for pre-commit checks
   - `setup_git_hooks.sh` - Pre-commit hooks to prevent accidental secret commits

2. **Secure Configuration Options**:
   - **Bitwarden Integration**: Secrets stored in password manager, config in TOML
   - **Local Secure Directory**: `~/.config/zwift-race-finder/` with restricted permissions
   - **GPG Encrypted**: Passphrase-protected configuration

3. **Configuration Architecture**:
   - Separated secrets (Bitwarden/env vars) from settings (TOML files)
   - Multi-source config loading: local → secure dir → env vars → defaults
   - Smart wrappers that auto-load from preferred source

### Technical Implementation

**Config Loading Priority** (src/config.rs):
```rust
1. ./config.toml (local)
2. ~/.config/zwift-race-finder/config.toml (secure)
3. Environment variables (from Bitwarden)
4. Default values
```

**TOML Configuration Structure**:
```toml
[defaults]
zwift_score = 195
category = "D"

[import]
windows_username = "jackc"

[preferences]
default_duration = 120
default_tolerance = 30
```

### Lessons Learned
- Always separate secrets from configuration
- Provide multiple security options for different user preferences
- Use standard tools (Bitwarden, GPG) rather than custom encryption
- Make the "right thing" (security) the easy thing (one command setup)
- TOML is more user-friendly than JSON for configuration files

---
## Key Commands

```bash
# One-time personal config setup (interactive)
./setup_personal_config.sh

# Bitwarden setup and usage
bw login
export BW_SESSION=$(bw unlock --raw)
./bw_config.sh setup
source <(./bw_config.sh export)

# Security checks before committing
./check_secrets.sh

# Sanitize repository for public release
./sanitize_personal_data.sh

# Install pre-commit hooks
./setup_git_hooks.sh

# Use personal wrapper (auto-loads config)
./zwift-race-finder-personal

# Restore config from secure location
./restore_personal_config.sh

# Decrypt config (if using encryption)
./decrypt_config.sh
```