# Secure Token Storage Migration Guide

## Overview

The Zwift Race Finder now supports secure storage for OAuth tokens, addressing the security concern of storing sensitive credentials in plain text files.

## Storage Options

### 1. Environment Variables (Recommended for CI/CD)
Most secure for automated environments and server deployments.

**Setup:**
```bash
# Add to your ~/.bashrc or shell profile
export STRAVA_CLIENT_ID="your_client_id"
export STRAVA_CLIENT_SECRET="your_client_secret"
export STRAVA_ACCESS_TOKEN="your_access_token"
export STRAVA_REFRESH_TOKEN="your_refresh_token"
export STRAVA_EXPIRES_AT="1234567890"
export STRAVA_ATHLETE_ID="your_athlete_id"
```

**Benefits:**
- No files on disk
- Easy to manage in CI/CD pipelines
- Can be injected by secret managers

### 2. System Keyring (Recommended for Desktop)
Uses your operating system's secure credential storage.

**Supported Systems:**
- Linux: GNOME Keyring, KWallet
- macOS: Keychain
- Windows: Credential Manager

**Benefits:**
- Encrypted at rest
- OS-level protection
- No plain text files

### 3. File Storage (Backward Compatible)
Original method, now with improved permissions.

**Location:** `strava_config.json` in project directory

**Security Improvements:**
- File permissions set to 600 (owner read/write only)
- Clear warnings about plain text storage
- Migration path to more secure options

## Migration Steps

### From Existing strava_config.json

1. **Check current setup:**
   ```bash
   ls -la strava_config.json
   ```

2. **Choose your storage method:**
   - For development machines: Use system keyring
   - For servers/CI: Use environment variables
   - For testing: Keep file storage

3. **Run secure authentication:**
   ```bash
   ./strava_auth_secure.sh
   ```
   This will:
   - Detect existing credentials
   - Offer migration options
   - Set up your chosen storage method

4. **Update your scripts:**
   ```bash
   # Replace old scripts with secure versions
   ./strava_fetch_activities_secure.sh  # Instead of strava_fetch_activities.sh
   ```

### For New Installations

1. **Run secure setup directly:**
   ```bash
   ./strava_auth_secure.sh
   ```

2. **Choose your preferred storage method**

3. **Follow the prompts to authenticate**

## Security Best Practices

### DO:
- Use environment variables for CI/CD
- Use system keyring for desktop development
- Rotate tokens regularly
- Keep client secret truly secret
- Use `.gitignore` to exclude token files

### DON'T:
- Commit tokens to git (even encrypted ones)
- Share tokens between environments
- Log token values
- Store tokens in world-readable locations
- Use the same tokens for production and development

## Troubleshooting

### "No keyring available"
- Install keyring support: `sudo apt install gnome-keyring` (Debian/Ubuntu)
- Or fall back to environment variables

### "Token refresh failed"
- Check if your app is still authorized in Strava settings
- Verify client ID and secret are correct
- Re-run authentication if needed

### "Environment variables not found"
- Ensure you've sourced your shell profile: `source ~/.bashrc`
- Check variable names are exactly as specified
- Use `printenv | grep STRAVA` to verify

## Future Enhancements

1. **Rust Integration**: The secure storage module is ready for integration into the main Rust application
2. **Token Rotation**: Automatic token refresh before expiration
3. **Multi-Account Support**: Store tokens for multiple Strava accounts
4. **Audit Logging**: Track token usage and access

## Questions?

If you encounter issues with token migration or storage, please:
1. Check this guide first
2. Review error messages carefully
3. Submit an issue with details (excluding any actual token values)