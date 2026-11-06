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
| Previous Coach status and detailed receipts | FitnessData; [evidence index](../reference/EVIDENCE_LIBRARY.md) |
| Detailed bench, operational and machine inventories | FitnessData; generic research remains in `docs/reference/` |
| Original bridge design | Byte-preserved in the FitnessData archive and original Git history |
| Historical diagnostic source and upstream notice | `tools/historical-coach/` |
| Imported project licence notices | `coach-licenses/`; MIT copyright also retained at root |

Original Coach README, startup instructions, task queue and docs index remain
available in its Git history, not competing active project entrypoints.
Detailed report links now lead to the evidence-library index. The owner clarified
that this is a storage-ownership decision, not a confidentiality concern: original
commits are retained publicly without sanitization or history rewriting.
The private library retains the original six-commit Coach bundle, original tracked
checkpoint and combined reports with exact provenance and verified-copy receipts.

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

GitHub rename, publication and Coach archival are complete; observed checkpoints
and readbacks are in STATUS.md. Coach retirement means superseded/read-only
archival, not deletion. The canonical local checkout move requires a new Pi
session handoff. No compatibility symlink or frozen-evidence rewrite is introduced.
Cross-project maintained references should be updated from their owning sessions,
not by rewriting historical evidence or startup files across unrelated projects.

## Validation

Validation and publication-audit results are recorded in the root `STATUS.md`.
No personal-model accuracy claim follows from unchanged-code regression checks.
