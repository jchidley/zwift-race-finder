//! Validated observations from an explicitly exported ZwiftPower profile.
//!
//! No network access, score inference, route-distance inference, or persistence occurs here.

use std::collections::HashSet;

use anyhow::{bail, ensure, Context, Result};
use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const SOURCE: &str = "zwiftpower-profile";
const EXPORT_SOURCE: &str = "ZwiftPower profile results DataTable";
const MAX_ID_LENGTH: usize = 256;

/// A measured result with source provenance, independent of Racing Score.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryRecord {
    pub route_id: u32,
    pub event_name: String,
    pub actual_seconds: f64,
    pub zwift_score: Option<u32>,
    pub race_date: String,
    pub event_start_utc: String,
    pub event_distance_km: Option<f64>,
    pub laps: Option<u32>,
    pub result_category: Option<String>,
    pub gun_time_seconds: Option<f64>,
    pub source: String,
    pub source_rider_id: String,
    pub source_result_id: String,
    pub source_event_id: String,
    pub source_sha256: String,
    pub source_payload_json: String,
}

impl HistoryRecord {
    /// Validate before persistence, including the rounded-minute compatibility field.
    pub fn validate(&self) -> Result<()> {
        ensure!(
            !self.event_name.trim().is_empty(),
            "event name must not be blank"
        );
        ensure!(
            self.actual_seconds.is_finite() && self.actual_seconds > 0.0,
            "actual seconds must be finite and positive"
        );
        let minutes = (self.actual_seconds / 60.0).round();
        ensure!(
            minutes >= 1.0 && minutes <= f64::from(u32::MAX),
            "rounded actual minutes must be between 1 and u32::MAX"
        );
        let start = DateTime::parse_from_rfc3339(&self.event_start_utc)
            .context("event start must be RFC3339")?;
        ensure!(
            start.timestamp() > 0 && start.offset().local_minus_utc() == 0,
            "event start must be positive Unix time in UTC"
        );
        ensure!(
            self.race_date == start.format("%Y-%m-%d").to_string(),
            "race date must match UTC event start"
        );
        for (name, value) in [
            ("event distance", self.event_distance_km),
            ("gun time", self.gun_time_seconds),
        ] {
            if let Some(value) = value {
                ensure!(
                    value.is_finite() && value > 0.0,
                    "{name} must be finite and positive"
                );
            }
        }
        ensure!(self.laps != Some(0), "laps must be positive");
        ensure!(self.zwift_score.is_none_or(|score| score <= 1000), "Racing Score must be at most 1000");
        if let Some(category) = &self.result_category {
            ensure!(
                !category.trim().is_empty(),
                "result category must not be blank"
            );
        }
        ensure!(self.source == SOURCE, "unsupported history source");
        decimal_id(&self.source_rider_id).context("invalid source rider ID")?;
        decimal_id(&self.source_event_id).context("invalid source event ID")?;
        bounded_id(&self.source_result_id)?;
        validate_sha256(&self.source_sha256)?;
        let payload: Value = serde_json::from_str(&self.source_payload_json)
            .context("source payload must be JSON")?;
        ensure!(payload.is_object(), "source payload must be an object");
        Ok(())
    }
}

fn validate_sha256(value: &str) -> Result<()> {
    ensure!(
        value.len() == 64 && value.bytes().all(|b| b.is_ascii_hexdigit()),
        "SHA-256 must contain exactly 64 hexadecimal characters"
    );
    Ok(())
}

fn decimal_id(value: &str) -> Result<u64> {
    ensure!(
        !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit()),
        "ID must be an unsigned decimal string"
    );
    value.parse().context("ID exceeds u64")
}

fn bounded_id(value: &str) -> Result<()> {
    ensure!(
        !value.trim().is_empty()
            && value.len() <= MAX_ID_LENGTH
            && !value.chars().any(char::is_control),
        "result ID must be nonblank, at most 256 bytes, and contain no control characters"
    );
    Ok(())
}

fn string<'a>(row: &'a Value, field: &str) -> Result<&'a str> {
    row.get(field)
        .and_then(Value::as_str)
        .with_context(|| format!("{field} must be a string"))
}

fn number(value: &Value, field: &str) -> Result<f64> {
    let number = value
        .as_f64()
        .with_context(|| format!("{field} must be a number"))?;
    ensure!(
        number.is_finite() && number >= 0.0,
        "{field} must be finite and nonnegative"
    );
    Ok(number)
}

fn optional_measurement(row: &Value, field: &str) -> Result<Option<f64>> {
    row.get(field)
        .map(|value| number(value, field).map(|n| (n > 0.0).then_some(n)))
        .transpose()
        .map(Option::flatten)
}

fn parse_row(row: &Value, sha256: &str) -> Result<HistoryRecord> {
    ensure!(row.is_object(), "result row must be an object");
    let route = decimal_id(string(row, "rt")?).context("invalid rt")?;
    let route_id = u32::try_from(route).context("rt exceeds u32")?;
    let event_id = string(row, "zid")?;
    decimal_id(event_id).context("invalid zid")?;
    let rider_id = row
        .get("zwid")
        .and_then(Value::as_u64)
        .context("zwid must be an unsigned integer number")?;
    let result_id = string(row, "res_id")?;
    bounded_id(result_id)?;
    let time = row
        .get("time")
        .and_then(Value::as_array)
        .context("time must be an array")?;
    ensure!(time.len() == 2, "time must contain exactly two numbers");
    let actual_seconds = number(&time[0], "time[0]")?;
    // The second number is contextual source data, never a duration or score.
    number(&time[1], "time[1]")?;
    let timestamp = row
        .get("event_date")
        .and_then(Value::as_i64)
        .filter(|t| *t > 0)
        .context("event_date must be positive integer Unix seconds")?;
    let start = DateTime::<Utc>::from_timestamp(timestamp, 0)
        .context("event_date is outside the supported UTC range")?;
    let laps = match row.get("laps") {
        None => None,
        Some(value) => {
            let value = value
                .as_str()
                .context("laps must be a decimal string or blank")?;
            if value.trim().is_empty() {
                None
            } else {
                let value = u32::try_from(decimal_id(value)?).context("laps exceeds u32")?;
                ensure!(value > 0, "laps must be positive");
                Some(value)
            }
        }
    };
    let result_category = match row.get("category") {
        None => None,
        Some(value) => {
            let value = value.as_str().context("category must be a string")?;
            (!value.trim().is_empty()).then(|| value.to_owned())
        }
    };
    let record = HistoryRecord {
        route_id,
        event_name: string(row, "event_title")?.to_owned(),
        actual_seconds,
        zwift_score: None,
        race_date: start.format("%Y-%m-%d").to_string(),
        event_start_utc: start.to_rfc3339_opts(SecondsFormat::Secs, true),
        event_distance_km: optional_measurement(row, "distance")?,
        laps,
        result_category,
        gun_time_seconds: optional_measurement(row, "time_gun")?,
        source: SOURCE.to_owned(),
        source_rider_id: rider_id.to_string(),
        source_result_id: result_id.to_owned(),
        source_event_id: event_id.to_owned(),
        source_sha256: sha256.to_owned(),
        // serde_json's ordered object maps provide deterministic key ordering.
        source_payload_json: serde_json::to_string(row)?,
    };
    record.validate()?;
    Ok(record)
}

/// Parse and validate the entire version-1 envelope before returning any rows.
///
/// The caller hashes the original bytes; this function validates the supplied
/// digest's shape, not its correspondence to those bytes. Errors identify row
/// positions without including private source measurements or identifiers.
pub fn parse_zwiftpower_export(bytes: &[u8], sha256: &str) -> Result<Vec<HistoryRecord>> {
    validate_sha256(sha256)?;
    let envelope: Value = serde_json::from_slice(bytes).context("invalid export JSON")?;
    ensure!(envelope.is_object(), "export envelope must be an object");
    ensure!(
        envelope.get("schema_version").and_then(Value::as_u64) == Some(1),
        "unsupported export schema_version (expected 1)"
    );
    ensure!(
        envelope.get("source").and_then(Value::as_str) == Some(EXPORT_SOURCE),
        "unsupported export source"
    );
    DateTime::parse_from_rfc3339(string(&envelope, "exported_at")?)
        .context("exported_at must be RFC3339")?;
    let url = reqwest::Url::parse(string(&envelope, "profile_url")?)
        .context("profile_url must be a URL")?;
    ensure!(
        url.scheme() == "https"
            && url.host_str() == Some("zwiftpower.com")
            && url.path() == "/profile.php"
            && url.query().is_some_and(|q| !q.is_empty())
            && url.username().is_empty()
            && url.password().is_none()
            && url.port_or_known_default() == Some(443)
            && url.fragment().is_none(),
        "profile_url must be an HTTPS zwiftpower.com/profile.php URL with a query"
    );
    let rows = envelope
        .get("results")
        .and_then(Value::as_array)
        .context("results must be an array")?;
    ensure!(!rows.is_empty(), "results must not be empty");
    let count = envelope
        .get("row_count")
        .and_then(Value::as_u64)
        .context("row_count must be an unsigned integer")?;
    ensure!(
        usize::try_from(count).ok() == Some(rows.len()),
        "row_count does not match results"
    );
    let mut records = Vec::with_capacity(rows.len());
    let mut identities = HashSet::new();
    for (index, row) in rows.iter().enumerate() {
        let record =
            parse_row(row, sha256).with_context(|| format!("invalid result row {}", index + 1))?;
        if let Some(first) = records.first() {
            let first: &HistoryRecord = first;
            ensure!(
                first.source_rider_id == record.source_rider_id,
                "export contains multiple rider IDs"
            );
        }
        if !identities.insert((
            record.source_rider_id.clone(),
            record.source_result_id.clone(),
        )) {
            bail!("duplicate source identity at result row {}", index + 1);
        }
        records.push(record);
    }
    records.sort_by(|a, b| {
        a.event_start_utc
            .cmp(&b.event_start_utc)
            .then_with(|| a.source_result_id.cmp(&b.source_result_id))
    });
    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn row() -> Value {
        json!({
            "rt": "4294967295", "zid": "8000000001", "zwid": 12345,
            "res_id": "synthetic-result-A", "event_title": "Synthetic race",
            "time": [1800.125, 99], "time_gun": 1802.5,
            "event_date": 1704067200, "distance": 24.375, "laps": "2",
            "category": "B", "skill": 999, "skill_b": 888
        })
    }

    fn envelope(rows: Vec<Value>) -> Value {
        json!({
            "schema_version": 1, "source": EXPORT_SOURCE,
            "exported_at": "2026-01-01T12:00:00Z",
            "profile_url": "https://zwiftpower.com/profile.php?z=12345",
            "row_count": rows.len(), "results": rows
        })
    }

    fn parse(value: &Value) -> Result<Vec<HistoryRecord>> {
        parse_zwiftpower_export(&serde_json::to_vec(value)?, &"a".repeat(64))
    }

    #[test]
    fn preserves_precision_provenance_and_unknown_score() {
        let original = row();
        let records = parse(&envelope(vec![original.clone()])).unwrap();
        let r = &records[0];
        assert_eq!(r.route_id, u32::MAX);
        assert_eq!(r.actual_seconds, 1800.125);
        assert_eq!(r.event_distance_km, Some(24.375));
        assert_eq!(r.gun_time_seconds, Some(1802.5));
        assert_eq!(r.laps, Some(2));
        assert_eq!(r.zwift_score, None);
        assert_eq!(r.result_category.as_deref(), Some("B"));
        assert_eq!(r.race_date, "2024-01-01");
        assert_eq!(r.event_start_utc, "2024-01-01T00:00:00Z");
        assert_eq!(r.source, SOURCE);
        assert_eq!(
            serde_json::from_str::<Value>(&r.source_payload_json).unwrap(),
            original
        );
        let serialized = serde_json::to_value(r).unwrap();
        assert!(serialized["zwift_score"].is_null());
        assert_eq!(
            serde_json::from_value::<HistoryRecord>(serialized).unwrap(),
            *r
        );
    }

    #[test]
    fn missing_and_zero_measurements_are_not_fabricated() {
        for remove in [true, false] {
            let mut r = row();
            if remove {
                for field in ["distance", "time_gun", "laps", "category"] {
                    r.as_object_mut().unwrap().remove(field);
                }
            } else {
                r["distance"] = json!(0);
                r["time_gun"] = json!(0);
                r["laps"] = json!(" ");
                r["category"] = json!("");
            }
            let records = parse(&envelope(vec![r])).unwrap();
            let r = &records[0];
            assert_eq!(r.event_distance_km, None);
            assert_eq!(r.gun_time_seconds, None);
            assert_eq!(r.laps, None);
            assert_eq!(r.result_category, None);
        }
    }

    #[test]
    fn rejects_malformed_present_row_fields() {
        let invalid = [
            ("rt", json!(1)),
            ("rt", json!("4294967296")),
            ("rt", json!("-1")),
            ("zid", json!(123)),
            ("zid", json!("18446744073709551616")),
            ("zwid", json!(true)),
            ("zwid", json!(-1)),
            ("zwid", json!("123")),
            ("zwid", json!(1.5)),
            ("res_id", json!("")),
            ("res_id", json!(123)),
            ("res_id", json!("a".repeat(257))),
            ("res_id", json!("a\nb")),
            ("event_title", json!(" ")),
            ("event_title", json!(null)),
            ("time", json!([0, 10])),
            ("time", json!([true, 10])),
            ("time", json!([1800, "10"])),
            ("time", json!([1800])),
            ("time", json!([1800, -1])),
            ("time", json!([-1, 10])),
            ("event_date", json!(0)),
            ("event_date", json!(-1)),
            ("event_date", json!(1704067200.5)),
            ("event_date", json!(true)),
            ("event_date", json!(i64::MAX)),
            ("distance", json!(-1)),
            ("distance", json!(null)),
            ("distance", json!("24")),
            ("distance", json!(true)),
            ("time_gun", json!(-1)),
            ("time_gun", json!(null)),
            ("laps", json!("0")),
            ("laps", json!(2)),
            ("laps", json!("2.5")),
            ("laps", json!("4294967296")),
            ("laps", json!(null)),
            ("category", json!(null)),
            ("category", json!(123)),
        ];
        for (field, value) in invalid {
            let mut r = row();
            r[field] = value.clone();
            assert!(
                parse(&envelope(vec![r])).is_err(),
                "accepted {field}: {value}"
            );
        }
    }

    #[test]
    fn rejects_invalid_envelopes_and_hashes() {
        for (field, value) in [
            ("schema_version", json!(2)),
            ("schema_version", json!(true)),
            ("source", json!("other")),
            ("row_count", json!(2)),
            ("row_count", json!(true)),
            ("exported_at", json!("2024-01-01")),
            (
                "profile_url",
                json!("http://zwiftpower.com/profile.php?z=1"),
            ),
            (
                "profile_url",
                json!("https://zwiftpower.com.evil.test/profile.php?z=1"),
            ),
            ("profile_url", json!("https://zwiftpower.com/profile.php")),
            ("results", json!({})),
        ] {
            let mut e = envelope(vec![row()]);
            e[field] = value;
            assert!(parse(&e).is_err(), "accepted invalid {field}");
        }
        assert!(parse(&envelope(vec![])).is_err());
        let bytes = serde_json::to_vec(&envelope(vec![row()])).unwrap();
        for hash in ["".to_owned(), "a".repeat(63), "g".repeat(64)] {
            assert!(parse_zwiftpower_export(&bytes, &hash).is_err());
        }
        assert!(parse_zwiftpower_export(&bytes, &"A".repeat(64)).is_ok());
        assert!(parse_zwiftpower_export(b"{}", &"a".repeat(64)).is_err());
        assert!(parse_zwiftpower_export(b"not JSON", &"a".repeat(64)).is_err());
        assert!(parse_zwiftpower_export(b"{\"distance\":1e999}", &"a".repeat(64)).is_err());
    }

    #[test]
    fn rejects_duplicate_results_and_multiple_owners() {
        assert!(parse(&envelope(vec![row(), row()])).is_err());
        let mut other = row();
        other["res_id"] = json!("synthetic-result-B");
        other["zwid"] = json!(54321);
        assert!(parse(&envelope(vec![row(), other])).is_err());
    }

    #[test]
    fn sorts_results_and_keeps_context_out_of_duration() {
        let mut earlier = row();
        earlier["event_date"] = json!(1703980800);
        earlier["res_id"] = json!("synthetic-result-B");
        earlier["time"] = json!([1800.125, 0]);
        let mut same_day = row();
        same_day["res_id"] = json!("synthetic-result-C");
        let r = parse(&envelope(vec![same_day, row(), earlier])).unwrap();
        assert_eq!(r[0].race_date, "2023-12-31");
        assert_eq!(r[1].source_result_id, "synthetic-result-A");
        assert_eq!(r[2].source_result_id, "synthetic-result-C");
        assert_eq!(r[0].actual_seconds, 1800.125);
    }

    #[test]
    fn validates_rounded_minutes_and_nonfinite_values() {
        let mut r = parse(&envelope(vec![row()])).unwrap().remove(0);
        for seconds in [
            29.999,
            f64::NAN,
            f64::INFINITY,
            -1.0,
            (f64::from(u32::MAX) + 0.5) * 60.0,
        ] {
            r.actual_seconds = seconds;
            assert!(r.validate().is_err());
        }
        for seconds in [30.0, f64::from(u32::MAX) * 60.0] {
            r.actual_seconds = seconds;
            assert!(r.validate().is_ok());
        }
        r.event_distance_km = Some(f64::INFINITY);
        assert!(r.validate().is_err());
        r.event_distance_km = None;
        r.gun_time_seconds = Some(f64::NAN);
        assert!(r.validate().is_err());
    }
}
