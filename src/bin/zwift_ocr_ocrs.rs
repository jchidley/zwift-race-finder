// ABOUTME: Simple binary to test ocrs OCR functionality
// Mimics the ocrs CLI but uses our library integration

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "Simple OCR using ocrs library")]
struct Args {
    /// Path to the image file
    image_path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("Processing image: {:?}", args.image_path);
    println!("Note: Models will auto-download on first run (may take a minute)...\n");
    
    // Extract text using our ocrs wrapper
    match zwift_race_finder::ocr_ocrs::extract_text(&args.image_path) {
        Ok(text) => {
            println!("Extracted text:\n{}", text);
        }
        Err(e) => {
            eprintln!("Error: {:#}", e);
            eprintln!("\nIf models are missing, they should auto-download.");
            eprintln!("You can also manually download them:");
            eprintln!("  - Detection: https://ocrs-models.s3-accelerate.amazonaws.com/text-detection.rten");
            eprintln!("  - Recognition: https://ocrs-models.s3-accelerate.amazonaws.com/text-recognition.rten");
            eprintln!("\nPlace them in: ~/.cache/ocrs/");
        }
    }
    
    Ok(())
}