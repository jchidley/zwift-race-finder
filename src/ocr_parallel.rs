// ABOUTME: Parallel OCR implementation for high-performance telemetry extraction
// Uses rayon for data parallelism and cached OCR engines for efficiency

use anyhow::{Context, Result};
use image::DynamicImage;
use leptess::{LepTess, Variable};
use once_cell::sync::Lazy;
use rayon::prelude::*;
use rten_tensor::NdTensor;
use std::sync::{Arc, Mutex};

use crate::ocr_compact::{LeaderboardEntry, RiderPose, TelemetryData};
use crate::ocr_constants::{regions, scale_factors, thresholds};
use crate::ocr_image_processing::preprocess_for_ocr;
use crate::ocr_regex;

/// Cached OCRS engine for leaderboard extraction
static OCRS_ENGINE: Lazy<Mutex<ocrs::OcrEngine>> = Lazy::new(|| {
    Mutex::new(
        crate::ocr_ocrs::create_engine()
            .expect("Failed to create OCRS engine for parallel processing"),
    )
});

/// Pool of Tesseract instances for parallel field extraction
static TESSERACT_POOL: Lazy<Mutex<Vec<LepTess>>> = Lazy::new(|| {
    let pool_size = rayon::current_num_threads().min(8);
    let mut pool = Vec::with_capacity(pool_size);
    
    for _ in 0..pool_size {
        let mut ocr = LepTess::new(None, "eng").expect("Failed to initialize Tesseract");
        ocr.set_variable(Variable::TesseditCharWhitelist, "0123456789.:+-/kmhWrpmbg%")
            .expect("Failed to set character whitelist");
        pool.push(ocr);
    }
    
    Mutex::new(pool)
});

/// Result of field extraction
enum FieldValue {
    Speed(Option<u32>),
    Distance(Option<f64>),
    Altitude(Option<u32>),
    RaceTime(Option<String>),
    Power(Option<u32>),
    Cadence(Option<u32>),
    HeartRate(Option<u32>),
    Gradient(Option<f64>),
    DistanceToFinish(Option<f64>),
}

/// Extract telemetry using parallel processing
pub fn extract_telemetry_parallel(image_path: &std::path::Path) -> Result<TelemetryData> {
    let img = Arc::new(image::open(image_path).context("Failed to open image")?);
    
    // Share image using Arc instead of cloning
    let img_for_leaderboard = Arc::clone(&img);
    let img_for_pose = Arc::clone(&img);
    let img_for_fields = Arc::clone(&img);
    
    // Start leaderboard extraction in background using crossbeam scope
    let mut data = TelemetryData::default();
    
    crossbeam::thread::scope(|s| {
        let leaderboard_handle = s.spawn(|_| extract_leaderboard_cached(&img_for_leaderboard));
        let pose_handle = s.spawn(|_| extract_rider_pose_parallel(&img_for_pose));
        
        // Extract telemetry fields in parallel while leaderboard processes
        extract_fields_parallel(&mut data, &img_for_fields)?;
        
        // Wait for concurrent tasks
        data.leaderboard = leaderboard_handle.join()
            .map_err(|_| anyhow::anyhow!("Leaderboard thread panicked"))??;
        data.rider_pose = pose_handle.join()
            .map_err(|_| anyhow::anyhow!("Pose detection thread panicked"))??;
        
        Ok::<(), anyhow::Error>(())
    }).map_err(|_| anyhow::anyhow!("Thread scope error"))?;
    
    Ok(data)
}

/// Extract all telemetry fields in parallel
fn extract_fields_parallel(data: &mut TelemetryData, img: &Arc<DynamicImage>) -> Result<()> {
    let field_regions = vec![
        ("speed", regions::SPEED),
        ("distance", regions::DISTANCE),
        ("altitude", regions::ALTITUDE),
        ("race_time", regions::RACE_TIME),
        ("power", regions::POWER),
        ("cadence", regions::CADENCE),
        ("heart_rate", regions::HEART_RATE),
        ("gradient", regions::GRADIENT),
        ("distance_to_finish", regions::DISTANCE_TO_FINISH),
    ];
    
    // Process all fields in parallel
    let results: Vec<_> = field_regions
        .par_iter()
        .map(|(field, (x, y, width, height))| {
            extract_single_field(img, field, *x, *y, *width, *height)
        })
        .collect();
    
    // Assign results to data structure
    for result in results {
        match result? {
            FieldValue::Speed(v) => data.speed = v,
            FieldValue::Distance(v) => data.distance = v,
            FieldValue::Altitude(v) => data.altitude = v,
            FieldValue::RaceTime(v) => data.race_time = v,
            FieldValue::Power(v) => data.power = v,
            FieldValue::Cadence(v) => data.cadence = v,
            FieldValue::HeartRate(v) => data.heart_rate = v,
            FieldValue::Gradient(v) => data.gradient = v,
            FieldValue::DistanceToFinish(v) => data.distance_to_finish = v,
        }
    }
    
    Ok(())
}

/// Extract a single field with its own Tesseract instance
fn extract_single_field(
    img: &Arc<DynamicImage>,
    field: &str,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<FieldValue> {
    match field {
        "gradient" => {
            // Special handling for gradient
            let result = extract_gradient_parallel(img, x, y, width, height)?;
            Ok(FieldValue::Gradient(result))
        }
        _ => {
            // Get Tesseract instance from pool
            let text = {
                let mut pool = TESSERACT_POOL
                    .lock()
                    .map_err(|_| anyhow::anyhow!("Tesseract pool lock poisoned"))?;
                
                let mut ocr = pool.pop()
                    .ok_or_else(|| anyhow::anyhow!("No available Tesseract instance"))?;
                
                // Extract field
                let roi = img.crop_imm(x, y, width, height);
                let threshold_value = if field == "distance_to_finish" {
                    thresholds::DISTANCE_TO_FINISH
                } else {
                    thresholds::DEFAULT
                };
                
                let buf = preprocess_for_ocr(&roi, threshold_value, scale_factors::DEFAULT)?;
                ocr.set_image_from_mem(&buf)?;
                ocr.set_variable(Variable::TesseditPagesegMode, "7")?;
                let text = ocr.get_utf8_text()?;
                
                // Return instance to pool
                pool.push(ocr);
                text
            };
            
            // Parse based on field type
            let value = match field {
                "speed" => {
                    let clean = ocr_regex::NON_DIGITS.replace_all(&text, "").to_string();
                    FieldValue::Speed(clean.parse().ok())
                }
                "distance" | "distance_to_finish" => {
                    let clean = ocr_regex::NON_DIGITS_DECIMAL.replace_all(&text, "").to_string();
                    match field {
                        "distance" => FieldValue::Distance(clean.parse().ok()),
                        "distance_to_finish" => FieldValue::DistanceToFinish(clean.parse().ok()),
                        _ => unreachable!(),
                    }
                }
                "altitude" => {
                    let clean = ocr_regex::NON_DIGITS.replace_all(&text, "").to_string();
                    FieldValue::Altitude(clean.parse().ok())
                }
                "race_time" => {
                    let time = crate::ocr_compact::parse_time(&text);
                    FieldValue::RaceTime(time)
                }
                "power" => {
                    let clean = ocr_regex::NON_DIGITS.replace_all(&text, "").to_string();
                    FieldValue::Power(clean.parse().ok())
                }
                "cadence" => {
                    let clean = ocr_regex::NON_DIGITS.replace_all(&text, "").to_string();
                    FieldValue::Cadence(clean.parse().ok())
                }
                "heart_rate" => {
                    let clean = ocr_regex::NON_DIGITS.replace_all(&text, "").to_string();
                    FieldValue::HeartRate(clean.parse().ok())
                }
                _ => unreachable!("Unknown field: {}", field),
            };
            
            Ok(value)
        }
    }
}

/// Extract gradient with parallel-safe implementation
fn extract_gradient_parallel(
    img: &Arc<DynamicImage>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<Option<f64>> {
    let roi = img.crop_imm(x, y, width, height);
    let buf = preprocess_for_ocr(&roi, thresholds::GRADIENT, scale_factors::GRADIENT)?;
    
    // Create temporary Tesseract instance for gradient
    let mut ocr = LepTess::new(None, "eng").context("Failed to initialize Tesseract")?;
    ocr.set_variable(Variable::TesseditCharWhitelist, "0123456789.-%")?;
    ocr.set_variable(Variable::TesseditPagesegMode, "7")?;
    ocr.set_image_from_mem(&buf)?;
    
    let text = ocr.get_utf8_text()?;
    let clean_text = ocr_regex::NON_DIGITS_DECIMAL.replace_all(&text, "").to_string();
    
    Ok(clean_text.parse().ok())
}

/// Extract leaderboard using cached OCRS engine
fn extract_leaderboard_cached(img: &Arc<DynamicImage>) -> Result<Option<Vec<LeaderboardEntry>>> {
    let x = regions::LEADERBOARD_X;
    let y = regions::LEADERBOARD_Y;
    let width = regions::LEADERBOARD_WIDTH;
    let height = regions::LEADERBOARD_HEIGHT;
    
    // Extract text using cached engine
    let text = {
        let engine = OCRS_ENGINE
            .lock()
            .map_err(|_| anyhow::anyhow!("OCRS engine lock poisoned"))?;
        
        // Crop and convert region
        let cropped = img.crop_imm(x, y, width, height);
        let rgb = cropped.into_rgb8();
        let (img_width, img_height) = rgb.dimensions();
        
        // Convert to tensor
        use rten_tensor::prelude::*;
        let color_img = NdTensor::from_data(
            [img_height as usize, img_width as usize, 3],
            rgb.into_vec(),
        );
        
        // Prepare input
        let img_source = ocrs::ImageSource::from_tensor(color_img.view(), ocrs::DimOrder::Hwc)
            .map_err(|e| anyhow::anyhow!("Failed to create image source: {}", e))?;
        
        let ocr_input = engine
            .prepare_input(img_source)
            .map_err(|e| anyhow::anyhow!("Failed to prepare input: {}", e))?;
        
        // Get text
        engine
            .get_text(&ocr_input)
            .map_err(|e| anyhow::anyhow!("Failed to get text: {}", e))?
    };
    
    // Parse leaderboard entries
    parse_leaderboard_text(&text)
}

/// Parse leaderboard text into entries
fn parse_leaderboard_text(text: &str) -> Result<Option<Vec<LeaderboardEntry>>> {
    let mut entries = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    let mut i = 0;
    
    while i < lines.len() {
        let line_text = lines[i].trim();
        
        if crate::ocr_compact::is_likely_name(line_text) {
            let mut entry = LeaderboardEntry {
                name: line_text.to_string(),
                current: false,
                delta: None,
                km: None,
                wkg: None,
            };
            
            if i + 1 < lines.len() {
                let data_line_text = lines[i + 1];
                crate::ocr_compact::parse_leaderboard_data(&mut entry, data_line_text);
                
                if entry.delta.is_none() && (entry.wkg.is_some() || entry.km.is_some()) {
                    entry.current = true;
                }
            }
            
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

/// Extract rider pose with parallel-safe edge detection
fn extract_rider_pose_parallel(img: &Arc<DynamicImage>) -> Result<Option<RiderPose>> {
    // For parallel processing, we need to get a lock on the config manager
    // This is a limitation of the current design - could be improved with Arc<RwLock>
    let config_manager = crate::ocr_compact::get_config_manager();
    let manager = config_manager.lock().unwrap();
    crate::ocr_compact::extract_rider_pose(img, &manager)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    
    #[test]
    fn test_parallel_extraction() {
        let test_image = Path::new("docs/screenshots/normal_1_01_16_02_21.jpg");
        if test_image.exists() {
            let result = extract_telemetry_parallel(test_image);
            assert!(result.is_ok());
            
            let data = result.unwrap();
            assert!(data.speed.is_some());
            assert!(data.power.is_some());
        }
    }
}