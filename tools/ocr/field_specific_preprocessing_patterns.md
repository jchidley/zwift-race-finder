# Field-Specific OCR Preprocessing Patterns

## Overview

Both the Python (PaddleOCR) and Rust (Tesseract/ocrs) implementations use field-specific preprocessing to optimize OCR accuracy for different types of Zwift UI elements.

## Standard Preprocessing Pattern

Most fields use the standard preprocessing:
- **Threshold**: 200 (binary threshold for white text on dark background)
- **Scale Factor**: 3x (upscale for better OCR accuracy)
- **Process**: Convert to grayscale → Apply threshold → Scale up

### Fields Using Standard Preprocessing:
- Speed
- Distance  
- Altitude
- Race Time
- Power
- Cadence
- Heart Rate

## Field-Specific Variations

### 1. Gradient Field
The gradient field uses special preprocessing because it has a stylized font:

**Python Implementation:**
```python
# Special processing for stylized gradient font
inverted = cv2.bitwise_not(gray)  # Invert the image
_, binary = cv2.threshold(inverted, 100, 255, cv2.THRESH_BINARY)
scaled = cv2.resize(binary, None, fx=4, fy=4, interpolation=cv2.INTER_CUBIC)
```

**Rust Implementation:**
```rust
// Constants:
pub const GRADIENT: u8 = 150;  // Lower threshold
pub const GRADIENT: u32 = 4;   // Higher scale factor
```

**Key Differences:**
- Python inverts the image first (bitwise_not) with threshold 100
- Rust uses lower threshold (150 vs 200) without inversion
- Both use 4x scaling instead of 3x
- The inversion in Python suggests the gradient text might be darker on lighter background

### 2. Distance to Finish Field
This field has dimmer text that requires adjusted preprocessing:

**Constants:**
- **Threshold**: 150 (lower than standard 200)
- **Scale Factor**: 3x (standard)

**Reason**: The distance to finish text appears dimmer/lower contrast than other UI elements

### 3. Leaderboard Processing
The leaderboard uses completely different preprocessing due to its complex layout:

**Python Implementation:**
```python
gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY)
clahe = cv2.createCLAHE(clipLimit=2.0, tileGridSize=(8, 8))
enhanced = clahe.apply(gray)
```

**Rust Implementation:**
- Uses ocrs library instead of Tesseract
- No explicit CLAHE, relies on ocrs's built-in preprocessing
- Processes the entire region as one block for better context

**Key Differences:**
- Python uses CLAHE (Contrast Limited Adaptive Histogram Equalization) for better local contrast
- Rust switches to a different OCR engine (ocrs) that's better suited for UI text
- No threshold/binary conversion - works on enhanced grayscale

## Processing Pipeline Summary

### Top Bar Fields (Speed, Distance, etc.)
1. Crop ROI
2. Convert to grayscale
3. Apply binary threshold (200)
4. Upscale 3x
5. OCR with single line mode

### Gradient Field
1. Crop ROI
2. Convert to grayscale
3. (Python only: Invert image)
4. Apply threshold (Python: 100 after inversion, Rust: 150)
5. Upscale 4x
6. OCR with numeric whitelist

### Distance to Finish
1. Crop ROI
2. Convert to grayscale
3. Apply binary threshold (150 - lower for dimmer text)
4. Upscale 3x
5. OCR with decimal number whitelist

### Leaderboard
1. Crop large ROI (420x600 pixels)
2. Convert to grayscale
3. Apply CLAHE enhancement (Python) or use ocrs (Rust)
4. OCR full region
5. Parse structure from detected text positions

## Key Insights

1. **Threshold Selection**: Higher thresholds (200) work for bright white text, lower thresholds (150) needed for dimmer elements

2. **Scale Factors**: 3x is standard, but stylized fonts (gradient) benefit from 4x scaling

3. **Image Inversion**: The Python gradient processing inverts the image, suggesting it might handle light text on dark background differently than expected

4. **Adaptive Enhancement**: Complex regions like leaderboards benefit from CLAHE or specialized OCR engines rather than simple thresholding

5. **OCR Engine Selection**: Tesseract works well for simple numeric fields, but ocrs is better for complex UI layouts with mixed content