// ABOUTME: Compact OCR module for extracting telemetry from Zwift screenshots
// Minimal implementation using only Tesseract and basic image processing

use anyhow::{Context, Result};
use leptess::{LepTess, Variable};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;
use image::{DynamicImage, GrayImage, Luma};
use imageproc::contrast::threshold;

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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeaderboardEntry {
    pub name: String,
    pub current: bool,
    pub delta: Option<String>,
    pub km: Option<f64>,
    pub wkg: Option<f64>,
}

/// Extract telemetry from a Zwift screenshot
pub fn extract_telemetry(image_path: &Path) -> Result<TelemetryData> {
    let img = image::open(image_path).context("Failed to open image")?;
    let mut ocr = LepTess::new(None, "eng").context("Failed to initialize Tesseract")?;
    
    // Configure for numbers  
    ocr.set_variable(Variable::TesseditCharWhitelist, "0123456789.:+-/kmhWrpmbg%")?;

    let mut data = TelemetryData::default();

    // Define regions (1920x1080 resolution)
    let regions = [
        ("speed", 693, 44, 71, 61),
        ("distance", 833, 44, 84, 55),
        ("altitude", 975, 45, 75, 50),
        ("race_time", 1070, 45, 134, 49),
        ("power", 268, 49, 117, 61),
        ("cadence", 240, 135, 45, 31),
        ("heart_rate", 341, 129, 69, 38),
        ("gradient", 1695, 71, 50, 50),
        ("distance_to_finish", 1143, 138, 50, 27),
    ];

    for (field, x, y, width, height) in regions {
        match field {
            "gradient" => {
                // Special handling for gradient
                data.gradient = extract_gradient(&img, x, y, width, height)?;
            }
            _ => {
                // Standard extraction for other fields
                let roi = img.crop_imm(x, y, width, height);
                
                let value = extract_field(&mut ocr, &roi, field)?;
                
                match field {
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

    // Extract leaderboard
    data.leaderboard = extract_leaderboard(&img)?;
    
    // Extract rider pose
    data.rider_pose = extract_rider_pose(&img)?;

    Ok(data)
}

/// Extract text from a region of interest
fn extract_field(ocr: &mut LepTess, roi: &DynamicImage, field: &str) -> Result<String> {
    // Preprocess: convert to grayscale, threshold, scale up 3x
    let gray = roi.to_luma8();
    
    // Different threshold for distance_to_finish (dimmer text)
    let threshold_value = if field == "distance_to_finish" { 150 } else { 200 };
    let binary = threshold(&gray, threshold_value);
    
    let scaled = image::imageops::resize(
        &binary,
        binary.width() * 3,
        binary.height() * 3,
        image::imageops::FilterType::CatmullRom,
    );
    
    // Convert to PNG for Tesseract
    let mut buf = Vec::new();
    image::DynamicImage::ImageLuma8(scaled)
        .write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png)?;
    
    ocr.set_image_from_mem(&buf)?;
    
    // Set page segmentation mode for single text line
    ocr.set_variable(Variable::TesseditPagesegMode, "7")?;
    
    let text = ocr.get_utf8_text()?;
    
    
    // Clean based on field type
    let clean_text = match field {
        "race_time" => text.trim().to_string(),
        "distance" | "distance_to_finish" => Regex::new(r"[^0-9.]")?.replace_all(&text, "").to_string(),
        _ => Regex::new(r"[^0-9]")?.replace_all(&text, "").to_string(),
    };
    
    Ok(clean_text)
}

/// Parse time from OCR text (MM:SS format)
fn parse_time(text: &str) -> Option<String> {
    // Look for time format
    if let Some(caps) = Regex::new(r"(\d{1,2}:\d{2})").ok()?.captures(text) {
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
    
    let gray = roi.to_luma8();
    
    // Don't invert - gradient is bright text on dark background
    // Threshold at a lower value to capture the yellow/orange text
    let binary = threshold(&gray, 150);
    
    // Scale 4x for better OCR
    let scaled = image::imageops::resize(
        &binary,
        binary.width() * 4,
        binary.height() * 4,
        image::imageops::FilterType::CatmullRom,
    );
    
    // Convert to PNG for Tesseract
    let mut buf = Vec::new();
    image::DynamicImage::ImageLuma8(scaled)
        .write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png)?;
    
    let mut ocr = LepTess::new(None, "eng").context("Failed to initialize Tesseract")?;
    ocr.set_variable(Variable::TesseditCharWhitelist, "0123456789.-%")?;
    ocr.set_variable(Variable::TesseditPagesegMode, "7")?; // Single text line
    ocr.set_image_from_mem(&buf)?;
    
    let text = ocr.get_utf8_text()?;
    let clean_text = Regex::new(r"[^0-9.]")?.replace_all(&text, "").to_string();
    
    Ok(clean_text.parse().ok())
}

/// Extract leaderboard data from the right side of the screen
fn extract_leaderboard(img: &DynamicImage) -> Result<Option<Vec<LeaderboardEntry>>> {
    // Leaderboard region (right side of screen)
    let x = 1500;
    let y = 200;
    let width = 420;
    let height = 600;
    
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
fn is_likely_name(text: &str) -> bool {
    // Clean the text first
    let cleaned = text.trim();
    
    // Filter out obvious non-names
    if cleaned.len() < 2 || cleaned.len() > 30 {
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
    if Regex::new(r"^[A-Z]\.[A-Za-z]").unwrap().is_match(cleaned) {
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
    if Regex::new(r"^[A-Za-z]\.$").unwrap().is_match(cleaned) {
        return true;
    }
    
    // At least has some letters and reasonable length
    cleaned.chars().filter(|c| c.is_alphabetic()).count() >= 2
}

/// Parse leaderboard data line
fn parse_leaderboard_data(entry: &mut LeaderboardEntry, text: &str) {
    // Look for time delta (+00:00 or -00:00)
    if let Some(caps) = Regex::new(r"([+-]\d{1,2}:\d{2})").unwrap().captures(text) {
        entry.delta = Some(caps[1].to_string());
    }
    
    // Look for distance (XX.X KM)
    if let Some(caps) = Regex::new(r"(\d+\.?\d*)\s*[Kk][Mm]").unwrap().captures(text) {
        entry.km = caps[1].parse().ok();
    }
    
    // Look for w/kg (X.X w/kg or just X.X in middle of line)
    if let Some(caps) = Regex::new(r"(\d+\.\d+)\s*w/kg").unwrap().captures(text) {
        entry.wkg = caps[1].parse().ok();
    } else if let Some(caps) = Regex::new(r"(\d+\.\d+)").unwrap().captures(text) {
        // Check if this number is in a reasonable w/kg range
        if let Ok(val) = caps[1].parse::<f64>() {
            if (0.5..=7.0).contains(&val) {
                entry.wkg = Some(val);
            }
        }
    }
}

/// Extract rider pose from avatar region
fn extract_rider_pose(img: &DynamicImage) -> Result<Option<RiderPose>> {
    // Avatar region (center of screen)
    let x = 860;
    let y = 400;
    let width = 200;
    let height = 300;
    
    let roi = img.crop_imm(x, y, width, height);
    let gray = roi.to_luma8();
    
    // Apply edge detection using Canny-like approach
    // First apply Gaussian blur to reduce noise
    let blurred = imageproc::filter::gaussian_blur_f32(&gray, 1.0);
    
    // Apply Sobel edge detection
    let edges = imageproc::edges::canny(&blurred, 50.0, 150.0);
    
    // Find contours (simplified approach using connected components)
    let components = imageproc::region_labelling::connected_components(&edges, imageproc::region_labelling::Connectivity::Eight, Luma([255u8]));
    
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
    upper_density: f32,
    lower_density: f32,
    symmetry_score: f32,
}

/// Calculate pose features from edge image
fn calculate_pose_features(components: &GrayImage, edges: &GrayImage) -> PoseFeatures {
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
    if features.aspect_ratio > 1.7 && features.center_of_mass_y < 0.45 {
        return RiderPose::ClimbingStanding;
    }
    
    // Check for tuck (low aspect ratio, high center of mass)
    if features.aspect_ratio < 1.3 && features.center_of_mass_y > 0.55 {
        return RiderPose::NormalTuck;
    }
    
    // Check for seated climbing (medium aspect ratio, slightly forward lean)
    if features.aspect_ratio > 1.4 && features.aspect_ratio < 1.8 && 
       features.center_of_mass_y > 0.45 && features.center_of_mass_y < 0.6 {
        return RiderPose::ClimbingSeated;
    }
    
    // Check for normal upright (medium-high aspect ratio, centered)
    if features.aspect_ratio > 1.3 && features.aspect_ratio < 1.7 &&
       features.center_of_mass_y > 0.45 && features.center_of_mass_y < 0.55 {
        return RiderPose::NormalNormal;
    }
    
    // Default to unknown if no clear match
    RiderPose::Unknown
}