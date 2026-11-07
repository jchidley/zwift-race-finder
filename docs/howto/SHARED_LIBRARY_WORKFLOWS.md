# Use shared inputs with local Finder processing

Use Bash from the combined checkout. Rust builds use stable Cargo and the saved
prediction evaluator uses Node.js. The FitnessData root README owns directory and
publication conventions. Select explicit paths on this machine; no application
setting named `FITNESS_DATA_ROOT` is implemented. The current library on this
machine is `$HOME/FitnessData` (`/home/jack/FitnessData`), not OneDrive.

The [evidence library index](../reference/EVIDENCE_LIBRARY.md) locates earlier
worked examples, exact private run identities, dependency/path inventories and
validation receipts. They are evidence, not maintained command entrypoints.

## Ownership and effects

| Workflow | Supported interface | Effects |
|---|---|---|
| Measured-history preview | `--input FILE --database DB --dry-run` | Reads export and database; previews in memory without creating, migrating or writing the selected target. Omitting `--dry-run` applies changes and is outside this procedure. |
| Saved-prediction evaluation | `node tools/analysis/evaluate_predictions.mjs INPUT.jsonl OUTPUT.json` | Reads predictions and exclusively creates a mode-0600 report; refuses overwrite. No API, model execution or DB access. |
| Prediction production | No maintained main-CLI JSONL exporter | Frozen probes are run-specific evidence, not a supported public exporter. Do not invent an export flag or redirect a live search as JSONL. |

Each recording host may write live captures into its own local FitnessData library;
optional synchronization is separate from acquisition. The library root's
`SYNCHRONIZATION.md` documents rsync options, session completion and ownership.
These Finder procedures consume completed inputs, not active or interrupted copies.

Keep active databases, disposable copies, builds, caches and intermediates on local
Linux/WSL ext4, outside the data library. Use completed consistent database backups;
never move a live SQLite file into the evidence library. Read finalized library
inputs directly or copy selected inputs locally and verify hashes. Keep original
predictions, joined outcomes and new reports separate. Frozen scripts, manifests and provenance retain original paths.

## Preview against a disposable populated database

Requires an already built importer, `sqlite3` and explicitly approved export and
consistent-backup paths. Build if needed with
`cargo build --release --bin import_zwiftpower_history`. Replace the placeholders
below; the output directory must not be an active application-state directory.

```bash
umask 077
library="$HOME/FitnessData"
history="$library/<approved-history-relative-path>"
backup="$library/<consistent-populated-backup-relative-path>"
mkdir -p "$HOME/.local/share/zwift-race-finder/investigations"
work=$(mktemp -d "$HOME/.local/share/zwift-race-finder/investigations/shared-check-XXXXXXXX")
cp -- "$history" "$work/history.json"
cp -- "$backup" "$work/disposable.db"
sha256sum "$history" "$backup" "$work/history.json" "$work/disposable.db" > "$work/before.sha256"
sqlite3 -readonly "$work/disposable.db" 'PRAGMA integrity_check;'
env -i PATH="$PATH" HOME="$work/home" XDG_DATA_HOME="$work/data" \
  XDG_CONFIG_HOME="$work/config" XDG_CACHE_HOME="$work/cache" \
  ./target/release/import_zwiftpower_history \
  --input "$work/history.json" --database "$work/disposable.db" --dry-run \
  > "$work/import-preview.log"
sha256sum --check "$work/before.sha256"
sqlite3 -readonly "$work/disposable.db" 'PRAGMA integrity_check;'
```

Before this example, compare library hashes with the approved evidence record and
verify each copy matches its source; self-consistent before/after hashes alone do
not establish source identity. A populated route catalogue is needed where the
export references routes. Never substitute the active DB. Independently authorized
real imports use the [backup/preview/apply procedure](ZWIFTPOWER_EXPORT_STEPS.md).

## Reproduce a saved evaluation

Use saved predictions already joined with measured outcomes, not an original
forecast lacking a result. Select explicit input and frozen-report paths, verify
their provenance, then write into a fresh local working directory:

```bash
predictions=/path/to/approved-predictions-with-outcomes.jsonl
frozen=/path/to/approved-frozen-evaluation.json
sha256sum "$predictions" "$frozen" > "$work/evaluation-inputs.sha256"
node tools/analysis/evaluate_predictions.mjs "$predictions" "$work/evaluation.json"
cmp -- "$work/evaluation.json" "$frozen"
sha256sum --check "$work/evaluation-inputs.sha256"
```

Declare `$work` as a fresh local output directory first, as in the preview example.
See the [evaluation contract](TESTING_GUIDE.md#evaluate-saved-personal-predictions)
for raw seconds, unavailability and duration-window decisions. Reproduction checks
arithmetic/bytes, not model validity, source authenticity or predictive chronology.
Experimental models are not promoted by this workflow.

## Finalized evidence and frozen dependencies

Publish only an explicitly approved finalized artifact allowlist into a new
library destination. Include sufficient calculation source, versions, input
references and checks to trace claims. Exclude disposable DBs, caches, credentials,
unrelated patches and duplicated shared originals. Verify copied bytes and retain
local sources; do not overwrite frozen evidence or update session indexes implicitly.

Historical probes may pin checkout/target paths, write beside their source or open
a mutable database. Do not build or run them inside the finalized library. For
separately approved reuse, inspect effects first, copy necessary source to local
ext4 and use explicit disposable inputs and fresh outputs. Keep frozen provenance unchanged.
The library is local; OneDrive synchronization and pinning are no longer part of
this workflow. Local readability does not establish independent backup, restore
coverage or portability of every historical artifact.
