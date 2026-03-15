# Secrets Setup (direnv + ak)

This project expects a profile ID in the environment:

- `ZWIFTPOWER_PROFILE_ID`

## Prerequisites

1. `direnv` installed and hooked into your shell.
2. `ak` installed from `~/tools/api-keys` with GPG set up.

## One-Time Setup

1. Create `ak` service metadata for proper env var names:
   ```bash
   ak edit zwiftpower-profile-id
   ```
   Set `env_var` to `ZWIFTPOWER_PROFILE_ID`.

2. Store the profile ID:
   ```bash
   ak set zwiftpower-profile-id
   ```

3. Allow direnv in this repo:
   ```bash
   direnv allow
   ```

## Daily Usage

With direnv allowed, the environment variables are loaded automatically when you `cd` into the repo.

## Refresh ZwiftPower Stats (No Stored Session Cookie)

To avoid storing a session cookie, refresh stats through a browser session state file.

```bash
scripts/refresh_zwiftpower_stats.sh
# Or pass profile ID directly:
scripts/refresh_zwiftpower_stats.sh 1106548
```

This will:
- Open a browser to your ZwiftPower profile
- Let you log in once
- Save a Playwright storage state file under your cache directory
- Refresh `~/.cache/zwift-race-finder/user_stats.json` for 24 hours

Delete the storage state file anytime to require a fresh login.

## Bitwarden Migration (One-Off)

If you previously stored the profile ID in Bitwarden, convert once:
```bash
ak set zwiftpower-profile-id "$(bw get item 'Zwift Race Finder' | jq -r '.fields[] | select(.name=="zwiftpower_profile_id") | .value')"
```
