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

## Boundaries

No bundled route seeds, community imports, website scraping or guessed name-based
mappings. Fresh databases remain empty until explicit client-route import;
existing route rows/history are preserved. Internal event-description distance/lap
parsing is separate from route metadata authority. Updating documentation does not
refresh the installed client cache or write the normal database.
