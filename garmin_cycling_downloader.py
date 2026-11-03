#!/usr/bin/env python3
"""Streamlined Garmin Connect Cycling Activities Downloader.

Downloads cycling activities from Garmin Connect with full data preservation.
Supports incremental downloads to keep your collection up to date.

Requires: pip install garminconnect rich

Usage:
    # Download last 21 days (default)
    python garmin_downloader.py

    # Get new activities since last download
    python garmin_downloader.py --mode newer

    # Get older activities (30 days before oldest existing)
    python garmin_downloader.py --mode older --limit 30

    # Traditional mode with custom days
    python garmin_downloader.py --mode days --days 60 --format tcx
"""

import argparse
import getpass
import io
import logging
import sys
import zipfile
from concurrent.futures import ThreadPoolExecutor, as_completed
from datetime import datetime, timedelta
from pathlib import Path
from typing import Dict, List, Optional, Tuple

try:
    from garminconnect import Garmin
    from rich.console import Console
    from rich.logging import RichHandler
    from rich.progress import track
    from rich.prompt import Confirm
    from rich.table import Table
except ImportError as e:
    print(f"Missing required package: {e}")
    print("Install with: pip install garminconnect rich")
    sys.exit(1)


class GarminDownloader:
    """Streamlined Garmin Connect cycling activities downloader."""

    CYCLING_TYPES = {
        "cycling",
        "road_biking",
        "mountain_biking",
        "gravel_cycling",
        "cyclocross",
        "track_cycling",
        "bmx",
        "indoor_cycling",
        "virtual_ride",
        "virtual_cycling",
    }

    FORMATS = {
        "fit": "ORIGINAL",
        "tcx": "TCX",
        "gpx": "GPX",
    }

    def __init__(
        self,
        days: int = 21,
        format_type: str = "fit",
        output_dir: str = "cycling_activities",
        max_workers: int = 3,
        mode: str = "days",
        limit: int = 30,
    ):
        self.days = days
        self.format_type = format_type.lower()
        self.output_dir = Path(output_dir)
        self.max_workers = max_workers
        self.mode = mode
        self.limit = limit

        if self.format_type not in self.FORMATS:
            raise ValueError(f"Format must be one of: {', '.join(self.FORMATS.keys())}")

        self.output_dir.mkdir(parents=True, exist_ok=True)
        self.console = Console()
        self.client: Optional[Garmin] = None
        self._setup_logging()

    def _setup_logging(self) -> None:
        """Configure logging with Rich handler."""
        logging.basicConfig(
            level=logging.INFO,
            format="%(message)s",
            handlers=[RichHandler(console=self.console, rich_tracebacks=True)],
        )
        self.logger = logging.getLogger(__name__)

    def _handle_error(
        self, operation: str, error: Exception, item_id: str = ""
    ) -> None:
        """Centralized error handling."""
        item_str = f" {item_id}" if item_id else ""
        self.logger.error(f"✗ {operation}{item_str}: {error}")

    def _make_filename(self, activity: Dict[str, any]) -> str:
        """Create safe filename from activity data."""
        activity_id = activity["activityId"]
        name = activity.get("activityName", "Unnamed Activity")
        start_time = activity.get("startTimeLocal", "")

        if "T" in start_time:
            date_part, time_part = start_time.split("T")
            time_part = time_part.split(".")[0]
        else:
            date_part = start_time
            time_part = ""

        safe_name = "".join(c for c in name if c.isalnum() or c in " -_").strip()

        if time_part:
            return f"{date_part} {time_part}_{safe_name}_{activity_id}.{self.format_type}"
        else:
            return f"{date_part}_{safe_name}_{activity_id}.{self.format_type}"

    def _scan_existing_activities(
        self,
    ) -> Tuple[Optional[datetime], Optional[datetime], set[str]]:
        """Scan output directory for existing activity files.

        Returns:
            Tuple of (earliest_date, latest_date, activity_ids)
        """
        if not self.output_dir.exists():
            return None, None, set()

        earliest_date = None
        latest_date = None
        activity_ids = set()

        for file in self.output_dir.glob(f"*.{self.format_type}"):
            parts = file.stem.split("_")
            if len(parts) >= 2:
                try:
                    first_part = parts[0]

                    if " " in first_part and len(parts) >= 3:
                        date_str = first_part.split(" ")[0]
                        date = datetime.strptime(date_str, "%Y-%m-%d")
                        activity_id = parts[-1]
                    else:
                        date_str = first_part
                        date = datetime.strptime(date_str, "%Y-%m-%d")
                        activity_id = parts[-1]

                    if earliest_date is None or date < earliest_date:
                        earliest_date = date
                    if latest_date is None or date > latest_date:
                        latest_date = date

                    activity_ids.add(activity_id)

                except (ValueError, IndexError):
                    continue

        return earliest_date, latest_date, activity_ids

    def authenticate(self) -> bool:
        """Authenticate with Garmin Connect."""
        self.console.print("🔐 [bold blue]Garmin Connect Authentication[/bold blue]")

        try:
            email = input("Enter your Garmin Connect email: ")
            password = getpass.getpass("Enter your password: ")

            self.client = Garmin(email, password)
            self.client.login()
            self.logger.info("✓ Successfully authenticated")
            return True
        except Exception as e:
            self._handle_error("Authentication failed", e)
            return False

    def get_activities(self, skip_existing: bool = True) -> List[Dict]:
        """Fetch cycling activities based on mode."""
        if not self.client:
            raise RuntimeError("Not authenticated")

        if self.mode == "newer":
            earliest, latest, existing_ids = self._scan_existing_activities()
            if latest is None:
                self.logger.warning(
                    "No existing activities found. Using default 21 days."
                )
                end_date = datetime.now()
                start_date = end_date - timedelta(days=21)
            else:
                start_date = latest + timedelta(days=1)
                end_date = datetime.now()
                self.logger.info(f"Looking for activities newer than {latest.date()}")

        elif self.mode == "older":
            earliest, latest, existing_ids = self._scan_existing_activities()
            if earliest is None:
                self.logger.warning(
                    "No existing activities found. Using default 21 days."
                )
                end_date = datetime.now()
                start_date = end_date - timedelta(days=21)
            else:
                end_date = earliest - timedelta(days=1)
                start_date = end_date - timedelta(days=self.limit)
                self.logger.info(f"Looking for activities older than {earliest.date()}")

        else:
            end_date = datetime.now()
            start_date = end_date - timedelta(days=self.days)
            existing_ids = set()
            if skip_existing:
                _, _, existing_ids = self._scan_existing_activities()

        self.logger.info(
            f"Fetching activities from {start_date.date()} to {end_date.date()}"
        )

        try:
            all_activities = self.client.get_activities_by_date(
                start_date.strftime("%Y-%m-%d"), end_date.strftime("%Y-%m-%d")
            )

            cycling_activities = [
                activity
                for activity in all_activities
                if activity.get("activityType", {}).get("typeKey") in self.CYCLING_TYPES
            ]

            if skip_existing and existing_ids:
                original_count = len(cycling_activities)
                cycling_activities = [
                    activity
                    for activity in cycling_activities
                    if str(activity["activityId"]) not in existing_ids
                ]
                skipped = original_count - len(cycling_activities)
                if skipped > 0:
                    self.logger.info(
                        f"Skipping {skipped} already downloaded activities"
                    )

            self.logger.info(
                f"Found {len(cycling_activities)} cycling activities to download"
            )
            return cycling_activities

        except Exception as e:
            self._handle_error("Failed to fetch activities", e)
            return []

    def download_single_activity(self, activity: Dict) -> bool:
        """Download a single activity file."""
        activity_id = activity["activityId"]
        filename = self._make_filename(activity)
        filepath = self.output_dir / filename

        if filepath.exists():
            self.logger.debug(f"Skipping existing file: {filename}")
            return True

        try:
            format_attr = getattr(
                self.client.ActivityDownloadFormat, self.FORMATS[self.format_type]
            )
            data = self.client.download_activity(activity_id, dl_fmt=format_attr)

            if not data:
                self.logger.warning(f"Empty data for activity {activity_id}")
                return False

            if len(data) > 4 and data[:4] == b"PK\x03\x04":
                try:
                    with zipfile.ZipFile(io.BytesIO(data)) as zf:
                        file_list = zf.namelist()
                        if file_list:
                            data = zf.read(file_list[0])
                            self.logger.debug(f"Extracted {file_list[0]} from ZIP")
                        else:
                            self.logger.warning(
                                f"Empty ZIP file for activity {activity_id}"
                            )
                            return False
                except zipfile.BadZipFile:
                    self.logger.warning(f"Invalid ZIP file for activity {activity_id}")
                    return False

            with open(filepath, "wb") as f:
                f.write(data)

            return True

        except Exception as e:
            self._handle_error("Download failed", e, str(activity_id))
            return False

    def download_activities(self, activities: List[Dict]) -> int:
        """Download activities with concurrent processing and progress tracking."""
        if not activities:
            return 0

        successful = 0

        with ThreadPoolExecutor(max_workers=self.max_workers) as executor:
            future_to_activity = {
                executor.submit(self.download_single_activity, activity): activity
                for activity in activities
            }

            for future in track(
                as_completed(future_to_activity),
                total=len(activities),
                description="Downloading activities...",
            ):
                if future.result():
                    successful += 1

        return successful

    def display_activities(self, activities: List[Dict]) -> None:
        """Display activities in a formatted table."""
        if not activities:
            return

        table = Table(title=f"Found {len(activities)} Cycling Activities")
        table.add_column("Date", style="cyan", width=12)
        table.add_column("Activity Name", style="white", width=30)
        table.add_column("Type", style="yellow", width=15)
        table.add_column("Distance", justify="right", style="green", width=10)
        table.add_column("Duration", justify="right", style="blue", width=10)

        for activity in activities:
            date = activity.get("startTimeLocal", "").split("T")[0]
            name = activity.get("activityName", "Unnamed")[:28]
            activity_type = (
                activity.get("activityType", {})
                .get("typeKey", "")
                .replace("_", " ")
                .title()
            )

            distance = activity.get("distance")
            distance_str = f"{distance / 1000:.1f} km" if distance else "N/A"

            duration = activity.get("duration")
            if duration:
                hours, remainder = divmod(int(duration), 3600)
                minutes, _ = divmod(remainder, 60)
                duration_str = (
                    f"{hours:02d}:{minutes:02d}" if hours else f"{minutes} min"
                )
            else:
                duration_str = "N/A"

            table.add_row(date, name, activity_type, distance_str, duration_str)

        self.console.print(table)

    def run(self) -> None:
        """Execute the download process."""
        self.console.print(
            "🚴 [bold green]Garmin Connect Cycling Downloader[/bold green]"
        )
        self.console.print(f"📁 Output: {self.output_dir.absolute()}")
        self.console.print(f"📊 Format: {self.format_type.upper()}")

        if self.mode == "newer":
            self.console.print("🔄 Mode: Download newer activities")
        elif self.mode == "older":
            self.console.print(
                f"🔄 Mode: Download older activities (limit: {self.limit} days)"
            )
        else:
            self.console.print(f"🔄 Mode: Download last {self.days} days")

        if self.mode in ["newer", "older"]:
            earliest, latest, existing_count = self._scan_existing_activities()
            if earliest and latest:
                self.console.print(
                    f"📅 Existing activities: {earliest.date()} to {latest.date()} ({len(existing_count)} files)"
                )

        if not self.authenticate():
            return

        activities = self.get_activities()
        if not activities:
            if self.mode == "newer":
                self.console.print("✅ No new activities found - you're up to date!")
            else:
                self.console.print(
                    "❌ No cycling activities found in the specified range"
                )
            return

        self.display_activities(activities)

        if not Confirm.ask(f"\nDownload {len(activities)} activities?", default=True):
            self.console.print("Cancelled")
            return

        successful = self.download_activities(activities)

        self.console.print("\n🎉 [bold green]Complete![/bold green]")
        self.console.print(f"✅ Downloaded: {successful}/{len(activities)} activities")

        if successful < len(activities):
            self.console.print(f"⚠️  {len(activities) - successful} failed")


def main():
    """Main entry point with argument parsing."""
    parser = argparse.ArgumentParser(
        description="Download cycling activities from Garmin Connect",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Format Guide:
  fit = Maximum data preservation (power, cadence, all sensors) - RECOMMENDED
  tcx = Good compatibility (HR, cadence, basic power data)
  gpx = Basic GPS only (smallest files, basic compatibility)
        """,
    )

    parser.add_argument(
        "--days", type=int, default=21, help="Days to look back (default: 21)"
    )
    parser.add_argument(
        "--format",
        choices=["fit", "tcx", "gpx"],
        default="fit",
        help="Download format (default: fit)",
    )
    parser.add_argument(
        "--output",
        default="cycling_activities",
        help="Output directory (default: cycling_activities)",
    )
    parser.add_argument(
        "--workers", type=int, default=3, help="Concurrent downloads (default: 3)"
    )
    parser.add_argument(
        "--mode",
        choices=["days", "newer", "older"],
        default="days",
        help="Download mode: 'days' (last N days), 'newer' (after newest existing), 'older' (before oldest existing)",
    )
    parser.add_argument(
        "--limit",
        type=int,
        default=30,
        help="For 'older' mode: how many days back to look (default: 30)",
    )

    args = parser.parse_args()

    try:
        downloader = GarminDownloader(
            days=args.days,
            format_type=args.format,
            output_dir=args.output,
            max_workers=args.workers,
            mode=args.mode,
            limit=args.limit,
        )
        downloader.run()

    except KeyboardInterrupt:
        print("\n⏹️  Interrupted by user")
    except Exception as e:
        print(f"❌ Error: {e}")
        raise


if __name__ == "__main__":
    main()
