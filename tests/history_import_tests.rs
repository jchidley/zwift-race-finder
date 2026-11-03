use rusqlite::{params, Connection};
use serde_json::{json, Value};
use std::{fs, path::Path, process::Command};
use zwift_race_finder::{
    database::Database,
    history::{parse_zwiftpower_export, HistoryRecord},
};

fn export(ids: &[&str]) -> Value {
    json!({"schema_version":1,"source":"ZwiftPower profile results DataTable",
        "exported_at":"2025-02-02T00:00:00Z","profile_url":"https://zwiftpower.com/profile.php?z=42",
        "row_count":ids.len(),"results":ids.iter().map(|id| json!({
            "rt":u32::MAX.to_string(),"zid":"123","zwid":42,"res_id":id,
            "event_title":format!("Synthetic event {id}"),"event_date":1738368000i64,
            "time":[1800.25,0],"time_gun":1802.5,"distance":20.125,"laps":"2","category":"D",
            "skill":999,"skill_b":999
        })).collect::<Vec<_>>()})
}
fn records(ids: &[&str]) -> Vec<HistoryRecord> {
    parse_zwiftpower_export(&serde_json::to_vec(&export(ids)).unwrap(), &"a".repeat(64)).unwrap()
}
fn fresh(path: &Path) -> Database {
    let db = Database::open(path).unwrap();
    let route = serde_json::from_value(
        json!({"route_id":u32::MAX,"distance_km":10.0,"elevation_m":50.0,
        "name":"Synthetic circuit","world":"Test","surface":"road","lead_in_distance_km":0.5}),
    )
    .unwrap();
    db.add_route(&route).unwrap();
    db
}
fn legacy(path: &Path) {
    drop(fresh(path));
    let db = Connection::open(path).unwrap();
    db.execute_batch(
        "DROP TABLE race_results;
        CREATE TABLE race_results (id INTEGER PRIMARY KEY AUTOINCREMENT,route_id INTEGER NOT NULL,
            event_name TEXT NOT NULL,actual_minutes INTEGER NOT NULL,zwift_score INTEGER NOT NULL,
            race_date TIMESTAMP NOT NULL,notes TEXT,created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY(route_id) REFERENCES routes(route_id));",
    )
    .unwrap();
    let r = &records(&["r1"])[0];
    let notes =
        json!({"source":r.source,"rider_id":r.source_rider_id,"result_id":r.source_result_id,
        "event_id":r.source_event_id,"actual_seconds":r.actual_seconds})
        .to_string();
    db.execute("INSERT INTO race_results(id,route_id,event_name,actual_minutes,zwift_score,race_date,notes,created_at)
        VALUES(7,?1,?2,30,0,?3,?4,'2001-01-01')", params![r.route_id,r.event_name,r.race_date,notes]).unwrap();
    db.execute_batch("UPDATE sqlite_sequence SET seq=100 WHERE name='race_results';
        CREATE INDEX test_history_date ON race_results(race_date);
        CREATE TABLE test_audit(message TEXT);
        CREATE TRIGGER test_history_insert AFTER INSERT ON race_results BEGIN INSERT INTO test_audit VALUES('insert'); END;").unwrap();
}
fn run(path: &Path, input: &Path, preview: bool) -> std::process::Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_import_zwiftpower_history"));
    cmd.env_clear()
        .arg("--database")
        .arg(path)
        .arg("--input")
        .arg(input);
    if preview {
        cmd.arg("--dry-run");
    }
    cmd.output().unwrap()
}
#[test]
fn migration_preserves_rows_timestamps_sequence_objects_and_allows_unknown_score() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("history.db");
    legacy(&path);
    let db = Database::open(&path).unwrap();
    let rows = db.performance_history().unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].result.id, Some(7));
    assert_eq!(rows[0].result.zwift_score, Some(0));
    assert_eq!(rows[0].actual_seconds, None);
    let sql = Connection::open(&path).unwrap();
    assert_eq!(
        sql.query_row("SELECT created_at FROM race_results", [], |r| r
            .get::<_, String>(0))
            .unwrap(),
        "2001-01-01"
    );
    assert_eq!(
        sql.query_row("SELECT count(*) FROM test_audit", [], |r| r
            .get::<_, u32>(0))
            .unwrap(),
        0
    );
    assert_eq!(sql.query_row("SELECT count(*) FROM sqlite_master WHERE name IN ('test_history_date','test_history_insert')",[],|r|r.get::<_,u32>(0)).unwrap(),2);
    db.import_history(&records(&["r2"])).unwrap();
    let rows = db.get_all_race_results().unwrap();
    assert_eq!(rows.len(), 2);
    assert!(rows
        .iter()
        .any(|r| r.id == Some(101) && r.zwift_score.is_none()));
    drop(db);
    assert_eq!(
        Database::open(&path)
            .unwrap()
            .performance_history()
            .unwrap()
            .len(),
        2
    );
}
#[test]
fn enriches_legacy_without_duplicate_or_score_loss_and_retains_observations() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("history.db");
    legacy(&path);
    let db = Database::open(&path).unwrap();
    let input = records(&["r1", "r2"]);
    let result = db.import_history(&input).unwrap();
    assert_eq!((result.inserted, result.enriched), (1, 1));
    let rows = db.performance_history().unwrap();
    assert_eq!(rows.len(), 2);
    let known = rows.iter().find(|r| r.result.id == Some(7)).unwrap();
    assert_eq!(known.result.zwift_score, Some(0));
    assert!(known.result.notes.is_some());
    assert_eq!(known.actual_seconds, Some(1800.25));
    assert_eq!(known.event_distance_km, Some(20.125));
    assert_eq!(known.laps, Some(2));
    assert_eq!(known.source_event_id.as_deref(), Some("123"));
    assert!(known.event_type.is_none());
    assert!(known.subgroup_id.is_none());
    assert_eq!(
        rows.iter()
            .filter(|r| r.result.zwift_score.is_none())
            .count(),
        1
    );
    let mut refreshed = input.clone();
    refreshed[0].source_sha256 = "b".repeat(64);
    assert_eq!(db.import_history(&refreshed).unwrap().unchanged, 2);
    assert_eq!(db.performance_history().unwrap().len(), 2);
    let sql = Connection::open(&path).unwrap();
    assert_eq!(
        sql.query_row("SELECT created_at FROM race_results WHERE id=7", [], |r| {
            r.get::<_, String>(0)
        })
        .unwrap(),
        "2001-01-01"
    );
    assert!(sql.execute("INSERT INTO race_results(route_id,event_name,actual_minutes,race_date,source,source_rider_id,source_result_id)
        SELECT route_id,event_name,actual_minutes,race_date,source,source_rider_id,source_result_id FROM race_results WHERE id=7",[]).is_err());
}
#[test]
fn conflicts_duplicates_and_mid_batch_failure_leave_no_partial_history() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("history.db");
    let db = fresh(&path);
    let mut input = records(&["r1", "r2"]);
    assert!(db
        .import_history(&[input[0].clone(), input[0].clone()])
        .is_err());
    let sql = Connection::open(&path).unwrap();
    sql.execute_batch(
        "CREATE TRIGGER reject_second BEFORE UPDATE ON race_results WHEN NEW.source_result_id='r2'
        BEGIN SELECT RAISE(ABORT,'synthetic failure'); END;",
    )
    .unwrap();
    assert!(db.import_history(&input).is_err());
    assert!(db.performance_history().unwrap().is_empty());
    sql.execute_batch("DROP TRIGGER reject_second").unwrap();
    db.import_history(&input).unwrap();
    input[0].actual_seconds += 60.0;
    assert!(db.import_history(&input).is_err());
    assert_eq!(
        db.performance_history().unwrap()[0].actual_seconds,
        Some(1800.25)
    );
    let mut scored = records(&["r1"]);
    scored[0].zwift_score = Some(210);
    assert_eq!(db.import_history(&scored).unwrap().enriched, 1);
    scored[0].zwift_score = Some(211);
    assert!(db.import_history(&scored).is_err());
}
#[test]
fn unsupported_schema_and_dependent_tables_are_not_silently_destroyed() {
    for extra in [
        "ALTER TABLE race_results ADD COLUMN external_activity_id TEXT",
        "CREATE TABLE dependent(id INTEGER REFERENCES race_results(id))",
    ] {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("history.db");
        legacy(&path);
        let sql = Connection::open(&path).unwrap();
        sql.execute_batch(extra).unwrap();
        let before: String = sql
            .query_row(
                "SELECT sql FROM sqlite_master WHERE name='race_results'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert!(Database::open(&path).is_err());
        let after: String = sql
            .query_row(
                "SELECT sql FROM sqlite_master WHERE name='race_results'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(before, after);
        assert_eq!(
            sql.query_row("SELECT count(*) FROM race_results", [], |r| r
                .get::<_, u32>(0))
                .unwrap(),
            1
        );
    }
}
#[test]
fn preview_does_not_change_target_and_cli_reconciles_then_repeats() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("history.db");
    legacy(&path);
    let input = temp.path().join("export.json");
    fs::write(&input, serde_json::to_vec(&export(&["r1", "r2"])).unwrap()).unwrap();
    let before = fs::read(&path).unwrap();
    let preview = run(&path, &input, true);
    assert!(
        preview.status.success(),
        "{}",
        String::from_utf8_lossy(&preview.stderr)
    );
    assert_eq!(before, fs::read(&path).unwrap());
    assert!(String::from_utf8_lossy(&preview.stdout).contains("1 new; 1 enriched"));
    let applied = run(&path, &input, false);
    assert!(
        applied.status.success(),
        "{}",
        String::from_utf8_lossy(&applied.stderr)
    );
    let repeat = run(&path, &input, false);
    assert!(repeat.status.success());
    assert!(String::from_utf8_lossy(&repeat.stdout).contains("0 new; 0 enriched; 2 unchanged"));
    let missing = temp.path().join("absent/subdir/history.db");
    assert!(!run(&missing, &input, true).status.success());
    assert!(!missing.parent().unwrap().exists());
}
