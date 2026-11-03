use mockito::Server;
use rstest::rstest;
use serde_json::json;
use zwift_race_finder::{
    api::fetch_events_from_url,
    event_filtering::{filter_by_excluded_tags, filter_by_tags},
    models::{is_racing_score_event, ZwiftEvent},
};

async fn fetch_mocked(status: usize, body: &str) -> anyhow::Result<Vec<ZwiftEvent>> {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/api/public/events/upcoming")
        .match_header("user-agent", "Zwift Race Finder")
        .match_header("content-type", "application/json")
        .with_status(status)
        .with_header("content-type", "application/json")
        .with_body(body)
        .expect(1)
        .create_async()
        .await;

    let result =
        fetch_events_from_url(&format!("{}/api/public/events/upcoming", server.url())).await;
    mock.assert_async().await;
    result
}

#[tokio::test]
async fn test_fetch_events_success() {
    let body = json!([{
        "id": 1,
        "name": "Synthetic Racing Score Race",
        "eventStart": "2025-01-07T12:00:00Z",
        "eventType": "RACE",
        "distanceInMeters": 0.0,
        "durationInMinutes": 30,
        "durationInSeconds": 1800,
        "routeId": 4_000_000_000_u32,
        "route": "Synthetic Route",
        "description": "Distance: 35.1km / 21.8mi",
        "categoryEnforcement": true,
        "eventSubgroups": [{
            "id": 10,
            "name": "Racing Score 0-650",
            "routeId": u32::MAX,
            "distanceInMeters": 35100.0,
            "durationInMinutes": 30,
            "categoryEnforcement": true,
            "rangeAccessLabel": "0-650",
            "laps": 3
        }],
        "sport": "CYCLING",
        "tags": ["ranked", "racing_score"]
    }]);

    let events = fetch_mocked(200, &body.to_string()).await.unwrap();
    assert_eq!(events.len(), 1);
    let event = &events[0];
    assert_eq!(event.id, 1);
    assert_eq!(event.name, "Synthetic Racing Score Race");
    assert_eq!(event.event_start.timestamp(), 1_736_251_200);
    assert_eq!(event.event_type, "RACE");
    assert_eq!(event.distance_in_meters, Some(0.0));
    assert_eq!(event.duration_in_minutes, Some(30));
    assert_eq!(event.duration_in_seconds, Some(1800));
    assert_eq!(event.route_id, Some(4_000_000_000));
    assert_eq!(event.route.as_deref(), Some("Synthetic Route"));
    assert_eq!(
        event.description.as_deref(),
        Some("Distance: 35.1km / 21.8mi")
    );
    assert!(event.category_enforcement);
    assert_eq!(event.sport, "CYCLING");
    assert_eq!(event.tags, ["ranked", "racing_score"]);

    assert_eq!(event.event_sub_groups.len(), 1);
    let subgroup = &event.event_sub_groups[0];
    assert_eq!(subgroup.id, 10);
    assert_eq!(subgroup.name, "Racing Score 0-650");
    assert_eq!(subgroup.route_id, Some(u32::MAX));
    assert_eq!(subgroup.distance_in_meters, Some(35100.0));
    assert_eq!(subgroup.duration_in_minutes, Some(30));
    assert_eq!(subgroup.category_enforcement, Some(true));
    assert_eq!(subgroup.range_access_label.as_deref(), Some("0-650"));
    assert_eq!(subgroup.laps, Some(3));
    assert!(is_racing_score_event(event));
}

#[tokio::test]
async fn test_fetch_events_optional_data_and_defaults() {
    let body = json!([
        {
            "id": 2,
            "name": "Minimal Race",
            "eventStart": "2025-01-07T12:00:00Z",
            "eventType": "RACE"
        },
        {
            "id": 3,
            "name": "Null Optional Data",
            "eventStart": "2025-01-07T13:00:00Z",
            "eventType": "RACE",
            "distanceInMeters": null,
            "durationInMinutes": null,
            "durationInSeconds": null,
            "routeId": null,
            "route": null,
            "description": null,
            "eventSubgroups": [{"id": 11, "name": "Open"}]
        }
    ]);

    let events = fetch_mocked(200, &body.to_string()).await.unwrap();
    assert_eq!(events.len(), 2);
    for event in &events {
        assert_eq!(event.distance_in_meters, None);
        assert_eq!(event.duration_in_minutes, None);
        assert_eq!(event.duration_in_seconds, None);
        assert_eq!(event.route_id, None);
        assert_eq!(event.route, None);
        assert_eq!(event.description, None);
        assert!(!event.category_enforcement);
        assert_eq!(event.sport, "CYCLING");
        assert!(event.tags.is_empty());
        assert!(!is_racing_score_event(event));
    }
    assert!(events[0].event_sub_groups.is_empty());
    assert_eq!(events[1].event_sub_groups.len(), 1);
    let subgroup = &events[1].event_sub_groups[0];
    assert_eq!(subgroup.id, 11);
    assert_eq!(subgroup.name, "Open");
    assert_eq!(subgroup.route_id, None);
    assert_eq!(subgroup.distance_in_meters, None);
    assert_eq!(subgroup.duration_in_minutes, None);
    assert_eq!(subgroup.category_enforcement, None);
    assert_eq!(subgroup.range_access_label, None);
    assert_eq!(subgroup.laps, None);
}

#[tokio::test]
async fn test_fetch_events_empty_response() {
    let events = fetch_mocked(200, "[]").await.unwrap();
    assert!(events.is_empty());
}

#[tokio::test]
async fn test_fetch_events_malformed_json() {
    let error = fetch_mocked(200, "{ invalid json").await.unwrap_err();
    let source = error.downcast_ref::<reqwest::Error>().unwrap();
    assert!(
        source.is_decode(),
        "expected a response decode error: {error}"
    );
}

#[rstest]
#[case::wrong_top_level(json!({"events": []}))]
#[case::missing_required_fields(json!([{"id": 4}]))]
#[case::invalid_field_type(json!([{
    "id": 4, "name": "Invalid", "eventStart": "2025-01-07T12:00:00Z",
    "eventType": "RACE", "routeId": "not a route ID"
}]))]
#[case::negative_route_id(json!([{
    "id": 4, "name": "Invalid", "eventStart": "2025-01-07T12:00:00Z",
    "eventType": "RACE", "routeId": -1
}]))]
#[case::overflowing_route_id(json!([{
    "id": 4, "name": "Invalid", "eventStart": "2025-01-07T12:00:00Z",
    "eventType": "RACE", "routeId": 4_294_967_296_u64
}]))]
#[case::invalid_subgroup(json!([{
    "id": 4, "name": "Invalid", "eventStart": "2025-01-07T12:00:00Z",
    "eventType": "RACE", "eventSubgroups": [{"id": 12}]
}]))]
#[tokio::test]
async fn test_fetch_events_incompatible_json(#[case] body: serde_json::Value) {
    let error = fetch_mocked(200, &body.to_string()).await.unwrap_err();
    let source = error.downcast_ref::<reqwest::Error>().unwrap();
    assert!(
        source.is_decode(),
        "expected a typed response decode error: {error}"
    );
}

#[rstest]
#[case::not_found(404, "API returned status: 404 Not Found")]
#[case::rate_limited(429, "API rate limit exceeded")]
#[case::server_error(500, "API returned status: 500 Internal Server Error")]
#[tokio::test]
async fn test_fetch_events_http_errors(#[case] status: usize, #[case] expected: &str) {
    let error = fetch_mocked(status, "not JSON").await.unwrap_err();
    assert_eq!(error.to_string(), expected);
}

#[tokio::test]
async fn test_event_filtering_with_tags() {
    let body = json!([
        {
            "id": 1, "name": "Ranked Race",
            "eventStart": "2025-01-07T12:00:00Z", "eventType": "RACE",
            "tags": ["ranked", "zracing"]
        },
        {
            "id": 2, "name": "Casual Race",
            "eventStart": "2025-01-07T13:00:00Z", "eventType": "RACE",
            "tags": ["social"]
        },
        {
            "id": 3, "name": "Untagged Race",
            "eventStart": "2025-01-07T14:00:00Z", "eventType": "RACE"
        }
    ]);
    let events = fetch_mocked(200, &body.to_string()).await.unwrap();
    assert_eq!(events.len(), 3);

    let mut included = events.clone();
    assert_eq!(filter_by_tags(&mut included, &["ranked".into()]), 2);
    assert_eq!(included.len(), 1);
    assert_eq!(included[0].id, 1);
    assert_eq!(included[0].name, "Ranked Race");

    let mut excluded = events;
    assert_eq!(
        filter_by_excluded_tags(&mut excluded, &["ranked".into()]),
        1
    );
    assert_eq!(excluded.len(), 2);
    assert_eq!(excluded[0].id, 2);
    assert_eq!(excluded[1].id, 3);
}
