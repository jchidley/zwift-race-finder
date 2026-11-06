# Documentation

Zwift Racing combines race selection, preparation and recorded-ride review.
The existing Finder CLI selects event geometry from Racing Score, estimates personal
race duration and filters events to fit the rider's available time.

## Use the finder

- [Quick Start](../README.md) and [first-run tutorial](tutorial/getting-started.md)
- [Configuration](reference/CONFIGURATION.md) and [optional secret setup](howto/SECRETS_SETUP.md)
- [Install and update](howto/DEPLOYMENT.md)
- [Import client routes](howto/CLIENT_ROUTE_DATA.md)
- [Import measured history](howto/ZWIFTPOWER_EXPORT_STEPS.md)
- [Data boundaries](howto/DATA_IMPORT.md)
- [Returning to racing](howto/RETURN_TO_RACING.md)

## Prepare and review

- [First recorded-ride review](explanation/FIRST_RIDE_REVIEW.md)
- [Pacing, objectives and preserved before/after plans](explanation/PACING_AND_RACE_EXPERIMENTS.md)
- [Race recording integration and external ownership](explanation/BRIDGE_AND_MULTI_RIDER_DESIGN.md)
- [Shared-library review procedure](howto/SHARED_LIBRARY_REVIEW.md)

Review is agent-assisted; there is no maintained coaching executable yet.

## Work on predictions

- [Current status](../STATUS.md) and [open tasks](../TASKS.md)
- [Active algorithms](reference/ALGORITHMS.md)
- [Versioned baseline and recency experiment](project-history/RECENCY_BASELINE_2026-11-03.md)
- [Observation/prediction separation](explanation/HISTORY_DATA_MODEL.md)
- [Shared inputs, local import previews and saved evaluations](howto/SHARED_LIBRARY_WORKFLOWS.md)

## Change and validate the implementation

- [Agent operations](../AGENTS.md)
- [Requirements](reference/REQUIREMENTS.md) and [architecture](reference/ARCHITECTURE.md)
- [Code map and selected examples](code-truth/README.md)
- [Testing procedure](howto/TESTING_GUIDE.md) and [test boundaries](reference/TEST_SUITE_SUMMARY.md)
- [Database](reference/DATABASE.md), [route sources](reference/ROUTE_DATA_EXTRACTION.md)
  and [event terminology](reference/ZWIFT_DOMAIN.md)
- [Optional OCR tooling](../tools/ocr/README.md); screenshot fixtures remain in
  `docs/screenshots/` because OCR checks use them.

## Ownership

`STATUS.md` owns current state; `TASKS.md` owns unfinished work. Reference pages own
stable contracts and algorithms; `docs/code-truth/` directs implementation changes.
The current baseline/experiment record retains the evidence needed for comparison.

Superseded audits, learning logs and research reports are available in Git at
`v0.3.0`, not maintained as competing current guidance. For example,
`git show v0.3.0:PROJECT_WISDOM.md` retrieves the old learning log. This cleanup does
not certify old security findings resolved or adopt historical proposals as current
requirements. `archive/` remains immutable historical evidence. Sensor acquisition, protocol output and physical control remain external.
Coach device reports in `reference/`, the dated original bridge design and
[diagnostic scripts](../tools/historical-coach/README.md) are historical evidence,
not active implementation backlog. See the [consolidation record](project-history/CONSOLIDATION.md)
and [pre-consolidation Coach status](project-history/COACH_STATUS_PRE_CONSOLIDATION.md).
