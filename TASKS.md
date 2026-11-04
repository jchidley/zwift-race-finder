# Open Tasks

The current objective is a useful personal race selector plus bounded comparisons
against **`v0.3.0`**. [Status](STATUS.md) owns current results; the
[baseline/experiment record](docs/project-history/RECENCY_BASELINE_2026-11-03.md)
owns the original frozen B0/R1 comparison. The
[recency evidence note](docs/explanation/RECENCY_EVIDENCE.md) records the expanded
comparison and open interpretation: recent evidence is insufficient for a strong
signal either way. R1 stays experimental; retaining B0 does not close research.

## Immediate plan — recent-performance relevance

- [ ] Resolve the remaining source-coverage gap: a race-labelled activity in the
  expanded period is absent from the supported results export. The two previously
  missing later results and EVR geometry are now reconciled in private copies;
  normal history is unchanged. Preserve exact seconds and avoid duplicate results.
- [ ] Record unchanged B0/R1 predictions before new races, using identical geometry
  and earlier history. Accumulate source-qualified outcomes without tuning to the
  seven inspected results. Keep frozen and refreshed-data runs separate; report
  geometry qualifications and missing-source coverage independently of model
  unavailability. Do not treat inconclusive evidence as rejection or validation of
  recency weighting, current-form adjustment or B0's existing policy.
- [ ] Before another recency/current-form candidate, specify its mechanism and
  evaluation protocol. Address a concrete limitation rather than tune inspected
  races to perfection. Preserve cross-course estimates, source/rider boundaries,
  honest sparse support and unknown durations. A separate current-form adjustment
  remains open; useful baseline acceptance does not cancel research.
- [ ] Keep error, duration-window inclusion/exclusion and prediction availability
  visible in comparisons. Do not promote R1 automatically or add a category fallback.

## Correctness and validation

- [ ] Require populated accuracy qualification to distinguish zero usable samples
  from measured success. Keep the legacy category regression separate from personal
  chronological evaluation. Any future acceptance threshold must identify units
  and cohort; older percentage wording remains unresolved, not silently redefined.
- [ ] Resolve explicit CLI duration/tolerance/days values equal to defaults being
  replaced by configuration preferences.
- [ ] Isolate config/cache tests that mutate process-wide cwd/environment. Reduce
  live-service dependence in CLI tests without weakening production-adapter tests.
- [ ] Consider one immutable estimate per event if independent filter/display reads
  cause a demonstrated discrepancy; it is not implemented today.
- [ ] Select the applicable stage distance in multi-stage descriptions rather than
  taking the first matching distance.
- [ ] Resolve repository-wide formatting/Clippy debt without suppressing gates.
- [ ] Review tracked credential-bearing/personal-data artifacts before any publication;
  no credential rotation or Git-history rewriting is authorized by cleanup.

## Component limits outside the immediate model work

- Client-cache freshness is not established by file time; verify selected sources
  when updating the installed game's route data.
- Optional OCR has unresolved parsing/snapshot/model prerequisites and ignored tests.
  Do not replace golden data or swallow failures to claim support. It is not a
  prerequisite for race selection; coaching/sensors/recorder work belongs elsewhere.

Superseded task inventories, 2025 requirements and old audits are retained in Git
at `v0.3.0`, not a parallel active backlog. Their removal from the working tree does
not certify all historical findings fixed or turn untriaged ideas into requirements.
