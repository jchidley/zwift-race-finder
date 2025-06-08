//! A/B testing framework for comparing implementations
//!
//! This module provides infrastructure for running both old and new
//! implementations side-by-side and comparing their results.

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Result of an A/B test comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ABTestResult<T> {
    pub test_name: String,
    pub inputs: serde_json::Value,
    pub old_result: T,
    pub new_result: T,
    pub matches: bool,
    pub context: String,
}

/// Failure when A/B test results don't match
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ABTestFailure {
    pub test_name: String,
    pub inputs: String,
    pub old_result: String,
    pub new_result: String,
    pub context: String,
}

/// A/B test runner for comparing two implementations
pub struct ABTest<T> {
    pub name: String,
    pub old_impl: Box<dyn Fn() -> T>,
    pub new_impl: Box<dyn Fn() -> T>,
    pub context: String,
}

impl<T: PartialEq + Debug + Clone> ABTest<T> {
    /// Run both implementations and compare results
    pub fn run(&self) -> Result<ABTestResult<T>, ABTestFailure> {
        let old_result = (self.old_impl)();
        let new_result = (self.new_impl)();
        
        let matches = old_result == new_result;
        
        if matches {
            Ok(ABTestResult {
                test_name: self.name.clone(),
                inputs: serde_json::Value::Null, // To be filled by caller
                old_result: old_result.clone(),
                new_result,
                matches: true,
                context: self.context.clone(),
            })
        } else {
            Err(ABTestFailure {
                test_name: self.name.clone(),
                inputs: String::new(), // To be filled by caller
                old_result: format!("{:?}", old_result),
                new_result: format!("{:?}", new_result),
                context: self.context.clone(),
            })
        }
    }
}

/// Batch A/B test runner for multiple test cases
pub struct ABTestBatch<T> {
    pub name: String,
    pub results: Vec<Result<ABTestResult<T>, ABTestFailure>>,
}

impl<T: Debug> ABTestBatch<T> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            results: Vec::new(),
        }
    }
    
    pub fn add_result(&mut self, result: Result<ABTestResult<T>, ABTestFailure>) {
        self.results.push(result);
    }
    
    pub fn success_rate(&self) -> f64 {
        let successes = self.results.iter().filter(|r| r.is_ok()).count();
        successes as f64 / self.results.len() as f64
    }
    
    pub fn failures(&self) -> Vec<&ABTestFailure> {
        self.results
            .iter()
            .filter_map(|r| r.as_ref().err())
            .collect()
    }
    
    pub fn summary(&self) -> String {
        format!(
            "A/B Test Batch: {}\n\
             Total tests: {}\n\
             Successes: {}\n\
             Failures: {}\n\
             Success rate: {:.1}%",
            self.name,
            self.results.len(),
            self.results.iter().filter(|r| r.is_ok()).count(),
            self.results.iter().filter(|r| r.is_err()).count(),
            self.success_rate() * 100.0
        )
    }
}

/// Macro for easily creating A/B tests
#[macro_export]
macro_rules! ab_test {
    ($name:expr, $old:expr, $new:expr) => {
        ABTest {
            name: $name.to_string(),
            old_impl: Box::new($old),
            new_impl: Box::new($new),
            context: format!("Testing {} at {}:{}", $name, file!(), line!()),
        }
    };
    
    ($name:expr, $old:expr, $new:expr, $context:expr) => {
        ABTest {
            name: $name.to_string(),
            old_impl: Box::new($old),
            new_impl: Box::new($new),
            context: $context.to_string(),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ab_test_matching_results() {
        let test = ABTest {
            name: "simple_addition".to_string(),
            old_impl: Box::new(|| 2 + 2),
            new_impl: Box::new(|| 4),
            context: "Testing that 2+2=4".to_string(),
        };
        
        let result = test.run();
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.matches);
        assert_eq!(result.old_result, 4);
        assert_eq!(result.new_result, 4);
    }
    
    #[test]
    fn test_ab_test_different_results() {
        let test = ABTest {
            name: "different_calculation".to_string(),
            old_impl: Box::new(|| 2 + 2),
            new_impl: Box::new(|| 2 * 3),
            context: "Testing different calculations".to_string(),
        };
        
        let result = test.run();
        assert!(result.is_err());
        let failure = result.unwrap_err();
        assert_eq!(failure.old_result, "4");
        assert_eq!(failure.new_result, "6");
    }
    
    #[test]
    fn test_ab_test_batch() {
        let mut batch = ABTestBatch::new("batch_test");
        
        // Add some successful tests
        batch.add_result(Ok(ABTestResult {
            test_name: "test1".to_string(),
            inputs: serde_json::Value::Null,
            old_result: 10,
            new_result: 10,
            matches: true,
            context: "test".to_string(),
        }));
        
        // Add a failure
        batch.add_result(Err(ABTestFailure {
            test_name: "test2".to_string(),
            inputs: "input".to_string(),
            old_result: "20".to_string(),
            new_result: "21".to_string(),
            context: "test".to_string(),
        }));
        
        assert_eq!(batch.success_rate(), 0.5);
        assert_eq!(batch.failures().len(), 1);
    }
}