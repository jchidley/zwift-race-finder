// ABOUTME: Simple OCR test binary to verify config loading works
// Usage: cargo run --bin test_ocr --features ocr -- path/to/image.png

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use zwift_race_finder::ocr_compact::extract_telemetry;

#[derive(Parser)]
#[command(name = "test_ocr")]
#[command(about = "Test OCR extraction on a Zwift screenshot")]
struct Args {
    /// Path to the screenshot image
    image_path: PathBuf,
    
    /// Enable debug output
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    if args.debug {
        std::env::set_var("DEBUG_OCR", "1");
    }
    
    println!("Testing OCR on: {:?}", args.image_path);
    println!("{}", "=".repeat(60));
    
    match extract_telemetry(&args.image_path) {
        Ok(data) => {
            println!("✓ OCR extraction successful!");
            println!();
            
            // Display extracted data
            if let Some(speed) = data.speed {
                println!("Speed: {} km/h", speed);
            }
            if let Some(distance) = data.distance {
                println!("Distance: {:.1} km", distance);
            }
            if let Some(altitude) = data.altitude {
                println!("Altitude: {} m", altitude);
            }
            if let Some(ref race_time) = data.race_time {
                println!("Race Time: {}", race_time);
            }
            if let Some(power) = data.power {
                println!("Power: {} W", power);
            }
            if let Some(cadence) = data.cadence {
                println!("Cadence: {} rpm", cadence);
            }
            if let Some(heart_rate) = data.heart_rate {
                println!("Heart Rate: {} bpm", heart_rate);
            }
            if let Some(gradient) = data.gradient {
                println!("Gradient: {:.1}%", gradient);
            }
            if let Some(distance_to_finish) = data.distance_to_finish {
                println!("Distance to Finish: {:.1} km", distance_to_finish);
            }
            
            if let Some(ref pose) = data.rider_pose {
                println!("Rider Pose: {:?}", pose);
            }
            
            if let Some(ref leaderboard) = data.leaderboard {
                println!("\nLeaderboard ({} riders):", leaderboard.len());
                for (i, entry) in leaderboard.iter().take(5).enumerate() {
                    print!("  {}. {}", i + 1, entry.name);
                    if entry.current {
                        print!(" [YOU]");
                    }
                    if let Some(ref delta) = entry.delta {
                        print!(" {}", delta);
                    }
                    if let Some(km) = entry.km {
                        print!(" {:.1}km", km);
                    }
                    if let Some(wkg) = entry.wkg {
                        print!(" {:.1}w/kg", wkg);
                    }
                    println!();
                }
                if leaderboard.len() > 5 {
                    println!("  ... and {} more", leaderboard.len() - 5);
                }
            }
            
            // Save result as JSON
            let json_path = args.image_path.with_extension("ocr.json");
            let json = serde_json::to_string_pretty(&data)?;
            std::fs::write(&json_path, json)?;
            println!("\nResults saved to: {:?}", json_path);
        }
        Err(e) => {
            eprintln!("✗ OCR extraction failed: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}