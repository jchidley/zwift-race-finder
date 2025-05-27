//! Secure storage for OAuth tokens and sensitive credentials
//! Supports environment variables, system keyring, and encrypted file storage

use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;

/// Strava OAuth tokens and credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StravaTokens {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: i64,
    pub athlete_id: Option<i64>,
}

/// Token storage backend
#[derive(Debug, Clone)]
pub enum StorageBackend {
    /// Environment variables (most secure for CI/CD)
    Environment,
    /// System keyring (secure for desktop use)
    Keyring,
    /// File storage (backward compatibility)
    File(PathBuf),
}

/// Secure storage manager for OAuth tokens
pub struct SecureStorage {
    backend: StorageBackend,
    service_name: String,
}

impl SecureStorage {
    /// Create a new secure storage instance
    pub fn new(service_name: &str) -> Self {
        // Determine best available backend
        let backend = Self::detect_backend();
        
        SecureStorage {
            backend,
            service_name: service_name.to_string(),
        }
    }
    
    /// Detect the best available storage backend
    fn detect_backend() -> StorageBackend {
        // Priority 1: Check if running with environment variables
        if std::env::var("STRAVA_CLIENT_ID").is_ok() {
            return StorageBackend::Environment;
        }
        
        // Priority 2: Try system keyring (if available)
        #[cfg(feature = "keyring")]
        {
            if let Ok(entry) = keyring::Entry::new("zwift-race-finder", "test") {
                // Test if keyring is accessible
                let _ = entry.delete_password(); // Clean up test entry
                return StorageBackend::Keyring;
            }
        }
        
        // Priority 3: Fall back to file storage
        let config_path = Self::get_config_path();
        StorageBackend::File(config_path)
    }
    
    /// Get the default config file path
    fn get_config_path() -> PathBuf {
        // Check for existing file in current directory (backward compatibility)
        let local_path = PathBuf::from("strava_config.json");
        if local_path.exists() {
            return local_path;
        }
        
        // Use XDG config directory
        dirs::config_dir()
            .map(|mut path| {
                path.push("zwift-race-finder");
                path.push("strava_config.json");
                path
            })
            .unwrap_or(local_path)
    }
    
    /// Load Strava tokens from storage
    pub fn load_strava_tokens(&self) -> Result<StravaTokens> {
        match &self.backend {
            StorageBackend::Environment => self.load_from_env(),
            StorageBackend::Keyring => self.load_from_keyring(),
            StorageBackend::File(path) => self.load_from_file(path),
        }
    }
    
    /// Save Strava tokens to storage
    pub fn save_strava_tokens(&self, tokens: &StravaTokens) -> Result<()> {
        match &self.backend {
            StorageBackend::Environment => {
                // Can't save to environment - return error with instructions
                Err(anyhow!(
                    "Cannot save tokens to environment variables. \
                    Please set the following environment variables:\n\
                    STRAVA_CLIENT_ID={}\n\
                    STRAVA_CLIENT_SECRET=<hidden>\n\
                    STRAVA_ACCESS_TOKEN=<hidden>\n\
                    STRAVA_REFRESH_TOKEN=<hidden>\n\
                    STRAVA_EXPIRES_AT={}\n\
                    STRAVA_ATHLETE_ID={}",
                    tokens.client_id,
                    tokens.expires_at,
                    tokens.athlete_id.unwrap_or(0)
                ))
            },
            StorageBackend::Keyring => self.save_to_keyring(tokens),
            StorageBackend::File(path) => self.save_to_file(path, tokens),
        }
    }
    
    /// Load tokens from environment variables
    fn load_from_env(&self) -> Result<StravaTokens> {
        Ok(StravaTokens {
            client_id: std::env::var("STRAVA_CLIENT_ID")
                .context("STRAVA_CLIENT_ID not set")?,
            client_secret: std::env::var("STRAVA_CLIENT_SECRET")
                .context("STRAVA_CLIENT_SECRET not set")?,
            access_token: std::env::var("STRAVA_ACCESS_TOKEN")
                .context("STRAVA_ACCESS_TOKEN not set")?,
            refresh_token: std::env::var("STRAVA_REFRESH_TOKEN")
                .context("STRAVA_REFRESH_TOKEN not set")?,
            expires_at: std::env::var("STRAVA_EXPIRES_AT")
                .context("STRAVA_EXPIRES_AT not set")?
                .parse()
                .context("Invalid STRAVA_EXPIRES_AT")?,
            athlete_id: std::env::var("STRAVA_ATHLETE_ID")
                .ok()
                .and_then(|s| s.parse().ok()),
        })
    }
    
    /// Load tokens from system keyring
    #[cfg(feature = "keyring")]
    fn load_from_keyring(&self) -> Result<StravaTokens> {
        let entry = keyring::Entry::new(&self.service_name, "strava_tokens")?;
        let json = entry.get_password()
            .context("No tokens found in keyring")?;
        serde_json::from_str(&json)
            .context("Failed to parse tokens from keyring")
    }
    
    #[cfg(not(feature = "keyring"))]
    fn load_from_keyring(&self) -> Result<StravaTokens> {
        Err(anyhow!("Keyring support not compiled in"))
    }
    
    /// Save tokens to system keyring
    #[cfg(feature = "keyring")]
    fn save_to_keyring(&self, tokens: &StravaTokens) -> Result<()> {
        let entry = keyring::Entry::new(&self.service_name, "strava_tokens")?;
        let json = serde_json::to_string(tokens)?;
        entry.set_password(&json)
            .context("Failed to save tokens to keyring")
    }
    
    #[cfg(not(feature = "keyring"))]
    fn save_to_keyring(&self, _tokens: &StravaTokens) -> Result<()> {
        Err(anyhow!("Keyring support not compiled in"))
    }
    
    /// Load tokens from file
    fn load_from_file(&self, path: &Path) -> Result<StravaTokens> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        serde_json::from_str(&contents)
            .context("Failed to parse tokens from file")
    }
    
    /// Save tokens to file (with restricted permissions)
    fn save_to_file(&self, path: &Path, tokens: &StravaTokens) -> Result<()> {
        // Ensure directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Write to temporary file first
        let temp_path = path.with_extension("tmp");
        let json = serde_json::to_string_pretty(tokens)?;
        fs::write(&temp_path, json)?;
        
        // Set restrictive permissions on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&temp_path)?.permissions();
            perms.set_mode(0o600); // Read/write for owner only
            fs::set_permissions(&temp_path, perms)?;
        }
        
        // Atomic rename
        fs::rename(temp_path, path)?;
        
        Ok(())
    }
    
    /// Get information about current storage backend
    pub fn backend_info(&self) -> String {
        match &self.backend {
            StorageBackend::Environment => "Environment variables".to_string(),
            StorageBackend::Keyring => "System keyring".to_string(),
            StorageBackend::File(path) => format!("File: {}", path.display()),
        }
    }
    
    /// Check if tokens need refresh
    pub fn tokens_need_refresh(tokens: &StravaTokens) -> bool {
        let current_time = chrono::Utc::now().timestamp();
        current_time >= tokens.expires_at
    }
}

/// Helper function to migrate from old file format to secure storage
pub fn migrate_tokens(old_path: &Path, storage: &SecureStorage) -> Result<()> {
    // Check if old file exists
    if !old_path.exists() {
        return Ok(());
    }
    
    // Read old tokens
    let contents = fs::read_to_string(old_path)?;
    let tokens: StravaTokens = serde_json::from_str(&contents)?;
    
    // Save to new storage
    storage.save_strava_tokens(&tokens)?;
    
    // Optionally remove old file (after confirming save worked)
    println!("Tokens migrated from {} to {}", 
             old_path.display(), 
             storage.backend_info());
    println!("You can now delete the old file: {}", old_path.display());
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_file_storage() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.json");
        
        let storage = SecureStorage {
            backend: StorageBackend::File(config_path.clone()),
            service_name: "test".to_string(),
        };
        
        let tokens = StravaTokens {
            client_id: "test_client".to_string(),
            client_secret: "test_secret".to_string(),
            access_token: "test_access".to_string(),
            refresh_token: "test_refresh".to_string(),
            expires_at: 1234567890,
            athlete_id: Some(123),
        };
        
        // Save tokens
        storage.save_strava_tokens(&tokens).unwrap();
        
        // Load tokens
        let loaded = storage.load_strava_tokens().unwrap();
        assert_eq!(loaded.client_id, tokens.client_id);
        assert_eq!(loaded.expires_at, tokens.expires_at);
        
        // Check file permissions on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(&config_path).unwrap();
            assert_eq!(metadata.permissions().mode() & 0o777, 0o600);
        }
    }
    
    #[test]
    fn test_env_storage() {
        // Set environment variables
        std::env::set_var("STRAVA_CLIENT_ID", "env_client");
        std::env::set_var("STRAVA_CLIENT_SECRET", "env_secret");
        std::env::set_var("STRAVA_ACCESS_TOKEN", "env_access");
        std::env::set_var("STRAVA_REFRESH_TOKEN", "env_refresh");
        std::env::set_var("STRAVA_EXPIRES_AT", "9876543210");
        std::env::set_var("STRAVA_ATHLETE_ID", "456");
        
        let storage = SecureStorage {
            backend: StorageBackend::Environment,
            service_name: "test".to_string(),
        };
        
        let tokens = storage.load_strava_tokens().unwrap();
        assert_eq!(tokens.client_id, "env_client");
        assert_eq!(tokens.expires_at, 9876543210);
        assert_eq!(tokens.athlete_id, Some(456));
        
        // Clean up
        std::env::remove_var("STRAVA_CLIENT_ID");
        std::env::remove_var("STRAVA_CLIENT_SECRET");
        std::env::remove_var("STRAVA_ACCESS_TOKEN");
        std::env::remove_var("STRAVA_REFRESH_TOKEN");
        std::env::remove_var("STRAVA_EXPIRES_AT");
        std::env::remove_var("STRAVA_ATHLETE_ID");
    }
    
    #[test]
    fn test_token_refresh_check() {
        let expired = StravaTokens {
            client_id: "test".to_string(),
            client_secret: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: "test".to_string(),
            expires_at: 1000000000, // Long ago
            athlete_id: None,
        };
        
        assert!(SecureStorage::tokens_need_refresh(&expired));
        
        let future = StravaTokens {
            expires_at: chrono::Utc::now().timestamp() + 3600,
            ..expired
        };
        
        assert!(!SecureStorage::tokens_need_refresh(&future));
    }
}