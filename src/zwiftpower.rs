use anyhow::Result;
use regex::Regex;

use crate::config::{FullConfig, Secrets};
use zwift_race_finder::cache::*;
use zwift_race_finder::category::*;
use zwift_race_finder::models::*;

pub async fn fetch_zwiftpower_stats(secrets: &Secrets) -> Result<Option<UserStats>> {
    let debug = std::env::var("ZRF_ZP_DEBUG").is_ok();
    let (profile_id, session_id) = match (
        &secrets.zwiftpower_profile_id,
        &secrets.zwiftpower_session_id,
    ) {
        (Some(pid), Some(sid)) => (pid, sid),
        (Some(pid), None) => {
            let url = format!("https://zwiftpower.com/profile.php?z={}", pid);
            if debug {
                eprintln!("ZP: no session ID configured, trying public profile access");
            }
            eprintln!("Note: No session ID configured, trying public profile access...");
            return fetch_zwiftpower_public(&url).await;
        }
        _ => {
            if debug {
                eprintln!("ZP: missing profile ID, skipping fetch");
            }
            return Ok(None);
        }
    };

    let url = format!(
        "https://zwiftpower.com/profile.php?z={}&sid={}",
        profile_id, session_id
    );

    let client = reqwest::Client::builder()
        .user_agent("Zwift Race Finder")
        .build()?;

    let response = client.get(&url).send().await;

    match response {
        Ok(resp) if resp.status().is_success() => {
            let html = resp.text().await?;

            let score_regex = Regex::new(r"(?s)Zwift Racing Score.*?(\d+)").unwrap();
            let category_regex =
                Regex::new(r"Category(?:[^A-Z]+)?([A-E]\\+?)").unwrap();

            if debug {
                eprintln!(
                    "ZP: body has score label? {}",
                    html.contains("Zwift Racing Score")
                );
                eprintln!("ZP: score regex match? {}", score_regex.is_match(&html));
                eprintln!(
                    "ZP: category regex match? {}",
                    category_regex.is_match(&html)
                );
            }

            if let Some(score_match) = score_regex.captures(&html) {
                let zwift_score: u32 = score_match[1].parse().unwrap_or(195);
                let category = category_regex
                    .captures(&html)
                    .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
                    .unwrap_or_else(|| get_category_from_score(zwift_score).to_string());

                return Ok(Some(UserStats {
                    zwift_score,
                    category,
                    username: "ZwiftPower".to_string(),
                }));
            }
        }
        _ => {
            return Ok(None);
        }
    }

    Ok(None)
}

async fn fetch_zwiftpower_public(url: &str) -> Result<Option<UserStats>> {
    let client = reqwest::Client::builder()
        .user_agent("Zwift Race Finder")
        .build()?;

    match client.get(url).send().await {
        Ok(resp) if resp.status().is_success() => {
            Ok(None)
        }
        _ => Ok(None),
    }
}

pub async fn get_user_stats(config: &FullConfig) -> Result<UserStats> {
    if let Ok(Some(stats)) = load_cached_stats() {
        return Ok(stats);
    }

    if let Ok(Some(stats)) = fetch_zwiftpower_stats(&config.secrets).await {
        let _ = save_cached_stats(&stats);
        return Ok(stats);
    }

    let zwift_score = config.default_zwift_score().unwrap_or(195);
    Ok(UserStats {
        zwift_score,
        category: config
            .default_category()
            .cloned()
            .unwrap_or_else(|| get_category_from_score(zwift_score).to_string()),
        username: "User".to_string(),
    })
}
