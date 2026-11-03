use anyhow::{Context, Result};
use image::DynamicImage;
use ocrs::{DimOrder, ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;
use rten_tensor::prelude::*;
use rten_tensor::NdTensor;
use std::path::PathBuf;

fn get_models_dir() -> PathBuf {
    let mut cache_dir = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    cache_dir.push(".cache");
    cache_dir.push("ocrs");
    cache_dir
}

pub fn create_engine() -> Result<OcrEngine> {
    let models_dir = get_models_dir();
    let detection_model_path = models_dir.join("text-detection.rten");
    let recognition_model_path = models_dir.join("text-recognition.rten");

    let detection_model = Model::load_file(detection_model_path)
        .context("Failed to load detection model. Download from https://ocrs-models.s3-accelerate.amazonaws.com/text-detection.rten")?;
    let recognition_model = Model::load_file(recognition_model_path)
        .context("Failed to load recognition model. Download from https://ocrs-models.s3-accelerate.amazonaws.com/text-recognition.rten")?;

    OcrEngine::new(OcrEngineParams {
        detection_model: Some(detection_model),
        recognition_model: Some(recognition_model),
        ..Default::default()
    }).map_err(|e| anyhow::anyhow!("Failed to create OCR engine: {}", e))
}

pub fn extract_text_from_region(
    image: &DynamicImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32
) -> Result<String> {
    let cropped = image.crop_imm(x, y, width, height);
    let img = cropped.into_rgb8();
    let (img_width, img_height) = img.dimensions();

    let color_img = NdTensor::from_data(
        [img_height as usize, img_width as usize, 3],
        img.into_vec(),
    );

    let engine = create_engine()?;

    let img_source = ImageSource::from_tensor(color_img.view(), DimOrder::Hwc)
        .map_err(|e| anyhow::anyhow!("Failed to create image source: {}", e))?;

    let ocr_input = engine.prepare_input(img_source)
        .map_err(|e| anyhow::anyhow!("Failed to prepare input: {}", e))?;

    let text = engine.get_text(&ocr_input)
        .map_err(|e| anyhow::anyhow!("Failed to get text: {}", e))?;

    Ok(text)
}

