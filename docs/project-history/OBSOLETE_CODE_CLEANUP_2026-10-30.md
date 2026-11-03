# Obsolete-code cleanup — 2026-10-30

## Publication boundary

The owner-approved v0.2.0 checkpoint `3e13a78` is pushed to `origin/main`.
This subsequent cleanup is local and uncommitted at verification time. The two
previously tracked personal activity exports were privately backed up, untracked
and ignored at the checkpoint; their older Git history was not rewritten.
`archive/` and private data exports were not edited by this cleanup.

## Changes

- Retired legacy ZwiftPower and Strava database importers, generated matching SQL,
  profile extractor and physical-stat updater. They conflicted with measured,
  source-qualified history. Retained Strava auth/fetch/analyzers and the independent
  event-metrics extractor; neither is a supported history importer.
- Removed unused physical-stat, display and import-path configuration, unused
  error/DB accessors, always-zero quality output, the unused lap-name parser,
  redundant elevation wrappers/constants and scratch coverage/test files.
- Fresh databases no longer create `rider_stats`. Existing rider, completion and
  discovery tables remain untouched. A synthetic test checks both non-creation
  and preservation. History migration/import and score-independent queries remain.
- Shared API/config/database implementations through the library, removing
  duplicate compilation and obsolete dead-code allowances. The production API
  still uses its fixed endpoint and 30-second timeout.
- Replaced misleading HTTP tests with 14 tests of the real typed adapter and tag
  helpers. Removed arithmetic-only/tautological tests; added legacy configuration
  compatibility and example checks. CLI tests use the built binary with isolated
  HOME/XDG and no inherited secrets, but some still use the live public API.
- Removed unused `keyring`, `rten-imageproc` direct dependency and reqwest blocking
  feature. `rstest` remains because the replacement API tests use it.
- Retired two broken missing-module OCR wrappers and their task/console entries;
  removed direct EasyOCR/click dependencies and refreshed `tools/ocr/uv.lock`.
  Transitive PyTorch/CUDA entries disappeared; no installed packages were removed.
  Retained Groq and supported screenshot/calibration tools.
- Updated configuration, database, architecture, import, OCR and test references.
  Legacy inert TOML fields are accepted and ignored, not migrated into new settings.

## Verification

Tests used isolated HOME/XDG, an empty inherited environment, explicit Cargo and
Rustup homes, `INSTA_UPDATE=no`, and one test thread. No snapshots were regenerated.

| Check | Result |
|---|---|
| `cargo test --locked -- --test-threads=1` | **170 passed, 7 ignored** |
| Release build: main CLI and both supported importers | Passed |
| `cargo test --locked --all-features --no-run` | Passed; existing OCR thread-result warning remains |
| History exporter synthetic checks | Valid input plus six rejection cases passed |
| Private-copy accuracy regression | **13.3% MAPE before and after**, six scored records; 188 unknown-score observations explicitly excluded |
| Built CLI live smoke test at score 210, 50 ± 10 minutes | 18 compact/verbose rows agree; every displayed duration is within the window |
| Six retired flags | Rejected before state creation |
| Fresh schema | No rider/completion/discovery tables |
| Current history importer dry-run on copied database | 194 unchanged, zero new/enriched records |
| Normal database SHA-256 | Unchanged |
| OCR lockfile | Offline consistency check passed after metadata refresh |
| Scoped script syntax, Python AST, TOML and documentation links | Passed |
| Diff whitespace | Passed |

The test count fell from the checkpoint's 186 because duplicate module tests and
non-behavioural tests were removed, not because failures were ignored. Current
history parser and migration/import tests remain in the passing suite.

A fresh-context read-only review (`openai/gpt-6-luna`, requested and used) found no
issues in the scoped production Rust diff. It reviewed code, not independently
rerun builds or private data. The parent independently ran the checks above.
Local logs are under the directory recorded by `/tmp/zwift-cleanup-current`;
temporary evidence is not a durable or public test artifact.

## Limits and deferred work

- Production estimator mathematics is unchanged; six-record MAPE is not evidence
  for a new or improved model. History import does not personalize predictions.
- OCR compilation is not runtime validation. Existing OCR model/image/fuzzy-name
  issues, the ignored parallel thread-scope result, and an existing Python invalid-escape warning are not fixed here.
- The retained browser event-metrics extractor was syntax-checked, not validated
  against the current site. Retired scripts were not executed.
- Known CLI eligibility/subgroup, explicit-default/config precedence, extreme
  numeric input and invalid event-type issues remain separate behaviour work.
- No broad lint remediation, external-consumer compatibility guarantee, full
  OCR runtime suite, or GitHub CI validation is claimed for this local cleanup.
- Public unused helpers/fields were removed deliberately; downstream Rust users
  may need adaptation. Existing stored database records are preserved.
