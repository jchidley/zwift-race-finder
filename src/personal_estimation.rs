use crate::database::{Database, HistoryObservation, RouteData};
use chrono::{DateTime, NaiveDate, Utc};
use std::fmt;

const MIN_HISTORY: usize = 12;
const H_GRID: [f64; 7] = [0.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersonalBasis {
    SameRoute,
    DistanceAscent,
}

impl fmt::Display for PersonalBasis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::SameRoute => "same-route comparable-distance pace (approximate distance scaling)",
            Self::DistanceAscent => "global distance + ascent",
        })
    }
}

#[derive(Debug, Clone)]
pub struct PersonalEstimate {
    pub minutes: f64,
    pub basis: PersonalBasis,
    pub sample_count: usize,
    pub typical_error_minutes: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PersonalFallback {
    UnsupportedFormat,
    InsufficientHistory { available: usize, required: usize },
    MissingGeometry,
    MixedRiders,
    HistoryUnavailable(String),
}

impl fmt::Display for PersonalFallback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedFormat => f.write_str("unsupported format (history supports ordinary RACE only)"),
            Self::InsufficientHistory { available, required } => write!(
                f, "absent or insufficient usable earlier ordinary-race history ({available}/{required})"
            ),
            Self::MissingGeometry => f.write_str(
                "missing distance/ascent geometry for global history fit and no supported same-route match"
            ),
            Self::MixedRiders => f.write_str("history contains multiple riders; personal fitting requires one rider"),
            Self::HistoryUnavailable(error) => write!(f, "history database unavailable: {error}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrainingRace {
    pub day: NaiveDate,
    pub route_id: Option<u32>,
    pub distance_km: f64,
    pub ascent_m: Option<f64>,
    pub measured_seconds: f64,
}

#[derive(Debug, Clone)]
pub struct PersonalTarget {
    pub event_day: NaiveDate,
    pub route_id: Option<u32>,
    pub distance_km: f64,
    pub ascent_m: Option<f64>,
}

fn positive(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

fn known_gain(value: Option<f64>) -> Option<f64> {
    value.filter(|gain| gain.is_finite() && *gain >= 0.0)
}

pub fn is_ordinary_race(explicit: Option<&str>, payload: Option<&str>) -> bool {
    if let Some(kind) = explicit.filter(|kind| !kind.trim().is_empty()) {
        return matches!(kind.trim(), "RACE" | "TYPE_RACE");
    }
    let Some(payload) =
        payload.and_then(|json| serde_json::from_str::<serde_json::Value>(json).ok())
    else {
        return false;
    };
    let Some(tokens) = payload.get("f_t").and_then(serde_json::Value::as_str) else {
        return false;
    };
    let tokens: Vec<_> = tokens.split_whitespace().collect();
    !tokens.is_empty() && tokens.iter().all(|token| *token == "TYPE_RACE")
}

fn weighted_median(mut pairs: Vec<(f64, f64)>) -> f64 {
    pairs.sort_by(|a, b| a.0.total_cmp(&b.0));
    let half = pairs.iter().map(|(_, weight)| weight).sum::<f64>() / 2.0;
    let mut cumulative = 0.0;
    for (value, weight) in &pairs {
        cumulative += weight;
        if cumulative >= half {
            return *value;
        }
    }
    pairs.last().expect("nonempty supported history").0
}

fn residual_error(rows: &[(&TrainingRace, f64)], predict: impl Fn(&TrainingRace) -> f64) -> f64 {
    rows.iter()
        .map(|(row, weight)| weight * (row.measured_seconds / 60.0 - predict(row)).abs())
        .sum::<f64>()
        / rows.iter().map(|(_, weight)| weight).sum::<f64>()
}

pub fn predict(
    target: &PersonalTarget,
    history: &[TrainingRace],
    today: NaiveDate,
) -> Result<PersonalEstimate, PersonalFallback> {
    if !positive(target.distance_km) {
        return Err(PersonalFallback::MissingGeometry);
    }
    let cutoff = today.min(target.event_day);
    let rows: Vec<_> = history
        .iter()
        .filter(|row| {
            row.day < cutoff && positive(row.distance_km) && positive(row.measured_seconds)
        })
        .filter_map(|row| {
            let age = (cutoff - row.day).num_days() as f64;
            let weight = 2.0_f64.powf(-age / 365.0);
            (weight > 0.0).then_some((row, weight))
        })
        .collect();
    let same_route: Vec<_> = rows
        .iter()
        .copied()
        .filter(|(row, _)| {
            target.route_id.is_some()
                && row.route_id == target.route_id
                && (0.8..=1.25).contains(&(row.distance_km / target.distance_km))
        })
        .collect();
    if same_route.len() >= 3 {
        let pace = weighted_median(
            same_route
                .iter()
                .map(|(row, weight)| (row.measured_seconds / 60.0 / row.distance_km, *weight))
                .collect(),
        );
        return Ok(PersonalEstimate {
            minutes: pace * target.distance_km,
            basis: PersonalBasis::SameRoute,
            sample_count: same_route.len(),
            typical_error_minutes: residual_error(&same_route, |row| pace * row.distance_km),
        });
    }

    if rows.len() < MIN_HISTORY {
        return Err(PersonalFallback::InsufficientHistory {
            available: rows.len(),
            required: MIN_HISTORY,
        });
    }
    let gain = known_gain(target.ascent_m).ok_or(PersonalFallback::MissingGeometry)?;
    let global: Vec<_> = rows
        .into_iter()
        .filter(|(row, _)| known_gain(row.ascent_m).is_some())
        .collect();
    if global.len() < MIN_HISTORY {
        return Err(PersonalFallback::InsufficientHistory {
            available: global.len(),
            required: MIN_HISTORY,
        });
    }
    let mut best: Option<(f64, f64, f64)> = None;
    for h in H_GRID {
        let feature = |row: &TrainingRace| row.distance_km + h * row.ascent_m.unwrap() / 1000.0;
        let pace = weighted_median(
            global
                .iter()
                .map(|(row, weight)| {
                    let distance = feature(row);
                    (row.measured_seconds / 60.0 / distance, weight * distance)
                })
                .collect(),
        )
        .clamp(1.0, 4.0);
        let loss = residual_error(&global, |row| pace * feature(row));
        if best.is_none_or(|(best_loss, _, _)| loss < best_loss) {
            best = Some((loss, h, pace));
        }
    }
    let (loss, h, pace) = best.expect("fixed nonempty grid");
    Ok(PersonalEstimate {
        minutes: pace * (target.distance_km + h * gain / 1000.0),
        basis: PersonalBasis::DistanceAscent,
        sample_count: global.len(),
        typical_error_minutes: loss,
    })
}

fn event_gain(route: &RouteData, distance: f64, laps: Option<u32>) -> Option<f64> {
    let geometry = crate::models::RouteData {
        distance_km: route.distance_km,
        elevation_m: route.elevation_m,
        lead_in_distance_km: route.lead_in_distance_km,
        lead_in_elevation_m: route.lead_in_elevation_m,
        name: "",
        world: "",
        surface: "",
    };
    match laps.filter(|laps| *laps > 0) {
        Some(laps) => crate::estimation::elevation_for_laps(&geometry, laps),
        None => crate::estimation::elevation_for_distance(&geometry, distance),
    }
}

fn source_qualified(row: &HistoryObservation) -> bool {
    let decimal_id = |value: Option<&str>| {
        value.is_some_and(|id| {
            !id.is_empty() && id.bytes().all(|b| b.is_ascii_digit()) && id.parse::<u64>().is_ok()
        })
    };
    row.source.as_deref() == Some("zwiftpower-profile")
        && decimal_id(row.source_rider_id.as_deref())
        && decimal_id(row.source_event_id.as_deref())
        && row.source_result_id.as_deref().is_some_and(|id| {
            !id.trim().is_empty() && id.len() <= 256 && !id.chars().any(char::is_control)
        })
        && row
            .source_sha256
            .as_deref()
            .is_some_and(|hash| hash.len() == 64 && hash.bytes().all(|b| b.is_ascii_hexdigit()))
}

fn observation_day(row: &HistoryObservation) -> Option<NaiveDate> {
    let day = NaiveDate::parse_from_str(&row.result.race_date, "%Y-%m-%d").ok()?;
    if let Some(start) = row.event_start_utc.as_deref() {
        let utc_day = DateTime::parse_from_rfc3339(start)
            .ok()?
            .with_timezone(&Utc)
            .date_naive();
        if utc_day != day {
            return None;
        }
    }
    Some(day)
}

pub fn predict_from_database(
    db: &Database,
    mut target: PersonalTarget,
    laps: Option<u32>,
    today: NaiveDate,
) -> Result<PersonalEstimate, PersonalFallback> {
    let observations = db
        .performance_history()
        .map_err(|error| PersonalFallback::HistoryUnavailable(error.to_string()))?;
    let cutoff = today.min(target.event_day);
    let riders: std::collections::HashSet<_> = observations
        .iter()
        .filter(|row| {
            source_qualified(row)
                && is_ordinary_race(
                    row.event_type.as_deref(),
                    row.source_payload_json.as_deref(),
                )
                && observation_day(row).is_some_and(|day| day < cutoff)
                && row.event_distance_km.is_some_and(positive)
                && row.actual_seconds.is_some_and(positive)
        })
        .filter_map(|row| row.source_rider_id.as_deref())
        .collect();
    if riders.len() > 1 {
        return Err(PersonalFallback::MixedRiders);
    }
    let load = || -> anyhow::Result<(PersonalTarget, Vec<TrainingRace>)> {
        let route = target
            .route_id
            .map(|id| db.get_route(id))
            .transpose()?
            .flatten();
        target.route_id = route.as_ref().map(|route| route.route_id);
        target.ascent_m = route
            .as_ref()
            .and_then(|route| event_gain(route, target.distance_km, laps));
        let mut history = Vec::new();
        let cutoff = today.min(target.event_day);
        for row in observations {
            if !source_qualified(&row)
                || !is_ordinary_race(
                    row.event_type.as_deref(),
                    row.source_payload_json.as_deref(),
                )
            {
                continue;
            }
            let Some(day) = observation_day(&row).filter(|day| *day < cutoff) else {
                continue;
            };
            let (Some(distance), Some(seconds)) = (row.event_distance_km, row.actual_seconds)
            else {
                continue;
            };
            if !positive(distance) || !positive(seconds) {
                continue;
            }
            let route = db.get_route(row.result.route_id)?;
            history.push(TrainingRace {
                day,
                route_id: route.as_ref().map(|route| route.route_id),
                distance_km: distance,
                ascent_m: route
                    .as_ref()
                    .and_then(|route| event_gain(route, distance, None)),
                measured_seconds: seconds,
            });
        }
        Ok((target, history))
    };
    let (target, history) =
        load().map_err(|error| PersonalFallback::HistoryUnavailable(error.to_string()))?;
    predict(&target, &history, today)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn day() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 11, 2).unwrap()
    }

    fn target() -> PersonalTarget {
        PersonalTarget {
            event_day: day(),
            route_id: Some(42),
            distance_km: 20.0,
            ascent_m: Some(100.0),
        }
    }

    fn races() -> Vec<TrainingRace> {
        (0..12)
            .map(|i| TrainingRace {
                day: day() - Duration::days(i + 1),
                route_id: Some(100 + i as u32),
                distance_km: 10.0 + i as f64 * 2.0,
                ascent_m: Some(if i % 2 == 0 {
                    0.0
                } else {
                    100.0 + i as f64 * 25.0
                }),
                measured_seconds: 0.0,
            })
            .map(|mut row| {
                row.measured_seconds =
                    60.0 * 2.0 * (row.distance_km + 8.0 * row.ascent_m.unwrap() / 1000.0);
                row
            })
            .collect()
    }

    #[test]
    fn adapter_uses_total_event_gain_without_rounding_or_inventing_unknown_zero() {
        let mut route: RouteData = serde_json::from_value(serde_json::json!({
            "route_id": 4_000_003_010u32, "name": "Synthetic gain fixture", "world": "Test",
            "distance_km": 10.0, "elevation_m": 50.25, "surface": "road",
            "lead_in_distance_km": 1.0, "lead_in_elevation_m": 10.75
        }))
        .unwrap();
        assert_eq!(event_gain(&route, 26.0, None), Some(136.375));
        assert_eq!(event_gain(&route, 26.0, Some(3)), Some(161.5));
        route.elevation_m = Some(0.0);
        route.lead_in_elevation_m = Some(0.0);
        assert_eq!(event_gain(&route, 26.0, None), Some(0.0));
        route.lead_in_elevation_m = None;
        assert_eq!(event_gain(&route, 26.0, None), None);
        assert_eq!(event_gain(&route, 26.0, Some(3)), None);
    }

    #[test]
    fn global_fit_recovers_distance_ascent_and_descriptive_residual() {
        let result = predict(&target(), &races(), day()).unwrap();
        assert_eq!(result.basis, PersonalBasis::DistanceAscent);
        assert_eq!(result.sample_count, 12);
        assert!((result.minutes - 41.6).abs() < 1e-10);
        assert!(result.typical_error_minutes < 1e-10);
    }

    #[test]
    fn same_route_precedes_global_and_allows_unknown_ascent() {
        let mut rows = races();
        for (row, distance) in rows.iter_mut().take(3).zip([16.0, 20.0, 25.0]) {
            row.route_id = Some(42);
            row.distance_km = distance;
            row.ascent_m = None;
            row.measured_seconds = distance * 3.0 * 60.0;
        }
        let mut target = target();
        target.ascent_m = None;
        let result = predict(&target, &rows, day()).unwrap();
        assert_eq!(result.basis, PersonalBasis::SameRoute);
        assert_eq!(result.sample_count, 3);
        assert_eq!(result.minutes, 60.0);
        assert_eq!(result.typical_error_minutes, 0.0);
        rows[0].distance_km = 15.99;
        assert_eq!(
            predict(&target, &rows, day()).unwrap_err(),
            PersonalFallback::MissingGeometry
        );
        target.ascent_m = Some(0.0);
        assert_eq!(
            predict(&target, &rows, day()).unwrap_err(),
            PersonalFallback::InsufficientHistory {
                available: 9,
                required: 12
            }
        );
    }

    #[test]
    fn three_same_route_results_need_no_unrelated_history_or_flat_pace_clamp() {
        let mut rows = races();
        rows.truncate(3);
        for row in &mut rows {
            row.route_id = Some(42);
            row.distance_km = 20.0;
            row.measured_seconds = 120.0 * 60.0;
        }
        let result = predict(&target(), &rows, day()).unwrap();
        assert_eq!(result.sample_count, 3);
        assert_eq!(result.minutes, 120.0);
        rows.pop();
        assert_eq!(
            predict(&target(), &rows, day()).unwrap_err(),
            PersonalFallback::InsufficientHistory {
                available: 2,
                required: 12
            }
        );
    }

    #[test]
    fn training_is_strictly_before_today_and_event_day() {
        let mut rows = races();
        rows[0].day = day();
        let mut target = target();
        target.event_day = day() + Duration::days(30);
        for ineligible_day in [day(), day() + Duration::days(1), target.event_day] {
            rows[0].day = ineligible_day;
            assert_eq!(
                predict(&target, &rows, day()).unwrap_err(),
                PersonalFallback::InsufficientHistory {
                    available: 11,
                    required: 12
                }
            );
        }
        target.event_day = day() - Duration::days(1);
        rows[0].day = target.event_day;
        assert!(predict(&target, &rows, day()).is_err());
        rows[0].day = target.event_day - Duration::days(1);
        assert!(predict(&target, &rows, day()).is_ok());
    }

    #[test]
    fn weighted_time_mae_not_median_relative_error_and_pace_is_bounded() {
        let mut rows = races();
        for row in &mut rows {
            row.distance_km = 1.0;
            row.ascent_m = Some(0.0);
            row.measured_seconds = 60.0;
        }
        rows[0].distance_km = 1000.0;
        rows[0].measured_seconds = 1000.0 * 3.0 * 60.0;
        let mut target = target();
        target.ascent_m = Some(0.0);
        let result = predict(&target, &rows, day()).unwrap();
        assert_eq!(result.minutes, 60.0);
        assert!(result.typical_error_minutes > 1.0);
        for row in &mut rows {
            row.measured_seconds = row.distance_km * 60.0 * 10.0;
        }
        assert_eq!(predict(&target, &rows, day()).unwrap().minutes, 80.0);
        for row in &mut rows {
            row.measured_seconds = row.distance_km * 60.0 * 0.5;
        }
        assert_eq!(predict(&target, &rows, day()).unwrap().minutes, 20.0);
    }

    #[test]
    fn same_route_median_uses_recency_not_feature_weights() {
        let mut rows = races();
        for (i, row) in rows.iter_mut().take(3).enumerate() {
            row.route_id = Some(42);
            row.distance_km = 20.0;
            row.day = day() - Duration::days(if i == 0 { 1 } else { 2000 });
            row.measured_seconds = 20.0 * 60.0 * if i == 0 { 3.0 } else { 1.0 };
        }
        assert_eq!(predict(&target(), &rows, day()).unwrap().minutes, 60.0);
    }

    #[test]
    fn missing_or_invalid_geometry_never_becomes_flat_but_known_zero_is_valid() {
        let mut target = target();
        let rows = races();
        for ascent in [None, Some(f64::NAN), Some(-1.0), Some(f64::INFINITY)] {
            target.ascent_m = ascent;
            assert_eq!(
                predict(&target, &rows, day()).unwrap_err(),
                PersonalFallback::MissingGeometry
            );
        }
        target.ascent_m = Some(0.0);
        assert!(predict(&target, &rows, day()).is_ok());
        for distance in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            target.distance_km = distance;
            assert_eq!(
                predict(&target, &rows, day()).unwrap_err(),
                PersonalFallback::MissingGeometry
            );
        }
    }

    #[test]
    fn only_explicit_ordinary_races_or_exact_source_tokens_are_eligible() {
        for kind in ["RACE", "TYPE_RACE"] {
            assert!(is_ordinary_race(Some(kind), None));
        }
        for kind in ["TIME_TRIAL", "GROUP_RIDE", "UNKNOWN", "RACE TYPE_RIDE"] {
            assert!(!is_ordinary_race(
                Some(kind),
                Some(r#"{"f_t":"TYPE_RACE"}"#)
            ));
        }
        assert!(!is_ordinary_race(
            None,
            Some(r#"{"f_t":"TYPE_RACE TYPE_OTHER"}"#)
        ));
        assert!(is_ordinary_race(Some(""), Some(r#"{"f_t":"TYPE_RACE"}"#)));
        for payload in [
            r#"{"f_t":"TYPE_RACE TYPE_RIDE"}"#,
            r#"{"f_t":"TYPE_TIME_TRIAL TYPE_RACE"}"#,
            r#"{"f_t":"NOT_TYPE_RACE"}"#,
            r#"{"f_t":"TYPE_RIDE"}"#,
            "{}",
            "bad json",
        ] {
            assert!(!is_ordinary_race(None, Some(payload)));
        }
        assert!(!is_ordinary_race(None, None));
    }
}
