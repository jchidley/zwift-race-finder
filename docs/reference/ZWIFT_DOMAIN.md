# Event terminology

## Racing Score and subgroups

Racing Score is used to select an event's declared subgroup range. Traditional
letter categories are event labels, not inferred personal speeds. Do not infer
historical scores or modern score bands from old result categories. The selection
and inclusive range rules are in [Algorithms](ALGORITHMS.md#estimation-priority).
For obtaining a current score after a break, see [Returning to racing](../howto/RETURN_TO_RACING.md).

## Routes and geometry

- A route is identified by unsigned numeric `route_id`, not its display name.
- Base lap distance is distinct from event total distance and lead-in.
- Add one lead-in for lap/route-only calculations, not for explicit total distance.
- Unknown climbing is not zero; total climbing is not an ordered gradient profile.
- Subgroup route/distance/laps can differ from event-level metadata.
- A current client route definition does not prove a historical event's geometry.

## Event formats and outcomes

Ordinary races can use qualified personal performance history. Other supported
search types may match if they supply fixed duration; the ordinary-race fit is not
a time-trial or group-ride model. A saved activity can contain racing plus other
riding, so elapsed recording time is not measured race finish time.

Drafting, field strength, pacing, motivation and physical capacity can all affect
race outcomes. The finder uses empirical race performance rather than isolating
these contributions. Coaching/physics simulation is outside this application's
scope. See [data boundaries](../howto/DATA_IMPORT.md).
