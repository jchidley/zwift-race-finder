/// Performance benchmarks for critical functions
/// Run with: cargo bench
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tempfile::TempDir;
use zwift_race_finder::database::{Database, Route};

fn create_test_database() -> (Database, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("bench.db");
    let db = Database::new(Some(db_path.to_str().unwrap().to_string())).unwrap();

    // Add test routes
    for i in 1..=100 {
        db.add_route(Route {
            route_id: i,
            distance_km: 20.0 + (i as f64),
            elevation_m: 100 + (i * 5),
            name: format!("Route {}", i),
            world: Some("Watopia".to_string()),
            surface: "tarmac".to_string(),
            lead_in_distance_km: 0.5,
            lead_in_elevation_m: 10,
            slug: Some(format!("route-{}", i)),
        })
        .unwrap();
    }

    (db, temp_dir)
}

fn bench_route_lookup(c: &mut Criterion) {
    let (db, _temp_dir) = create_test_database();

    c.bench_function("route_lookup", |b| {
        b.iter(|| db.get_route(black_box(50)).unwrap())
    });
}

fn bench_route_batch_lookup(c: &mut Criterion) {
    let (db, _temp_dir) = create_test_database();

    c.bench_function("route_batch_lookup", |b| {
        b.iter(|| db.get_all_routes().unwrap())
    });
}

fn bench_duration_estimation(c: &mut Criterion) {
    c.bench_function("duration_estimation", |b| {
        b.iter(|| {
            // Simulate duration calculation
            let distance_km = black_box(30.0);
            let elevation_m = black_box(250);
            let zwift_score = black_box(195);

            // Simple duration calculation
            let base_speed = match zwift_score {
                0..=199 => 30.9,
                200..=299 => 33.5,
                300..=399 => 36.2,
                _ => 38.9,
            };

            let meters_per_km = elevation_m as f64 / distance_km;
            let difficulty_multiplier = match meters_per_km {
                m if m < 5.0 => 1.1,
                m if m < 10.0 => 1.0,
                m if m < 20.0 => 0.9,
                m if m < 40.0 => 0.8,
                _ => 0.7,
            };

            let effective_speed = base_speed * difficulty_multiplier;
            distance_km / effective_speed * 60.0
        })
    });
}

fn bench_format_duration(c: &mut Criterion) {
    c.bench_function("format_duration", |b| {
        b.iter(|| {
            let minutes = black_box(125.5);
            let hours = (minutes / 60.0) as u32;
            let mins = (minutes % 60.0) as u32;
            format!("{:02}:{:02}", hours, mins)
        })
    });
}

fn bench_url_parsing(c: &mut Criterion) {
    let url = "https://example.com/?duration=90&tolerance=30&event-type=race&days=7&zwift-score=195&tags=ranked,zracing&exclude-tags=women_only";

    c.bench_function("url_parsing", |b| {
        b.iter(|| {
            let url_str = black_box(url);
            if let Some(query_start) = url_str.find('?') {
                let query = &url_str[query_start + 1..];
                let params: Vec<(&str, &str)> = query
                    .split('&')
                    .filter_map(|param| {
                        let mut parts = param.split('=');
                        match (parts.next(), parts.next()) {
                            (Some(key), Some(value)) => Some((key, value)),
                            _ => None,
                        }
                    })
                    .collect();
                params
            } else {
                vec![]
            }
        })
    });
}

fn bench_event_filtering(c: &mut Criterion) {
    use chrono::Utc;

    // Create test events
    let events: Vec<_> = (0..1000)
        .map(|i| {
            serde_json::json!({
                "id": i,
                "name": format!("Event {}", i),
                "eventStart": Utc::now().to_rfc3339(),
                "eventType": if i % 3 == 0 { "RACE" } else { "GROUP_RIDE" },
                "distanceInMeters": 20000.0 + (i as f64 * 100.0),
                "routeId": i % 100,
                "sport": "CYCLING",
                "tags": if i % 5 == 0 { vec!["ranked"] } else { vec![] }
            })
        })
        .collect();

    c.bench_function("event_filtering", |b| {
        b.iter(|| {
            events
                .iter()
                .filter(|e| e["eventType"] == "RACE")
                .filter(|e| e["sport"] == "CYCLING")
                .filter(|e| {
                    let distance = e["distanceInMeters"].as_f64().unwrap_or(0.0);
                    distance >= 25000.0 && distance <= 35000.0
                })
                .count()
        })
    });
}

criterion_group!(
    benches,
    bench_route_lookup,
    bench_route_batch_lookup,
    bench_duration_estimation,
    bench_format_duration,
    bench_url_parsing,
    bench_event_filtering
);
criterion_main!(benches);
