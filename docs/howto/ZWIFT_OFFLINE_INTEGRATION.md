# zwift-offline Route Integration — Retired

The zwift-offline route export/import workflow is retired. Its importer, client
adapter, import wrapper, and bundled export data are no longer supported.
Community route datasets and website route scrapers are also retired.

Use [client-only route data](CLIENT_ROUTE_DATA.md): explicitly select a client
cache archive, with extracted client XML only for IDs missing from that archive.
Keep client exports private outside Git; no route dataset is bundled. This does
not establish which cache is latest or active.

Existing database rows and race history are retained. Fresh databases have no
seeded route rows and require an explicit `import_client_routes` import.
The discovery and description-analysis CLI options are removed; internal
event-description parsing remains supported.

Older integration and coverage claims in [project history](../project-history/)
and `archive/` are historical evidence, not executable instructions.
