use rusqlite::Connection;
use serde_json::{json, Value};
use std::{fs, path::Path, process::Command};

fn record(id: u32, kind: &str, gain: Option<f64>) -> Value {
    json!({
        "route_id": id, "name": "Downtown Dolphin", "world": "Crit City", "surface": "road",
        "distance_km": 1.9653959960938, "elevation_m": gain,
        "lead_in_distance_km": 0.0938338317871, "lead_in_elevation_m": 0.0,
        "lead_in_distance_free_ride_km": 0.0436344757080, "lead_in_elevation_free_ride_m": 0.0,
        "lead_in_distance_meetups_km": 0.0497484130859, "lead_in_elevation_meetups_m": 0.0,
        "slug": null, "source": {"kind":kind,"path":"test-client.zip","entry":"world8.routes.xml","sha256":"a".repeat(64)},
        "event_only":"1","sport_type":"3","supported_laps":"1"
    })
}

fn run(root: &Path, records: &[Value], dry_run: bool) -> std::process::Output {
    let input = root.join("client.json");
    fs::write(
        &input,
        serde_json::to_vec(&json!({"schema_version":1,"routes":records})).unwrap(),
    )
    .unwrap();
    let mut command = Command::new(env!("CARGO_BIN_EXE_import_client_routes"));
    command
        .env_clear()
        .env("HOME", root)
        .env("XDG_DATA_HOME", root.join("state"))
        .env("XDG_CONFIG_HOME", root.join("config"))
        .arg("--input")
        .arg(input);
    if dry_run {
        command.arg("--dry-run");
    }
    command.output().unwrap()
}

fn successful(root: &Path, records: &[Value], dry_run: bool) -> String {
    let output = run(root, records, dry_run);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn database(root: &Path) -> Connection {
    Connection::open(root.join("state/zwift-race-finder/races.db")).unwrap()
}

#[test]
fn precision_precedence_and_history_survive_primary_import() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    let extracted = record(947394567, "extracted-client", Some(17.0));
    successful(root, &[extracted.clone()], false);
    let db = database(root);
    db.execute("PRAGMA foreign_keys=ON", []).unwrap();
    db.execute(
        "INSERT INTO race_results (route_id, actual_minutes, zwift_score, event_name, race_date)
        VALUES (947394567, 45, 195, 'Fixture', '2026-10-29')",
        [],
    )
    .unwrap();
    db.execute(
        "UPDATE routes SET created_at='2000-01-01' WHERE route_id=947394567",
        [],
    )
    .unwrap();

    let primary = record(947394567, "client-cache", Some(16.6176948547));
    successful(root, &[primary.clone()], false);
    assert!(successful(root, &[primary.clone()], false).contains("0 written; 1"));
    assert!(successful(root, &[extracted], false).contains("0 written; 1"));
    let (gain, known, lead, created): (f64, bool, f64, String) = db.query_row(
        "SELECT elevation_m, elevation_known, lead_in_elevation_m, created_at FROM routes WHERE route_id=947394567",
        [], |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?))).unwrap();
    assert_eq!(gain, 16.6176948547);
    assert!(known);
    assert_eq!(lead, 0.0);
    assert_eq!((gain * 15.0).round(), 249.0);
    assert_eq!(created, "2000-01-01");
    assert_eq!(
        db.query_row("SELECT COUNT(*) FROM race_results", [], |r| r
            .get::<_, u32>(0))
            .unwrap(),
        1
    );
    let metadata: String = db
        .query_row(
            "SELECT metadata FROM route_sources WHERE route_id=947394567",
            [],
            |r| r.get(0),
        )
        .unwrap();
    let metadata: Value = serde_json::from_str(&metadata).unwrap();
    assert_eq!(metadata["source"]["kind"], "client-cache");
    assert_eq!(metadata["sport_type"], "3");


}

#[test]
fn client_import_reconciles_seed_identity_and_preserves_unknown_zero() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    successful(root, &[record(1258415487, "extracted-client", Some(59.0))], true);
    database(root).execute("INSERT INTO routes (route_id, distance_km, elevation_m, name, world, surface) VALUES (1258415487, 14.1, 59, 'Incorrect legacy name', 'Fixture', 'road')", []).unwrap();
    let mut correct_seed = record(1258415487, "client-cache", Some(33.1));
    correct_seed["name"] = json!("Glasgow Reverse");
    correct_seed["distance_km"] = json!(3.012);
    correct_seed["lead_in_elevation_m"] = json!(6.5);
    successful(
        root,
        &[
            correct_seed,
            record(4_000_000_000, "client-cache", None),
            record(4_000_000_001, "client-cache", Some(0.0)),
        ],
        false,
    );
    let db = database(root);
    let name: String = db
        .query_row(
            "SELECT name FROM routes WHERE route_id=1258415487",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(name, "Glasgow Reverse");
    let flags: Vec<bool> = db
        .prepare("SELECT elevation_known FROM routes WHERE route_id>=4000000000 ORDER BY route_id")
        .unwrap()
        .query_map([], |r| r.get(0))
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(flags, vec![false, true]);
}

#[test]
fn invalid_duplicate_and_failed_batches_do_not_partially_apply() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    let first = record(947394567, "client-cache", Some(16.6176948547));
    successful(root, &[first.clone()], false);
    let db = database(root);
    let mut changed = first.clone();
    changed["elevation_m"] = json!(100.0);
    assert!(!run(root, &[changed.clone(), changed.clone()], false)
        .status
        .success());
    let mut invalid = record(4_000_000_000, "client-cache", Some(-1.0));
    assert!(!run(root, &[changed.clone(), invalid.clone()], false)
        .status
        .success());
    invalid["elevation_m"] = json!(10.0);
    successful(root, &[changed.clone(), invalid.clone()], true);
    assert_eq!(
        db.query_row(
            "SELECT elevation_m FROM routes WHERE route_id=947394567",
            [],
            |r| r.get::<_, f64>(0)
        )
        .unwrap(),
        16.6176948547
    );
    db.execute_batch(
        "CREATE TRIGGER reject_fixture BEFORE INSERT ON routes WHEN NEW.route_id=4000000000
        BEGIN SELECT RAISE(ABORT, 'fixture failure'); END;",
    )
    .unwrap();
    assert!(!run(root, &[changed, invalid], false).status.success());
    assert_eq!(
        db.query_row(
            "SELECT elevation_m FROM routes WHERE route_id=947394567",
            [],
            |r| r.get::<_, f64>(0)
        )
        .unwrap(),
        16.6176948547
    );
    assert_eq!(
        db.query_row("SELECT COUNT(*) FROM route_sources", [], |r| r
            .get::<_, u32>(0))
            .unwrap(),
        1
    );
}

#[test]
fn missing_lead_in_distance_remains_unknown_and_unicode_is_preserved() {
    let temp = tempfile::tempdir().unwrap();
    let mut route = record(4_000_000_000, "client-cache", Some(0.0));
    route["lead_in_distance_km"] = Value::Null;
    route["name"] = json!("Sacré Trois!");
    successful(temp.path(), &[route], false);
    let db = database(temp.path());
    let (distance, name): (Option<f64>, String) = db
        .query_row(
            "SELECT lead_in_distance_km, name FROM routes WHERE route_id=4000000000",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    assert_eq!(distance, None);
    assert_eq!(name, "Sacré Trois!");
}

#[test]
fn fresh_database_is_empty_until_explicit_import_and_reopening_preserves_rows() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    let route = record(4_000_000_002, "client-cache", Some(1.25));
    successful(root, &[route.clone()], true);
    let db = database(root);
    let count = || db.query_row("SELECT COUNT(*) FROM routes", [], |r| r.get::<_, u32>(0)).unwrap();
    assert_eq!(count(), 0, "preview must not seed or import any route");
    assert_eq!(db.query_row("SELECT COUNT(*) FROM route_sources", [], |r| r.get::<_, u32>(0)).unwrap(), 0);
    successful(root, &[route.clone()], false);
    assert_eq!(count(), 1);
    db.execute("INSERT INTO routes (route_id, distance_km, elevation_m, name, world, surface) VALUES (42, 1, 0, 'Unmatched retained record', 'Test', 'road')", []).unwrap();
    successful(root, &[route.clone()], false);
    assert_eq!(count(), 2, "reopening must preserve unmatched legacy rows without adding seeds");
    db.execute("DELETE FROM route_sources", []).unwrap();
    db.execute("DELETE FROM routes", []).unwrap();
    successful(root, &[route], true);
    assert_eq!(count(), 0, "reopening an emptied database must not reseed it");
}
