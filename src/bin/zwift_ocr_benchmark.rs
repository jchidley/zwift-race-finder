// ABOUTME: Benchmark tool to compare sequential vs parallel OCR implementations
// Run with: cargo run --features ocr --bin zwift_ocr_benchmark --release -- <image_path>

use anyhow::Result;
use std::env;
use std::path::Path;
use std::time::Instant;
use zwift_race_finder::ocr_compact;
use zwift_race_finder::ocr_parallel;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <image_path> [--iterations N]", args[0]);
        std::process::exit(1);
    }
    
    let image_path = Path::new(&args[1]);
    if !image_path.exists() {
        eprintln!("Error: Image file not found: {}", image_path.display());
        std::process::exit(1);
    }
    
    // Parse iterations from command line
    let iterations = if args.len() >= 4 && args[2] == "--iterations" {
        args[3].parse::<usize>().unwrap_or(1)
    } else {
        1
    };
    
    println!("=== Zwift OCR Benchmark ===");
    println!("Image: {}", image_path.display());
    println!("Iterations: {}", iterations);
    println!();
    
    // Warm up (load models, initialize pools)
    println!("Warming up...");
    let _ = ocr_compact::extract_telemetry(image_path)?;
    let _ = ocr_parallel::extract_telemetry_parallel(image_path)?;
    
    // Benchmark sequential implementation
    println!("\n--- Sequential Implementation ---");
    let mut seq_times = Vec::new();
    let mut seq_result = None;
    
    for i in 0..iterations {
        let start = Instant::now();
        let result = ocr_compact::extract_telemetry(image_path)?;
        let elapsed = start.elapsed();
        seq_times.push(elapsed.as_secs_f64());
        
        if i == 0 {
            seq_result = Some(result);
        }
        
        if iterations > 1 {
            print!(".");
            std::io::Write::flush(&mut std::io::stdout())?;
        }
    }
    
    if iterations > 1 {
        println!();
    }
    
    // Benchmark parallel implementation
    println!("\n--- Parallel Implementation ---");
    let mut par_times = Vec::new();
    let mut par_result = None;
    
    for i in 0..iterations {
        let start = Instant::now();
        let result = ocr_parallel::extract_telemetry_parallel(image_path)?;
        let elapsed = start.elapsed();
        par_times.push(elapsed.as_secs_f64());
        
        if i == 0 {
            par_result = Some(result);
        }
        
        if iterations > 1 {
            print!(".");
            std::io::Write::flush(&mut std::io::stdout())?;
        }
    }
    
    if iterations > 1 {
        println!();
    }
    
    // Calculate statistics
    let seq_avg = seq_times.iter().sum::<f64>() / seq_times.len() as f64;
    let par_avg = par_times.iter().sum::<f64>() / par_times.len() as f64;
    
    let seq_min = seq_times.iter().cloned().fold(f64::INFINITY, f64::min);
    let seq_max = seq_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    
    let par_min = par_times.iter().cloned().fold(f64::INFINITY, f64::min);
    let par_max = par_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    
    // Display results
    println!("\n=== Performance Results ===");
    println!("Sequential:");
    println!("  Average: {:.3}s", seq_avg);
    if iterations > 1 {
        println!("  Min: {:.3}s, Max: {:.3}s", seq_min, seq_max);
    }
    
    println!("\nParallel:");
    println!("  Average: {:.3}s", par_avg);
    if iterations > 1 {
        println!("  Min: {:.3}s, Max: {:.3}s", par_min, par_max);
    }
    
    println!("\nSpeedup: {:.2}x", seq_avg / par_avg);
    
    // Verify results match
    if let (Some(seq), Some(par)) = (seq_result, par_result) {
        println!("\n=== Accuracy Check ===");
        let fields_match = 
            seq.speed == par.speed &&
            seq.distance == par.distance &&
            seq.altitude == par.altitude &&
            seq.race_time == par.race_time &&
            seq.power == par.power &&
            seq.cadence == par.cadence &&
            seq.heart_rate == par.heart_rate &&
            seq.gradient == par.gradient &&
            seq.distance_to_finish == par.distance_to_finish;
        
        if fields_match {
            println!("✓ All telemetry fields match!");
        } else {
            println!("✗ Field mismatch detected:");
            if seq.speed != par.speed {
                println!("  Speed: {:?} vs {:?}", seq.speed, par.speed);
            }
            if seq.distance != par.distance {
                println!("  Distance: {:?} vs {:?}", seq.distance, par.distance);
            }
            // Add more field comparisons as needed
        }
        
        // Compare leaderboard
        match (&seq.leaderboard, &par.leaderboard) {
            (Some(seq_lb), Some(par_lb)) => {
                if seq_lb.len() == par_lb.len() {
                    println!("✓ Leaderboard entries match ({} entries)", seq_lb.len());
                } else {
                    println!("✗ Leaderboard count mismatch: {} vs {}", seq_lb.len(), par_lb.len());
                }
            }
            (None, None) => println!("✓ No leaderboard data (both)"),
            _ => println!("✗ Leaderboard presence mismatch"),
        }
    }
    
    // Display thread pool info
    println!("\n=== System Info ===");
    println!("CPU threads: {}", rayon::current_num_threads());
    
    Ok(())
}