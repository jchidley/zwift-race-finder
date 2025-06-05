//! Category-related utility functions

// Average speeds by category (km/h) - based on actual race data with draft
pub const CAT_A_SPEED: f64 = 42.0;  // Estimated based on Cat D scaling
pub const CAT_B_SPEED: f64 = 37.0;  // Estimated based on Cat D scaling
pub const CAT_C_SPEED: f64 = 33.0;  // Estimated based on Cat D scaling
pub const CAT_D_SPEED: f64 = 30.9;  // Jack's actual average from 151 races

// Get category letter from Zwift Racing Score
pub fn get_category_from_score(zwift_score: u32) -> &'static str {
    match zwift_score {
        0..=199 => "D",
        200..=299 => "C",
        300..=399 => "B",
        _ => "A",
    }
}

// Get average speed for a category
pub fn get_category_speed(category: &str) -> f64 {
    match category {
        "A" => CAT_A_SPEED,
        "B" => CAT_B_SPEED,
        "C" => CAT_C_SPEED,
        "D" => CAT_D_SPEED,
        _ => CAT_D_SPEED, // Default to Cat D speed for unknown categories
    }
}