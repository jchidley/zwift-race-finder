fn main() {
    // 3R Watopia Flat Route Race parameters
    let distance_km = 33.4;
    let cat_d_speed = 30.9;
    let flat_multiplier = 1.1;
    
    let effective_speed = cat_d_speed * flat_multiplier;
    let duration_hours = distance_km / effective_speed;
    let duration_minutes = (duration_hours * 60.0) as u32;
    
    println\!("Distance: {} km", distance_km);
    println\!("Base Cat D speed: {} km/h", cat_d_speed);
    println\!("Flat route multiplier: {}", flat_multiplier);
    println\!("Effective speed: {} km/h", effective_speed);
    println\!("Duration: {} minutes", duration_minutes);
    
    // What the test expects
    let expected_base_speed = 27.0;
    let expected_speed = expected_base_speed * flat_multiplier;
    let expected_duration = (distance_km / expected_speed * 60.0) as u32;
    
    println\!("\nTest expectation:");
    println\!("Expected base speed: {} km/h", expected_base_speed);
    println\!("Expected effective speed: {} km/h", expected_speed);
    println\!("Expected duration: {} minutes", expected_duration);
}
