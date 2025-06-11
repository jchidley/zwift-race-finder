//! A/B testing for behavioral preservation during UOM migration
//!
//! These tests compare the current implementation against UOM versions
//! to ensure perfect behavioral compatibility.

use zwift_race_finder::{
    ab_testing::{ABTest, ABTestBatch},
    category::get_category_from_score,
    duration_estimation::estimate_duration_for_category,
    estimation::{estimate_duration_from_route_id, estimate_duration_with_distance},
};

#[cfg(feature = "uom-migration")]
use zwift_race_finder::{
    duration_estimation::estimate_duration_for_category_uom,
    estimation::{estimate_duration_from_route_id_uom, estimate_duration_with_distance_uom},
    units::{Distance, Duration},
};

#[cfg(feature = "uom-migration")]
use uom::si::length::kilometer;

/// Test estimation functions have identical behavior
#[cfg(feature = "uom-migration")]
#[test]
fn test_duration_estimation_ab() {
    let mut batch = ABTestBatch::new("duration_estimation");
    
    // Test various scenarios
    let test_cases = vec![
        (10.0, "Watopia's Waistband", 195),
        (25.0, "Hilly Route", 250),
        (40.0, "Tempus Fugit", 150),
        (60.0, "Road to Sky", 350),
        (0.1, "Short Sprint", 450),
        (200.0, "Ultra Distance", 100),
    ];
    
    for (distance, route, score) in test_cases {
        let test = ABTest {
            name: format!("estimate_{}km_{}_{}", distance, route, score),
            old_impl: Box::new(|| estimate_duration_for_category(distance, route, score)),
            new_impl: Box::new(|| {
                let dist = Distance::new::<kilometer>(distance);
                let dur = estimate_duration_for_category_uom(dist, route, score);
                dur.value as u32 // Convert back to minutes
            }),
            context: format!("Testing {} km on {} with score {}", distance, route, score),
        };
        
        batch.add_result(test.run());
    }
    
    println!("{}", batch.summary());
    
    // All tests must pass for behavioral preservation
    assert_eq!(batch.success_rate(), 1.0, 
        "Not all duration estimations matched! Failures: {:?}", 
        batch.failures());
}

/// Test route ID based estimation
#[cfg(feature = "uom-migration")]
#[test]
fn test_route_id_estimation_ab() {
    let mut batch = ABTestBatch::new("route_id_estimation");
    
    let test_cases = vec![
        (1, 195),    // Watopia's Waistband
        (12, 250),   // Tempus Fugit
        (22, 150),   // Road to Sky
        (9999, 350), // Unknown route
    ];
    
    for (route_id, score) in test_cases {
        let test = ABTest {
            name: format!("route_{}_score_{}", route_id, score),
            old_impl: Box::new(|| estimate_duration_from_route_id(route_id, score)),
            new_impl: Box::new(|| estimate_duration_from_route_id_uom(route_id, score)),
            context: format!("Testing route_id {} with score {}", route_id, score),
        };
        
        batch.add_result(test.run());
    }
    
    println!("{}", batch.summary());
    assert_eq!(batch.success_rate(), 1.0,
        "Route ID estimations don't match! Failures: {:?}",
        batch.failures());
}

/// Test custom distance estimation
#[cfg(feature = "uom-migration")]
#[test]
fn test_custom_distance_estimation_ab() {
    let mut batch = ABTestBatch::new("custom_distance_estimation");
    
    let test_cases = vec![
        (1, 15.0, 195),
        (12, 50.0, 250),
        (22, 100.0, 150),
    ];
    
    for (route_id, distance, score) in test_cases {
        let test = ABTest {
            name: format!("route_{}_dist_{}_score_{}", route_id, distance, score),
            old_impl: Box::new(|| estimate_duration_with_distance(route_id, distance, score)),
            new_impl: Box::new(|| {
                let dist = Distance::new::<kilometer>(distance);
                estimate_duration_with_distance_uom(route_id, dist, score)
            }),
            context: format!("Testing route {} with {} km at score {}", route_id, distance, score),
        };
        
        batch.add_result(test.run());
    }
    
    println!("{}", batch.summary());
    assert_eq!(batch.success_rate(), 1.0,
        "Custom distance estimations don't match! Failures: {:?}",
        batch.failures());
}

/// Test category boundaries for consistency
#[cfg(feature = "uom-migration")]
#[test]
fn test_category_boundary_ab() {
    let mut batch = ABTestBatch::new("category_boundaries");
    
    // Test at exact category boundaries
    let boundary_scores = vec![99, 100, 199, 200, 299, 300, 399, 400];
    let distance = 40.0;
    let route = "Tempus Fugit";
    
    for score in boundary_scores {
        let test = ABTest {
            name: format!("boundary_score_{}", score),
            old_impl: Box::new(|| estimate_duration_for_category(distance, route, score)),
            new_impl: Box::new(|| {
                let dist = Distance::new::<kilometer>(distance);
                let dur = estimate_duration_for_category_uom(dist, route, score);
                dur.value as u32
            }),
            context: format!("Testing category boundary at score {}", score),
        };
        
        batch.add_result(test.run());
    }
    
    println!("{}", batch.summary());
    assert_eq!(batch.success_rate(), 1.0,
        "Category boundary behaviors don't match! Failures: {:?}",
        batch.failures());
}