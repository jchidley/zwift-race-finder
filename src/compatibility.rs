//! Compatibility tracking for behavioral preservation
//!
//! This module provides tools for tracking and reporting on
//! behavioral compatibility during migrations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Overall compatibility report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityReport {
    pub timestamp: DateTime<Utc>,
    pub total_tests: usize,
    pub passing: usize,
    pub failures: Vec<BehavioralDivergence>,
    pub performance: PerformanceComparison,
    pub metadata: HashMap<String, String>,
}

/// A single behavioral divergence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralDivergence {
    pub function: String,
    pub inputs: String,
    pub expected: String,
    pub actual: String,
    pub severity: DivergenceSeverity,
    pub context: String,
}

/// Severity levels for divergences
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DivergenceSeverity {
    /// User-visible difference in behavior
    Critical,
    /// Internal difference with same user outcome
    Minor,
    /// Performance difference only
    Performance,
}

/// Performance comparison between implementations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceComparison {
    pub average_difference: f64,
    pub median_difference: f64,
    pub p95_difference: f64,
    pub p99_difference: f64,
    pub measurements: Vec<PerformanceMeasurement>,
}

/// Single performance measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMeasurement {
    pub function: String,
    pub old_duration_us: u64,
    pub new_duration_us: u64,
    pub difference_percent: f64,
}

impl CompatibilityReport {
    /// Create a new compatibility report
    pub fn new() -> Self {
        Self {
            timestamp: Utc::now(),
            total_tests: 0,
            passing: 0,
            failures: Vec::new(),
            performance: PerformanceComparison {
                average_difference: 0.0,
                median_difference: 0.0,
                p95_difference: 0.0,
                p99_difference: 0.0,
                measurements: Vec::new(),
            },
            metadata: HashMap::new(),
        }
    }
    
    /// Get compatibility percentage
    pub fn compatibility_percent(&self) -> f64 {
        if self.total_tests == 0 {
            100.0
        } else {
            (self.passing as f64 / self.total_tests as f64) * 100.0
        }
    }
    
    /// Count critical issues
    pub fn critical_issues(&self) -> usize {
        self.failures
            .iter()
            .filter(|f| matches!(f.severity, DivergenceSeverity::Critical))
            .count()
    }
    
    /// Generate a markdown dashboard
    pub fn generate_dashboard(&self) -> String {
        let perf_indicator = if self.performance.average_difference > 0.0 {
            "slower"
        } else {
            "faster"
        };
        
        let critical_section = if self.critical_issues() > 0 {
            format!(
                "\n## ⚠️  Critical Issues\n\n{} critical behavioral divergences found:\n\n{}",
                self.critical_issues(),
                self.format_critical_divergences()
            )
        } else {
            String::new()
        };
        
        format!(
            r#"# Behavioral Compatibility Dashboard

Generated: {}

## Summary
- **Compatibility**: {:.1}% ({}/{} tests passing)
- **Critical Issues**: {}
- **Performance Impact**: {:.1}% {} (median)

## Performance Analysis
- Average: {:.1}% {}
- Median: {:.1}% {}
- 95th percentile: {:.1}% {}
- 99th percentile: {:.1}% {}
{}
## Divergence Summary

{}

## Metadata
{}

---
*This report tracks behavioral compatibility between implementations.*"#,
            self.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            self.compatibility_percent(),
            self.passing,
            self.total_tests,
            self.critical_issues(),
            self.performance.median_difference.abs() * 100.0,
            perf_indicator,
            self.performance.average_difference.abs() * 100.0,
            if self.performance.average_difference > 0.0 { "slower" } else { "faster" },
            self.performance.median_difference.abs() * 100.0,
            if self.performance.median_difference > 0.0 { "slower" } else { "faster" },
            self.performance.p95_difference.abs() * 100.0,
            if self.performance.p95_difference > 0.0 { "slower" } else { "faster" },
            self.performance.p99_difference.abs() * 100.0,
            if self.performance.p99_difference > 0.0 { "slower" } else { "faster" },
            critical_section,
            self.format_divergences(),
            self.format_metadata()
        )
    }
    
    fn format_divergences(&self) -> String {
        if self.failures.is_empty() {
            "✅ No behavioral divergences detected!".to_string()
        } else {
            let mut by_severity = HashMap::new();
            for failure in &self.failures {
                by_severity
                    .entry(failure.severity)
                    .or_insert_with(Vec::new)
                    .push(failure);
            }
            
            let mut sections = Vec::new();
            
            if let Some(critical) = by_severity.get(&DivergenceSeverity::Critical) {
                sections.push(format!("### Critical ({} issues)\n{}", 
                    critical.len(),
                    self.format_failure_list(critical)
                ));
            }
            
            if let Some(minor) = by_severity.get(&DivergenceSeverity::Minor) {
                sections.push(format!("### Minor ({} issues)\n{}",
                    minor.len(),
                    self.format_failure_list(minor)
                ));
            }
            
            if let Some(perf) = by_severity.get(&DivergenceSeverity::Performance) {
                sections.push(format!("### Performance ({} issues)\n{}",
                    perf.len(),
                    self.format_failure_list(perf)
                ));
            }
            
            sections.join("\n\n")
        }
    }
    
    fn format_critical_divergences(&self) -> String {
        self.failures
            .iter()
            .filter(|f| matches!(f.severity, DivergenceSeverity::Critical))
            .map(|f| format!(
                "- **{}**: Expected `{}`, got `{}`\n  Inputs: {}\n  Context: {}",
                f.function, f.expected, f.actual, f.inputs, f.context
            ))
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    fn format_failure_list(&self, failures: &[&BehavioralDivergence]) -> String {
        failures
            .iter()
            .map(|f| format!(
                "- `{}`: {} → {} (inputs: {})",
                f.function, f.expected, f.actual, f.inputs
            ))
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    fn format_metadata(&self) -> String {
        self.metadata
            .iter()
            .map(|(k, v)| format!("- {}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Default for CompatibilityReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating compatibility reports
pub struct CompatibilityReportBuilder {
    report: CompatibilityReport,
}

impl CompatibilityReportBuilder {
    pub fn new() -> Self {
        Self {
            report: CompatibilityReport::new(),
        }
    }
    
    pub fn add_test_result(&mut self, passed: bool) -> &mut Self {
        self.report.total_tests += 1;
        if passed {
            self.report.passing += 1;
        }
        self
    }
    
    pub fn add_divergence(&mut self, divergence: BehavioralDivergence) -> &mut Self {
        self.report.failures.push(divergence);
        self
    }
    
    pub fn add_performance_measurement(&mut self, measurement: PerformanceMeasurement) -> &mut Self {
        self.report.performance.measurements.push(measurement);
        self
    }
    
    pub fn add_metadata(&mut self, key: String, value: String) -> &mut Self {
        self.report.metadata.insert(key, value);
        self
    }
    
    pub fn build(mut self) -> CompatibilityReport {
        // Calculate performance statistics
        if !self.report.performance.measurements.is_empty() {
            let mut diffs: Vec<f64> = self.report.performance.measurements
                .iter()
                .map(|m| m.difference_percent)
                .collect();
            
            diffs.sort_by(|a, b| a.partial_cmp(b).unwrap());
            
            let sum: f64 = diffs.iter().sum();
            self.report.performance.average_difference = sum / diffs.len() as f64;
            
            let mid = diffs.len() / 2;
            self.report.performance.median_difference = if diffs.len() % 2 == 0 {
                (diffs[mid - 1] + diffs[mid]) / 2.0
            } else {
                diffs[mid]
            };
            
            let p95_idx = (diffs.len() as f64 * 0.95) as usize;
            let p99_idx = (diffs.len() as f64 * 0.99) as usize;
            
            self.report.performance.p95_difference = diffs.get(p95_idx).copied().unwrap_or(0.0);
            self.report.performance.p99_difference = diffs.get(p99_idx).copied().unwrap_or(0.0);
        }
        
        self.report
    }
}

impl Default for CompatibilityReportBuilder {
    fn default() -> Self {
        Self::new()
    }
}