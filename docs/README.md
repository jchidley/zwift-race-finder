# Documentation

## Start here

New to the tool? Follow the **[Tutorial: Find your first race](tutorial/getting-started.md)**.

## How-to guides

Task-oriented instructions for specific goals.

- [How to deploy and update](howto/DEPLOYMENT.md)
- [How to configure settings](howto/CONFIG_MANAGEMENT.md)
- [How to set up secrets](howto/SECRETS_SETUP.md)
- [How to migrate to secure token storage](howto/SECURE_TOKEN_MIGRATION.md)
- [How to import data from Strava and ZwiftPower](howto/DATA_IMPORT.md)
- [How to export from ZwiftPower](howto/ZWIFTPOWER_EXPORT_STEPS.md)
- [How to integrate with zwift-offline](howto/ZWIFT_OFFLINE_INTEGRATION.md)
- [How to test (mutation testing, golden tests, validation)](howto/TESTING_GUIDE.md)

## Reference

Facts, specs, and lookup tables. No step-by-step instructions.

- [Algorithms](reference/ALGORITHMS.md) — duration estimation model
- [Architecture](reference/ARCHITECTURE.md) — system components and data flow
- [Database](reference/DATABASE.md) — schema, tables, queries
- [Requirements](reference/REQUIREMENTS.md) — functional and non-functional requirements
- [Zwift domain](reference/ZWIFT_DOMAIN.md) — event types, categories, route IDs
- [Test suite](reference/TEST_SUITE_SUMMARY.md) — what's tested and how
- [Integration tests](reference/INTEGRATION_TEST_COVERAGE.md) — API and DB test coverage
- [Security audit](reference/SECURITY_AUDIT.md) — credential handling review
- [Route data extraction](reference/ROUTE_DATA_EXTRACTION.md) — how route data is sourced
- [Simulation tools](reference/SIMULATION_TOOLS.md) — power simulation and testing
- [Refactoring rules](reference/REFACTORING_RULES.md) — contract, mechanics, and Rust patterns for behaviour-preserving changes
- [Physical stats](reference/PHYSICAL_STATS.md) — height, weight, aerodynamics
- [Zwift quick reference](reference/ZWIFT_QUICK_REFERENCE.md) — group size, power, gap decision tables

## Explanation

Background, rationale, and deep dives. Read away from the keyboard.

### Zwift racing
- [Zwift racing tactics](explanation/ZWIFT_RACING_TACTICS.md) — pack dynamics, breakaways, positioning, timing
- [Zwift physics](explanation/ZWIFT_PHYSICS.md) — equations, draft, aerodynamics, real vs virtual
- [Zwift physics equations](explanation/ZWIFT_PHYSICS_EQUATIONS.md) — CdA formula, Crr values, Martin equation

### Development
- [AI-assisted development](explanation/AI_DEVELOPMENT.md) — building software with Claude Code
- [Testing philosophy](explanation/TESTING_PHILOSOPHY.md) — why we test this way, the 0% mutation lesson
- [Refactoring explained](explanation/REFACTORING_EXPLAINED.md) — LLM refactoring research, competence boundaries, and prompting strategies

## Project history

- [Accuracy timeline](project-history/ACCURACY_TIMELINE.md)
- [Historical discoveries](project-history/HISTORICAL_DISCOVERIES.md)
- [User feedback](project-history/FEEDBACK.md)
