# Client-only route data

The source order for route metadata is:

1. A deliberately selected route archive from the installed Zwift client's cache.
2. Previously extracted client route XML, for route IDs absent from that archive.

These sources are merged by **unsigned numeric route ID**, never by name.
Primary records win as a whole; missing primary fields are not silently filled
from a different snapshot. The selected archive is explicit: the collector does
not guess which of several cached versions is active.

## Collect without changing Zwift

In PowerShell 7, from a Windows-accessible checkout:

```powershell
$output = Join-Path $env:TEMP ("client-routes-" + [guid]::NewGuid().ToString("N") + ".json")
pwsh.exe -NoProfile -File .\scripts\export_client_routes.ps1 `
  -ClientArchive 'C:\Users\jackc\AppData\Local\Zwift\dlc\692649580f39a8b2.zip' `
  -ExtractedDirectory 'C:\Users\jackc\AppData\Local\Temp\zwift-world-inspection-b46aeaf98b0e4e37b80b680213b17041' `
  -OutputPath $output
$output
```

These paths are examples from the October 2026 investigation, not verification
of the latest or active cache. Select the intended archive and extraction
directory on your machine; cache freshness must be established separately. The extracted
directory is optional. A supplied missing/invalid source fails explicitly.

The collector reads ZIP entries and extracted XML; it does not run the unpacker,
download data, launch Zwift, or modify source files. It rejects duplicate IDs
within a source, invalid measurements, malformed XML/DTDs, and existing output
files. XML `nameHash` signed/unsigned forms are normalized to `u32`.
Each record retains its source kind, path, archive entry/file, SHA-256, and
availability/sport/lap flags.

Keep exports **private and outside Git**, including outside the repository.
No route dataset is bundled. This workflow does not assume redistribution rights
for client assets. Surface is currently a model default (`road`), not an extracted fact.

## Import on the machine running the race finder

Transfer the export as bytes, not through a console that may recode Unicode.
For example, a Windows console using IBM850 corrupts accented names in a plain
text pipeline. Verify matching SHA-256 hashes after a cross-OS transfer.

Back up the race database before applying an import. From the canonical project root:

```bash
cargo run --release --bin import_client_routes -- --input /path/to/client-routes.json --dry-run
cargo run --release --bin import_client_routes -- --input /path/to/client-routes.json
```

Preview may initialize the database/schema but does not import route changes.
The application still needs this explicit import: merely exporting a JSON file
does not change the route database.

Import validates input, writes routes and provenance in one transaction, preserves
route creation timestamps and dependent race history, and skips identical records.
Client records can replace conflicting legacy metadata by route ID; a later
extracted-only import cannot replace an existing cache-derived record. A fresh
database has no seeded routes and stays empty until `import_client_routes` imports
metadata. Retirement does not delete existing routes or race history, and does
not itself import data or change the normal user database.

Community datasets, zwift-offline imports, and website route scraping are retired.
The discovery and description-analysis CLI options are removed; use this client
import workflow. Internal event-description parsing remains supported. Direct SQL is not a
supported route metadata input and can bypass provenance policy.

## Precision and unknown values

Distance and climbing retain fractional precision. SQLite's existing INTEGER-affinity
elevation columns also preserve non-integral values; fresh schemas declare REAL.
Rust reads elevations as optional `f64`, and event climbing is rounded only
for display. Missing lead-in distance is also preserved as unknown; route-only/lap
distance estimates require that distance rather than inventing a zero lead-in.

An explicit zero is distinct from an omitted value. Event, free-ride, and meetup
lead-ins are kept separately. Asset world IDs are not public event course IDs.

For Downtown Dolphin, client climbing is 16.6176948547 m per lap, so 15 laps plus
its zero-climbing event lead-in total approximately 249.2654 m: **249 m displayed**,
not 255 m obtained by rounding each lap to 17 m first.

Client definitions can include running, internal, or unpublished routes. Stored
flags do not prove current public availability. Full checkpoint/elevation profiles
are not imported here, so distance-limited climbing remains a labelled proportional
estimate.

## Validation

```powershell
pwsh.exe -NoProfile -File .\tests\export_client_routes_tests.ps1
```

```bash
cargo test --test client_route_import_tests --test elevation_consistency_tests
```

The importer tests use private databases, including forced transaction failure,
source precedence, precision, Unicode, null/zero semantics, and history preservation.
The separate default CLI integration suite still requires isolated HOME/XDG state.
