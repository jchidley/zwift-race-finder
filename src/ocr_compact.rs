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
    ];

    for (field, x, y, width, height) in regions {
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
            _ => {}
        }
    }

    Ok(data)
}

/// Extract text from a region of interest
fn extract_field(ocr: &mut LepTess, roi: &DynamicImage, field: &str) -> Result<String> {
    // Preprocess: convert to grayscale, threshold, scale up 3x
    let gray = roi.to_luma8();
    let binary = threshold(&gray, 200);
    
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
    let text = ocr.get_utf8_text()?;
    
    // Clean based on field type
    let clean_text = match field {
        "race_time" => text.trim().to_string(),
        "distance" => Regex::new(r"[^0-9.]")?.replace_all(&text, "").to_string(),
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