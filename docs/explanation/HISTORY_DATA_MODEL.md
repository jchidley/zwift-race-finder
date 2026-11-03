# Observations, eligibility and predictions

Keep three concerns separate:

1. **Eligibility:** current score and an event's declared subgroup rules.
2. **Observation:** measured finish, date, rider/source identity and event geometry.
3. **Prediction:** an earlier-history estimate for a target event.

Racing Score selects the subgroup/course. Historical score is optional context, not
personal pace and not a prerequisite for storing or fitting a race. Never stamp
a current score onto past results or infer one from category/another ranking.

`race_results` owns measured observations. Preserve exact seconds, rounded-minute
compatibility fields, event distance/laps, optional source type/subgroup, source
IDs and hashes. Gun time is separate from finish time. Unknown is not zero.
Source identity is `(source, source_rider_id, source_result_id)`, not name/date.
Repeated imports are idempotent; conflicts and ambiguous matches must fail visibly.

Route metadata is independent of event geometry: laps, lead-in and partial courses
matter. A current route definition cannot prove an old event's exact geometry.
Training uses qualified single-rider ordinary races on UTC dates strictly earlier
than the prediction cutoff. Whole recording duration is not race finish time.

Schema changes/imports belong in the database boundary, run transactionally and
preserve supported existing data. Reject unsupported dependent schemas rather
than discard them. Synthetic fixtures belong in Git; actual rider/client exports
and source receipts remain private. See [Database](../reference/DATABASE.md),
[history import](../howto/ZWIFTPOWER_EXPORT_STEPS.md) and
[Algorithms](../reference/ALGORITHMS.md) for exact contracts.
