#!/usr/bin/env python3
"""
Zwift Telemetry Analyzer - Advanced analysis using the compact OCR library

This program demonstrates how to use zwift_ocr_compact.py as a library
and adds extra analysis functionality on top of the core OCR.
"""

import sys
import json
from datetime import datetime
from pathlib import Path
from typing import Dict, Any, Optional, List
import statistics

# Import the compact OCR as a library
from zwift_ocr_compact import ZwiftOCR


class TelemetryAnalyzer:
    """Analyzes Zwift telemetry data with advanced features"""
    
    def __init__(self):
        self.ocr = ZwiftOCR()
        self.history = []
        
    def analyze_screenshot(self, image_path: str) -> Dict[str, Any]:
        """Extract and analyze telemetry from a screenshot"""
        
        # Use the compact OCR for core extraction
        telemetry = self.ocr.extract(image_path)
        
        # Add timestamp
        telemetry['timestamp'] = datetime.now().isoformat()
        telemetry['source_image'] = image_path
        
        # Perform additional analysis
        analysis = {
            'telemetry': telemetry,
            'analysis': self._analyze_telemetry(telemetry),
            'validation': self._validate_telemetry(telemetry)
        }
        
        # Add to history for trend analysis
        self.history.append(telemetry)
        
        return analysis
    
    def _analyze_telemetry(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """Perform advanced analysis on telemetry data"""
        
        analysis = {}
        
        # Power analysis
        if data.get('power') and data.get('heart_rate'):
            analysis['power_hr_ratio'] = round(data['power'] / data['heart_rate'], 2)
            analysis['estimated_intensity'] = self._estimate_intensity(
                data['power'], data['heart_rate']
            )
        
        # Speed analysis
        if data.get('speed') and data.get('gradient'):
            analysis['climbing_speed'] = self._analyze_climbing_speed(
                data['speed'], data['gradient']
            )
        
        # Race progress
        if data.get('distance') and data.get('distance_to_finish'):
            total_distance = data['distance'] + data['distance_to_finish']
            analysis['progress_percentage'] = round(
                (data['distance'] / total_distance) * 100, 1
            )
            analysis['total_race_distance'] = round(total_distance, 1)
        
        # Leaderboard analysis
        if data.get('leaderboard'):
            analysis['leaderboard_stats'] = self._analyze_leaderboard(
                data['leaderboard']
            )
        
        # Performance zones
        if data.get('power'):
            analysis['power_zone'] = self._get_power_zone(data['power'])
        
        if data.get('heart_rate'):
            analysis['hr_zone'] = self._get_hr_zone(data['heart_rate'])
        
        return analysis
    
    def _validate_telemetry(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """Validate telemetry data for anomalies"""
        
        validation = {
            'valid': True,
            'warnings': [],
            'field_status': {}
        }
        
        # Check each field
        checks = {
            'speed': (0, 100, "km/h"),
            'power': (0, 2000, "W"),
            'heart_rate': (40, 220, "bpm"),
            'cadence': (0, 150, "rpm"),
            'gradient': (-20, 20, "%")
        }
        
        for field, (min_val, max_val, unit) in checks.items():
            value = data.get(field)
            if value is not None:
                if min_val <= value <= max_val:
                    validation['field_status'][field] = "valid"
                else:
                    validation['field_status'][field] = "out_of_range"
                    validation['warnings'].append(
                        f"{field}: {value}{unit} is outside normal range ({min_val}-{max_val})"
                    )
                    validation['valid'] = False
            else:
                validation['field_status'][field] = "missing"
        
        # Check data consistency
        if data.get('power') and data.get('speed'):
            if data['power'] > 300 and data['speed'] < 20:
                validation['warnings'].append(
                    "High power with low speed - possible climb or calibration issue"
                )
        
        return validation
    
    def _estimate_intensity(self, power: int, heart_rate: int) -> str:
        """Estimate workout intensity based on power and HR"""
        
        ratio = power / heart_rate
        
        if ratio < 1.2:
            return "Easy/Recovery"
        elif ratio < 1.5:
            return "Endurance"
        elif ratio < 1.8:
            return "Tempo"
        elif ratio < 2.1:
            return "Threshold"
        else:
            return "VO2/Anaerobic"
    
    def _analyze_climbing_speed(self, speed: float, gradient: float) -> str:
        """Analyze climbing performance"""
        
        if gradient <= 0:
            return "Not climbing"
        elif gradient < 3:
            return "False flat"
        elif gradient < 6:
            if speed > 20:
                return "Strong climbing"
            else:
                return "Steady climbing"
        elif gradient < 10:
            if speed > 15:
                return "Excellent climbing"
            elif speed > 10:
                return "Good climbing"
            else:
                return "Struggling"
        else:  # gradient >= 10
            if speed > 10:
                return "Mountain goat!"
            elif speed > 7:
                return "Surviving"
            else:
                return "Grinding"
    
    def _analyze_leaderboard(self, leaderboard: List[Dict]) -> Dict[str, Any]:
        """Analyze leaderboard statistics"""
        
        stats = {
            'total_riders': len(leaderboard),
            'current_position': None,
            'gap_to_leader': None,
            'riders_ahead': 0,
            'riders_behind': 0,
            'avg_watts_per_kg': None
        }
        
        # Find current rider and calculate stats
        current_idx = None
        wkg_values = []
        
        for i, rider in enumerate(leaderboard):
            if rider.get('wkg'):
                wkg_values.append(rider['wkg'])
            
            if rider.get('current'):
                current_idx = i
                stats['current_position'] = i + 1
        
        if current_idx is not None:
            stats['riders_ahead'] = current_idx
            stats['riders_behind'] = len(leaderboard) - current_idx - 1
            
            # Gap to leader
            if current_idx > 0 and leaderboard[0].get('delta'):
                stats['gap_to_leader'] = leaderboard[0]['delta']
        
        if wkg_values:
            stats['avg_watts_per_kg'] = round(statistics.mean(wkg_values), 1)
            stats['max_watts_per_kg'] = round(max(wkg_values), 1)
            stats['min_watts_per_kg'] = round(min(wkg_values), 1)
        
        return stats
    
    def _get_power_zone(self, power: int) -> str:
        """Determine power training zone (assumes FTP of 250W)"""
        # This should be configurable based on user's FTP
        ftp = 250  # Default FTP
        
        pct_ftp = (power / ftp) * 100
        
        if pct_ftp < 55:
            return "Z1 Recovery"
        elif pct_ftp < 75:
            return "Z2 Endurance"
        elif pct_ftp < 90:
            return "Z3 Tempo"
        elif pct_ftp < 105:
            return "Z4 Threshold"
        elif pct_ftp < 120:
            return "Z5 VO2 Max"
        else:
            return "Z6+ Neuromuscular"
    
    def _get_hr_zone(self, heart_rate: int) -> str:
        """Determine heart rate zone (assumes max HR of 185)"""
        # This should be configurable based on user's max HR
        max_hr = 185  # Default max HR
        
        pct_max = (heart_rate / max_hr) * 100
        
        if pct_max < 60:
            return "Z1 Recovery"
        elif pct_max < 70:
            return "Z2 Easy"
        elif pct_max < 80:
            return "Z3 Moderate"
        elif pct_max < 90:
            return "Z4 Hard"
        else:
            return "Z5 Maximum"
    
    def generate_report(self) -> Dict[str, Any]:
        """Generate a summary report of all analyzed data"""
        
        if not self.history:
            return {"error": "No data analyzed yet"}
        
        report = {
            'total_samples': len(self.history),
            'time_range': {
                'start': self.history[0]['timestamp'],
                'end': self.history[-1]['timestamp']
            },
            'averages': {},
            'maximums': {},
            'summary': {}
        }
        
        # Calculate averages and maximums
        fields = ['speed', 'power', 'heart_rate', 'cadence', 'gradient']
        
        for field in fields:
            values = [h[field] for h in self.history if h.get(field) is not None]
            if values:
                report['averages'][field] = round(statistics.mean(values), 1)
                report['maximums'][field] = max(values)
        
        # Distance summary
        distances = [h['distance'] for h in self.history if h.get('distance')]
        if distances:
            report['summary']['total_distance'] = round(max(distances), 1)
        
        # Power analysis
        if 'power' in report['averages']:
            report['summary']['normalized_power'] = self._calculate_normalized_power()
        
        return report
    
    def _calculate_normalized_power(self) -> Optional[float]:
        """Calculate normalized power from history"""
        powers = [h['power'] for h in self.history if h.get('power') is not None]
        if len(powers) < 30:  # Need at least 30 seconds of data
            return None
        
        # Simplified NP calculation (should use 30s rolling average)
        return round(statistics.mean(powers) * 1.05, 0)  # Simplified!


def print_analysis(analysis: Dict[str, Any]):
    """Pretty print the analysis results"""
    
    print("\n" + "="*60)
    print("ZWIFT TELEMETRY ANALYSIS")
    print("="*60)
    
    # Basic telemetry
    telemetry = analysis['telemetry']
    print("\n📊 TELEMETRY DATA:")
    print(f"   Speed: {telemetry.get('speed', 'N/A')} km/h")
    print(f"   Power: {telemetry.get('power', 'N/A')} W")
    print(f"   Heart Rate: {telemetry.get('heart_rate', 'N/A')} bpm")
    print(f"   Cadence: {telemetry.get('cadence', 'N/A')} rpm")
    print(f"   Gradient: {telemetry.get('gradient', 'N/A')}%")
    print(f"   Distance: {telemetry.get('distance', 'N/A')} km")
    print(f"   Race Time: {telemetry.get('race_time', 'N/A')}")
    
    # Analysis results
    analysis_data = analysis['analysis']
    print("\n📈 ANALYSIS:")
    
    if 'power_hr_ratio' in analysis_data:
        print(f"   Power/HR Ratio: {analysis_data['power_hr_ratio']}")
        print(f"   Intensity: {analysis_data['estimated_intensity']}")
    
    if 'climbing_speed' in analysis_data:
        print(f"   Climbing: {analysis_data['climbing_speed']}")
    
    if 'progress_percentage' in analysis_data:
        print(f"   Race Progress: {analysis_data['progress_percentage']}%")
        print(f"   Total Distance: {analysis_data['total_race_distance']} km")
    
    if 'power_zone' in analysis_data:
        print(f"   Power Zone: {analysis_data['power_zone']}")
    
    if 'hr_zone' in analysis_data:
        print(f"   HR Zone: {analysis_data['hr_zone']}")
    
    # Leaderboard
    if 'leaderboard_stats' in analysis_data:
        lb = analysis_data['leaderboard_stats']
        print(f"\n🏁 RACE POSITION:")
        print(f"   Current: {lb['current_position']}/{lb['total_riders']}")
        print(f"   Riders Ahead: {lb['riders_ahead']}")
        print(f"   Riders Behind: {lb['riders_behind']}")
        if lb['gap_to_leader']:
            print(f"   Gap to Leader: {lb['gap_to_leader']}")
        if lb['avg_watts_per_kg']:
            print(f"   Field Average: {lb['avg_watts_per_kg']} w/kg")
    
    # Validation
    validation = analysis['validation']
    if validation['warnings']:
        print(f"\n⚠️  WARNINGS:")
        for warning in validation['warnings']:
            print(f"   - {warning}")
    else:
        print(f"\n✅ All data within normal ranges")
    
    print("\n" + "="*60)


def main():
    """Main entry point"""
    
    if len(sys.argv) < 2:
        print("Usage: python zwift_telemetry_analyzer.py <screenshot.jpg> [--report]")
        sys.exit(1)
    
    analyzer = TelemetryAnalyzer()
    
    # Process all provided screenshots
    for arg in sys.argv[1:]:
        if arg.startswith('--'):
            continue
            
        if not Path(arg).exists():
            print(f"Error: File not found: {arg}")
            continue
        
        print(f"\nAnalyzing: {arg}")
        analysis = analyzer.analyze_screenshot(arg)
        print_analysis(analysis)
        
        # Save detailed JSON
        output_file = Path(arg).stem + "_analysis.json"
        with open(output_file, 'w') as f:
            json.dump(analysis, f, indent=2)
        print(f"\nDetailed analysis saved to: {output_file}")
    
    # Generate summary report if requested
    if '--report' in sys.argv and len(analyzer.history) > 0:
        report = analyzer.generate_report()
        print("\n" + "="*60)
        print("SUMMARY REPORT")
        print("="*60)
        print(json.dumps(report, indent=2))
        
        with open("telemetry_report.json", 'w') as f:
            json.dump(report, f, indent=2)
        print("\nReport saved to: telemetry_report.json")


if __name__ == "__main__":
    main()