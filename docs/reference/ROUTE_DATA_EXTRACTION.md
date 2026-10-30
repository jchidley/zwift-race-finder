# Route Data Sources

The supported workflow is [client-only route import](../howto/CLIENT_ROUTE_DATA.md).
It reads an explicitly selected installed-client cache archive and previously
extracted client XML for route IDs absent from that archive. The collector does
not guess the active archive, unpack WAD files, download data, or launch Zwift.
Latest/active cache status is not verified by this documentation.

## Metadata contract

- Identity is the unsigned numeric `route_id` (`u32`), never the display name.
  Signed XML hash representations are normalized to unsigned IDs.
- The selected cache record wins as a whole; missing fields are not silently
  filled from another snapshot.
- Route distance, elevation, and lead-in measurements retain fractional
  precision. Unknown values are distinct from explicit zero.
- Imports record source provenance and preserve route creation timestamps and
  dependent race history. Asset world IDs are not public event course IDs.
- Client exports remain private outside Git. No route dataset is bundled, and
  this workflow does not assume redistribution rights for client assets.

## Retired sources

Community route datasets, zwift-offline exports/imports, and website route
scraping are retired. The discovery and description-analysis CLI options are
removed; internal event-description distance/lap parsing remains supported. See the
[zwift-offline retirement note](../howto/ZWIFT_OFFLINE_INTEGRATION.md).

Existing route rows and history are not deleted by this retirement. Fresh
databases contain no seeded routes until explicit `import_client_routes` import.
No normal user database is changed merely by updating these docs.

Earlier extraction investigations and coverage claims in
[project history](../project-history/) and `archive/` are historical evidence,
not the current source contract or recommended commands.
