# Session Checkpoint 190515
Created: 2025-01-12 19:05:15
Task: OCR calibration system for Zwift Race Finder
Progress: Discovered original approach was correct - targeted regions with field-specific preprocessing
Next: Polish calibration tools and document contribution process

## Work Done
- Created comprehensive calibration guide with field-specific preprocessing details
- Built multiple calibration tools (vision AI, PaddleOCR, multi-pass detection)
- Tested and validated manual 1920x1080 config with debug_ocr tool
- Updated OCR strategy document with key technical discoveries
- Discovered that full-image OCR fails due to 70+ text regions and false positives
- Found each field needs specific preprocessing (gradient needs inversion, altitude needs higher threshold)

## Failed Approaches
- Full-image PaddleOCR scanning - too many false positives, hard to classify
- One-size-fits-all preprocessing - different fields need different settings
- Automatic classification without manual validation - error-prone
- Using same approach for all fields - gradient font requires special handling