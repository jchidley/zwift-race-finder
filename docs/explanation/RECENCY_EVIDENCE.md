# Recency and current form: keep an open mind

## Working position — 2026-11-04

**There is not enough recent evidence to give a strong signal either way about
stronger recency weighting or a separate current-form adjustment.** Keep the
useful B0 (`v0.3.0`) baseline active and retain R1 as an unchanged experiment.
This is a practical decision under uncertainty, not proof that B0's weighting is
optimal or that recency/current-form research has failed.

The expanded comparison reduces the apparent advantage of R1 in the four recent
examples. It does not establish that stronger recency is wrong. Equally, the
four-race improvement does not establish that it is right. Both interpretations
must remain open as new evidence arrives.

## Evidence so far

The original [baseline/experiment record](../project-history/RECENCY_BASELINE_2026-11-03.md)
and frozen snapshot remain unchanged. Subsequent source reconciliation and replays
are separate runs, not replacements for that evidence.

| Retrospective comparison | B0 mean absolute error | R1 mean absolute error |
|---|---:|---:|
| Four recent races, refreshed source inputs | 2:42 | 2:19 |
| Seven supported races, 25 September–4 November 2026 | 2:13 | 2:12 |

Errors above are minutes:seconds, rounded for presentation. Predictions are the
whole-minute durations used by the finder, compared against precise measured
finish seconds. Both models receive identical event geometry and strictly
earlier-UTC-date history. R1's parameters have not changed.

Across the seven races, R1 improves two timing errors, worsens four and ties one.
Both models support all seven and all seven predictions are within five minutes.
Across the existing inclusive windows (20–40, 40–50, 50–70 and 75–105 minutes),
summed false inclusions plus misses change from two to four. These are separate,
sometimes overlapping window decisions, not counts of distinct races. Selection
is mixed: R1 catches a suitable long race but misses a suitable 40–50-minute race
and includes an unsuitable one in that window.

Recent support is sparse: each target has only two to seven eligible earlier
ordinary races within 180 days. The period also contains a race-labelled activity
absent from the fresh supported results export. Free rides and whole-activity
durations are not race-time observations. This is therefore a comparison of seven
supported results, not a complete evaluation of every ride in the period.

These outcomes have already been inspected; they are exploratory diagnostics,
not independent prospective validation. Small differences in this sample do not
establish model superiority or a physiological change.

## Interpretation, not a fitness diagnosis

The owner's visual impression is that performances over the last two years look
similar, recent riding has not yet had enough impact to stand out, and there has
not been a large loss of fitness. That is a plausible working interpretation,
**not a systematic two-year comparison or a measured fitness conclusion**.

Older results may still describe current ability reasonably well; age alone does
not make a performance irrelevant. Recent riding may not yet have changed race
pace enough to distinguish it from course, pack, effort and condition differences.
Nothing in this small comparison requires an explanation involving a large fitness
loss, but race duration alone cannot establish how much fitness has changed.

Keep open the possibilities that longer history remains representative, that new
riding will eventually reveal a meaningful change, or that a specific model or
geometry limitation explains an error. Do not assume either that recent results
must dominate or that they can never improve the estimate. Inconclusive evidence
is a reason to preserve uncertainty, not to close the research question.

## How the decision can change

- Keep B0 active because it is useful and replacement is not justified yet, not
  because its recency policy is proven optimal.
- Preserve R1 without tuning it to these inspected outcomes. Record B0 and R1
  predictions before new races, with identical geometry and earlier history.
- Compare duration error, suitable races shown or missed, unsuitable inclusions
  and unavailable predictions; keep missing source results distinct from model
  unavailability.
- Reconcile missing source-qualified results when available. Keep refreshed-data
  runs separate so data effects are not attributed to a model change.
- If accumulating evidence identifies a concrete limitation, define one separate
  candidate and protocol before testing it. A current-form layer remains an option,
  neither a required next step nor a rejected idea. No arbitrary new sample-size
  threshold or automatic promotion rule is adopted here.

[Status](../../STATUS.md) owns current state; [Tasks](../../TASKS.md) owns next steps.

## Evidence provenance

Private artifacts are retained under
`~/.local/share/zwift-race-finder/investigations/`:

- `20261103-recency-v030`: original frozen protocol and baseline/R1 comparison.
- `20261104-history-refresh-33CKbbXf`: fresh source reconciliation and backup preview.
- `20261104-four-race-refresh-MEiLKwk9`: separate four-race replay; 23 private
  probe tests passed and 16 model/history/target cases matched independent arithmetic.
- `20261104-since-sept25-pxliS4yB`: extended period, coverage gaps and 28 independently
  checked model/history/target cases, reusing the tested probe.

These runs leave the normal database and active estimator unchanged. Refreshed
observations were applied only to private analysis copies. Individual rider data
remain private; source-reported whole-kilometre distances and retrospective client
metadata remain geometry qualifications. This write-up changes no model or data
and makes no new test-suite or CI claim.
