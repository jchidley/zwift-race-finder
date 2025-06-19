// ABOUTME: Compact OCR module for extracting telemetry from Zwift screenshots
// Minimal implementation using only Tesseract and basic image processing

use anyhow::{Context, Result};
use leptess::{LepTess, Variable};
use serde::{Deserialize, Serialize};
use std::path::Path;
use image::{DynamicImage, GrayImage, Luma};
use crate::ocr_constants::{regions, thresholds, scale_factors, pose, wkg, name_limits, edge_detection};
use crate::ocr_config::OcrConfigManager;
use crate::ocr_image_processing::preprocess_for_ocr;
use crate::ocr_regex;
use std::sync::OnceLock;

/// Rider pose types with their drag characteristics
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum RiderPose {
    #[serde(rename = "normal_tuck")]
    NormalTuck,         // Tucked position (HIGH DRAG in Zwift!)
    #[serde(rename = "normal_normal")]
    NormalNormal,       // Standard upright (NORMAL DRAG)
    #[serde(rename = "climbing_seated")]
    ClimbingSeated,     // Seated climbing (NORMAL DRAG)
    #[serde(rename = "climbing_standing")]
    ClimbingStanding,   // Out of saddle (HIGH DRAG)
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for RiderPose {
    fn default() -> Self {
        RiderPose::Unknown
    }
}

/// Telemetry data extracted from screenshots
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TelemetryData {
    pub speed: Option<u32>,
    pub distance: Option<f64>,
    pub altitude: Option<u32>,
    pub race_time: Option<String>,
    pub power: Option<u32>,
    pub cadence: Option<u32>,
    pub heart_rate: Option<u32>,
    pub gradient: Option<f64>,
    pub distance_to_finish: Option<f64>,
    pub leaderboard: Option<Vec<LeaderboardEntry>>,
    pub rider_pose: Option<RiderPose>,
}

/// Leaderboard entry for a rider
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LeaderboardEntry {
    pub name: String,
    pub current: bool,
    pub delta: Option<String>,
    pub km: Option<f64>,
    pub wkg: Option<f64>,
}

/// Global configuration manager (initialized on first use)
static CONFIG_MANAGER: OnceLock<std::sync::Mutex<OcrConfigManager>> = OnceLock::new();

/// Initialize or get the config manager
pub fn get_config_manager() -> &'static std::sync::Mutex<OcrConfigManager> {
    CONFIG_MANAGER.get_or_init(|| {
        // Look for config directory relative to the executable or in standard locations
        let config_dir = if let Ok(exe_path) = std::env::current_exe() {
            // Try relative to executable first
            if let Some(parent) = exe_path.parent() {
                let ocr_configs = parent.join("ocr-configs");
                if ocr_configs.exists() {
                    ocr_configs
                } else {
                    // Try one level up (if running from target/debug or target/release)
                    if let Some(grandparent) = parent.parent() {
                        let ocr_configs = grandparent.join("ocr-configs");
                        if ocr_configs.exists() {
                            ocr_configs
                        } else {
                            // Default to current directory
                            std::path::PathBuf::from("ocr-configs")
                        }
                    } else {
                        std::path::PathBuf::from("ocr-configs")
                    }
                }
            } else {
                std::path::PathBuf::from("ocr-configs")
            }
        } else {
            std::path::PathBuf::from("ocr-configs")
        };
        
        std::sync::Mutex::new(OcrConfigManager::new(config_dir))
    })
}

/// Extract telemetry from a Zwift screenshot
pub fn extract_telemetry(image_path: &Path) -> Result<TelemetryData> {
    let img = image::open(image_path).context("Failed to open image")?;
    let mut ocr = LepTess::new(None, "eng").context("Failed to initialize Tesseract")?;
    
    // Configure for numbers  
    ocr.set_variable(Variable::TesseditCharWhitelist, "0123456789.:+-/kmhWrpmbg%")?;

    let mut data = TelemetryData::default();

    // Try to load configuration for image resolution
    let (width, height) = (img.width(), img.height());
    let config_manager = get_config_manager();
    let mut manager = config_manager.lock().unwrap();
    
    // Attempt to load config, fall back to hardcoded if not available
    let use_config = if !manager.has_config() {
        match manager.load_for_resolution(width, height) {
            Ok(_) => {
                eprintln!("Loaded OCR config for {}x{}", width, height);
                true
            }
            Err(e) => {
                eprintln!("Warning: No OCR config for {}x{}, using hardcoded regions: {}", width, height, e);
                false
            }
        }
    } else {
        true
    };

    if use_config && manager.has_config() {
        // Use configuration-based regions
        if let Some(regions_map) = manager.get_all_regions() {
            let regions: Vec<(&str, u32, u32, u32, u32)> = vec![
                "speed", "distance", "altitude", "race_time", "power", 
                "cadence", "heart_rate", "gradient", "distance_to_finish"
            ].into_iter()
            .filter_map(|name| {
                regions_map.get(name).map(|r| {
                    (name, r.x, r.y, r.width, r.height)
                })
            })
            .collect();
            
            extract_standard_fields(&mut data, &img, &mut ocr, &regions)?;
        }
    } else {
        // Fall back to hardcoded regions
        let regions = [
            ("speed", regions::SPEED.0, regions::SPEED.1, regions::SPEED.2, regions::SPEED.3),
            ("distance", regions::DISTANCE.0, regions::DISTANCE.1, regions::DISTANCE.2, regions::DISTANCE.3),
            ("altitude", regions::ALTITUDE.0, regions::ALTITUDE.1, regions::ALTITUDE.2, regions::ALTITUDE.3),
            ("race_time", regions::RACE_TIME.0, regions::RACE_TIME.1, regions::RACE_TIME.2, regions::RACE_TIME.3),
            ("power", regions::POWER.0, regions::POWER.1, regions::POWER.2, regions::POWER.3),
            ("cadence", regions::CADENCE.0, regions::CADENCE.1, regions::CADENCE.2, regions::CADENCE.3),
            ("heart_rate", regions::HEART_RATE.0, regions::HEART_RATE.1, regions::HEART_RATE.2, regions::HEART_RATE.3),
            ("gradient", regions::GRADIENT.0, regions::GRADIENT.1, regions::GRADIENT.2, regions::GRADIENT.3),
            ("distance_to_finish", regions::DISTANCE_TO_FINISH.0, regions::DISTANCE_TO_FINISH.1, regions::DISTANCE_TO_FINISH.2, regions::DISTANCE_TO_FINISH.3),
        ];
        
        extract_standard_fields(&mut data, &img, &mut ocr, &regions)?;
    }

    // Extract leaderboard
    data.leaderboard = extract_leaderboard(&img, &manager)?;
    
    // Extract rider pose
    data.rider_pose = extract_rider_pose(&img, &manager)?;

    Ok(data)
}

/// Extract standard telemetry fields from regions
fn extract_standard_fields(
    data: &mut TelemetryData,
    img: &DynamicImage,
    ocr: &mut LepTess,
    regions: &[(&str, u32, u32, u32, u32)],
) -> Result<()> {
    for (field, x, y, width, height) in regions {
        match *field {
            "gradient" => {
                // Special handling for gradient
                data.gradient = extract_gradient(img, *x, *y, *width, *height)?;
            }
            _ => {
                // Standard extraction for other fields
                let roi = img.crop_imm(*x, *y, *width, *height);
                
                let value = extract_field(ocr, &roi, field)?;
                
                match *field {
                    "speed" => data.speed = value.parse().ok(),
                    "distance" => data.distance = value.parse().ok(),
                    "altitude" => data.altitude = value.parse().ok(),
                    "race_time" => data.race_time = parse_time(&value),
                    "power" => data.power = value.parse().ok(),
                    "cadence" => data.cadence = value.parse().ok(),
                    "heart_rate" => data.heart_rate = value.parse().ok(),
                    "distance_to_finish" => data.distance_to_finish = value.parse().ok(),
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

/// Extract text from a region of interest
fn extract_field(ocr: &mut LepTess, roi: &DynamicImage, field: &str) -> Result<String> {
    // Different threshold for distance_to_finish (dimmer text)
    let threshold_value = if field == "distance_to_finish" { thresholds::DISTANCE_TO_FINISH } else { thresholds::DEFAULT };
    
    // Preprocess the image
    let buf = preprocess_for_ocr(roi, threshold_value, scale_factors::DEFAULT)?;
    
    ocr.set_image_from_mem(&buf)?;
    
    // Set page segmentation mode for single text line
    ocr.set_variable(Variable::TesseditPagesegMode, "7")?;
    
    let text = ocr.get_utf8_text()?;
    
    
    // Clean based on field type
    let clean_text = match field {
        "race_time" => text.trim().to_string(),
        "distance" | "distance_to_finish" => ocr_regex::NON_DIGITS_DECIMAL.replace_all(&text, "").to_string(),
        _ => ocr_regex::NON_DIGITS.replace_all(&text, "").to_string(),
    };
    
    Ok(clean_text)
}

/// Parse time from OCR text (MM:SS format)
#[doc(hidden)]
pub fn parse_time(text: &str) -> Option<String> {
    // Look for time format
    if let Some(caps) = ocr_regex::TIME_FORMAT.captures(text) {
        return Some(caps[1].to_string());
    }
    
    // Try to reconstruct from digits
    let digits: String = text.chars().filter(|c| c.is_ascii_digit()).collect();
    match digits.len() {
        4 => Some(format!("{}:{}", &digits[..2], &digits[2..])),
        3 => Some(format!("{}:{}", &digits[..1], &digits[1..])),
        _ => None,
    }
}

/// Extract gradient with special processing for stylized font
fn extract_gradient(img: &DynamicImage, x: u32, y: u32, width: u32, height: u32) -> Result<Option<f64>> {
    let roi = img.crop_imm(x, y, width, height);
    
    // Preprocess with gradient-specific settings
    let buf = preprocess_for_ocr(&roi, thresholds::GRADIENT, scale_factors::GRADIENT)?;
    
    let mut ocr = LepTess::new(None, "eng").context("Failed to initialize Tesseract")?;
    ocr.set_variable(Variable::TesseditCharWhitelist, "0123456789.-%")?;
    ocr.set_variable(Variable::TesseditPagesegMode, "7")?; // Single text line
    ocr.set_image_from_mem(&buf)?;
    
    let text = ocr.get_utf8_text()?;
    let clean_text = ocr_regex::NON_DIGITS_DECIMAL.replace_all(&text, "").to_string();
    
    Ok(clean_text.parse().ok())
}

/// Extract leaderboard data from the right side of the screen
fn extract_leaderboard(img: &DynamicImage, manager: &std::sync::MutexGuard<OcrConfigManager>) -> Result<Option<Vec<LeaderboardEntry>>> {
    // Get leaderboard region from config or use hardcoded
    let (x, y, width, height) = if let Some(region) = manager.get_region("leaderboard") {
        (region.x, region.y, region.width, region.height)
    } else {
        (regions::LEADERBOARD_X, regions::LEADERBOARD_Y, regions::LEADERBOARD_WIDTH, regions::LEADERBOARD_HEIGHT)
    };
    
    // Use ocrs for better accuracy on stylized UI text
    let text = crate::ocr_ocrs::extract_text_from_region(img, x, y, width, height)?;
    
    // Debug: print detected text
    if std::env::var("DEBUG_OCR").is_ok() {
        eprintln!("=== OCRS detected text ===");
        eprintln!("{}", text);
        eprintln!("=== End OCRS output ===");
    }
    
    // Parse text into entries
    let mut entries = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    let mut i = 0;
    
    while i < lines.len() {
        let line_text = lines[i].trim();
        
        // Check if this line looks like a rider name
        if is_likely_name(line_text) {
            let mut entry = LeaderboardEntry {
                name: line_text.to_string(),
                current: false,
                delta: None,
                km: None,
                wkg: None,
            };
            
            // Look for data in the next line
            if i + 1 < lines.len() {
                let data_line_text = lines[i + 1];
                parse_leaderboard_data(&mut entry, data_line_text);
                
                // Check if this is the current rider (no time delta but has other data)
                if entry.delta.is_none() && (entry.wkg.is_some() || entry.km.is_some()) {
                    entry.current = true;
                }
            }
            
            // Only add entries with some data
            if entry.wkg.is_some() || entry.km.is_some() || entry.delta.is_some() {
                entries.push(entry);
            }
        }
        
        i += 1;
    }
    
    if entries.is_empty() {
        Ok(None)
    } else {
        Ok(Some(entries))
    }
}

/// Check if text is likely a rider name
#[doc(hidden)]
pub fn is_likely_name(text: &str) -> bool {
    // Clean the text first
    let cleaned = text.trim();
    
    // Filter out obvious non-names
    if cleaned.len() < name_limits::MIN_LENGTH || cleaned.len() > name_limits::MAX_LENGTH {
        return false;
    }
    
    // Skip numeric-only entries
    if cleaned.chars().all(|c| c.is_numeric() || c == '.' || c == ',' || c == ':' || c == '+' || c == '-') {
        return false;
    }
    
    // Skip entries that are clearly data fields
    let lower = cleaned.to_lowercase();
    if lower.contains("km") || lower.contains("w/kg") || lower.contains("wkg") {
        return false;
    }
    
    // Skip time entries
    if cleaned.contains(':') && cleaned.chars().filter(|&c| c.is_numeric()).count() >= 3 {
        return false;
    }
    
    // Positive indicators for names
    // Has dots between letters (J.Chidley)
    if ocr_regex::NAME_INITIAL_DOT.is_match(cleaned) {
        return true;
    }
    
    // Multiple dots (C.J.Y.S)
    if cleaned.matches('.').count() >= 2 && cleaned.chars().any(|c| c.is_alphabetic()) {
        return true;
    }
    
    // Starts with uppercase and has lowercase (Laindre)
    if cleaned.chars().nth(0).map_or(false, |c| c.is_uppercase()) &&
       cleaned.chars().any(|c| c.is_lowercase()) {
        return true;
    }
    
    // Contains parenthesis (common in names)
    if cleaned.contains('(') || cleaned.contains(')') {
        return true;
    }
    
    // Single letter followed by dot
    if ocr_regex::SINGLE_LETTER_DOT.is_match(cleaned) {
        return true;
    }
    
    // At least has some letters and reasonable length
    cleaned.chars().filter(|c| c.is_alphabetic()).count() >= name_limits::MIN_LETTERS
}

/// Parse leaderboard data line
#[doc(hidden)]
pub fn parse_leaderboard_data(entry: &mut LeaderboardEntry, text: &str) {
    // Look for time delta (+00:00 or -00:00)
    if let Some(caps) = ocr_regex::TIME_DELTA.captures(text) {
        entry.delta = Some(caps[1].to_string());
    }
    
    // Look for distance (XX.X KM)
    if let Some(caps) = ocr_regex::DISTANCE_KM.captures(text) {
        entry.km = caps[1].parse().ok();
    }
    
    // Look for w/kg (X.X w/kg or just X.X in middle of line)
    if let Some(caps) = ocr_regex::WATTS_PER_KG.captures(text) {
        entry.wkg = caps[1].parse().ok();
    } else if let Some(caps) = ocr_regex::DECIMAL_NUMBER.captures(text) {
        // Check if this number is in a reasonable w/kg range
        if let Ok(val) = caps[1].parse::<f64>() {
            if (wkg::MIN..=wkg::MAX).contains(&val) {
                entry.wkg = Some(val);
            }
        }
    }
}

/// Extract rider pose from avatar region
pub fn extract_rider_pose(img: &DynamicImage, manager: &std::sync::MutexGuard<OcrConfigManager>) -> Result<Option<RiderPose>> {
    // Get avatar region from config or use hardcoded
    let (x, y, width, height) = if let Some(region) = manager.get_region("rider_pose_avatar") {
        (region.x, region.y, region.width, region.height)
    } else {
        (regions::AVATAR_X, regions::AVATAR_Y, regions::AVATAR_WIDTH, regions::AVATAR_HEIGHT)
    };
    
    let roi = img.crop_imm(x, y, width, height);
    let gray = roi.to_luma8();
    
    // Apply edge detection using Canny-like approach
    // First apply Gaussian blur to reduce noise
    let blurred = imageproc::filter::gaussian_blur_f32(&gray, edge_detection::GAUSSIAN_BLUR_SIGMA);
    
    // Apply Sobel edge detection
    let edges = imageproc::edges::canny(&blurred, edge_detection::CANNY_LOW_THRESHOLD, edge_detection::CANNY_HIGH_THRESHOLD);
    
    // Find contours (simplified approach using connected components)
    let _components = imageproc::region_labelling::connected_components(&edges, imageproc::region_labelling::Connectivity::Eight, Luma([edge_detection::EDGE_PIXEL_VALUE]));
    
    // Calculate features for pose detection
    let features = calculate_pose_features(&edges, &edges);
    
    // Classify pose based on features
    let pose = classify_pose(&features);
    
    Ok(Some(pose))
}

/// Features for pose detection
struct PoseFeatures {
    aspect_ratio: f32,
    center_of_mass_y: f32,
    #[allow(dead_code)]
    upper_density: f32,
    #[allow(dead_code)]
    lower_density: f32,
    #[allow(dead_code)]
    symmetry_score: f32,
}

/// Calculate pose features from edge image
fn calculate_pose_features(_components: &GrayImage, edges: &GrayImage) -> PoseFeatures {
    let height = edges.height() as f32;
    let width = edges.width() as f32;
    
    // Find bounding box of largest component (rider silhouette)
    let mut min_x = width;
    let mut max_x: f32 = 0.0;
    let mut min_y = height;
    let mut max_y: f32 = 0.0;
    let mut pixel_count = 0;
    let mut y_sum = 0.0;
    
    for y in 0..edges.height() {
        for x in 0..edges.width() {
            if edges.get_pixel(x, y)[0] > 0 {
                let fx = x as f32;
                let fy = y as f32;
                min_x = min_x.min(fx);
                max_x = max_x.max(fx);
                min_y = min_y.min(fy);
                max_y = max_y.max(fy);
                y_sum += fy;
                pixel_count += 1;
            }
        }
    }
    
    let bbox_width = (max_x - min_x).max(1.0);
    let bbox_height = (max_y - min_y).max(1.0);
    let aspect_ratio = bbox_height / bbox_width;
    
    let center_of_mass_y = if pixel_count > 0 {
        (y_sum / pixel_count as f32) / height
    } else {
        0.5
    };
    
    // Calculate upper/lower density
    let mid_y = edges.height() / 2;
    let mut upper_pixels = 0;
    let mut lower_pixels = 0;
    
    for y in 0..edges.height() {
        for x in 0..edges.width() {
            if edges.get_pixel(x, y)[0] > 0 {
                if y < mid_y {
                    upper_pixels += 1;
                } else {
                    lower_pixels += 1;
                }
            }
        }
    }
    
    let total_upper = (edges.width() * mid_y) as f32;
    let total_lower = (edges.width() * (edges.height() - mid_y)) as f32;
    
    let upper_density = upper_pixels as f32 / total_upper.max(1.0);
    let lower_density = lower_pixels as f32 / total_lower.max(1.0);
    
    // Calculate symmetry
    let mid_x = edges.width() / 2;
    let mut symmetry_diff = 0;
    let mut symmetry_pixels = 0;
    
    for y in 0..edges.height() {
        for x in 0..mid_x {
            let left_pixel = edges.get_pixel(x, y)[0];
            let right_x = edges.width() - 1 - x;
            let right_pixel = edges.get_pixel(right_x, y)[0];
            
            if left_pixel != right_pixel {
                symmetry_diff += 1;
            }
            symmetry_pixels += 1;
        }
    }
    
    let symmetry_score = 1.0 - (symmetry_diff as f32 / symmetry_pixels.max(1) as f32);
    
    PoseFeatures {
        aspect_ratio,
        center_of_mass_y,
        upper_density,
        lower_density,
        symmetry_score,
    }
}

/// Classify pose based on extracted features
fn classify_pose(features: &PoseFeatures) -> RiderPose {
    // Thresholds based on Python calibration data
    
    // Check for standing (high aspect ratio, low center of mass)
    if features.aspect_ratio > pose::ASPECT_RATIO_STANDING_MIN && features.center_of_mass_y < pose::CENTER_OF_MASS_STANDING_MAX {
        return RiderPose::ClimbingStanding;
    }
    
    // Check for tuck (low aspect ratio, high center of mass)
    if features.aspect_ratio < pose::ASPECT_RATIO_TUCK_MAX && features.center_of_mass_y > pose::CENTER_OF_MASS_TUCK_MIN {
        return RiderPose::NormalTuck;
    }
    
    // Check for seated climbing (medium aspect ratio, slightly forward lean)
    if features.aspect_ratio > pose::ASPECT_RATIO_SEATED_MIN && features.aspect_ratio < pose::ASPECT_RATIO_SEATED_MAX && 
       features.center_of_mass_y > pose::CENTER_OF_MASS_SEATED_MIN && features.center_of_mass_y < pose::CENTER_OF_MASS_SEATED_MAX {
        return RiderPose::ClimbingSeated;
    }
    
    // Check for normal upright (medium-high aspect ratio, centered)
    if features.aspect_ratio > pose::ASPECT_RATIO_NORMAL_MIN && features.aspect_ratio < pose::ASPECT_RATIO_NORMAL_MAX &&
       features.center_of_mass_y > pose::CENTER_OF_MASS_NORMAL_MIN && features.center_of_mass_y < pose::CENTER_OF_MASS_NORMAL_MAX {
        return RiderPose::NormalNormal;
    }
    
    // Default to unknown if no clear match
    RiderPose::Unknown
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Luma};
    
    /// Create a test edge image with known properties
    fn create_test_edge_image(width: u32, height: u32, pattern: &str) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let mut img = ImageBuffer::new(width, height);
        
        match pattern {
            "centered_square" => {
                // Draw a square in the center
                let quarter_w = width / 4;
                let quarter_h = height / 4;
                for y in quarter_h..3*quarter_h {
                    for x in quarter_w..3*quarter_w {
                        img.put_pixel(x, y, Luma([255]));
                    }
                }
            }
            "top_heavy" => {
                // More pixels in top half
                for y in 0..height/2 {
                    for x in 0..width {
                        if (x + y) % 3 == 0 {
                            img.put_pixel(x, y, Luma([255]));
                        }
                    }
                }
            }
            "symmetric" => {
                // Create perfectly symmetric pattern
                let mid_x = width / 2;
                for y in 0..height {
                    for x in 0..mid_x {
                        if (x + y) % 5 == 0 {
                            img.put_pixel(x, y, Luma([255]));
                            img.put_pixel(width - 1 - x, y, Luma([255]));
                        }
                    }
                }
            }
            _ => {}
        }
        
        img
    }
    
    #[test]
    fn test_calculate_pose_features_aspect_ratio() {
        // This test catches: replace - with / in bbox_height calculation
        let edges = create_test_edge_image(100, 100, "centered_square");
        let features = calculate_pose_features(&edges, &edges);
        
        // A centered square should have aspect ratio close to 1.0
        assert!((features.aspect_ratio - 1.0).abs() < 0.2, 
                "Expected aspect ratio ~1.0, got {}", features.aspect_ratio);
    }
    
    #[test]
    fn test_calculate_pose_features_center_of_mass() {
        // This test catches: replace += with -= in y_sum
        let top_heavy = create_test_edge_image(100, 100, "top_heavy");
        let features = calculate_pose_features(&top_heavy, &top_heavy);
        
        // Top heavy image should have center of mass < 0.5
        assert!(features.center_of_mass_y < 0.5, 
                "Top heavy image should have center of mass < 0.5, got {}", 
                features.center_of_mass_y);
    }
    
    #[test]
    fn test_calculate_pose_features_division_not_modulo() {
        // This test catches: replace / with % in center_of_mass_y
        let edges = create_test_edge_image(100, 100, "centered_square");
        let features = calculate_pose_features(&edges, &edges);
        
        // Center of mass should be between 0 and 1 (normalized)
        assert!(features.center_of_mass_y >= 0.0 && features.center_of_mass_y <= 1.0,
                "Center of mass should be normalized 0-1, got {}", 
                features.center_of_mass_y);
    }
    
    #[test]
    fn test_classify_pose_standing_threshold() {
        // This test catches: replace > with ==, replace > with >=
        let features = PoseFeatures {
            aspect_ratio: pose::ASPECT_RATIO_STANDING_MIN + 0.01,
            center_of_mass_y: pose::CENTER_OF_MASS_STANDING_MAX - 0.01,
            upper_density: 0.5,
            lower_density: 0.5,
            symmetry_score: 0.5,
        };
        
        assert_eq!(classify_pose(&features), RiderPose::ClimbingStanding);
        
        // Test boundary
        let boundary_features = PoseFeatures {
            aspect_ratio: pose::ASPECT_RATIO_STANDING_MIN - 0.01,
            center_of_mass_y: pose::CENTER_OF_MASS_STANDING_MAX - 0.01,
            upper_density: 0.5,
            lower_density: 0.5,
            symmetry_score: 0.5,
        };
        
        assert_ne!(classify_pose(&boundary_features), RiderPose::ClimbingStanding);
    }
    
    #[test]
    fn test_classify_pose_logical_and() {
        // This test catches: replace && with ||
        let features = PoseFeatures {
            aspect_ratio: pose::ASPECT_RATIO_STANDING_MIN + 0.1,  // Satisfies
            center_of_mass_y: pose::CENTER_OF_MASS_STANDING_MAX + 0.1, // Does NOT satisfy
            upper_density: 0.5,
            lower_density: 0.5,
            symmetry_score: 0.5,
        };
        
        // Should NOT be standing because BOTH conditions must be true
        assert_ne!(classify_pose(&features), RiderPose::ClimbingStanding);
    }
    
    #[test]
    fn test_is_likely_name_numeric_check() {
        // This test catches: replace || with && in numeric check
        assert!(!is_likely_name("123"));        // Only numbers
        assert!(!is_likely_name("..."));        // Only punctuation
        assert!(!is_likely_name("123.45"));     // Numbers and punctuation
        assert!(is_likely_name("John123"));     // Letters and numbers
        assert!(is_likely_name("Test"));        // Only letters
    }
    
    #[test]
    fn test_is_likely_name_length_check() {
        // This test catches: replace < with >, replace > with ==
        assert!(!is_likely_name(""));          // Too short
        assert!(!is_likely_name("A"));         // Below minimum
        assert!(is_likely_name("AB"));         // At minimum (assuming MIN_LENGTH is 2)
        
        let long_name = "A".repeat(51);       // Above maximum (assuming MAX_LENGTH is 50)
        assert!(!is_likely_name(&long_name));
    }
    
    #[test]
    fn test_parse_time_valid_formats() {
        // This test catches: digit extraction logic mutations
        assert_eq!(parse_time("15:42"), Some("15:42".to_string()));
        assert_eq!(parse_time("5:42"), Some("5:42".to_string()));
        assert_eq!(parse_time("text 12:34 text"), Some("12:34".to_string()));
        
        // Test digit reconstruction (catches index mutations)
        assert_eq!(parse_time("1234"), Some("12:34".to_string()));
        assert_eq!(parse_time("523"), Some("5:23".to_string()));
        
        // Invalid formats
        assert_eq!(parse_time("12"), None);
        assert_eq!(parse_time("12345"), None);
        assert_eq!(parse_time(""), None);
    }
    
    #[test]
    fn test_parse_leaderboard_data_time_delta() {
        // This test catches: regex capture group mutations
        let mut entry = LeaderboardEntry {
            name: "Test".to_string(),
            current: false,
            delta: None,
            km: None,
            wkg: None,
        };
        
        parse_leaderboard_data(&mut entry, "+01:23");
        assert_eq!(entry.delta, Some("+01:23".to_string()));
        
        entry.delta = None;
        parse_leaderboard_data(&mut entry, "-00:45");
        assert_eq!(entry.delta, Some("-00:45".to_string()));
        
        // No delta in text
        entry.delta = None;
        parse_leaderboard_data(&mut entry, "3.5 w/kg");
        assert_eq!(entry.delta, None);
    }
    
    #[test]
    fn test_parse_leaderboard_data_distance() {
        // This test catches: parse().ok() mutations
        let mut entry = LeaderboardEntry {
            name: "Test".to_string(),
            current: false,
            delta: None,
            km: None,
            wkg: None,
        };
        
        parse_leaderboard_data(&mut entry, "12.5 km");
        assert_eq!(entry.km, Some(12.5));
        
        entry.km = None;
        parse_leaderboard_data(&mut entry, "5.0 KM");
        assert_eq!(entry.km, Some(5.0));
        
        // Invalid distance
        entry.km = None;
        parse_leaderboard_data(&mut entry, "abc km");
        assert_eq!(entry.km, None);
    }
    
    #[test]
    fn test_parse_leaderboard_data_wkg() {
        // This test catches: range check mutations (MIN..=MAX)
        let mut entry = LeaderboardEntry {
            name: "Test".to_string(),
            current: false,
            delta: None,
            km: None,
            wkg: None,
        };
        
        // Valid w/kg
        parse_leaderboard_data(&mut entry, "3.5 w/kg");
        assert_eq!(entry.wkg, Some(3.5));
        
        // Edge case: at MIN boundary
        entry.wkg = None;
        parse_leaderboard_data(&mut entry, "0.5");  // wkg::MIN
        assert_eq!(entry.wkg, Some(0.5));
        
        // Edge case: at MAX boundary  
        entry.wkg = None;
        parse_leaderboard_data(&mut entry, "7.0");  // wkg::MAX
        assert_eq!(entry.wkg, Some(7.0));
        
        // Out of range (catches > to >= mutations)
        entry.wkg = None;
        parse_leaderboard_data(&mut entry, "0.4");  // Below MIN
        assert_eq!(entry.wkg, None);
        
        entry.wkg = None;
        parse_leaderboard_data(&mut entry, "7.1");  // Above MAX
        assert_eq!(entry.wkg, None);
    }
    
    #[test]
    fn test_parse_time_digit_slicing() {
        // This test specifically catches slice index mutations
        // 4 digits: [..2] and [2..]
        assert_eq!(parse_time("1234"), Some("12:34".to_string()));
        assert_eq!(parse_time("0056"), Some("00:56".to_string()));
        
        // 3 digits: [..1] and [1..]
        assert_eq!(parse_time("123"), Some("1:23".to_string()));
        assert_eq!(parse_time("959"), Some("9:59".to_string()));
        
        // Edge cases that would fail with wrong indices
        assert_eq!(parse_time("0000"), Some("00:00".to_string()));
        assert_eq!(parse_time("000"), Some("0:00".to_string()));
    }
    
    #[test]
    fn test_is_likely_name_multiple_dots() {
        // This test catches: replace >= with < in multiple dots check
        // Exactly 2 dots should be valid
        assert!(is_likely_name("C.J.S"));          // Exactly 2 dots with letters
        assert!(is_likely_name("A.B.C"));          // Exactly 2 dots
        assert!(is_likely_name("J.P.Morgan"));     // 2 dots with more text
        
        // 1 dot should not trigger this rule (but may pass other rules)
        let single_dot = is_likely_name("J.Smith");
        // Should still pass due to initial dot rule
        assert!(single_dot);
        
        // Only dots and no letters should fail
        assert!(!is_likely_name("..."));           // 2 dots but no letters
        assert!(!is_likely_name(".."));            // 1 dot, no letters
        
        // More than 2 dots
        assert!(is_likely_name("C.J.Y.S."));       // 3 dots with letters
        assert!(is_likely_name("A.B.C.D.E."));     // Many dots with letters
    }
}