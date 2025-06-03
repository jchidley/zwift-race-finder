# Bitwarden Integration Setup

This project uses Bitwarden to securely store sensitive credentials.

## Initial Setup

1. **Install Bitwarden CLI:**
   ```bash
   npm install -g @bitwarden/cli
   # or for Rust users:
   cargo install rbw
   ```

2. **Login to Bitwarden:**
   ```bash
   bw login
   export BW_SESSION=$(bw unlock --raw)
   ```

3. **Create the secrets in Bitwarden:**
   ```bash
   ./bw_config.sh setup
   ```

4. **Create your local config.toml:**
   ```bash
   cp config.example.toml config.toml
   # Edit with your non-secret preferences
   ```

## Daily Usage

### Option 1: Export to Environment
```bash
# Load secrets into current shell
source <(./bw_config.sh export)
zwift-race-finder
```

### Option 2: Use Wrapper Script
```bash
# Automatically loads from Bitwarden
./zwift-race-finder-bw
```

### Option 3: Shell Function
Add to your ~/.bashrc:
```bash
zwift-race-finder() {
    source <(~/tools/rust/zwift-race-finder/bw_config.sh export) && \
    command zwift-race-finder "$@"
}
```

## Configuration Files

- **config.toml** - Non-secret configuration (safe to commit)
- **Bitwarden** - Secrets (zwiftpower_profile_id, zwiftpower_session_id)

## Security Benefits

- Secrets never stored in plain text files
- Bitwarden handles encryption and secure storage
- Can sync across devices securely
- No manual entry after initial setup
