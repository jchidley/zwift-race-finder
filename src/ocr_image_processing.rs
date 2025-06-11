//! Image processing utilities for OCR
//! Extracted common functionality following mechanical refactoring rules

use anyhow::Result;
use image::{DynamicImage, GrayImage};
use imageproc::contrast::threshold;

/// Preprocess an image region for OCR
/// Converts to grayscale, applies threshold, and scales up
pub fn preprocess_for_ocr(
    roi: &DynamicImage,
    threshold_value: u8,
    scale_factor: u32,
) -> Result<Vec<u8>> {
    // Convert to grayscale
    let gray = roi.to_luma8();
    
    // Apply threshold
    let binary = threshold(&gray, threshold_value);
    
    // Scale up for better OCR
    let scaled = image::imageops::resize(
        &binary,
        binary.width() * scale_factor,
        binary.height() * scale_factor,
        image::imageops::FilterType::CatmullRom,
    );
    
    // Convert to PNG for OCR engines
    to_png_bytes(&DynamicImage::ImageLuma8(scaled))
}

/// Convert a dynamic image to PNG bytes
pub fn to_png_bytes(image: &DynamicImage) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    image.write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png)?;
    Ok(buf)
}