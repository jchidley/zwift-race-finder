//! Golden baseline tests for behavioral preservation
//!
//! These tests capture and verify the exact behavior of the system
//! before any refactoring or migration.

#[cfg(test)]
mod generate_baseline;

#[cfg(test)]
mod generate_baseline_improved;

#[cfg(test)]
mod validate_test_data;