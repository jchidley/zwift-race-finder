#!/usr/bin/env python3
# ABOUTME: Process Zwift video streams/recordings to extract telemetry data
# Handles live streams, video files, and can output to various formats

import cv2
import numpy as np
from pathlib import Path
import json
import time
import sqlite3
import csv
from typing import Dict, List, Optional, Tuple, Generator
from dataclasses import dataclass, asdict
from datetime import datetime
import threading
import queue
from collections import deque

# Import our OCR extractor
from zwift_ocr_improved import ZwiftOCRExtractor, ZwiftUILayout


@dataclass
class TelemetryFrame:
    """Single frame of telemetry data"""

    timestamp: float
    frame_number: int
    speed: Optional[float] = None
    distance: Optional[float] = None
    altitude: Optional[int] = None
    race_time: Optional[int] = None
    power: Optional[int] = None
    cadence: Optional[int] = None
    heart_rate: Optional[int] = None
    avg_power: Optional[int] = None
    energy: Optional[int] = None
    gradient: Optional[float] = None
    distance_to_finish: Optional[float] = None
    powerup_name: Optional[str] = None
    powerup_remaining: Optional[float] = None
    rider_pose: Optional[str] = None

    def to_dict(self) -> dict:
        """Convert to dictionary for JSON/CSV export"""
        return asdict(self)


class TelemetryStorage:
    """Handle storage of telemetry data to various formats"""

    def __init__(self, base_path: str, session_name: str):
        self.base_path = Path(base_path)
        self.session_name = session_name
        self.timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")

        # Create session directory
        self.session_dir = self.base_path / f"{self.session_name}_{self.timestamp}"
        self.session_dir.mkdir(parents=True, exist_ok=True)

        # Initialize storage backends
        self._init_sqlite()
        self._init_csv()
        self.json_data = []

    def _init_sqlite(self):
        """Initialize SQLite database"""
        self.db_path = self.session_dir / "telemetry.db"
        self.conn = sqlite3.connect(str(self.db_path))
        self.conn.execute(
            """
            CREATE TABLE IF NOT EXISTS telemetry (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp REAL NOT NULL,
                frame_number INTEGER NOT NULL,
                speed REAL,
                distance REAL,
                altitude INTEGER,
                race_time INTEGER,
                power INTEGER,
                cadence INTEGER,
                heart_rate INTEGER,
                avg_power INTEGER,
                energy INTEGER,
                gradient REAL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        """
        )
        self.conn.commit()

    def _init_csv(self):
        """Initialize CSV file"""
        self.csv_path = self.session_dir / "telemetry.csv"
        with open(self.csv_path, "w", newline="") as f:
            writer = csv.DictWriter(
                f,
                fieldnames=[
                    "timestamp",
                    "frame_number",
                    "speed",
                    "distance",
                    "altitude",
                    "race_time",
                    "power",
                    "cadence",
                    "heart_rate",
                    "avg_power",
                    "energy",
                    "gradient",
                ],
            )
            writer.writeheader()

    def add_frame(self, frame: TelemetryFrame):
        """Add a telemetry frame to all storage backends"""
        # SQLite
        self.conn.execute(
            """
            INSERT INTO telemetry (
                timestamp, frame_number, speed, distance, altitude,
                race_time, power, cadence, heart_rate, avg_power, energy, gradient
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        """,
            (
                frame.timestamp,
                frame.frame_number,
                frame.speed,
                frame.distance,
                frame.altitude,
                frame.race_time,
                frame.power,
                frame.cadence,
                frame.heart_rate,
                frame.avg_power,
                frame.energy,
                frame.gradient,
            ),
        )
        self.conn.commit()

        # CSV
        with open(self.csv_path, "a", newline="") as f:
            writer = csv.DictWriter(f, fieldnames=frame.to_dict().keys())
            writer.writerow(frame.to_dict())

        # JSON
        self.json_data.append(frame.to_dict())

    def save_json(self):
        """Save accumulated data to JSON file"""
        json_path = self.session_dir / "telemetry.json"
        with open(json_path, "w") as f:
            json.dump(self.json_data, f, indent=2)

    def close(self):
        """Close all storage backends"""
        self.save_json()
        self.conn.close()


class ZwiftVideoProcessor:
    """Process Zwift video to extract telemetry data"""

    def __init__(self, ocr_engine="paddle", storage_path="./telemetry_data"):
        self.ocr_extractor = ZwiftOCRExtractor(ocr_engine)
        self.storage_path = storage_path
        self.is_processing = False
        self.frame_queue = queue.Queue(maxsize=30)
        self.result_queue = queue.Queue()

        # Performance tracking
        self.fps_counter = deque(maxlen=30)
        self.last_fps_time = time.time()

    def process_frame(
        self, frame: np.ndarray, frame_number: int, timestamp: float
    ) -> TelemetryFrame:
        """Extract telemetry from a single frame"""
        # Create temporary file for OCR (required by current implementation)
        # In production, we'd modify the OCR extractor to work directly with numpy arrays
        temp_path = f"/tmp/zwift_frame_{frame_number}.jpg"
        cv2.imwrite(temp_path, frame)

        try:
            # Extract telemetry
            telemetry = self.ocr_extractor.extract_telemetry(temp_path)

            # Build telemetry frame
            frame_data = TelemetryFrame(timestamp=timestamp, frame_number=frame_number)

            # Map extracted values
            for field, data in telemetry.items():
                if data["value"] is not None:
                    setattr(frame_data, field, data["value"])

            return frame_data
        finally:
            # Clean up temp file
            Path(temp_path).unlink(missing_ok=True)

    def ocr_worker(self):
        """Worker thread for OCR processing"""
        while self.is_processing:
            try:
                frame_data = self.frame_queue.get(timeout=1)
                if frame_data is None:
                    break

                frame, frame_number, timestamp = frame_data
                result = self.process_frame(frame, frame_number, timestamp)
                self.result_queue.put(result)

            except queue.Empty:
                continue
            except Exception as e:
                print(f"OCR error: {e}")

    def process_video(
        self,
        video_path: str,
        skip_frames: int = 30,
        realtime: bool = False,
        show_preview: bool = True,
    ) -> str:
        """
        Process video file to extract telemetry

        Args:
            video_path: Path to video file
            skip_frames: Process every Nth frame (30 = 1 per second at 30fps)
            realtime: Process at video framerate (for live streams)
            show_preview: Show preview window with extraction overlay

        Returns:
            Path to session directory with extracted data
        """
        cap = cv2.VideoCapture(video_path)
        if not cap.isOpened():
            raise ValueError(f"Could not open video: {video_path}")

        # Get video properties
        fps = cap.get(cv2.CAP_PROP_FPS)
        total_frames = int(cap.get(cv2.CAP_PROP_FRAME_COUNT))

        print(f"Video properties:")
        print(f"  FPS: {fps}")
        print(f"  Total frames: {total_frames}")
        print(f"  Duration: {total_frames/fps:.1f}s")
        print(
            f"  Processing every {skip_frames} frames ({fps/skip_frames:.1f} samples/sec)"
        )

        # Initialize storage
        session_name = Path(video_path).stem
        storage = TelemetryStorage(self.storage_path, session_name)

        # Start OCR worker thread
        self.is_processing = True
        ocr_thread = threading.Thread(target=self.ocr_worker)
        ocr_thread.start()

        frame_count = 0
        processed_count = 0
        start_time = time.time()

        try:
            while True:
                ret, frame = cap.read()
                if not ret:
                    break

                # Calculate timestamp
                timestamp = frame_count / fps

                # Process frame at interval
                if frame_count % skip_frames == 0:
                    # Add to processing queue
                    try:
                        self.frame_queue.put(
                            (frame.copy(), frame_count, timestamp), timeout=0.1
                        )
                        processed_count += 1
                    except queue.Full:
                        print("Warning: Frame queue full, skipping frame")

                # Get results from queue
                try:
                    while True:
                        result = self.result_queue.get_nowait()
                        storage.add_frame(result)

                        # Update FPS counter
                        self.fps_counter.append(time.time())

                        # Print status
                        if len(self.fps_counter) > 1:
                            actual_fps = len(self.fps_counter) / (
                                self.fps_counter[-1] - self.fps_counter[0]
                            )
                            print(
                                f"\rFrame {frame_count}/{total_frames} "
                                f"({frame_count/total_frames*100:.1f}%) "
                                f"Processing: {actual_fps:.1f} fps",
                                end="",
                            )

                except queue.Empty:
                    pass

                # Show preview if requested
                if show_preview:
                    # Draw extraction regions
                    preview = frame.copy()
                    for region in ZwiftUILayout.get_all_regions():
                        if region.name not in ["minimap", "rider_list"]:
                            cv2.rectangle(
                                preview,
                                (region.x, region.y),
                                (region.x + region.width, region.y + region.height),
                                (0, 255, 0),
                                2,
                            )

                    # Resize for display
                    scale = 0.5
                    preview = cv2.resize(preview, None, fx=scale, fy=scale)
                    cv2.imshow("Zwift Telemetry Extraction", preview)

                    if cv2.waitKey(1) & 0xFF == ord("q"):
                        print("\nProcessing interrupted by user")
                        break

                # Simulate realtime processing if requested
                if realtime and frame_count > 0:
                    expected_time = frame_count / fps
                    actual_time = time.time() - start_time
                    if expected_time > actual_time:
                        time.sleep(expected_time - actual_time)

                frame_count += 1

        finally:
            # Cleanup
            self.is_processing = False
            self.frame_queue.put(None)  # Signal worker to stop
            ocr_thread.join()

            # Process remaining results
            while not self.result_queue.empty():
                result = self.result_queue.get()
                storage.add_frame(result)

            cap.release()
            cv2.destroyAllWindows()
            storage.close()

            # Print summary
            elapsed = time.time() - start_time
            print(f"\n\nProcessing complete:")
            print(f"  Total time: {elapsed:.1f}s")
            print(f"  Frames processed: {processed_count}")
            print(f"  Average processing rate: {processed_count/elapsed:.1f} fps")
            print(f"  Data saved to: {storage.session_dir}")

            return str(storage.session_dir)

    def process_stream(
        self,
        stream_url: str,
        duration: Optional[int] = None,
        skip_frames: int = 30,
        show_preview: bool = True,
    ) -> str:
        """
        Process live stream to extract telemetry

        Args:
            stream_url: URL or device index for stream
            duration: Maximum duration in seconds (None for unlimited)
            skip_frames: Process every Nth frame
            show_preview: Show preview window

        Returns:
            Path to session directory
        """
        # Similar to process_video but handles live streams
        # Can be RTMP URL, HTTP stream, or webcam index
        if stream_url.isdigit():
            stream_url = int(stream_url)  # Webcam device index

        return self.process_video(
            stream_url, skip_frames, realtime=True, show_preview=show_preview
        )


def analyze_telemetry(session_dir: str):
    """Analyze extracted telemetry data and generate summary statistics"""
    session_path = Path(session_dir)
    db_path = session_path / "telemetry.db"

    conn = sqlite3.connect(str(db_path))

    # Get summary statistics
    stats = conn.execute(
        """
        SELECT 
            COUNT(*) as total_samples,
            MIN(timestamp) as start_time,
            MAX(timestamp) as end_time,
            AVG(speed) as avg_speed,
            MAX(speed) as max_speed,
            AVG(power) as avg_power,
            MAX(power) as max_power,
            AVG(heart_rate) as avg_hr,
            MAX(heart_rate) as max_hr,
            MAX(distance) as total_distance,
            MAX(elevation) as total_elevation
        FROM telemetry
    """
    ).fetchone()

    print("\nTelemetry Analysis:")
    print(f"  Total samples: {stats[0]}")
    print(f"  Duration: {stats[2] - stats[1]:.1f}s")
    print(f"  Average speed: {stats[3]:.1f} km/h")
    print(f"  Max speed: {stats[4]:.1f} km/h")
    print(f"  Average power: {stats[5]:.0f}W")
    print(f"  Max power: {stats[6]:.0f}W")
    print(f"  Average HR: {stats[7]:.0f} bpm")
    print(f"  Max HR: {stats[8]:.0f} bpm")
    print(f"  Total distance: {stats[9]:.1f} km")
    print(f"  Total elevation: {stats[10]:.0f}m")

    conn.close()


def main():
    """Example usage of video processor"""
    import argparse

    parser = argparse.ArgumentParser(description="Extract telemetry from Zwift video")
    parser.add_argument("input", help="Video file path or stream URL")
    parser.add_argument(
        "--skip-frames",
        type=int,
        default=30,
        help="Process every Nth frame (default: 30)",
    )
    parser.add_argument(
        "--no-preview", action="store_true", help="Disable preview window"
    )
    parser.add_argument(
        "--realtime", action="store_true", help="Process at video framerate"
    )
    parser.add_argument(
        "--analyze", action="store_true", help="Analyze telemetry after extraction"
    )

    args = parser.parse_args()

    # Process video
    processor = ZwiftVideoProcessor()
    session_dir = processor.process_video(
        args.input,
        skip_frames=args.skip_frames,
        realtime=args.realtime,
        show_preview=not args.no_preview,
    )

    # Analyze if requested
    if args.analyze:
        analyze_telemetry(session_dir)


if __name__ == "__main__":
    main()
