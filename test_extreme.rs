use zwift_race_finder::duration_estimation::estimate_duration_for_category;

fn main() {
    let distance = 282.8;
    let route = "Road to Sky";
    let score = 50;
    
    let duration = estimate_duration_for_category(distance, route, score);
    println\!("Distance: {} km, Score: {}, Duration: {} minutes ({:.1} hours)", 
             distance, score, duration, duration as f64 / 60.0);
}
