#!/usr/bin/env python3
# ABOUTME: Test pose detection on Zwift example images
# Validates detection of the four main rider positions and their drag characteristics

from pathlib import Path
from zwift_ocr_improved import ZwiftOCRExtractor, ZwiftUILayout
from rider_pose_detector import RiderPoseDetector, RiderPose
import cv2
import json


def test_pose_detection_on_examples():
    """Test pose detection on the provided example images"""
    
    # Define test images with expected poses and drag levels
    test_cases = [
        {
            'file': 'docs/screenshots/normal_tuck.jpg',
            'expected_pose': 'normal_tuck',
            'expected_drag': 'HIGH',
            'description': 'Tucked position (counterintuitively HIGH drag in Zwift)'
        },
        {
            'file': 'docs/screenshots/normal_normal_drag.jpg',
            'expected_pose': 'normal_normal',
            'expected_drag': 'NORMAL',
            'description': 'Standard upright riding position'
        },
        {
            'file': 'docs/screenshots/climbing_in_the_saddle.jpg',
            'expected_pose': 'climbing_seated',
            'expected_drag': 'NORMAL',
            'description': 'Seated climbing position'
        },
        {
            'file': 'docs/screenshots/climbing_out_of_the_saddle.jpg',
            'expected_pose': 'climbing_standing',
            'expected_drag': 'HIGH',
            'description': 'Standing/dancing on pedals'
        }
    ]
    
    # Initialize detectors
    ocr_extractor = ZwiftOCRExtractor('paddle' if 'paddle' in globals() else 'easy')
    advanced_detector = RiderPoseDetector()
    
    print("Testing Pose Detection on Example Images")
    print("=" * 60)
    
    results = []
    
    for test in test_cases:
        print(f"\nTesting: {test['file']}")
        print(f"Expected: {test['description']}")
        print(f"Expected pose: {test['expected_pose']} ({test['expected_drag']} DRAG)")
        
        if not Path(test['file']).exists():
            print(f"  ERROR: File not found!")
            continue
        
        # Load image
        image = cv2.imread(test['file'])
        
        # Method 1: Quick OCR-based detection
        print("\n  Method 1 - OCR Extractor:")
        telemetry = ocr_extractor.extract_telemetry(test['file'])
        if 'rider_pose' in telemetry:
            detected_pose = telemetry['rider_pose']['value']
            detected_drag = 'HIGH' if detected_pose in ['normal_tuck', 'climbing_standing'] else 'NORMAL'
            match = "✓" if detected_pose == test['expected_pose'] else "✗"
            print(f"    Detected: {detected_pose} ({detected_drag} DRAG) {match}")
        
        # Method 2: Advanced feature-based detection
        print("\n  Method 2 - Advanced Detector:")
        # Extract avatar region (approximate center of screen)
        avatar_region = image[400:700, 860:1060]  # Adjust based on actual avatar location
        pose, features = advanced_detector.detect_pose(avatar_region)
        detected_drag = 'HIGH' if pose.value in ['normal_tuck', 'climbing_standing'] else 'NORMAL'
        match = "✓" if pose.value == test['expected_pose'] else "✗"
        print(f"    Detected: {pose.value} ({detected_drag} DRAG) {match}")
        print(f"    Features:")
        print(f"      Aspect ratio: {features.aspect_ratio:.2f}")
        print(f"      Torso angle: {features.torso_angle:.1f}°")
        print(f"      Head height: {features.head_height_ratio:.2f}")
        print(f"      Center of mass Y: {features.center_of_mass_y:.2f}")
        
        # Save visualization
        vis_output = test['file'].replace('.jpg', '_pose_detection.jpg')
        vis_image = advanced_detector.visualize_detection(avatar_region, pose, features)
        cv2.imwrite(vis_output, vis_image)
        print(f"    Visualization saved to: {vis_output}")
        
        results.append({
            'file': test['file'],
            'expected': test['expected_pose'],
            'detected_ocr': detected_pose if 'rider_pose' in telemetry else 'failed',
            'detected_advanced': pose.value,
            'features': {
                'aspect_ratio': features.aspect_ratio,
                'torso_angle': features.torso_angle,
                'head_height_ratio': features.head_height_ratio,
                'center_of_mass_y': features.center_of_mass_y
            }
        })
    
    # Save results
    with open('pose_detection_results.json', 'w') as f:
        json.dump(results, f, indent=2)
    
    print("\n" + "=" * 60)
    print("Summary:")
    print("=" * 60)
    
    # Calculate accuracy
    correct_ocr = sum(1 for r in results if r['detected_ocr'] == r['expected'])
    correct_advanced = sum(1 for r in results if r['detected_advanced'] == r['expected'])
    
    print(f"OCR Detector Accuracy: {correct_ocr}/{len(results)} ({correct_ocr/len(results)*100:.0f}%)")
    print(f"Advanced Detector Accuracy: {correct_advanced}/{len(results)} ({correct_advanced/len(results)*100:.0f}%)")
    
    # Print drag level summary
    print("\nDrag Level Mapping:")
    print("  HIGH DRAG: normal_tuck, climbing_standing")
    print("  NORMAL DRAG: normal_normal, climbing_seated")
    print("\nNote: In Zwift, the tucked position has HIGH drag, not low!")


def analyze_pose_differences():
    """Analyze the visual differences between poses"""
    
    print("\n\nAnalyzing Visual Differences Between Poses")
    print("=" * 60)
    
    # Load results if available
    if Path('pose_detection_results.json').exists():
        with open('pose_detection_results.json', 'r') as f:
            results = json.load(f)
        
        # Group by drag level
        high_drag = [r for r in results if r['expected'] in ['normal_tuck', 'climbing_standing']]
        normal_drag = [r for r in results if r['expected'] in ['normal_normal', 'climbing_seated']]
        
        print("\nHIGH DRAG Poses:")
        for r in high_drag:
            print(f"  {r['expected']}:")
            print(f"    - Aspect ratio: {r['features']['aspect_ratio']:.2f}")
            print(f"    - Torso angle: {r['features']['torso_angle']:.1f}°")
        
        print("\nNORMAL DRAG Poses:")
        for r in normal_drag:
            print(f"  {r['expected']}:")
            print(f"    - Aspect ratio: {r['features']['aspect_ratio']:.2f}")
            print(f"    - Torso angle: {r['features']['torso_angle']:.1f}°")


if __name__ == "__main__":
    test_pose_detection_on_examples()
    analyze_pose_differences()