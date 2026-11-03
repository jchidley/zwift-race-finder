# Zwift OCR telemetry extraction

The retained OCR tools extract telemetry from **screenshots**. Start with the
compact Rust or Python implementation below. Calibration, region visualization,
and rider-pose tools remain available; none of these tools automatically imports
race history or changes the production prediction model.

These entrypoints were checked against source during workflow retirement, not
run against current Zwift images. Accuracy, speed, current PaddleOCR compatibility,
and calibration on your UI layout are **not validated by this cleanup**. Earlier
benchmark figures are historical observations, not current guarantees. Fixed
regions generally target a 1920×1080 layout; verify extracted values against the
image before using them as evidence.

## Compact Rust OCR

From the repository root, with the native prerequisites and model files described
in [SETUP_GUIDE.md](SETUP_GUIDE.md):

```bash
cargo build --release --features ocr --bin zwift_ocr_compact
./target/release/zwift_ocr_compact /absolute/path/to/screenshot.png
./target/release/zwift_ocr_compact /absolute/path/to/screenshot.png --format text
```

JSON is the default output format. `--parallel` selects the parallel extractor;
it is not a video-processing mode. The code supports speed, distance, altitude,
race time, power, cadence, heart rate, gradient, distance to finish, leaderboard,
and rider-pose extraction. Availability and correctness depend on the image.

## Compact Python OCR

From the repository root:

```bash
uv sync --project tools/ocr
uv run --project tools/ocr python tools/ocr/zwift_ocr_compact.py /absolute/path/to/screenshot.png
```

The script prints telemetry and **overwrites `telemetry.json` in the current
working directory**. Do not run it where that file contains data you need to
preserve. Keep screenshots and personal outputs private and outside Git.
Dependency installation may require network access and may refresh `uv.lock`;
neither installation nor a lock refresh was performed during this cleanup.

`zwift_ocr_improved_final.py` remains as an enhanced screenshot extractor. Its
CLI creates debug images by default and accepts `--validate` for plausibility
checks; this is not independent verification of OCR accuracy.

## Calibration and visualization

Retained tools include:

- `visual_region_mapper.py`: interactive region mapping (requires a GUI).
- `visualize_regions.py`: draw boxes from a region configuration onto an image.
- `calibrate_like_original.py`, `calibrate_multipass.py`, and
  `calibrate_with_paddleocr.py`: local calibration experiments.
- `calibrate_with_vision.py`: vision-assisted calibration. The `groq` dependency
  is retained because this tool uses it. Hosted providers transmit the image to
  an external service; obtain approval before sending private screenshots and
  supply credentials through the approved environment mechanism. Provider/model
  availability is not validated here.
- `rider_pose_detector.py` and `leaderboard_extractor_v3.py`: standalone helpers.

See [SETUP_GUIDE.md](SETUP_GUIDE.md) for direct commands. Calibration output does
not automatically update the compact extractors' hard-coded regions.

## Retired entrypoints and unsupported integration

- `zwift_video_processor.py` was removed: it imported the missing
  `zwift_ocr_improved` module.
- `debug_visualizer_v3.py` was removed: it imported the missing
  `zwift_ocr_improved_v3` module. Use `visualize_regions.py` for region boxes or
  the enhanced screenshot extractor for debug output.
- The `zwift-ocr` console entrypoint targeted the missing
  `zwift_ocr_prototype` module and was removed. Invoke the retained scripts
  directly with `uv run ... python ...`.
- Unused direct `easyocr` and `click` dependencies were removed; `groq` remains.

There is no supported end-to-end video/stream wrapper or telemetry-to-race-DB
import here. `maskfile.md` and older OCR guides contain legacy examples;
**`mask video` and `mask debug` still refer to removed wrappers**. Use the direct
commands above rather than those tasks. Other legacy task-runner recipes have
not been revalidated.

For supported **measured race-history ingestion**, use
[the ZwiftPower export/import workflow](../../docs/howto/ZWIFTPOWER_EXPORT_STEPS.md).
OCR race-clock text is not automatically a measured finish result.

## Further reference

[TECHNICAL_REFERENCE.md](TECHNICAL_REFERENCE.md),
[CALIBRATION_GUIDE.md](CALIBRATION_GUIDE.md), and
[TESTING_GUIDE.md](TESTING_GUIDE.md) retain implementation/history context. Treat
older wrapper, setup, accuracy, and performance claims there as unvalidated;
this README and the setup guide define the current entrypoints.
