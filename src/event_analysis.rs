//! Event analysis utilities
//!
//! This module contains functions for analyzing Zwift events.

use std::collections::HashMap;
use crate::models::{ZwiftEvent, EventSubGroup};
use crate::category::{get_category_from_score, category_matches_subgroup};

/// Find the subgroup that matches the user's category
pub fn find_user_subgroup<'a>(event: &'a ZwiftEvent, zwift_score: u32) -> Option<&'a EventSubGroup> {
    if event.event_sub_groups.is_empty() {
        return None;
    }
    
    let user_category = get_category_from_score(zwift_score);
    
    // Use the new category matching function from the category module
    event.event_sub_groups.iter().find(|sg| {
        category_matches_subgroup(user_category, &sg.name)
    })
}

/// Count events by type for display
pub fn count_events_by_type(events: &[ZwiftEvent]) -> Vec<(String, usize)> {
    let mut event_counts = HashMap::new();
    for event in events {
        if event.sport.to_uppercase() == "CYCLING" {
            *event_counts.entry(event.event_type.clone()).or_insert(0) += 1;
        }
    }
    
    let mut counts: Vec<_> = event_counts.into_iter().collect();
    counts.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
    counts
}