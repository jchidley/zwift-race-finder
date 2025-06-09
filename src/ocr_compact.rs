// ABOUTME: Compact OCR module for extracting telemetry from Zwift screenshots
// Minimal implementation using only Tesseract and basic image processing

use anyhow::{Context, Result};
use leptess::{LepTess, Variable};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;
use image::DynamicImage;
use imageproc::contrast::threshold;

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
}

/// Leaderboard entry for a rider
#[derive(Debug, Serialize, Deserialize)]
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