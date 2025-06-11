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
# Test the main OCR extractor
uv run python zwift_ocr_improved_final.py ../../docs/screenshots/normal_1_01_16_02_21.jpg
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
> Extract telemetry from a screenshot using Python (full features)
>
> **POSITIONAL ARGUMENTS**
> * path - Path to the screenshot file

~~~bash
# Convert relative paths to absolute if needed
if [[ ! "$path" = /* ]]; then
    # Not an absolute path, make it absolute
    path="$(pwd)/$path"
fi

# Use final extractor with 100% accuracy including leaderboard
uv run python zwift_ocr_improved_final.py "$path"
~~~

## rust-ocr (path)
> Extract telemetry using Rust implementation (fastest)
>
> **POSITIONAL ARGUMENTS**  
> * path - Path to the screenshot file

~~~bash
# Build if needed and run Rust implementation
cd ../..
if [ ! -f target/release/zwift_ocr_compact ]; then
    echo "Building Rust OCR implementation..."
    cargo build --features ocr --bin zwift_ocr_compact --release
fi

# Convert relative path for Rust binary
if [[ ! "$path" = /* ]]; then
    path="tools/ocr/$path"
fi

./target/release/zwift_ocr_compact "$path"
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

## compact (path)
> Extract telemetry using the compact version
>
> **POSITIONAL ARGUMENTS**
> * path - Path to the screenshot file

~~~bash
uv run python zwift_ocr_compact.py "$path"
~~~

## calibrate-poses
> Calibrate pose detection with sample images

~~~bash
uv run python rider_pose_detector.py ../../docs/screenshots/normal_1_01_16_02_21.jpg
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

uv run python debug_visualizer_v3.py "$path" "debug_${path##*/}"
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

## compare (path)
> Compare Python and Rust OCR performance and output
>
> **POSITIONAL ARGUMENTS**
> * path - Path to the screenshot file

~~~bash
echo "=== Performance Comparison ==="
echo "Python (PaddleOCR):"
time uv run python zwift_ocr_compact.py "$path" > python_output.json

echo -e "\nRust (Tesseract):"
cd ../..
if [ ! -f target/release/zwift_ocr_compact ]; then
    echo "Building Rust implementation..."
    cargo build --features ocr --bin zwift_ocr_compact --release
fi

# Fix path for rust binary
rust_path="$path"
if [[ ! "$path" = /* ]]; then
    rust_path="tools/ocr/$path"
fi

time ./target/release/zwift_ocr_compact "$rust_path" > tools/ocr/rust_output.json

echo -e "\n=== Output Comparison ==="
cd tools/ocr

# Pretty print both outputs
echo "Python output:"
cat python_output.json | python -m json.tool | head -15

echo -e "\nRust output:"  
cat rust_output.json | python -m json.tool | head -15

# Show differences (ignore leaderboard field)
echo -e "\n=== Field Comparison (ignoring leaderboard) ==="
python -c "
import json
py = json.load(open('python_output.json'))
rs = json.load(open('rust_output.json'))
py.pop('leaderboard', None)
rs.pop('leaderboard', None)
print('Matching fields:', py == rs)
for k in py:
    if k in rs and py[k] != rs[k]:
        print(f'{k}: Python={py[k]} Rust={rs[k]}')
"
~~~

## build-rust
> Build the Rust OCR binary in release mode

~~~bash
cd ../.. && cargo build --features ocr --bin zwift_ocr_compact --release
~~~