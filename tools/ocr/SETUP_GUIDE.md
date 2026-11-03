# Zwift screenshot OCR setup

This guide covers the retained compact screenshot extractors and calibration
utilities. The broken video and v3 visualizer wrappers and prototype console
entrypoint have been retired; see [README.md](README.md). Commands below are
source-checked examples, **not a runtime validation of current dependencies,
model availability, image accuracy, or your platform**.

Run project commands from the repository root. Keep personal screenshots,
configuration exports, and OCR output outside Git. Installation and model
downloads need network access; review them separately before running them.

## Rust compact extractor

Debian/Ubuntu builds need `pkg-config`, OpenSSL development headers, Leptonica,
and Tesseract with English language data. An administrator can install the native
prerequisites:

```bash
sudo apt install pkg-config libssl-dev libleptonica-dev libtesseract-dev tesseract-ocr tesseract-ocr-eng
```

The ocrs leaderboard engine loads these files from `~/.cache/ocrs/`:

- `text-detection.rten`
- `text-recognition.rten`

The current implementation loads local files; it does **not** automatically
download them. The source error messages identify the upstream locations:

- https://ocrs-models.s3-accelerate.amazonaws.com/text-detection.rten
- https://ocrs-models.s3-accelerate.amazonaws.com/text-recognition.rten

Acquire those models explicitly before relying on leaderboard extraction.
Missing models can leave leaderboard data unavailable; inspect output rather
than assuming successful extraction of every field.

Build and use the feature-gated binary:

```bash
cargo build --release --features ocr --bin zwift_ocr_compact
./target/release/zwift_ocr_compact /absolute/path/to/screenshot.png
./target/release/zwift_ocr_compact /absolute/path/to/screenshot.png --format text
# Optional parallel screenshot extraction, not video processing:
./target/release/zwift_ocr_compact /absolute/path/to/screenshot.png --parallel
```

JSON output goes to standard output by default. If saving it, choose a new private
output path rather than overwriting existing results. No Rust OCR build or image
extraction was run as part of the obsolete-workflow cleanup.

## Python compact extractor

Use uv-managed Python. The dependency declarations are in
[`pyproject.toml`](pyproject.toml); the old direct `easyocr` and `click`
dependencies and broken `zwift-ocr` console command are gone. `groq` remains for
vision-assisted calibration.

```bash
uv sync --project tools/ocr
uv run --project tools/ocr python tools/ocr/zwift_ocr_compact.py /absolute/path/to/screenshot.png
```

`uv sync` may resolve dependencies, refresh the lockfile, and download packages.
The existing lockfile was not refreshed during this source-only cleanup. Python,
PaddlePaddle and PaddleOCR version compatibility must be validated on the target
machine; a successful syntax check does not establish that compatibility. OCR
engine startup may also download model files.

The compact script prints telemetry and **overwrites `telemetry.json` in the
current working directory**. Preserve any existing file before running it, and
keep generated personal outputs outside Git.

The enhanced screenshot extractor is also retained:

```bash
uv run --project tools/ocr python tools/ocr/zwift_ocr_improved_final.py /absolute/path/to/screenshot.png --validate
```

It creates debug images by default and writes a timestamped results JSON file.
`--validate` checks plausibility; compare values with the actual screenshot to
assess accuracy. Historical single-image accuracy/speed figures are not current
guarantees.

## Local region inspection and calibration

For an existing configuration with a `regions` object containing `x`, `y`,
`width`, and `height`, draw the boxes to a new private image:

```bash
uv run --project tools/ocr python tools/ocr/visualize_regions.py /absolute/path/to/screenshot.png /absolute/path/to/regions.json /absolute/path/to/new-region-preview.png
```

For interactive mapping (a working graphical display is required):

```bash
uv run --project tools/ocr python tools/ocr/visual_region_mapper.py /absolute/path/to/screenshot.png
```

`calibrate_like_original.py`, `calibrate_multipass.py`,
`calibrate_with_paddleocr.py`, and the other retained calibration helpers remain
available for experiments. Their output is not automatically consumed by the
compact extractors; applying a calibration requires reviewing the relevant
extractor's regions. UI changes, scaling, and resolution can invalidate the
existing coordinates.

`calibrate_with_vision.py` retains Groq and other provider modes. Hosted modes
send screenshots externally and require separate approval plus approved
credential handling. No provider calls or credential checks were made during
this cleanup; provider/model support remains unvalidated.

## Unsupported paths

Do not use `zwift-ocr`, `mask video`, or `mask debug`. Their prototype/video/v3
visualizer targets were missing or have been removed. Other recipes in
`maskfile.md` and older guides have not been revalidated. There is no replacement
end-to-end video/stream pipeline or supported telemetry-database summary import
in this guide. Use screenshots with the compact extractor instead.

OCR output does not automatically update duration estimates or race results.
For measured finish-time ingestion, follow
[ZwiftPower history export/import](../../docs/howto/ZWIFTPOWER_EXPORT_STEPS.md);
do not substitute a screenshot clock or Strava moving time for a verified race
finish duration.
