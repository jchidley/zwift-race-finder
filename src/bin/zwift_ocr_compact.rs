// ABOUTME: Compact Zwift OCR extraction tool
// Minimal implementation for extracting telemetry from screenshots

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

/// Extract telemetry data from Zwift screenshots
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Path to the screenshot image
    image_path: PathBuf,

    /// Output format (json or text)
    #[arg(short, long, default_value = "json")]
    format: String,
}

#[cfg(feature = "ocr")]
fn main() -> Result<()> {
    let args = Args::parse();
    
    // Extract telemetry
    let telemetry = zwift_race_finder::ocr_compact::extract_telemetry(&args.image_path)?;
    
    // Output results
    match args.format.as_str() {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&telemetry)?);
        }
        "text" => {
            if let Some(speed) = telemetry.speed {
                println!("Speed: {} km/h", speed);
            }
            if let Some(distance) = telemetry.distance {
                println!("Distance: {} km", distance);
            }
            if let Some(altitude) = telemetry.altitude {
                println!("Altitude: {} m", altitude);
            }
            if let Some(time) = &telemetry.race_time {
                println!("Time: {}", time);
            }
            if let Some(power) = telemetry.power {
                println!("Power: {} W", power);
            }
            if let Some(cadence) = telemetry.cadence {
                println!("Cadence: {} rpm", cadence);
            }
            if let Some(heart_rate) = telemetry.heart_rate {
                println!("HR: {} bpm", heart_rate);
            }
        }
        _ => {
            eprintln!("Unknown format: {}. Use 'json' or 'text'", args.format);
            std::process::exit(1);
        }
    }
    
    Ok(())
}

#[cfg(not(feature = "ocr"))]
fn main() {
    eprintln!("Error: OCR requires the 'ocr' feature.");
    eprintln!("Build with: cargo build --features ocr --bin zwift_ocr_compact");
    std::process::exit(1);
}