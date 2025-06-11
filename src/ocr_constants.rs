//! Constants for OCR module
//! Extracted from ocr_compact.rs following mechanical refactoring rules

/// Region coordinates for telemetry extraction (x, y, width, height)
pub mod regions {
    pub const SPEED: (u32, u32, u32, u32) = (693, 44, 71, 61);
    pub const DISTANCE: (u32, u32, u32, u32) = (833, 44, 84, 55);
    pub const ALTITUDE: (u32, u32, u32, u32) = (975, 45, 75, 50);
    pub const RACE_TIME: (u32, u32, u32, u32) = (1070, 45, 134, 49);
    pub const POWER: (u32, u32, u32, u32) = (268, 49, 117, 61);
    pub const CADENCE: (u32, u32, u32, u32) = (240, 135, 45, 31);
    pub const HEART_RATE: (u32, u32, u32, u32) = (341, 129, 69, 38);
    pub const GRADIENT: (u32, u32, u32, u32) = (1695, 71, 50, 50);
    pub const DISTANCE_TO_FINISH: (u32, u32, u32, u32) = (1143, 138, 50, 27);
    
    pub const LEADERBOARD_X: u32 = 1500;
    pub const LEADERBOARD_Y: u32 = 200;
    pub const LEADERBOARD_WIDTH: u32 = 420;
    pub const LEADERBOARD_HEIGHT: u32 = 600;
    
    pub const AVATAR_X: u32 = 860;
    pub const AVATAR_Y: u32 = 400;
    pub const AVATAR_WIDTH: u32 = 200;
    pub const AVATAR_HEIGHT: u32 = 300;
}

/// Threshold values for image preprocessing
pub mod thresholds {
    pub const DEFAULT: u8 = 200;
    pub const DISTANCE_TO_FINISH: u8 = 150;
    pub const GRADIENT: u8 = 150;
}

/// Scale factors for OCR preprocessing
pub mod scale_factors {
    pub const DEFAULT: u32 = 3;
    pub const GRADIENT: u32 = 4;
}

/// Pose detection thresholds
pub mod pose {
    pub const ASPECT_RATIO_STANDING_MIN: f32 = 1.7;
    pub const CENTER_OF_MASS_STANDING_MAX: f32 = 0.45;
    
    pub const ASPECT_RATIO_TUCK_MAX: f32 = 1.3;
    pub const CENTER_OF_MASS_TUCK_MIN: f32 = 0.55;
    
    pub const ASPECT_RATIO_SEATED_MIN: f32 = 1.4;
    pub const ASPECT_RATIO_SEATED_MAX: f32 = 1.8;
    pub const CENTER_OF_MASS_SEATED_MIN: f32 = 0.45;
    pub const CENTER_OF_MASS_SEATED_MAX: f32 = 0.6;
    
    pub const ASPECT_RATIO_NORMAL_MIN: f32 = 1.3;
    pub const ASPECT_RATIO_NORMAL_MAX: f32 = 1.7;
    pub const CENTER_OF_MASS_NORMAL_MIN: f32 = 0.45;
    pub const CENTER_OF_MASS_NORMAL_MAX: f32 = 0.55;
}

/// W/kg range for validation
pub mod wkg {
    // Human physiological limits for cycling
    // 0.0 is valid (stopped/coasting)
    // Recreational: 0.0-3.0, Amateur: 3.0-4.0, Pro: 4.0-7.6
    pub const MIN: f64 = 0.5;  // Below this, likely not racing
    pub const MAX: f64 = 7.0;  // Above this, likely cheating on Zwift
}

/// Name detection limits
pub mod name_limits {
    pub const MIN_LENGTH: usize = 2;
    pub const MAX_LENGTH: usize = 30;
    pub const MIN_LETTERS: usize = 2;
}

/// Edge detection parameters
pub mod edge_detection {
    pub const GAUSSIAN_BLUR_SIGMA: f32 = 1.0;
    pub const CANNY_LOW_THRESHOLD: f32 = 50.0;
    pub const CANNY_HIGH_THRESHOLD: f32 = 150.0;
    pub const EDGE_PIXEL_VALUE: u8 = 255;
}