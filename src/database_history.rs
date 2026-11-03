use super::{Database, RaceResult};
use crate::history::HistoryRecord;
use anyhow::{bail, ensure, Result};
use rusqlite::{params, Connection, OptionalExtension};
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct HistoryImportSummary {
    pub inserted: usize,
    pub enriched: usize,
    pub unchanged: usize,
}

#[derive(Debug, Clone)]
pub struct HistoryObservation {
    pub result: RaceResult,
    pub actual_seconds: Option<f64>,
    pub event_start_utc: Option<String>,
    pub event_distance_km: Option<f64>,
    pub laps: Option<u32>,
    pub result_category: Option<String>,
    pub gun_time_seconds: Option<f64>,
    pub source: Option<String>,
    pub source_rider_id: Option<String>,
    pub source_result_id: Option<String>,
    pub source_event_id: Option<String>,
    pub source_sha256: Option<String>,
    pub source_payload_json: Option<String>,
    pub event_type: Option<String>,
    pub subgroup_id: Option<String>,
}

pub(super) fn create_table(conn: &Connection, name: &str) -> Result<()> {
    ensure!(
        matches!(name, "race_results" | "race_results_history_migration"),
        "Invalid migration table"
    );
    conn.execute_batch(&format!(
        "CREATE TABLE IF NOT EXISTS {name} (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            route_id INTEGER NOT NULL REFERENCES routes(route_id),
            event_name TEXT NOT NULL,
            actual_minutes INTEGER NOT NULL,
            zwift_score INTEGER,
            race_date TIMESTAMP NOT NULL,
            notes TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            actual_seconds REAL CHECK(actual_seconds IS NULL OR actual_seconds > 0),
            event_start_utc TEXT,
            event_distance_km REAL CHECK(event_distance_km IS NULL OR event_distance_km > 0),
            laps INTEGER CHECK(laps IS NULL OR laps > 0),
            result_category TEXT,
            gun_time_seconds REAL,
            source TEXT,
            source_rider_id TEXT,
            source_result_id TEXT,
            source_event_id TEXT,
            source_sha256 TEXT,
            source_payload_json TEXT,
            event_type TEXT,
            subgroup_id TEXT
        );"
    ))?;
    Ok(())
}

pub(super) fn migrate(conn: &Connection) -> Result<bool> {
    let columns: Vec<(String, bool)> = conn
        .prepare("PRAGMA table_info(race_results)")?
        .query_map([], |r| Ok((r.get(1)?, r.get(3)?)))?
        .collect::<rusqlite::Result<_>>()?;
    let modern = columns.iter().any(|(n, _)| n == "actual_seconds");
    if !modern {
        let expected = [
            "id",
            "route_id",
            "event_name",
            "actual_minutes",
            "zwift_score",
            "race_date",
            "notes",
            "created_at",
        ];
        ensure!(columns.iter().map(|(n, _)| n.as_str()).eq(expected),
            "Unrecognized race_results columns; back up and explicitly migrate custom columns before importing history");
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table'")?
            .query_map([], |r| r.get(0))?
            .collect::<rusqlite::Result<_>>()?;
        for table in tables {
            let sql = format!(
                "PRAGMA foreign_key_list(\"{}\")",
                table.replace('"', "\"\"")
            );
            let references: Vec<String> = conn
                .prepare(&sql)?
                .query_map([], |r| r.get(2))?
                .collect::<rusqlite::Result<_>>()?;
            ensure!(!references.iter().any(|s| s == "race_results"),
                "Table {table} references race_results; explicit dependent-schema migration required");
        }
        let dependent_views: i64 = conn.query_row(
            "SELECT count(*) FROM sqlite_master WHERE type='view' AND lower(sql) LIKE '%race_results%'", [], |r| r.get(0))?;
        ensure!(
            dependent_views == 0,
            "Views reference race_results; explicit dependent-schema migration required"
        );
        let objects: Vec<Option<String>> = conn.prepare(
            "SELECT sql FROM sqlite_master WHERE tbl_name='race_results' AND type IN ('index','trigger')")?
            .query_map([], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?;
        ensure!(
            objects.iter().all(Option::is_some),
            "Custom implicit constraints require explicit history migration"
        );
        let sequence: Option<i64> = conn
            .query_row(
                "SELECT seq FROM sqlite_sequence WHERE name='race_results'",
                [],
                |r| r.get(0),
            )
            .optional()?;
        let collision: i64 = conn.query_row(
            "SELECT count(*) FROM sqlite_master WHERE name='race_results_history_migration'",
            [],
            |r| r.get(0),
        )?;
        ensure!(
            collision == 0,
            "Reserved history migration table already exists; inspect it before proceeding"
        );
        create_table(conn, "race_results_history_migration")?;
        conn.execute_batch("INSERT INTO race_results_history_migration
            (id,route_id,event_name,actual_minutes,zwift_score,race_date,notes,created_at)
            SELECT id,route_id,event_name,actual_minutes,zwift_score,race_date,notes,created_at FROM race_results;
            DROP TABLE race_results;
            ALTER TABLE race_results_history_migration RENAME TO race_results;")?;
        if let Some(seq) = sequence {
            let changed = conn.execute(
                "UPDATE sqlite_sequence SET seq=MAX(seq,?1) WHERE name='race_results'",
                [seq],
            )?;
            if changed == 0 {
                conn.execute(
                    "INSERT INTO sqlite_sequence(name,seq) VALUES ('race_results',?1)",
                    [seq],
                )?;
            }
        }
        for sql in objects.into_iter().flatten() {
            conn.execute_batch(&sql)?;
        }
    } else {
        ensure!(
            columns
                .iter()
                .any(|(n, required)| n == "zwift_score" && !required),
            "History schema has required score; explicit repair required"
        );
        for required in [
            "event_start_utc",
            "event_distance_km",
            "laps",
            "result_category",
            "gun_time_seconds",
            "source",
            "source_rider_id",
            "source_result_id",
            "source_event_id",
            "source_sha256",
            "source_payload_json",
            "event_type",
            "subgroup_id",
        ] {
            ensure!(
                columns.iter().any(|(n, _)| n == required),
                "Incomplete history schema: missing {required}"
            );
        }
    }
    conn.execute_batch(
        "CREATE UNIQUE INDEX IF NOT EXISTS race_results_source_identity
        ON race_results(source,source_rider_id,source_result_id)
        WHERE source IS NOT NULL AND source_rider_id IS NOT NULL AND source_result_id IS NOT NULL;",
    )?;
    Ok(!modern)
}

impl Database {
    pub fn performance_history(&self) -> Result<Vec<HistoryObservation>> {
        let mut stmt = self.conn.prepare("SELECT id,route_id,event_name,actual_minutes,zwift_score,race_date,notes,
            actual_seconds,event_start_utc,event_distance_km,laps,result_category,gun_time_seconds,
            source,source_rider_id,source_result_id,source_event_id,source_sha256,source_payload_json,event_type,subgroup_id
            FROM race_results ORDER BY race_date,id")?;
        let result = stmt
            .query_map([], |r| {
                Ok(HistoryObservation {
                    result: RaceResult {
                        id: r.get(0)?,
                        route_id: r.get(1)?,
                        event_name: r.get(2)?,
                        actual_minutes: r.get(3)?,
                        zwift_score: r.get::<_, Option<f64>>(4)?.map(|v| v.round() as u32),
                        race_date: r.get(5)?,
                        notes: r.get(6)?,
                    },
                    actual_seconds: r.get(7)?,
                    event_start_utc: r.get(8)?,
                    event_distance_km: r.get(9)?,
                    laps: r.get(10)?,
                    result_category: r.get(11)?,
                    gun_time_seconds: r.get(12)?,
                    source: r.get(13)?,
                    source_rider_id: r.get(14)?,
                    source_result_id: r.get(15)?,
                    source_event_id: r.get(16)?,
                    source_sha256: r.get(17)?,
                    source_payload_json: r.get(18)?,
                    event_type: r.get(19)?,
                    subgroup_id: r.get(20)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(result)
    }

    pub fn import_history(&self, records: &[HistoryRecord]) -> Result<HistoryImportSummary> {
        ensure!(!records.is_empty(), "History export contains no records");
        let mut identities = HashSet::new();
        for r in records {
            r.validate()?;
            ensure!(
                identities.insert((&r.source, &r.source_rider_id, &r.source_result_id)),
                "Duplicate history source identity"
            );
        }
        let tx = self.conn.unchecked_transaction()?;
        let existing = self.performance_history()?;
        let mut claimed = HashSet::new();
        let mut summary = HistoryImportSummary::default();
        for r in records {
            let minutes = (r.actual_seconds / 60.0).round() as u32;
            let mut matched = Vec::new();
            for old in &existing {
                let direct = old.source.as_deref() == Some(&r.source)
                    && old.source_rider_id.as_deref() == Some(&r.source_rider_id)
                    && old.source_result_id.as_deref() == Some(&r.source_result_id);
                let note = old
                    .result
                    .notes
                    .as_deref()
                    .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok());
                let legacy = old.source.is_none()
                    && note.as_ref().is_some_and(|n| {
                        n["source"].as_str() == Some(&r.source)
                            && n["rider_id"].as_str() == Some(&r.source_rider_id)
                            && n["result_id"].as_str() == Some(&r.source_result_id)
                    });
                if direct || legacy {
                    if legacy {
                        let n = note.as_ref().unwrap();
                        ensure!(n["actual_seconds"].as_f64() == Some(r.actual_seconds)
                            && n["event_id"].as_str() == Some(&r.source_event_id),
                            "Legacy provenance conflicts with incoming result; no overwrite performed");
                    }
                    matched.push(old);
                }
            }
            ensure!(
                matched.len() <= 1,
                "Ambiguous legacy/source identity; reconcile duplicates explicitly"
            );
            let (id, score) = if let Some(old) = matched.first() {
                ensure!(
                    claimed.insert(old.result.id),
                    "Multiple records resolve to one legacy row"
                );
                ensure!(
                    old.result.route_id == r.route_id
                        && old.result.event_name == r.event_name
                        && old.result.race_date == r.race_date
                        && old.result.actual_minutes == minutes,
                    "Incoming result conflicts with stored observations; no overwrite performed"
                );
                if let (Some(a), Some(b)) = (old.result.zwift_score, r.zwift_score) {
                    ensure!(
                        a == b,
                        "Conflicting confirmed Racing Score; explicit correction required"
                    );
                }
                let score = old.result.zwift_score.or(r.zwift_score);
                if old.source.is_some() {
                    ensure!(
                        old.actual_seconds == Some(r.actual_seconds)
                            && old.event_start_utc.as_deref() == Some(&r.event_start_utc)
                            && old.event_distance_km == r.event_distance_km
                            && old.laps == r.laps
                            && old.result_category == r.result_category
                            && old.gun_time_seconds == r.gun_time_seconds
                            && old.source_event_id.as_deref() == Some(&r.source_event_id),
                        "Source measurements changed; explicit review required"
                    );
                    if score == old.result.zwift_score {
                        summary.unchanged += 1;
                        continue;
                    }
                }
                summary.enriched += 1;
                (old.result.id, score)
            } else {
                ensure!(!existing.iter().any(|old| old.result.route_id == r.route_id
                    && old.result.race_date == r.race_date && old.result.event_name == r.event_name),
                    "Possible existing manual result without matching source identity; reconcile explicitly");
                summary.inserted += 1;
                (None, r.zwift_score)
            };
            ensure!(
                self.get_route_direct(r.route_id)?.is_some(),
                "History route {} is not in the catalogue; import verified client metadata first",
                r.route_id
            );
            let id = if let Some(id) = id {
                id
            } else {
                self.conn.execute("INSERT INTO race_results (route_id,event_name,actual_minutes,zwift_score,race_date) VALUES (?1,?2,?3,?4,?5)",
                    params![r.route_id,r.event_name,minutes,score,r.race_date])?;
                self.conn.last_insert_rowid()
            };
            self.conn.execute("UPDATE race_results SET zwift_score=?2,actual_seconds=?3,event_start_utc=?4,
                event_distance_km=?5,laps=?6,result_category=?7,gun_time_seconds=?8,source=?9,
                source_rider_id=?10,source_result_id=?11,source_event_id=?12,source_sha256=?13,source_payload_json=?14 WHERE id=?1",
                params![id,score,r.actual_seconds,r.event_start_utc,r.event_distance_km,r.laps,r.result_category,
                    r.gun_time_seconds,r.source,r.source_rider_id,r.source_result_id,r.source_event_id,r.source_sha256,r.source_payload_json])?;
        }
        if self
            .conn
            .prepare("PRAGMA foreign_key_check")?
            .query([])?
            .next()?
            .is_some()
        {
            bail!("Foreign-key violations detected; history import rolled back");
        }
        tx.commit()?;
        Ok(summary)
    }
}
