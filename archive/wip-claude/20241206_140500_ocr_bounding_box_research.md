# OCR Bounding Box and Automatic UI Detection Research

## Overview
Research findings on how EasyOCR and PaddleOCR return bounding box coordinates, automatic UI element detection, and best practices for mapping detected text regions to semantic meanings.

## 1. OCR Tools Bounding Box Formats

### EasyOCR
- **Output Format**: Returns list of tuples containing:
  ```python
  [([[x1, y1], [x2, y2], [x3, y3], [x4, y4]], 'detected_text', confidence_score)]
  ```
- **Coordinate Points**: Four corners of bounding box (quadrilateral)
- **Data Type**: Usually integers, but occasionally floats (handle both)
- **Configuration Options**:
  - `detail=1` (default): Returns verbose output with bounding boxes
  - `detail=0`: Returns only text without coordinates
  - `add_margin`: Extends bounding boxes by specified value (default 0.1)

### PaddleOCR
- **Output Format**: Similar structure to EasyOCR:
  ```python
  [[[x1, y1], [x2, y2], [x3, y3], [x4, y4]], ['detected_text', confidence_score]]
  ```
- **Coordinate Order**: Upper left → upper right → lower right → lower left
- **Detection Level**: Line-based detection (not word-level by default)
- **Precision**: Floating-point pixel coordinates

### Key Differences
- Both support quadrilateral bounding boxes (not just rectangles)
- Both can handle skewed/angled text
- PaddleOCR detects by line, EasyOCR can be more granular

## 2. Processing Full Screenshots

Both tools can process entire screenshots and return all text regions:

### Implementation Pattern
```python
# EasyOCR
import easyocr
reader = easyocr.Reader(['en'])
results = reader.readtext('screenshot.png')
# Returns all detected text regions with coordinates

# PaddleOCR
from paddleocr import PaddleOCR
ocr = PaddleOCR(lang='en')
results = ocr.ocr('screenshot.png')
# Returns all detected text regions with coordinates
```

## 3. Text Region Classification Strategies

### Approach 1: Rule-Based Classification
- Use position heuristics (e.g., speed usually top-left, power bottom-center)
- Analyze text patterns (e.g., "km/h" for speed, "W" for power)
- Check numeric ranges (e.g., 0-50 for speed, 0-500 for power)

### Approach 2: Machine Learning Classification
- Train classifier on labeled bounding boxes
- Features: position, size, text content, numeric patterns
- Use context from nearby regions

### Approach 3: Vision LLM Classification
- Pass cropped regions to vision models (LLaVA, GPT-4V)
- Ask model to classify UI element type
- More flexible but higher latency/cost

## 4. Vision LLMs for UI Understanding

### LLaVA (Large Language-and-Vision Assistant)
- **Capabilities**: Visual Q&A, OCR, object detection, UI understanding
- **Performance**: 85.1% relative score vs GPT-4 on multimodal tasks
- **Limitations**: Struggles with accurate OCR on complex documents
- **Integration**: Available via Groq (3x faster, half cost of traditional)

### Groq Vision Support
- Offers LLaVA v1.5 7B in preview mode
- Exceptional speed for real-time applications
- Supports multimodal inputs (visual + text)

### Practical OCR Prompt Example
```python
prompt = """Act as an OCR assistant. Analyze the provided image and:
1. Identify and transcribe all visible text exactly as it appears
2. Preserve original formatting and spacing
3. Output only transcribed text without commentary"""
```

## 5. Best Practices for Automatic UI Calibration

### Multi-Stage Pipeline
1. **Detection**: Use OCR to find all text regions
2. **Classification**: Categorize regions by type
3. **Validation**: Verify mappings make sense
4. **Refinement**: Adjust based on feedback

### Recommended Architecture
```
Screenshot → OCR (EasyOCR/PaddleOCR) → Bounding Boxes
    ↓
Text + Position Features → Classifier → UI Element Types
    ↓
Semantic Mapping → Speed: Box1, Power: Box2, etc.
```

### Implementation Considerations
- **Combine Methods**: Use OCR for text detection + vision models for complex cases
- **Handle Variations**: UI layouts may change between updates
- **Performance**: Cache mappings when UI is stable
- **Robustness**: Implement fallbacks for failed detections

## 6. Example Projects and Tools

### Relevant Open Source Projects
- **UIED**: Combines Google OCR for text + CV for graphical elements
- **Label Studio**: OCR template for region annotation
- **DeepSceneTextReader**: Deep learning pipeline for scene text
- **LAREX**: Semi-automatic layout analysis tool

### Commercial Solutions
- **Google Cloud Vision**: Pre-built OCR with layout analysis
- **UiPath**: OCR activities with region clipping
- **Azure Cognitive Services**: Vision AI with UI understanding

## 7. Recommended Approach for Zwift

### Phase 1: Initial Calibration
1. Take full screenshot
2. Run EasyOCR/PaddleOCR to get all text regions
3. Display regions to user with numbers/labels
4. User identifies which region is speed, power, etc.
5. Save mapping configuration

### Phase 2: Automatic Classification
1. Build dataset from user calibrations
2. Train classifier on:
   - Relative position (normalized coordinates)
   - Text patterns (units, numeric ranges)
   - Size/aspect ratio of bounding box
3. Validate with new screenshots

### Phase 3: Vision LLM Enhancement
1. For ambiguous cases, use LLaVA/GPT-4V
2. Pass cropped region + context
3. Ask: "What UI element is this? (speed/power/distance/time)"
4. Cache results to minimize API calls

### Hybrid Approach (Recommended)
```python
def classify_ui_region(bbox, text, screenshot):
    # Try rule-based first (fast)
    if "km/h" in text or "mph" in text:
        return "speed"
    if "W" in text and text.replace("W", "").isdigit():
        return "power"
    
    # Try ML classifier (medium speed)
    features = extract_features(bbox, text)
    prediction = classifier.predict(features)
    if prediction.confidence > 0.8:
        return prediction.label
    
    # Fall back to vision LLM (slow but accurate)
    cropped = crop_region(screenshot, bbox)
    return vision_llm_classify(cropped, text)
```

## Key Takeaways

1. **Both EasyOCR and PaddleOCR** provide bounding box coordinates in quadrilateral format
2. **Full screenshot processing** is straightforward - both tools return all regions
3. **Classification can be hierarchical**: rules → ML → vision LLM
4. **Vision LLMs** are powerful but should be used judiciously due to latency/cost
5. **Hybrid approaches** combining traditional CV + ML + vision models work best
6. **User feedback loop** is crucial for handling UI variations

## Next Steps

1. Implement basic OCR region detection with EasyOCR/PaddleOCR
2. Create simple rule-based classifier for common patterns
3. Build calibration UI for user to map regions
4. Collect data for training ML classifier
5. Integrate vision LLM for edge cases
6. Test robustness across different Zwift UI configurations