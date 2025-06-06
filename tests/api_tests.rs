//! API interaction tests using mocks
//! These tests verify API error handling and response parsing

use mockito::Server;
use serde_json::json;

#[test]
fn test_fetch_events_success() {
    let mut server = Server::new();

    // Create mock server response
    let _m = server
        .mock("GET", "/api/public/events/upcoming")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!([
                {
                    "id": 1,
                    "name": "Test Race",
                    "eventStart": "2025-01-07T12:00:00Z",
                    "eventType": "RACE",
                    "distanceInMeters": 30000.0,
                    "durationInMinutes": null,
                    "durationInSeconds": null,
                    "routeId": 123,
                    "route": null,
                    "description": "Test race description",
                    "categoryEnforcement": false,
                    "eventSubGroups": [],
                    "sport": "CYCLING",
                    "tags": ["ranked"]
                }
            ])
            .to_string(),
        )
        .create();

    // Test API call using blocking client
    let url = format!("{}/api/public/events/upcoming", server.url());
    let response = reqwest::blocking::get(&url).unwrap();

    assert_eq!(response.status(), 200);
    let events: Vec<serde_json::Value> = response.json().unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0]["name"], "Test Race");
}

#[test]
fn test_fetch_events_404() {
    let mut server = Server::new();
    let _m = server
        .mock("GET", "/api/public/events/upcoming")
        .with_status(404)
        .with_body("Not Found")
        .create();

    let url = format!("{}/api/public/events/upcoming", server.url());
    let response = reqwest::blocking::get(&url).unwrap();

    assert_eq!(response.status(), 404);
}

#[test]
fn test_fetch_events_malformed_json() {
    let mut server = Server::new();
    let _m = server
        .mock("GET", "/api/public/events/upcoming")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("{ invalid json")
        .create();

    let url = format!("{}/api/public/events/upcoming", server.url());
    let response = reqwest::blocking::get(&url).unwrap();

    assert_eq!(response.status(), 200);

    // Parsing should fail
    let result: Result<Vec<serde_json::Value>, _> = response.json();
    assert!(result.is_err());
}

#[test]
fn test_fetch_events_empty_response() {
    let mut server = Server::new();
    let _m = server
        .mock("GET", "/api/public/events/upcoming")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("[]")
        .create();

    let url = format!("{}/api/public/events/upcoming", server.url());
    let response = reqwest::blocking::get(&url).unwrap();

    assert_eq!(response.status(), 200);
    let events: Vec<serde_json::Value> = response.json().unwrap();
    assert_eq!(events.len(), 0);
}

#[test]
fn test_racing_score_event_parsing() {
    // Test parsing of Racing Score events with specific structure
    let mut server = Server::new();
    let _m = server
        .mock("GET", "/api/public/events/upcoming")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!([
                {
                    "id": 2,
                    "name": "Racing Score Test",
                    "eventStart": "2025-01-07T14:00:00Z",
                    "eventType": "RACE",
                    "distanceInMeters": 0.0,  // Racing Score events have 0 distance
                    "durationInMinutes": null,
                    "durationInSeconds": null,
                    "routeId": 456,
                    "route": null,
                    "description": "Distance: 35.1km / 21.8mi\nElevation: 178m / 584ft",
                    "categoryEnforcement": true,
                    "eventSubGroups": [
                        {
                            "id": 1,
                            "rangeAccessLabel": "0-650",  // Indicates Racing Score event
                            "subGroupLabel": "Racing Score 0-650"
                        }
                    ],
                    "sport": "CYCLING",
                    "tags": ["racing_score"]
                }
            ])
            .to_string(),
        )
        .create();

    let url = format!("{}/api/public/events/upcoming", server.url());
    let response = reqwest::blocking::get(&url).unwrap();

    assert_eq!(response.status(), 200);
    let events: Vec<serde_json::Value> = response.json().unwrap();

    // Verify Racing Score event structure
    assert_eq!(events[0]["distanceInMeters"], 0.0);
    assert!(events[0]["description"]
        .as_str()
        .unwrap()
        .contains("Distance: 35.1km"));
    assert_eq!(events[0]["eventSubGroups"][0]["rangeAccessLabel"], "0-650");
}

#[test]
fn test_event_filtering_with_tags() {
    // Test filtering events by tags
    let mut server = Server::new();
    let _m = server
        .mock("GET", "/api/public/events/upcoming")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!([
                {
                    "id": 1,
                    "name": "Ranked Race",
                    "eventStart": "2025-01-07T12:00:00Z",
                    "eventType": "RACE",
                    "distanceInMeters": 30000.0,
                    "routeId": 123,
                    "sport": "CYCLING",
                    "tags": ["ranked", "zracing"]
                },
                {
                    "id": 2,
                    "name": "Casual Race",
                    "eventStart": "2025-01-07T13:00:00Z",
                    "eventType": "RACE",
                    "distanceInMeters": 25000.0,
                    "routeId": 124,
                    "sport": "CYCLING",
                    "tags": ["social"]
                }
            ])
            .to_string(),
        )
        .create();

    let url = format!("{}/api/public/events/upcoming", server.url());
    let response = reqwest::blocking::get(&url).unwrap();
    let events: Vec<serde_json::Value> = response.json().unwrap();

    // Both events fetched
    assert_eq!(events.len(), 2);

    // Filter for ranked events
    let ranked_events: Vec<_> = events
        .iter()
        .filter(|e| e["tags"].as_array().unwrap().contains(&json!("ranked")))
        .collect();

    assert_eq!(ranked_events.len(), 1);
    assert_eq!(ranked_events[0]["name"], "Ranked Race");
}
