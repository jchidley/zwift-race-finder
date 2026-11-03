pub mod ab_testing;

pub mod api;

pub mod constants;

pub mod models;

pub mod category;

pub mod parsing;

pub mod cache;

pub mod config;

pub mod database;

pub mod history;

pub mod personal_estimation;

pub mod duration_estimation;

pub mod errors;

pub mod estimation;

pub mod event_analysis;

pub mod event_display;

pub mod event_filtering;

pub mod formatting;

#[cfg(test)]
pub mod regression_test;




#[cfg(feature = "ocr")]
pub mod ocr_constants;

#[cfg(feature = "ocr")]
pub mod ocr_config;

#[cfg(feature = "ocr")]
pub mod ocr_image_processing;

#[cfg(feature = "ocr")]
pub mod ocr_regex;

#[cfg(feature = "ocr")]
pub mod ocr_compact;

#[cfg(feature = "ocr")]
pub mod ocr_ocrs;

#[cfg(feature = "ocr")]
pub mod ocr_parallel;
