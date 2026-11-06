# Use shared inputs with local Finder processing

Use Bash from `/home/jack/git/zwift-race-finder`. Rust builds use stable Cargo;
saved-prediction evaluation uses Node.js. Private inputs and outputs stay outside
Git. The library at `C:\Users\jackc\OneDrive\FitnessData` is readable here at
`/mnt/c/Users/jackc/OneDrive/FitnessData`; other machines select their own path.
Its root `README.md` owns the shared/project layout. `FITNESS_DATA_ROOT` is not
an implemented application setting: use explicit arguments, not a new config layer.

## Ownership and effects

| Workflow | Explicit inputs | Output and effects |
|---|---|---|
| Measured-history import | `--input FILE --database DB --dry-run` | Reads export bytes; previews schema/import in memory. Does not create, migrate or write the selected DB. Omitting `--dry-run` applies changes: not part of this procedure. |
| Prediction production | No maintained JSONL export command in the main CLI | Frozen research probes are run-specific evidence, not a supported public exporter. See below before reusing one. |
| Saved-prediction evaluation | `node tools/analysis/evaluate_predictions.mjs INPUT.jsonl OUTPUT.json` | Reads saved predictions, writes a new mode-0600 report exclusively; refuses overwrite. No API, model execution or DB access. |

Reusable exports belong in `shared/race-results/zwiftpower/<year>/`; activity
recordings belong in `shared/activities/zwift/<year>/`. Shared route exports are
reserved under `shared/routes/`; none was registered in the inspected migration.
Completed Finder evidence belongs in
`projects/zwift-race-finder/investigations/<run-id>/`; database backups belong in
`projects/zwift-race-finder/backups/`, **not** an active database location.
`shared/sessions/` indexes cross-project sources/evidence. `incoming/` is staging,
not authoritative completed evidence.

Keep active DBs, disposable copies, builds, logs and processing under local Linux
state, e.g. `~/.local/share/zwift-race-finder/investigations/<new-run-id>/`.
Treat library files as read-only. Copy selected bytes locally and verify SHA-256
against the session index and migration inventory/receipt before use. Recheck
inputs and frozen evidence afterward. A migration receipt alone does not establish
current hashes, cloud availability, independent backup or source authenticity.
Do not delete retained originals or run frozen scripts in their library directory.

## Preview the copied history on a disposable populated DB

This example uses the copied 5 November session, not the active DB. Build first
if necessary with `cargo build --offline --release --bin import_zwiftpower_history`.
Use an empty environment so inherited ZwiftPower credentials/config cannot leak
into validation. The explicit paths are supported without code changes.

```bash
umask 077
library=/mnt/c/Users/jackc/OneDrive/FitnessData
finder="$library/projects/zwift-race-finder"
work=$(mktemp -d "$HOME/.local/share/zwift-race-finder/investigations/shared-check-XXXXXXXX")
cp -- "$library/shared/race-results/zwiftpower/2026/zwiftpower-history-2026-11-05T16-45-51-851Z.json" "$work/history.json"
cp -- "$finder/backups/races-20261105-before-migration.db" "$work/disposable.db"
sha256sum "$work/history.json" "$work/disposable.db" > "$work/before.sha256"
sqlite3 -readonly "$work/disposable.db" 'PRAGMA integrity_check;'
env -i PATH="$PATH" HOME="$work/home" XDG_DATA_HOME="$work/data" \
  XDG_CONFIG_HOME="$work/config" XDG_CACHE_HOME="$work/cache" \
  ./target/release/import_zwiftpower_history \
  --input "$work/history.json" --database "$work/disposable.db" --dry-run \
  > "$work/import-preview.log"
sha256sum --check "$work/before.sha256"
```

Also hash the library originals before/after and compare copy hashes with inventory
entries; the abbreviated example does not automate that evidence check. Verify
integrity again afterward. Never replace the explicit disposable DB with the active
path. For an independently authorized real import, use the separate
[backup/preview/apply procedure](ZWIFTPOWER_EXPORT_STEPS.md).

## Reproduce a saved evaluation

Continue with the local `work` above. Use the predictions **joined with measured
result**, not the original pre-race JSONL (which has no measured outcome).
The join is post-race evidence and does not turn into a new prospective prediction.

```bash
precision="$finder/investigations/20261105-duration-precision-URErMC7k"
cp -- "$precision/prospective-with-result.jsonl" "$work/predictions.jsonl"
cp -- "$precision/prospective-unrounded.json" "$work/frozen-evaluation.json"
sha256sum "$work/predictions.jsonl" "$work/frozen-evaluation.json" > "$work/evaluation-inputs.sha256"
node tools/analysis/evaluate_predictions.mjs "$work/predictions.jsonl" "$work/evaluation.json" \
  > "$work/evaluation.log"
cmp -- "$work/evaluation.json" "$work/frozen-evaluation.json"
sha256sum --check "$work/evaluation-inputs.sha256"
```

Keep original forecasts, joined results and new evaluations separate. The
[evaluation contract](TESTING_GUIDE.md#evaluate-saved-personal-predictions) describes
raw seconds, availability and overlapping windows. Reproduction establishes
arithmetic/byte consistency, not chronology, source authenticity or general model
accuracy. R1/H1/RW1 remain private experiments; B0 remains active.

## Prediction production and remaining old-path dependencies

The main CLI's inspected help has no prediction-export or explicit DB switch.
Do not redirect a live search into JSONL or add a guessed export flag. A new
maintained production exporter would be separate scoped work, not needed to read
copied predictions or reproduce their evaluation.

The preserved `20261105-prospective-1510-LFdqRapT/probe/src/main.rs` accepts
positional `DATABASE CASES_JSON`, uses `Database::preview`, emits JSONL on stdout,
and asserts B0/current-adapter agreement. Its cases encode the original geometry,
lap choice and chronological targets. Its `probe/Cargo.toml` retains the absolute
`/home/jack/git/zwift-race-finder` dependency. If reusing it for an authorized new
research replay, first copy the complete probe and inputs to local ext4, select
explicit local DB/cases and a fresh output; record code/module hashes and version.
Do not build or redirect output inside OneDrive or overwrite frozen predictions.
This inspection did **not** rerun a predictor or qualify probe portability.

Other inspected frozen dependencies remain deliberately unchanged:

- `20261103-recency-v030/run.sh` pins `CARGO_TARGET_DIR` to the old checkout.
  Its probe uses `Database::open`, not the later in-memory preview, so it needs a
  disposable DB even when given a positional DB path.
- `20261103-recency-v030/validate.sh` writes beside itself, sets HOME/XDG below its
  run directory, and relies on checkout cwd/`./target/release`. Do not run it in
  the shared copy.
- Original `verify_and_summarize.py` uses script-relative files, writes reports
  beside them, checks `Path.home()`'s normal DB and an absolute estimator path.
  Later refresh verifiers also write script-relative reports. They are not the
  maintained explicit-input evaluator.
- Probe Cargo manifests retain absolute checkout dependencies. Frozen provenance,
  SHA256SUMS and migration source mappings retain original local/Downloads paths;
  those are historical identifiers, not permission to rewrite them.
- Current documentation still identifies local `history-imports/`, backups and
  `investigations/` as operational/evidence locations. They remain valid local
  working state; no automatic shared-library discovery/publication is implemented.

This is a bounded audit of the import/evaluation entry points and relevant copied
probes/verifiers, not certification that every historical artifact is portable.

## Verified local run — 6 November 2026

Private durable evidence:
`~/.local/share/zwift-race-finder/investigations/20261106-shared-workflows-wXrdzpNo/`.
`validation.json` records commands, inventory matches and before/after hashes;
`validate.mjs` retains the executable check. Source: `0813baf552984381ca4033c0ca34d038b9ff715d`
plus pre-existing uncommitted duration-precision changes, captured in
`pre-existing.patch`, `code-status-before.txt` and the evaluator snapshot. No
estimator/import/evaluation implementation was changed for this task.

- Offline release importer build passed (crate `0.3.0`); binary SHA-256:
  `7ed8aae6eef370878e7ae5aa1a3cfc90b73af8bd9f89f00e58f2625c7eae56c4`.
  The importer exposes help but no version flag; crate label alone is insufficient
  identity. Node was `v24.21.0`; evaluator SHA-256:
  `c7c75f9ccf9d40afdacdb01840d12f4d95b36d6327abcbd8d7aa70cc584a9808`,
  identical to the copied frozen evaluator.
- Copied backup integrity: `ok`, 196 history rows before/after. Dry run:
  **1 new, 0 enriched, 196 unchanged**. Disposable DB bytes remained unchanged.
- New report exactly matches frozen SHA-256
  `0b6cbd7ed0000373bae910f84aa3152adc5988e8ac76211f08422702a8f3be6b`.
  Exclusive-create overwrite refusal and mode 0600 verified; five evaluator tests
  passed, documentation links/anchors and `git diff --check` passed. No full Rust
  suite or live search was needed for these documentation-only changes. One target, all four
  models supported; this is reproduction, not model qualification.
- All selected inputs match migration inventory; active DB, copied/local inputs,
  all copied Finder evidence files and selected library control files retained
  their pre-check hashes. No library writes, source deletion or normal-state import.
- PowerShell helper failed with WSL `UtilAcceptVsock` error. Mounted-byte reads and
  copies worked; no Windows automation, sync or publication was verified.

## Proposed publication set — not authorized or performed

Propose a **new** destination only:
`FitnessData/projects/zwift-race-finder/investigations/20261106-shared-workflows-wXrdzpNo/`.
Copy exactly these local run files after review/explicit approval:

`PROVENANCE.md`, `validation.json`, `validate.mjs`, `code-head.txt`,
`code-status-before.txt`, `code-inputs.sha256`, `tool-versions.txt`,
`importer-build.log`, `importer-help.log`, `import-preview.log`,
`integrity-before.log`, `integrity-after.log`, `evaluation.json`, `evaluation.log`,
`overwrite-refusal.log`, `evaluate_predictions.mjs`, `evaluator-tests.log`,
`documentation-check.json`, `SHA256SUMS`.

Reference existing shared inputs through `validation.json`; do not duplicate the
export, predictions, frozen report or backup. Exclude disposable DB, isolated
state, binary, pre-existing patch and any credentials. No edits to the old run,
its provenance, migration receipt/inventory or session index are proposed. Retain
local sources. A later publication must verify exact destination bytes and does
not imply cloud-sync completion or an independent backup.
