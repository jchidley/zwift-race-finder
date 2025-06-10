//! Performance benchmarks for OCR operations

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use zwift_race_finder::ocr_compact::{
    extract_telemetry, is_likely_name, parse_leaderboard_data, parse_time, LeaderboardEntry, 
};
use zwift_race_finder::ocr_image_processing::{preprocess_for_ocr};
use image::{DynamicImage, RgbImage};
use std::path::Path;

/// Create a dummy image for benchmarking
fn create_test_image(width: u32, height: u32) -> DynamicImage {
    DynamicImage::ImageRgb8(RgbImage::new(width, height))
}

/// Benchmark string parsing functions
fn bench_parsing_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing");
    
    // Benchmark parse_time
    group.bench_function("parse_time_valid", |b| {
        b.iter(|| parse_time(black_box("12:34")))
    });
    
    group.bench_function("parse_time_with_text", |b| {
        b.iter(|| parse_time(black_box("Time remaining: 12:34")))
    });
    
    group.bench_function("parse_time_digits_only", |b| {
        b.iter(|| parse_time(black_box("1234")))
    });
    
    // Benchmark is_likely_name
    group.bench_function("is_likely_name_valid", |b| {
        b.iter(|| is_likely_name(black_box("J.Chidley")))
    });
    
    group.bench_function("is_likely_name_invalid", |b| {
        b.iter(|| is_likely_name(black_box("12.3 km")))
    });
    
    // Benchmark parse_leaderboard_data
    let mut entry = LeaderboardEntry {
        name: "Test".to_string(),
        current: false,
        delta: None,
        km: None,
        wkg: None,
    };
    
    group.bench_function("parse_leaderboard_full", |b| {
        b.iter(|| {
            let mut entry = entry.clone();
            parse_leaderboard_data(&mut entry, black_box("+01:23 3.2 w/kg 12.5 KM"))
        })
    });
    
    group.bench_function("parse_leaderboard_minimal", |b| {
        b.iter(|| {
            let mut entry = entry.clone();
            parse_leaderboard_data(&mut entry, black_box("+01:23"))
        })
    });
    
    group.finish();
}

/// Benchmark image preprocessing
fn bench_image_preprocessing(c: &mut Criterion) {
    let mut group = c.benchmark_group("preprocessing");
    
    let sizes = vec![(100, 50), (200, 100), (400, 200)];
    
    for (width, height) in sizes {
        let img = create_test_image(width, height);
        
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}x{}", width, height)),
            &img,
            |b, img| {
                b.iter(|| preprocess_for_ocr(black_box(img), 200, 3))
            },
        );
    }
    
    group.finish();
}


/// Benchmark full telemetry extraction
fn bench_full_extraction(c: &mut Criterion) {
    if !cfg!(feature = "ocr") {
        return;
    }
    
    let mut group = c.benchmark_group("full_extraction");
    group.sample_size(10); // Reduce sample size for slow operations
    
    // Check if we have a test image
    let test_image_path = Path::new("docs/screenshots/normal_1_01_16_02_21.jpg");
    if test_image_path.exists() {
        group.bench_function("extract_telemetry_real_image", |b| {
            b.iter(|| {
                let _ = extract_telemetry(black_box(test_image_path));
            })
        });
    } else {
        eprintln!("Note: Skipping real image benchmark - test image not found");
    }
    
    group.finish();
}

/// Benchmark regex operations
fn bench_regex_operations(c: &mut Criterion) {
    use zwift_race_finder::ocr_regex;
    
    let mut group = c.benchmark_group("regex");
    
    let test_strings = vec![
        "12:34",
        "+01:23",
        "12.5 km",
        "3.2 w/kg",
        "J.Chidley",
        "complex string with 12:34 time and 3.2 w/kg power",
    ];
    
    for test_str in &test_strings {
        group.bench_with_input(
            BenchmarkId::new("time_format", test_str),
            test_str,
            |b, s| b.iter(|| ocr_regex::TIME_FORMAT.is_match(black_box(s))),
        );
        
        group.bench_with_input(
            BenchmarkId::new("time_delta", test_str),
            test_str,
            |b, s| b.iter(|| ocr_regex::TIME_DELTA.captures(black_box(s))),
        );
        
        group.bench_with_input(
            BenchmarkId::new("distance_km", test_str),
            test_str,
            |b, s| b.iter(|| ocr_regex::DISTANCE_KM.captures(black_box(s))),
        );
        
        group.bench_with_input(
            BenchmarkId::new("watts_per_kg", test_str),
            test_str,
            |b, s| b.iter(|| ocr_regex::WATTS_PER_KG.captures(black_box(s))),
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_parsing_functions,
    bench_image_preprocessing,
    bench_full_extraction,
    bench_regex_operations
);
criterion_main!(benches);