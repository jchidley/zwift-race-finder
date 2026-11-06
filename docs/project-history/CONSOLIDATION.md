# Finder and Coach consolidation

## Decision and scope — 2026-11-06

The owner approved one public `zwift-racing` repository built on Finder,
including the previously private Coach history. One workflow owns race selection,
preparation, preserved predictions and recorded-ride review. Fitness Bridge keeps
acquisition, sensor transformations and BLE/ANT+ output; Life Fitness keeps
hardware, firmware and physical control. Private data remains outside Git.

No new workspace, coaching executable, shared library, physiological model or
estimator policy is introduced. The existing package, library, executable and
application-state paths keep their Finder names. Coaching remains agent-assisted.

## Git provenance

- Finder checkpoint: `a55d488` (clean before integration).
- Coach original checkpoint: `b73ebae`.
- Coach reviewed working-tree checkpoint: `b6bdccc`; nine documentation files,
  including recording integration and the explicit-path shared-library procedure.
- Non-squashed, unrelated-history merge: `aead11f`, preserving both original
  parent histories and initially placing Coach at `coach-import/`.
- The organization commit removes that temporary prefix. Use
  `git log --all -- coach-import/` and `git show b6bdccc:<original-path>` to trace
  imported material. Original commit identities are retained, not rewritten.

## Content ownership

| Material | Combined home |
|---|---|
| First review, pacing standards, recording integration | `docs/explanation/` |
| Shared-library review procedure | `docs/howto/SHARED_LIBRARY_REVIEW.md` |
| Unfinished review/planning work | Root `TASKS.md` |
| Current workflow and project instructions | Root README, AGENTS and docs map |
| Previous Coach status and detailed receipts | `COACH_STATUS_PRE_CONSOLIDATION.md` |
| Dated hardware, bridge and research reports | `docs/reference/`, labelled historical |
| Original bridge design | `docs/explanation/BRIDGE_AND_MULTI_RIDER_DESIGN_2026-10-30.md`, byte-preserved |
| Historical diagnostic source and upstream notice | `tools/historical-coach/` |
| Imported project licence notices | `coach-licenses/`; MIT copyright also retained at root |

Original Coach README, startup instructions, task queue and docs index remain
available in its Git history, not competing active project entrypoints.
Historical report links are adjusted to retained destinations; their dated
ownership statements do not supersede the root instructions.

## Paths and remaining lifecycle work

Work is performed from `/home/jack/git/zwift-race-finder` until this Pi session
hands off. The intended next canonical root is `/home/jack/git/zwift-racing`;
start a new Pi session there after the checkout move. Do not change only a shell's
cwd and assume Pi session identity has moved.

Keep frozen private experiment manifests, local Coach evidence, installed private
recorders and `FitnessData/projects/zwift-race-finder/` / `zwift-race-coach/` paths
unchanged. Repository consolidation is not publication of private reviews or
approval to change live acquisition. Future publications must retain exact
artifact/destination approval and separate verification receipts.

Remote publication, rename, old Coach retirement and the canonical checkout move
must be verified explicitly; local integration alone does not establish them.
Cross-project maintained references should be updated from their owning sessions,
not by rewriting historical evidence or startup files across unrelated projects.

## Validation

Validation and publication-audit results are recorded in the root `STATUS.md`.
No personal-model accuracy claim follows from unchanged-code regression checks.
