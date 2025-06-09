// ABOUTME: Benchmark comparing ocrs and Tesseract OCR performance
// Measures extraction speed for both OCR engines

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::Path;

fn benchmark_ocrs(c: &mut Criterion) {
    let test_image = Path::new("docs/screenshots/normal_1_01_16_02_21.jpg");
    
    if !test_image.exists() {
        eprintln!("Test image not found at {:?}", test_image);
        return;
    }
    
    c.bench_function("ocrs_extract_text", |b| {
        b.iter(|| {
            let result = zwift_race_finder::ocr_ocrs::extract_text(black_box(test_image));
            match result {
                Ok(text) => black_box(text),
                Err(e) => panic!("OCR extraction failed: {}", e),
            }
        })
    });
}

fn benchmark_tesseract(c: &mut Criterion) {
    let test_image = Path::new("docs/screenshots/normal_1_01_16_02_21.jpg");
    
    if !test_image.exists() {
        eprintln!("Test image not found at {:?}", test_image);
        return;
    }
    
    c.bench_function("tesseract_extract_text", |b| {
        b.iter(|| {
            let result = zwift_race_finder::ocr_image::extract_text(black_box(test_image));
            match result {
                Ok(text) => black_box(text),
                Err(e) => panic!("OCR extraction failed: {}", e),
            }
        })
    });
}

criterion_group!(benches, benchmark_ocrs, benchmark_tesseract);
criterion_main!(benches);