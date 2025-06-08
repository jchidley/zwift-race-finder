# Zwift OCR Tasks

## setup
> Install project dependencies and prepare environment

~~~bash
uv sync
~~~

## test
> Run all tests including OCR comparison

~~~bash
set -e
# Run from repository root to find test images
cd ../..
uv run python tools/ocr/zwift_ocr_prototype.py
uv run python tools/ocr/test_enhanced_extraction.py
uv run python tools/ocr/test_pose_detection.py
~~~

## lint
> Run code quality checks

~~~bash
set -e
uv run ruff check .
uv run ruff format . --check
~~~

## format
> Auto-format code

~~~bash
uv run ruff format .
uv run ruff check . --fix
~~~

## screenshot (path)
> Extract telemetry from a screenshot
>
> **POSITIONAL ARGUMENTS**
> * path - Path to the screenshot file

~~~bash
# Convert relative paths to absolute if needed
if [[ ! "$path" = /* ]]; then
    # Not an absolute path, make it absolute
    path="$(pwd)/$path"
fi

# Use v2 extractor for better accuracy
uv run python zwift_ocr_improved_v2.py "$path"
~~~

## video (path)
> Process a video file for telemetry extraction
>
> **POSITIONAL ARGUMENTS**
> * path - Path to the video file

**OPTIONS**
* --skip-frames
* --no-preview
* --analyze

~~~bash
uv run python zwift_video_processor.py "$path" "$@"
~~~

## compare-engines
> Compare OCR engine performance on test images

~~~bash
uv run python zwift_ocr_prototype.py
~~~

## calibrate-poses
> Calibrate pose detection with sample images

~~~bash
uv run python test_pose_detection.py
~~~

## debug (path)
> Create debug visualization showing extraction regions and results
>
> **POSITIONAL ARGUMENTS**
> * path - Path to the screenshot file

~~~bash
# Convert relative paths to absolute if needed
if [[ ! "$path" = /* ]]; then
    path="$(pwd)/$path"
fi

uv run python debug_visualizer.py "$path" "debug_${path##*/}"
echo "Debug image created: debug_${path##*/}"
~~~

## clean
> Remove generated files and caches

~~~bash
rm -rf __pycache__/
rm -rf .ruff_cache/
rm -f telemetry_*.csv telemetry_*.json telemetry.db
rm -f debug_*.jpg debug_*.png
find . -name "*.pyc" -delete
~~~