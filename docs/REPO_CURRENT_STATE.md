# About Repository Scope and Current Functionality

## Background

This repository started as a Zwift race duration predictor and event filter, then expanded into a broader workspace for data imports, OCR experiments, and research. The result is a functional core tool surrounded by supporting pipelines, experiments, and extensive documentation.

This document summarizes what the repo does today, what’s here, and what looks redundant or archival so you can decide what to keep.

## Core Product (What It Does Now)

**Primary CLI:** `zwift-race-finder`

What it does:
- Fetches upcoming Zwift events.
- Estimates race durations based on route distance/elevation and Racing Score.
- Filters events by target duration and other criteria.
- Displays results in compact or verbose formats.
- Includes route tracking and progress utilities.

Key inputs:
- Zwift Racing Score (auto-fetched from ZwiftPower and cached).
- Route data (from `data/`).
- Optional config (`config.toml` in standard locations).

Core code location:
- `src/` (main CLI, filtering, estimation, config, database).

## Secondary Capabilities (Optional/Supporting)

**Route data import / maintenance**
- `scripts/` and `sql/` include helpers for route imports, mapping fixes, and DB maintenance.
- `zwift-offline` integration exists for importing route data from a local zwift-offline server.

**Strava import pipeline**
- `tools/import/strava/` includes scripts to authenticate, fetch activities, and import into the local DB.
- `strava_config.json` and `strava_zwift_activities.json` are data artifacts for this pipeline.

**OCR experiments**
- A large OCR subsystem exists (Rust + Python) for extracting telemetry and leaderboard data.
- Rust OCR is feature-gated (`--features ocr`) and includes multiple binaries.
- Python OCR experiments live under `tools/ocr/`.

**Research and documentation**
- Extensive research notes on Zwift physics, data mapping, and testing.
- Project history, testing strategies, and architecture docs.

## What’s Here (High-Level Inventory)

**Core source and config**
- `src/`, `Cargo.toml`, `Cargo.lock`, `config.example.toml`

**Data and DB assets**
- `data/`, `sql/`, `match_activities.sql`, `debug_event_tags.json`

**Scripts and tools**
- `scripts/`, `tools/`, `install.sh`, `init_repo.sh`, `setup_git_hooks.sh`

**Docs**
- `docs/` (guides, reference, research, project history)
- `README.md`, `REQUIREMENTS.md`, `PROJECT_WISDOM.md`, `COMPREHENSIVE_TESTING_GUIDE.md`

**Experiments / archives / logs**
- `sessions/`, `project-history/`, `wip-claude/`, `PROJECT_WISDOM_ARCHIVE_*`

**External references / subprojects**
- `zwift-client-reference/`, `zwiftmap-reference/`, `zwift-data-reference/`, `zwift-offline/`

## Redundancy and Cleanup Candidates

These are **not necessarily wrong to keep**, but are likely redundant for the core CLI:

**1) OCR stack (if you don’t use it)**
- `src/ocr_*`, `src/bin/*ocr*`, `ocr-configs/`, `tools/ocr/`
- Substantial size and dependencies; feature-gated but still maintenance cost.

**2) Strava import pipeline (if unused)**
- `tools/import/strava/`, `strava_config.json`, `strava_zwift_activities.json`
- Useful only if you’re validating predictions with Strava data.

**3) Local data artifacts**
- `cycling_activities/`, `coverage-report/`, `lcov.info`, `debug_event_tags.json`
- Often ephemeral outputs rather than source of truth.

**4) Historical logs and archives**
- `sessions/`, `docs/project-history/`, `PROJECT_WISDOM_ARCHIVE_*`
- Valuable for history, not needed for runtime or development.

**5) Reference repos**
- `zwift-client-reference/`, `zwiftmap-reference/`, `zwift-data-reference/`, `zwift-offline/`
- Useful for research/backup; not required for the core CLI.

## Key Dependencies for “Just Use the CLI”

If the goal is only to run the race finder:
- `src/`
- `Cargo.toml`, `Cargo.lock`
- `data/`
- `config.example.toml` (optional but helpful)
- `scripts/` (optional; for imports and maintenance)
- Minimal docs (`README.md`, `docs/guides/SECRETS_SETUP.md`)

Everything else is optional or supporting.

## Alternatives and Tradeoffs

**Keep everything**
- Pros: full history, reproducibility of experiments, rich context.
- Cons: clutter, harder onboarding, more maintenance.

**Split into a “core” repo + “research/labs” repo**
- Pros: clean runtime repo; experiments remain available.
- Cons: more repo management, cross-references to maintain.

**Archive experiments in-place**
- Pros: no repo split; reduce noise by moving to `archive/` or `labs/`.
- Cons: still large; not as clean as a split.

## Suggested Next Steps

1. Decide whether OCR and Strava pipelines are still active.
2. If not, move them under a single `labs/` or `archive/` directory.
3. Keep a small, clear “runtime core” at root for day-to-day usage.
