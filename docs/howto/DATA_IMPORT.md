# Data inputs

The finder has two supported imports:

| Data | Procedure | Effect |
|---|---|---|
| Numeric route metadata | [Client routes](CLIENT_ROUTE_DATA.md) | Selected client cache, extracted XML for missing IDs; preserves history |
| Measured race observations | [ZwiftPower history](ZWIFTPOWER_EXPORT_STEPS.md) | Source-qualified finish seconds, event distance and identity; can change personal predictions |

Back up the SQLite database before imports/schema upgrades. Preview and reconcile
before applying. Client-route preview can initialize/migrate schema; measured-history
preview uses an in-memory copy and leaves the target unchanged. Fresh databases
contain no route seeds. Keep all client/rider exports outside Git.

## Identity and timing

Use numeric route IDs, not names. Preserve unknown versus zero, finish seconds
versus rounded display minutes, event distance/laps versus base route geometry,
and source/rider/result identity. Do not infer historical Racing Score.

A FIT/Strava recording may contain warm-up, cooldown or extra efforts. Its full
duration, moving time, title or distance does not establish race finish or route
identity. Owner-reported finishes can support a qualified analysis but must not be
manufactured into supported source records. Deduplicate exports and recordings.

Strava database ingestion, screenshot/OCR history ingestion, community route
imports and guessed mappings are unsupported. Standalone analysis utilities are
not supported database-update paths. Importing more history changes model inputs;
it does not itself prove better prediction accuracy. See
[Algorithms](../reference/ALGORITHMS.md) and [Database](../reference/DATABASE.md).
