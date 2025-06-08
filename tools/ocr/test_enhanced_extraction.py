#!/usr/bin/env python3
# ABOUTME: Test enhanced OCR extraction with gradient and leaderboard parsing
# Demonstrates extraction of all telemetry including climb gradient and rider data

from zwift_ocr_improved import ZwiftOCRExtractor
from pathlib import Path
import json


def test_enhanced_extraction():
    """Test extraction of gradient and leaderboard data from Zwift screenshots"""
    
    # Initialize extractor
    extractor = ZwiftOCRExtractor('paddle' if 'paddle' in globals() else 'easy', debug=True)
    
    # Test both screenshots
    test_images = {
        'normal': 'docs/screenshots/normal_1_01_16_02_21.jpg',
        'climbing': 'docs/screenshots/with_climbing_1_01_36_01_42.jpg'
    }
    
    for scenario, image_path in test_images.items():
        print(f"\n{'='*60}")
        print(f"Testing {scenario.upper()} scenario: {image_path}")
        print('='*60)
        
        # Extract telemetry
        telemetry = extractor.extract_telemetry(image_path)
        
        # Extract rider list
        riders = extractor.extract_rider_list(image_path)
        
        # Display core telemetry
        print("\nCore Telemetry:")
        core_fields = ['speed', 'distance', 'altitude', 'race_time', 'power', 'cadence', 
                      'heart_rate', 'avg_power', 'energy', 'gradient', 'distance_to_finish']
        for field in core_fields:
            if field in telemetry:
                data = telemetry[field]
                if data['value'] is not None:
                    unit = {
                        'speed': 'km/h',
                        'distance': 'km',
                        'altitude': 'm',
                        'race_time': 's',
                        'power': 'W',
                        'cadence': 'rpm',
                        'heart_rate': 'bpm',
                        'avg_power': 'W',
                        'energy': 'kJ',
                        'gradient': '%',
                        'distance_to_finish': 'km'
                    }.get(field, '')
                    
                    # Format race_time as MM:SS
                    if field == 'race_time':
                        minutes = int(data['value'] // 60)
                        seconds = int(data['value'] % 60)
                        print(f"  {field:12}: {minutes:02d}:{seconds:02d}   (raw: '{data['raw_text']}')")
                    else:
                        print(f"  {field:12}: {data['value']:6} {unit:5} (raw: '{data['raw_text']}')")
                else:
                    print(f"  {field:12}: FAILED       (raw: '{data['raw_text']}')")
        
        # Display gradient specifically for climbing scenario
        if scenario == 'climbing' and 'gradient' in telemetry:
            print(f"\n*** GRADIENT DETECTED: {telemetry['gradient']['value']}% ***")
        
        # Display power-up information if detected
        if 'powerup_name' in telemetry and telemetry['powerup_name']['value']:
            print(f"\n*** POWER-UP ACTIVE: {telemetry['powerup_name']['raw_text']} ***")
            if 'powerup_remaining' in telemetry and telemetry['powerup_remaining']['value']:
                print(f"    Remaining: {telemetry['powerup_remaining']['value']}%")
        
        # Display rider pose
        if 'rider_pose' in telemetry:
            pose = telemetry['rider_pose']['value']
            pose_descriptions = {
                'normal_tuck': 'Tucked position (HIGH DRAG)',
                'normal_normal': 'Normal upright (NORMAL DRAG)',
                'climbing_seated': 'Seated climbing (NORMAL DRAG)',
                'climbing_standing': 'Out of saddle (HIGH DRAG)',
                'unknown': 'Unable to detect pose'
            }
            drag_level = "HIGH" if pose in ['normal_tuck', 'climbing_standing'] else "NORMAL"
            print(f"\n*** RIDER POSE: {pose_descriptions.get(pose, pose)} ***")
            print(f"    Drag Level: {drag_level}")
        
        # Display rider leaderboard
        print(f"\nRider Leaderboard ({len(riders)} riders detected):")
        for i, rider in enumerate(riders[:5]):  # Show top 5
            if rider.get('is_current_rider'):
                print(f"  >>> {rider['name']:20} {rider['watts_per_kg']:4.1f} w/kg  {rider['distance_km']:4.1f} km <<<")
            else:
                print(f"  {i+1:2d}. {rider['name']:20} {rider['watts_per_kg']:4.1f} w/kg  {rider['distance_km']:4.1f} km")
        
        # Save detailed results
        results = {
            'telemetry': telemetry,
            'riders': riders,
            'scenario': scenario
        }
        
        output_path = Path(image_path).with_suffix('.enhanced_results.json')
        with open(output_path, 'w') as f:
            json.dump(results, f, indent=2)
        print(f"\nDetailed results saved to: {output_path}")
        
        # Create enhanced visualization
        vis_path = Path(image_path).with_suffix('.enhanced_viz.jpg')
        extractor.visualize_extraction(image_path, str(vis_path))


def compare_scenarios():
    """Compare telemetry between normal riding and climbing"""
    
    print(f"\n{'='*60}")
    print("SCENARIO COMPARISON")
    print('='*60)
    
    # Load results from both scenarios
    normal_path = Path('docs/screenshots/normal_1_01_16_02_21.enhanced_results.json')
    climbing_path = Path('docs/screenshots/with_climbing_1_01_36_01_42.enhanced_results.json')
    
    if normal_path.exists() and climbing_path.exists():
        with open(normal_path) as f:
            normal = json.load(f)
        with open(climbing_path) as f:
            climbing = json.load(f)
        
        print("\nKey Differences:")
        
        # Compare power
        normal_power = normal['telemetry']['power']['value']
        climbing_power = climbing['telemetry']['power']['value']
        if normal_power and climbing_power:
            print(f"  Power: {normal_power}W (normal) vs {climbing_power}W (climbing)")
            print(f"  Power increase: +{climbing_power - normal_power}W ({(climbing_power/normal_power - 1)*100:.1f}%)")
        
        # Compare speed
        normal_speed = normal['telemetry']['speed']['value']
        climbing_speed = climbing['telemetry']['speed']['value']
        if normal_speed and climbing_speed:
            print(f"  Speed: {normal_speed} km/h (normal) vs {climbing_speed} km/h (climbing)")
            print(f"  Speed decrease: -{normal_speed - climbing_speed} km/h ({(1 - climbing_speed/normal_speed)*100:.1f}%)")
        
        # Gradient (only in climbing)
        gradient = climbing['telemetry'].get('gradient', {}).get('value')
        if gradient:
            print(f"  Gradient: {gradient}% (climbing only)")
        
        # Heart rate comparison
        normal_hr = normal['telemetry']['heart_rate']['value']
        climbing_hr = climbing['telemetry']['heart_rate']['value']
        if normal_hr and climbing_hr:
            print(f"  Heart Rate: {normal_hr} bpm (normal) vs {climbing_hr} bpm (climbing)")
            print(f"  HR increase: +{climbing_hr - normal_hr} bpm")
    else:
        print("Run test_enhanced_extraction() first to generate results")


if __name__ == "__main__":
    # Test enhanced extraction
    test_enhanced_extraction()
    
    # Compare scenarios
    compare_scenarios()