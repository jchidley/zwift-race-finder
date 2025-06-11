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

    /// Use parallel extraction (faster)
    #[arg(short, long)]
    parallel: bool,
}

#[cfg(feature = "ocr")]
fn main() -> Result<()> {
    let args = Args::parse();
    
    // Extract telemetry
    let telemetry = if args.parallel {
        zwift_race_finder::ocr_parallel::extract_telemetry_parallel(&args.image_path)?
    } else {
        zwift_race_finder::ocr_compact::extract_telemetry(&args.image_path)?
    };
    
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
            if let Some(gradient) = telemetry.gradient {
                println!("Gradient: {}%", gradient);
            }
            if let Some(distance_to_finish) = telemetry.distance_to_finish {
                println!("Distance to finish: {} km", distance_to_finish);
            }
            if let Some(leaderboard) = &telemetry.leaderboard {
                println!("\nLeaderboard:");
                for (i, entry) in leaderboard.iter().enumerate() {
                    let you = if entry.current { " <-- YOU" } else { "" };
                    let delta = entry.delta.as_deref().unwrap_or("---");
                    let wkg = entry.wkg.map_or("---".to_string(), |w| format!("{:.1}", w));
                    let km = entry.km.map_or("---".to_string(), |k| format!("{:.1}", k));
                    println!("  {}. {:15} {:>6} {}w/kg {}km{}",
                        i + 1, entry.name, delta, wkg, km, you);
                }
            }
            if let Some(pose) = &telemetry.rider_pose {
                println!("Rider pose: {:?}", pose);
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