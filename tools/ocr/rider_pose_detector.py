#!/usr/bin/env python3
# ABOUTME: Advanced rider pose detection for Zwift avatars
# Uses template matching and feature analysis for accurate pose classification

import json
from dataclasses import dataclass
from enum import Enum
from pathlib import Path
from typing import Dict, Optional, Tuple

import cv2
import numpy as np


class RiderPose(Enum):
    """Zwift rider positions with their drag characteristics

    Note: In Zwift, the "tuck" position counterintuitively has HIGH drag!
    """

    NORMAL_TUCK = 'normal_tuck'  # Tucked position (HIGH DRAG in Zwift)
    NORMAL_NORMAL = 'normal_normal'  # Standard upright (NORMAL DRAG)
    CLIMBING_SEATED = 'climbing_seated'  # Seated climbing (NORMAL DRAG)
    CLIMBING_STANDING = 'climbing_standing'  # Out of saddle (HIGH DRAG)
    UNKNOWN = 'unknown'


@dataclass
class PoseFeatures:
    """Features extracted from rider avatar for pose classification"""

    aspect_ratio: float  # Height/width ratio of bounding box
    torso_angle: float  # Estimated angle of torso from vertical
    head_height_ratio: float  # Head position relative to total height
    center_of_mass_y: float  # Vertical center of mass (normalized)
    upper_body_density: float  # Pixel density in upper half
    lower_body_density: float  # Pixel density in lower half
    symmetry_score: float  # Left-right symmetry (0-1)


class RiderPoseDetector:
    """Advanced pose detection for Zwift rider avatars"""

    def __init__(self, calibration_file: Optional[str] = None):
        """
        Initialize detector with optional calibration data

        Args:
            calibration_file: Path to JSON file with pose templates/thresholds
        """
        self.calibration = self._load_default_calibration()
        if calibration_file and Path(calibration_file).exists():
            with open(calibration_file) as f:
                custom_calibration = json.load(f)
                self.calibration.update(custom_calibration)

    def _load_default_calibration(self) -> Dict:
        """Load default pose detection thresholds based on Zwift's drag model"""
        return {
            'normal_tuck': {
                # Tucked position - HIGH DRAG in Zwift
                'aspect_ratio_range': (0.8, 1.3),
                'torso_angle_range': (30, 60),  # more horizontal
                'head_height_ratio_range': (0.5, 0.7),
                'center_of_mass_y_range': (0.55, 0.7),
            },
            'normal_normal': {
                # Standard upright - NORMAL DRAG
                'aspect_ratio_range': (1.3, 1.7),
                'torso_angle_range': (-15, 15),  # degrees from vertical
                'head_height_ratio_range': (0.7, 0.85),
                'center_of_mass_y_range': (0.45, 0.55),
            },
            'climbing_standing': {
                # Out of saddle - HIGH DRAG
                'aspect_ratio_range': (1.7, 2.5),
                'torso_angle_range': (-5, 25),
                'head_height_ratio_range': (0.8, 0.95),
                'center_of_mass_y_range': (0.3, 0.45),
                'symmetry_threshold': 0.7,  # Less symmetric when standing
            },
            'climbing_seated': {
                # Seated climbing - NORMAL DRAG
                'aspect_ratio_range': (1.4, 1.8),
                'torso_angle_range': (5, 30),
                'head_height_ratio_range': (0.65, 0.8),
                'center_of_mass_y_range': (0.45, 0.6),
            },
        }

    def extract_features(self, avatar_region: np.ndarray) -> PoseFeatures:
        """
        Extract pose-related features from avatar region

        Args:
            avatar_region: Cropped image of rider avatar

        Returns:
            PoseFeatures object with extracted metrics
        """
        # Convert to grayscale
        gray = cv2.cvtColor(avatar_region, cv2.COLOR_BGR2GRAY)

        # Apply edge detection
        edges = cv2.Canny(gray, 50, 150)

        # Find contours
        contours, _ = cv2.findContours(edges, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)

        if not contours:
            return PoseFeatures(0, 0, 0, 0.5, 0, 0, 0)

        # Get largest contour (rider silhouette)
        rider_contour = max(contours, key=cv2.contourArea)

        # Bounding box
        x, y, w, h = cv2.boundingRect(rider_contour)
        aspect_ratio = h / w if w > 0 else 0

        # Moments for center of mass
        moments = cv2.moments(rider_contour)
        if moments['m00'] > 0:
            cx = moments['m10'] / moments['m00']
            cy = moments['m01'] / moments['m00']
            cy_norm = cy / avatar_region.shape[0]
        else:
            cy_norm = 0.5

        # Estimate torso angle using PCA on contour points
        if len(rider_contour) > 5:
            points = rider_contour.reshape(-1, 2).astype(np.float32)
            mean, eigenvectors = cv2.PCACompute(points, mean=None)
            # First eigenvector gives main direction
            angle = np.arctan2(eigenvectors[0][1], eigenvectors[0][0]) * 180 / np.pi
            torso_angle = abs(90 - abs(angle))  # Angle from vertical
        else:
            torso_angle = 0

        # Head position estimation (top of contour)
        top_points = rider_contour[rider_contour[:, :, 1] < y + h * 0.3]
        if len(top_points) > 0:
            head_y = np.min(top_points[:, :, 1])
            head_height_ratio = 1 - ((head_y - y) / h)
        else:
            head_height_ratio = 0.8

        # Density analysis
        upper_half = edges[: avatar_region.shape[0] // 2, :]
        lower_half = edges[avatar_region.shape[0] // 2 :, :]
        upper_density = np.sum(upper_half > 0) / upper_half.size
        lower_density = np.sum(lower_half > 0) / lower_half.size

        # Symmetry analysis
        left_half = edges[:, : avatar_region.shape[1] // 2]
        right_half = edges[:, avatar_region.shape[1] // 2 :]
        right_flipped = cv2.flip(right_half, 1)

        # Resize to same width if needed
        min_width = min(left_half.shape[1], right_flipped.shape[1])
        left_half = left_half[:, :min_width]
        right_flipped = right_flipped[:, :min_width]

        symmetry_score = 1 - (
            np.sum(np.abs(left_half - right_flipped)) / (min_width * edges.shape[0] * 255)
        )

        return PoseFeatures(
            aspect_ratio=aspect_ratio,
            torso_angle=torso_angle,
            head_height_ratio=head_height_ratio,
            center_of_mass_y=cy_norm,
            upper_body_density=upper_density,
            lower_body_density=lower_density,
            symmetry_score=symmetry_score,
        )

    def classify_pose(self, features: PoseFeatures) -> RiderPose:
        """
        Classify rider pose based on extracted features

        Args:
            features: Extracted pose features

        Returns:
            Detected RiderPose
        """
        scores = {}

        for pose_name, thresholds in self.calibration.items():
            score = 0
            total_checks = 0

            # Check aspect ratio
            if 'aspect_ratio_range' in thresholds:
                min_ar, max_ar = thresholds['aspect_ratio_range']
                if min_ar <= features.aspect_ratio <= max_ar:
                    score += 1
                total_checks += 1

            # Check torso angle
            if 'torso_angle_range' in thresholds:
                min_angle, max_angle = thresholds['torso_angle_range']
                if min_angle <= features.torso_angle <= max_angle:
                    score += 1
                total_checks += 1

            # Check head height
            if 'head_height_ratio_range' in thresholds:
                min_hh, max_hh = thresholds['head_height_ratio_range']
                if min_hh <= features.head_height_ratio <= max_hh:
                    score += 1
                total_checks += 1

            # Check center of mass
            if 'center_of_mass_y_range' in thresholds:
                min_cm, max_cm = thresholds['center_of_mass_y_range']
                if min_cm <= features.center_of_mass_y <= max_cm:
                    score += 1
                total_checks += 1

            # Check symmetry for standing position
            if 'symmetry_threshold' in thresholds:
                if features.symmetry_score < thresholds['symmetry_threshold']:
                    score += 0.5  # Partial score for asymmetry
                total_checks += 0.5

            if total_checks > 0:
                scores[pose_name] = score / total_checks

        # Find best matching pose
        if scores:
            best_pose = max(scores.items(), key=lambda x: x[1])
            if best_pose[1] > 0.6:  # Confidence threshold
                try:
                    return RiderPose(best_pose[0])
                except ValueError:
                    return RiderPose.UNKNOWN

        return RiderPose.UNKNOWN

    def detect_pose(self, avatar_image: np.ndarray) -> Tuple[RiderPose, PoseFeatures]:
        """
        Detect rider pose from avatar image

        Args:
            avatar_image: Image region containing rider avatar

        Returns:
            Tuple of (detected pose, extracted features)
        """
        features = self.extract_features(avatar_image)
        pose = self.classify_pose(features)
        return pose, features

    def visualize_detection(
        self, avatar_image: np.ndarray, pose: RiderPose, features: PoseFeatures
    ) -> np.ndarray:
        """
        Create visualization of pose detection

        Args:
            avatar_image: Original avatar image
            pose: Detected pose
            features: Extracted features

        Returns:
            Annotated image
        """
        result = avatar_image.copy()

        # Add text overlay
        text_color = (0, 255, 0) if pose != RiderPose.UNKNOWN else (0, 0, 255)
        cv2.putText(
            result,
            f'Pose: {pose.value}',
            (10, 30),
            cv2.FONT_HERSHEY_SIMPLEX,
            0.7,
            text_color,
            2,
        )

        # Add feature info
        info_text = [
            f'AR: {features.aspect_ratio:.2f}',
            f'Angle: {features.torso_angle:.1f}°',
            f'Head: {features.head_height_ratio:.2f}',
            f'CoM: {features.center_of_mass_y:.2f}',
        ]

        for i, text in enumerate(info_text):
            cv2.putText(
                result,
                text,
                (10, 60 + i * 20),
                cv2.FONT_HERSHEY_SIMPLEX,
                0.5,
                (255, 255, 255),
                1,
            )

        return result


def calibrate_from_samples(sample_dir: str, output_file: str):
    """
    Calibrate pose detector from labeled sample images

    Args:
        sample_dir: Directory with pose-labeled images
        output_file: Path to save calibration JSON
    """
    detector = RiderPoseDetector()
    pose_features = {pose.value: [] for pose in RiderPose if pose != RiderPose.UNKNOWN}

    sample_path = Path(sample_dir)
    for pose_dir in sample_path.iterdir():
        if pose_dir.is_dir() and pose_dir.name in pose_features:
            for img_file in pose_dir.glob('*.jpg'):
                img = cv2.imread(str(img_file))
                if img is not None:
                    features = detector.extract_features(img)
                    pose_features[pose_dir.name].append(features)

    # Calculate ranges for each pose
    calibration = {}
    for pose_name, features_list in pose_features.items():
        if features_list:
            calibration[pose_name] = {
                'aspect_ratio_range': (
                    min(f.aspect_ratio for f in features_list) * 0.9,
                    max(f.aspect_ratio for f in features_list) * 1.1,
                ),
                'torso_angle_range': (
                    min(f.torso_angle for f in features_list) - 5,
                    max(f.torso_angle for f in features_list) + 5,
                ),
                'head_height_ratio_range': (
                    min(f.head_height_ratio for f in features_list) * 0.95,
                    max(f.head_height_ratio for f in features_list) * 1.05,
                ),
                'center_of_mass_y_range': (
                    min(f.center_of_mass_y for f in features_list) - 0.05,
                    max(f.center_of_mass_y for f in features_list) + 0.05,
                ),
            }

    with open(output_file, 'w') as f:
        json.dump(calibration, f, indent=2)

    print(f'Calibration saved to {output_file}')


if __name__ == '__main__':
    # Example usage
    detector = RiderPoseDetector()

    # Test on sample image
    test_image_path = 'docs/screenshots/with_climbing_1_01_36_01_42.jpg'
    if Path(test_image_path).exists():
        img = cv2.imread(test_image_path)
        # Extract avatar region (approximate)
        avatar_region = img[400:700, 860:1060]

        pose, features = detector.detect_pose(avatar_region)
        print(f'Detected pose: {pose.value}')
        print(f'Features: {features}')

        # Visualize
        result = detector.visualize_detection(avatar_region, pose, features)
        cv2.imwrite('pose_detection_result.jpg', result)
