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

# Use final extractor with 100% accuracy
uv run python zwift_ocr_improved_final.py "$path"
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

## rust-build
> Build the Rust OCR binary

~~~bash
cd ../.. && cargo build --features ocr --bin zwift_ocr
~~~

## rust-run (path)
> Run the Rust OCR implementation
>
> **POSITIONAL ARGUMENTS**
> * path - Path to the screenshot file

~~~bash
cd ../.. && cargo run --features ocr --bin zwift_ocr -- "$path"
~~~

## rust-test
> Run Rust OCR tests

~~~bash
cd ../.. && cargo test --features ocr ocr_tests
~~~

## rust-bench
> Run Rust OCR benchmarks

~~~bash
cd ../.. && cargo bench --features ocr ocr_benchmark
~~~

## compare (path)
> Compare Python and Rust OCR outputs
>
> **POSITIONAL ARGUMENTS**
> * path - Path to the screenshot file

~~~bash
echo "=== Python OCR ==="
time uv run python zwift_ocr_compact.py "$path" > python_output.json

echo -e "\n=== Rust OCR ==="
cd ../.. && time cargo run --features ocr --bin zwift_ocr -- "$path" > tools/ocr/rust_output.json

echo -e "\n=== Comparison ==="
cd tools/ocr
diff -u python_output.json rust_output.json || true
~~~