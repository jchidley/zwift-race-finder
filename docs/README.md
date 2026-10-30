# Documentation

Use this page to choose the document that matches your task. Current implementation facts come from code and the commit-scoped [code-truth map](code-truth/); current work belongs in [`STATUS.md`](../STATUS.md) and [`TASKS.md`](../TASKS.md).

## Start here

New to the tool? Follow the **[Tutorial: Find your first race](tutorial/getting-started.md)**.

## How-to guides

Task-oriented instructions for a specific result:

- [Return to Zwift racing](howto/RETURN_TO_RACING.md)
- [Deploy and update](howto/DEPLOYMENT.md)
- [Configure settings](howto/CONFIG_MANAGEMENT.md)
- [Set up secrets](howto/SECRETS_SETUP.md)
- [Migrate to secure token storage](howto/SECURE_TOKEN_MIGRATION.md)
- [Import client-only route data](howto/CLIENT_ROUTE_DATA.md)
- [Import data from Strava and ZwiftPower](howto/DATA_IMPORT.md)
- [Export from ZwiftPower](howto/ZWIFTPOWER_EXPORT_STEPS.md)
- [Retired zwift-offline integration](howto/ZWIFT_OFFLINE_INTEGRATION.md)
- [Test the project](howto/TESTING_GUIDE.md)

## Reference

Current facts and lookup material:

- [Requirements](reference/REQUIREMENTS.md) — current product and engineering contract
- [Algorithms](reference/ALGORITHMS.md) — duration model and fallback priority
- [Architecture](reference/ARCHITECTURE.md) — stable component overview
- [Database](reference/DATABASE.md) — schema and local storage
- [Configuration](reference/CONFIGURATION.md) — fields, defaults, paths, and precedence
- [Zwift domain](reference/ZWIFT_DOMAIN.md) — event types, categories, and route IDs
- [Test suite](reference/TEST_SUITE_SUMMARY.md) — test targets, boundaries, and gaps
- [Route data extraction](reference/ROUTE_DATA_EXTRACTION.md) — route data sources
- Coaching and simulation notes now live in the sibling **zwift-race-coach** project: [local ecosystem references](../../zwift-race-coach/docs/reference/ZWIFT_ECOSYSTEM.md) and [simulation-tool research](../../zwift-race-coach/docs/reference/SIMULATION_TOOLS.md). These are local-workspace links; client route extraction/import stays here.
- [Refactoring rules](reference/REFACTORING_RULES.md) — behaviour-preserving mechanics
- [Physical stats](reference/PHYSICAL_STATS.md) — Zwift physics inputs versus estimator inputs
- [Zwift quick reference](reference/ZWIFT_QUICK_REFERENCE.md) — in-race decision tables
- [Pi release audit](reference/PI_RELEASE_AUDIT.md) — Pi 0.10.0–0.85.1 dispositions and evidence

## Code-truth snapshot

Use these pages to scope implementation work and catch misplaced changes:

- [Repository map and reading order](code-truth/README.md)
- [Implementation architecture](code-truth/ARCHITECTURE.md)
- [Implementation patterns](code-truth/PATTERNS.md)
- [Implementation decisions and open questions](code-truth/DECISIONS.md)

## Explanation

Background and rationale that can be read away from the keyboard:

- [Zwift racing tactics](explanation/ZWIFT_RACING_TACTICS.md)
- [Zwift physics](explanation/ZWIFT_PHYSICS.md)
- [Zwift physics equations](explanation/ZWIFT_PHYSICS_EQUATIONS.md)
- [AI-assisted development](explanation/AI_DEVELOPMENT.md)
- [Testing philosophy](explanation/TESTING_PHILOSOPHY.md)
- [Refactoring explained](explanation/REFACTORING_EXPLAINED.md)

## Project history

These records explain prior measurements and decisions but are not current operational instructions:

- [Test audit — 2026-10-29](project-history/TEST_AUDIT_2026-10-29.md) — executed checks, build blockers, and no-op cleanup
- [Accuracy timeline](project-history/ACCURACY_TIMELINE.md)
- [Historical discoveries](project-history/HISTORICAL_DISCOVERIES.md)
- [User feedback](project-history/FEEDBACK.md)
- [2025 requirements record](project-history/REQUIREMENTS_2025.md)
- [2025 security audit](project-history/SECURITY_AUDIT_2025.md)
- [2025 UOM integration-test review](project-history/UOM_INTEGRATION_TEST_COVERAGE_2025.md)

`PROJECT_WISDOM.md` is the dated learning log. `archive/` contains older evidence and must not be edited.

## Documentation governance

- Update `STATUS.md` for current focus and verified blockers.
- Update `TASKS.md` for unresolved work.
- Update reference pages when stable behaviour or interfaces change.
- Refresh `docs/code-truth/` after implementation changes that alter system shape or decisions.
- Preserve dated reviews as history instead of presenting them as current reference.
- Keep `AGENTS.md` operational and concise; do not duplicate the engineering backlog there.

## Performance-history redesign

- [History data model](explanation/HISTORY_DATA_MODEL.md): eligibility, observations and predictions are separate; historical scores are optional.
- [Measured ZwiftPower history](howto/ZWIFTPOWER_EXPORT_STEPS.md): private export, backup, read-only preview, migration and idempotent import.
