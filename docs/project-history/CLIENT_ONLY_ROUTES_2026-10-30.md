# Client-only route-data retirement — 2026-10-30

## Authorization and boundary

The owner approved retiring old bundled route datasets, misleading automatic seeds, and community/legacy route-import workflows. Existing database rows, race history, personal records, backups, and historical archive material were to remain untouched. This is a source/workflow cleanup, not a purge of the normal database or a new redistribution licence for client assets.

## Implemented

- Removed the ten hardcoded production route seeds. Fresh databases create schema only; reopening an empty database does not reseed it. Normal searches warn when no route metadata is installed and direct the user to the client collector/importer. Event-provided and heuristic estimates remain available; the duration formula is unchanged.
- Removed the bundled zwift-offline export and pinned community reference, reference generator, old import binary/library client/wrapper, and website scraper/import scripts. Removed obsolete executable importer artifacts from local debug/release output as well as their source/Cargo target.
- Retired website discovery: the existing CLI flag returns a clear error before opening a database or fetching websites. Retained pure event-description parsing and its tests.
- Removed guessed/manual route-definition, alias and result-mapping recipes and their wrappers. These included placeholder IDs, mismatched route identities, and approximate/reversed-route aliases. Nothing was applied to race results. Existing database aliases and compatibility lookup remain; cleanup does not certify those aliases.
- Supported ingestion now uses only the explicitly selected client archive and extracted client XML for missing IDs. Whole-record precedence, numeric IDs, fractional precision, null/zero, atomic import/provenance, and history preservation remain.
- Replaced seed-dependent tests with explicit synthetic inputs. Removed tests exclusively exercising the retired importer/scraper; retained migration and history coverage in client-import/database tests. Historical arithmetic snapshot fixtures are explicitly labelled non-authoritative rather than regenerated.
- Updated startup instructions, user guides, code-truth, database and test references. Added an ignore pattern for conventional client-export filenames; exports must still be kept outside the checkout. No client export was copied into Git.

The remaining multi-lap SQL and personal-result import tools are not automatically run and are not supported sources of route metadata. Their historical-result semantics need separate review before use. Existing research submodules and historical evidence are not part of the runtime ingestion path and were not deleted.

## Validation

All Rust runs used cleared environment, temporary HOME/XDG state, explicit existing Cargo/Rustup homes, `INSTA_UPDATE=no`, and one test thread. Final full testing used fresh post-retirement application directories, not the pre-change regression database.

| Check | Result |
|---|---|
| Default-feature suite | **182 passed, 0 failed, 7 ignored** |
| Release CLI and client importer | Built successfully |
| All-feature tests | Compiled successfully; OCR runtime failures were not rerun |
| Accuracy regression before / after | Both reported no mapped races; **MAE unavailable** |
| Real private client export into fresh isolated DB | **386 routes and 386 provenance rows**, no seed extras, no race results |
| Repeat real import preview | **0 writes**, 386 unchanged/skipped |
| Isolated imported database | Integrity OK, no foreign-key violations |
| New regressions | Empty preview, explicit import, preserved unmatched row, no reseeding after deletion, retired discovery fails without creating DB |

The suite count decreased from 196 because 16 checks exclusively covered removed website/importer paths; two new lifecycle/retirement tests were added. Existing tests were also corrected to supply their own synthetic routes. No snapshot baselines were regenerated. The first fresh-state run exposed two remaining seed assumptions; those were fixed and the full suite rerun successfully.

Collector PowerShell code was not changed; its earlier 90-assertion result is historical evidence, not a rerun in this task. The Python protection tests were removed with the importer they tested, not counted as current coverage.

Local logs and recovery copy: `/tmp/zwift-client-only-VsQrBf/`. The mode-0600 `retired-route-inputs.tar` preserves removed source/data from the dirty checkout; it is not a committed archive. Logs include `accuracy-before.log`, `accuracy-after.log`, `tests-final.log`, `release.log`, `all-feature-build.log`, and `real-import.log`.

Final documentation checks found 102 local link targets across 21 touched/current records, all present; `git diff --check` passed. A fresh standalone same-family read-only review found no evidenced regression within the retirement scope. It did not run tests or review unrelated earlier changes, and is not cross-family verification.

## Cache currency investigation — inconclusive

Read-only Windows inspection reconfirmed the selected archive:

- Cache filename: `692649580f39a8b2.zip`.
- SHA-256: `1d96e52198472351b2e519a42972f51461f245f924a1c1624501b63ef62a24ad`, unchanged from the verified export source.
- Cache timestamp: 2026-10-22T15:14:08Z. Other older cache archives remain installed. File time alone is not active-source evidence.
- Installed version pointer: `Zwift_ver_cur.167761.xml`, reporting **1.124.0 (167761)** and `Zwift_1.0.167761_9711551b_manifest.xml`.
- A broader read hit a WSL interoperability timeout; a focused retry succeeded. Searching that installed manifest for the selected hash, DLC, and route XML references returned no matches. It does not establish which cached route archive the running client uses.

Therefore this work **does not claim the export is the latest or active route snapshot**, nor that every client definition is a publicly available cycling race route. No game, launcher, downloader, container, or driver was started. Future refreshes must deliberately select and verify a source; do not silently select the newest file timestamp.

## Preserved state and remaining work

No normal database writes, personal-data changes, archive edits, credential operations, commits, or publication were performed. The earlier approved 386-record normal-database import remains the applicable application record: [client import audit](CLIENT_ROUTE_IMPORT_2026-10-30.md).

Active-cache identification, useful nonempty accuracy benchmarks, subgroup/stage parsing, existing OCR failures, and existing lint/formatting debt remain separate work. See [current status](../../STATUS.md), [tasks](../../TASKS.md), and [client route workflow](../howto/CLIENT_ROUTE_DATA.md).
