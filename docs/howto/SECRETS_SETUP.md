# Optional credentials

No credentials are needed to fetch public Zwift events. Supplying your current
Racing Score is sufficient for subgroup selection.

The optional ZwiftPower lookup reads these environment variables through
`FullConfig`:

- `ZWIFTPOWER_PROFILE_ID`
- `ZWIFTPOWER_SESSION_ID`

If configured, that lookup can run even when `--zwift-score` is supplied. Keep
credentials out of TOML, command arguments, source, exported logs and Git. Load
only approved variables through the machine's reviewed secret-management process;
this repository does not install a vault or prescribe a credential migration.
Do not use debugging output as a credential export mechanism.

## Browser-based cache refresh

`scripts/refresh_zwiftpower_stats.sh` is a separate optional browser/cache utility,
not part of the supported measured-history importer. It persists authenticated
browser storage under the user cache, which must be treated as sensitive. Review
its current behaviour and explicitly authorize login before using it; do not run
it merely to validate setup or documentation. No refresh is needed for a public
search with an explicit score.

The [history export procedure](ZWIFTPOWER_EXPORT_STEPS.md) uses your own logged-in
profile and keeps exports private. It does not require copying cookies into source
or manufacturing database observations from browser activity.
