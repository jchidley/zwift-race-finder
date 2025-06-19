// ABOUTME: Debug tool for OCR extraction showing each processing step
// Helps diagnose why OCR might fail on specific regions

use anyhow::{Context, Result};
use clap::Parser;
use image::DynamicImage;
use leptess::{LepTess, Variable};
use std::path::PathBuf;
use zwift_race_finder::ocr_config::{OcrConfigManager, RegionConfig};
use zwift_race_finder::ocr_image_processing::preprocess_for_ocr;

#[derive(Parser, Debug)]
#[command(author, version, about = "Debug OCR extraction for Zwift screenshots")]
struct Args {
    /// Path to the screenshot to debug
    image_path: PathBuf,
    
    /// Specific region to debug (e.g., speed, power, distance)
    #[arg(short, long)]
    region: Option<String>,
    
    /// Save intermediate images
    #[arg(short, long)]
    save_intermediates: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("=== OCR Debug Tool ===");
    println!("Image: {:?}", args.image_path);
    
    // Load image
    let img = image::open(&args.image_path).context("Failed to open image")?;
    let (width, height) = (img.width(), img.height());
    println!("Resolution: {}x{}", width, height);
    
    // Try to load config
    let config_dir = PathBuf::from("ocr-configs");
    let mut config_manager = OcrConfigManager::new(config_dir);
    
    let using_config = match config_manager.load_for_resolution(width, height) {
        Ok(_) => {
            println!("✓ Loaded OCR config for {}x{}", width, height);
            true
        }
        Err(e) => {
            println!("✗ No OCR config found: {}", e);
            println!("  Using hardcoded regions");
            false
        }
    };
    
    // Get regions to test
    let regions_to_test: Vec<(String, RegionConfig)> = if let Some(specific) = args.region {
        // Test specific region
        if using_config {
            if let Some(regions_map) = config_manager.get_all_regions() {
                if let Some(region) = regions_map.get(&specific) {
                    vec![(specific, region.clone())]
                } else {
                    println!("Region '{}' not found in config", specific);
                    return Ok(());
                }
            } else {
                println!("No regions in config");
                return Ok(());
            }
        } else {
            // Use hardcoded region
            use zwift_race_finder::ocr_constants::regions;
            let hardcoded = match specific.as_str() {
                "speed" => Some(("speed", regions::SPEED.0, regions::SPEED.1, regions::SPEED.2, regions::SPEED.3)),
                "power" => Some(("power", regions::POWER.0, regions::POWER.1, regions::POWER.2, regions::POWER.3)),
                "distance" => Some(("distance", regions::DISTANCE.0, regions::DISTANCE.1, regions::DISTANCE.2, regions::DISTANCE.3)),
                "altitude" => Some(("altitude", regions::ALTITUDE.0, regions::ALTITUDE.1, regions::ALTITUDE.2, regions::ALTITUDE.3)),
                "race_time" => Some(("race_time", regions::RACE_TIME.0, regions::RACE_TIME.1, regions::RACE_TIME.2, regions::RACE_TIME.3)),
                "cadence" => Some(("cadence", regions::CADENCE.0, regions::CADENCE.1, regions::CADENCE.2, regions::CADENCE.3)),
                "heart_rate" => Some(("heart_rate", regions::HEART_RATE.0, regions::HEART_RATE.1, regions::HEART_RATE.2, regions::HEART_RATE.3)),
                _ => None,
            };
            
            if let Some((name, x, y, w, h)) = hardcoded {
                vec![(name.to_string(), RegionConfig { x, y, width: w, height: h, note: None })]
            } else {
                println!("Unknown region: {}", specific);
                return Ok(());
            }
        }
    } else {
        // Test all regions
        if using_config {
            if let Some(regions_map) = config_manager.get_all_regions() {
                regions_map.into_iter()
                    .filter(|(name, _)| !matches!(name.as_str(), "leaderboard" | "rider_pose_avatar"))
                    .map(|(name, region)| (name.clone(), region.clone()))
                    .collect()
            } else {
                vec![]
            }
        } else {
            println!("Please specify a region with --region when not using config");
            return Ok(());
        }
    };
    
    // Initialize Tesseract
    let mut ocr = LepTess::new(None, "eng").context("Failed to initialize Tesseract")?;
    
    // Process each region
    for (name, region) in regions_to_test {
        println!("\n{}", "=".repeat(60));
        println!("Testing region: {}", name);
        println!("Position: ({}, {}) Size: {}x{}", region.x, region.y, region.width, region.height);
        
        // Extract ROI
        let roi = img.crop_imm(region.x, region.y, region.width, region.height);
        
        if args.save_intermediates {
            let roi_path = format!("debug_{}_1_original.png", name);
            roi.save(&roi_path)?;
            println!("Saved original ROI: {}", roi_path);
        }
        
        // Test with different preprocessing options
        test_ocr_variations(&mut ocr, &roi, &name, args.save_intermediates)?;
    }
    
    Ok(())
}

fn test_ocr_variations(ocr: &mut LepTess, roi: &DynamicImage, field_name: &str, save: bool) -> Result<()> {
    println!("\n--- Testing different preprocessing options ---");
    
    // Test 1: Standard preprocessing
    println!("\n1. Standard preprocessing (threshold=200, scale=3x):");
    test_single_variation(ocr, roi, field_name, 200, 3, "standard", save)?;
    
    // Test 2: Lower threshold
    println!("\n2. Lower threshold (threshold=150, scale=3x):");
    test_single_variation(ocr, roi, field_name, 150, 3, "low_threshold", save)?;
    
    // Test 3: No scaling
    println!("\n3. No scaling (threshold=200, scale=1x):");
    test_single_variation(ocr, roi, field_name, 200, 1, "no_scale", save)?;
    
    // Test 4: Higher threshold
    println!("\n4. Higher threshold (threshold=230, scale=3x):");
    test_single_variation(ocr, roi, field_name, 230, 3, "high_threshold", save)?;
    
    // Test 5: Different character whitelists
    println!("\n5. Testing different character whitelists:");
    
    // Standard whitelist
    ocr.set_variable(Variable::TesseditCharWhitelist, "0123456789.:+-/kmhWrpmbg%")?;
    let buf = preprocess_for_ocr(roi, 200, 3)?;
    ocr.set_image_from_mem(&buf)?;
    ocr.set_variable(Variable::TesseditPagesegMode, "7")?;
    let text1 = ocr.get_utf8_text()?;
    println!("  Standard whitelist: '{}'", text1.trim());
    
    // Numbers only
    ocr.set_variable(Variable::TesseditCharWhitelist, "0123456789.")?;
    ocr.set_image_from_mem(&buf)?;
    let text2 = ocr.get_utf8_text()?;
    println!("  Numbers only: '{}'", text2.trim());
    
    // No whitelist
    ocr.set_variable(Variable::TesseditCharWhitelist, "")?;
    ocr.set_image_from_mem(&buf)?;
    let text3 = ocr.get_utf8_text()?;
    println!("  No whitelist: '{}'", text3.trim());
    
    // Test 6: Different page segmentation modes
    println!("\n6. Testing different page segmentation modes:");
    ocr.set_variable(Variable::TesseditCharWhitelist, "0123456789.:+-/kmhWrpmbg%")?;
    
    for mode in &["6", "7", "8", "11", "13"] {
        ocr.set_variable(Variable::TesseditPagesegMode, mode)?;
        ocr.set_image_from_mem(&buf)?;
        let text = ocr.get_utf8_text()?;
        println!("  Mode {} ({}): '{}'", mode, psm_description(mode), text.trim());
    }
    
    Ok(())
}

fn test_single_variation(
    ocr: &mut LepTess,
    roi: &DynamicImage,
    field_name: &str,
    threshold: u8,
    scale: u32,
    variant_name: &str,
    save: bool,
) -> Result<()> {
    // Reset to standard settings
    ocr.set_variable(Variable::TesseditCharWhitelist, "0123456789.:+-/kmhWrpmbg%")?;
    ocr.set_variable(Variable::TesseditPagesegMode, "7")?;
    
    // Preprocess
    let buf = preprocess_for_ocr(roi, threshold, scale)?;
    
    if save {
        // Save the preprocessed image for inspection
        let debug_path = format!("debug_{}_2_{}.png", field_name, variant_name);
        
        // Reconstruct image from buffer for saving
        let img = image::load_from_memory(&buf)?;
        img.save(&debug_path)?;
        println!("  Saved preprocessed: {}", debug_path);
    }
    
    // Run OCR
    ocr.set_image_from_mem(&buf)?;
    let raw_text = ocr.get_utf8_text()?;
    
    // Get confidence
    let confidence = ocr.mean_text_conf();
    
    println!("  Raw OCR result: '{}' (confidence: {})", raw_text.trim(), confidence);
    
    // Apply field-specific cleaning
    let cleaned = clean_for_field(&raw_text, field_name);
    if cleaned != raw_text.trim() {
        println!("  After cleaning: '{}'", cleaned);
    }
    
    Ok(())
}

fn clean_for_field(text: &str, field: &str) -> String {
    use regex::Regex;
    lazy_static::lazy_static! {
        static ref NON_DIGITS: Regex = Regex::new(r"[^\d]").unwrap();
        static ref NON_DIGITS_DECIMAL: Regex = Regex::new(r"[^\d.]").unwrap();
    }
    
    match field {
        "race_time" => text.trim().to_string(),
        "distance" | "distance_to_finish" => NON_DIGITS_DECIMAL.replace_all(text, "").to_string(),
        _ => NON_DIGITS.replace_all(text, "").to_string(),
    }
}

fn psm_description(mode: &str) -> &'static str {
    match mode {
        "6" => "Uniform block of text",
        "7" => "Single text line",
        "8" => "Single word",
        "11" => "Sparse text, find as much as possible",
        "13" => "Raw line",
        _ => "Unknown"
    }
}