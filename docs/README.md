# Documentation

## Start here

New to the tool? Follow the **[Tutorial: Find your first race](tutorial/getting-started.md)**.

## How-to guides

Task-oriented instructions for specific goals.

- [How to deploy and update](howto/DEPLOYMENT.md)
- [How to configure settings](howto/CONFIG_MANAGEMENT.md)
- [How to set up secrets](howto/SECRETS_SETUP.md)
- [How to migrate to secure token storage](howto/SECURE_TOKEN_MIGRATION.md)
- [How to import data from Strava](howto/DATA_IMPORT.md)
- [How to export from ZwiftPower](howto/ZWIFTPOWER_EXPORT_STEPS.md)
- [How to integrate with zwift-offline](howto/ZWIFT_OFFLINE_INTEGRATION.md)
- [How to run mutation testing](howto/MUTATION_TESTING_GUIDE.md)
- [How to use golden tests](howto/GOLDEN_TEST_STRATEGY.md)
- [How to validate test data](howto/TEST_DATA_VALIDATION.md)

## Reference

Facts, specs, and lookup tables. No step-by-step instructions.

- [Algorithms](reference/ALGORITHMS.md) — duration estimation model
- [Architecture](reference/ARCHITECTURE.md) — system components and data flow
- [Database](reference/DATABASE.md) — schema, tables, queries
- [Zwift domain](reference/ZWIFT_DOMAIN.md) — event types, categories, route IDs
- [Requirements](reference/REQUIREMENTS.md) — functional and non-functional requirements
- [Physical stats](reference/PHYSICAL_STATS.md) — height, weight, aerodynamics
- [Peloton aerodynamics](reference/PELOTON_AERODYNAMICS.md) — draft physics
- [Test suite](reference/TEST_SUITE_SUMMARY.md) — what's tested and how
- [Integration tests](reference/INTEGRATION_TEST_COVERAGE.md) — API and DB test coverage
- [Security audit](reference/SECURITY_AUDIT.md) — credential handling review
- [Route data extraction](reference/ROUTE_DATA_EXTRACTION.md) — how route data is sourced
- [Simulation tools](reference/SIMULATION_TOOLS.md) — power simulation and testing
- [Refactoring rules](reference/REFACTORING_RULES.md) and [Rust-specific rules](reference/RUST_REFACTORING_RULES.md)
- [Refactoring tools](reference/RUST_REFACTORING_TOOLS.md) and [best practices](reference/RUST_REFACTORING_BEST_PRACTICES.md)
- [Testing strategy](reference/MODERN_TESTING_STRATEGY.md)

## Explanation

Background, rationale, and deep dives. Read away from the keyboard.

### Zwift racing
- [About Zwift racing categories](explanation/CATEGORY_RACING.md)
- [About draft strategies](explanation/DRAFT_STRATEGIES.md)
- [About blob dynamics](explanation/BLOB_DYNAMICS_MASTERY.md) and [blob size science](explanation/BLOB_SIZE_SCIENCE.md)
- [About breakaway strategy](explanation/BREAKAWAY_SIZE_STRATEGY.md)
- [About group size dynamics](explanation/GROUP_SIZE_MATRIX.md)
- [About power optimization](explanation/POWER_OPTIMIZATION.md)
- [About route tactics](explanation/ROUTE_TACTICS.md)
- [About Zwift vs road racing](explanation/ZWIFT_VS_ROAD_RACING.md)
- [About real vs virtual physics](explanation/REAL_VS_VIRTUAL_PHYSICS.md)
- [About Zwift as a game](explanation/ZWIFT_AS_GAME.md)
- [Racing research summary](explanation/RACING_RESEARCH_SUMMARY.md)

### Zwift data and physics
- [About Zwift physics equations](explanation/ZWIFT_PHYSICS_EQUATIONS.md)
- [About Zwift integrations](explanation/ZWIFT_INTEGRATIONS_RESEARCH.md)
- [About route mapping](explanation/ROUTE_MAPPING_RESEARCH.md)
- [About route tracking ideas](explanation/ROUTE_TRACKING_IDEAS.md)
- [About ZwiftHacks techniques](explanation/ZWIFTHACKS_TECHNIQUES.md)

### Development
- [About AI-assisted development](explanation/AI_DEVELOPMENT.md)
- [About testing effectiveness](explanation/COMPREHENSIVE_TESTING_GUIDE.md) — the 0% mutation testing lesson
- [About behavioral preservation](explanation/BEHAVIORAL_PRESERVATION_RESEARCH.md)
- [About software testing research](explanation/SOFTWARE_TESTING_STATE_OF_ART_2025.md)
- [About testing insights](explanation/TESTING_INSIGHTS_SUMMARY.md)
- [About refactoring](explanation/REFACTORING_EXPLAINED.md)
- [About why not 100% coverage](explanation/WHY_NOT_100_PERCENT_COVERAGE.md)

## Project history

- [Accuracy timeline](project-history/ACCURACY_TIMELINE.md)
- [Historical discoveries](project-history/HISTORICAL_DISCOVERIES.md)
- [User feedback](project-history/FEEDBACK.md)
