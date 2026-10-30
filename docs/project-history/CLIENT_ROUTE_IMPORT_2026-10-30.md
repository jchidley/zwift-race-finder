# Approved normal-database client import — 2026-10-30

The owner explicitly approved backing up the normal database and applying the previously validated 386-route client export.

- Target: `~/.local/share/zwift-race-finder/races.db`.
- Source: `~/.local/share/zwift-race-finder/client-routes-0862ca71d9b4.json`.
- Source SHA-256 verified before import: `0862ca71d9b44aaea32424f270a196446fc7017aa69e1f2d443e55cadf0c8c40`.
- Consistent SQLite backup, created with mode 0600 and verified by `PRAGMA integrity_check`: `~/.local/share/zwift-race-finder/backups/races-before-client-import-20261030T075234091278Z.db`.
- Applied the rebuilt `target/release/import_client_routes` to the explicit normal HOME/XDG data path. It wrote 386 client records; total route count changed from 63 to 387. The one unmatched route was preserved.
- Verified every imported route's measurements, names/world/surface/slug, knowledge flags, optional mode-specific lead-ins, and source provenance against the private export. Floating-point comparisons used 1e-12 tolerance.
- Before/after in-memory row digests confirmed all seven non-route tables, including race history, were unchanged. No personal row values or digests were printed or saved in the repository. Unmatched route values were also unchanged.
- Post-import database integrity is OK. Foreign-key violations remain zero.
- Repeat dry-run: zero writes, 386 unchanged records skipped.
- Updated `STATUS.md` and `TASKS.md`. No credentials, client assets, commits, or publication were involved.

Earlier statements in the October 29 audit that normal state was untouched describe the pre-approval phase and are superseded by this application record.
