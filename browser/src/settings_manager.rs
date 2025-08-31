//! Settings manager for the Matte browser

use common::{error::Result, BrowserSettings};
use tracing::{debug, error, info, warn};
use serde_json;
use std::path::PathBuf;

/// Settings manager for handling browser settings
pub struct SettingsManager {
    /// Current browser settings
    settings: BrowserSettings,
    
    /// Settings file path
    settings_file: PathBuf,
}

impl SettingsManager {
    /// Create a new settings manager
    pub async fn new() -> Result<Self> {
        info!("Initializing settings manager");
        
        let settings_file = Self::get_settings_file_path().await?;
        let settings = Self::load_settings(&settings_file).await?;
        
        info!("Settings manager initialized successfully");
        Ok(Self {
            settings,
            settings_file,
        })
    }
    
    /// Get current settings
    pub async fn get_settings(&self) -> Result<BrowserSettings> {
        Ok(self.settings.clone())
    }
    
    /// Update settings
    pub async fn update_settings(&mut self, new_settings: BrowserSettings) -> Result<()> {
        info!("Updating browser settings");
        
        self.settings = new_settings;
        self.save_settings().await?;
        
        info!("Settings updated successfully");
        Ok(())
    }
    
    /// Update a specific setting
    pub async fn update_setting<F>(&mut self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut BrowserSettings),
    {
        let mut settings = self.settings.clone();
        updater(&mut settings);
        self.update_settings(settings).await
    }
    
    /// Reset settings to defaults
    pub async fn reset_settings(&mut self) -> Result<()> {
        info!("Resetting settings to defaults");
        
        self.settings = BrowserSettings::default();
        self.save_settings().await?;
        
        info!("Settings reset successfully");
        Ok(())
    }
    
    /// Load settings from file
    async fn load_settings(settings_file: &PathBuf) -> Result<BrowserSettings> {
        if settings_file.exists() {
            match tokio::fs::read_to_string(settings_file).await {
                Ok(contents) => {
                    match serde_json::from_str::<BrowserSettings>(&contents) {
                        Ok(settings) => {
                            info!("Loaded settings from file");
                            Ok(settings)
                        }
                        Err(e) => {
                            warn!("Failed to parse settings file: {}, using defaults", e);
                            Ok(BrowserSettings::default())
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to read settings file: {}, using defaults", e);
                    Ok(BrowserSettings::default())
                }
            }
        } else {
            info!("Settings file not found, using defaults");
            Ok(BrowserSettings::default())
        }
    }
    
    /// Save settings to file
    async fn save_settings(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.settings)
            .map_err(|e| common::error::Error::ParseError(format!("Failed to serialize settings: {}", e)))?;
        
        // Ensure settings directory exists
        if let Some(parent) = self.settings_file.parent() {
            tokio::fs::create_dir_all(parent).await
                .map_err(|e| common::error::Error::IoError(format!("Failed to create settings directory: {}", e)))?;
        }
        
        tokio::fs::write(&self.settings_file, json).await
            .map_err(|e| common::error::Error::IoError(format!("Failed to write settings file: {}", e)))?;
        
        debug!("Settings saved to file");
        Ok(())
    }
    
    /// Get settings file path
    async fn get_settings_file_path() -> Result<PathBuf> {
        let data_dir = common::platform::PlatformPaths::data_directory()?;
        Ok(data_dir.join("settings.json"))
    }
    
    /// Get settings file path
    pub async fn get_settings_file(&self) -> &PathBuf {
        &self.settings_file
    }
    
    /// Export settings to a file
    pub async fn export_settings(&self, export_path: &PathBuf) -> Result<()> {
        info!("Exporting settings to: {:?}", export_path);
        
        let json = serde_json::to_string_pretty(&self.settings)
            .map_err(|e| common::error::Error::ParseError(format!("Failed to serialize settings: {}", e)))?;
        
        // Ensure export directory exists
        if let Some(parent) = export_path.parent() {
            tokio::fs::create_dir_all(parent).await
                .map_err(|e| common::error::Error::IoError(format!("Failed to create export directory: {}", e)))?;
        }
        
        tokio::fs::write(export_path, json).await
            .map_err(|e| common::error::Error::IoError(format!("Failed to write export file: {}", e)))?;
        
        info!("Settings exported successfully");
        Ok(())
    }
    
    /// Import settings from a file
    pub async fn import_settings(&mut self, import_path: &PathBuf) -> Result<()> {
        info!("Importing settings from: {:?}", import_path);
        
        if !import_path.exists() {
            return Err(common::error::Error::NotFound(
                format!("Import file not found: {:?}", import_path)
            ));
        }
        
        let contents = tokio::fs::read_to_string(import_path).await
            .map_err(|e| common::error::Error::IoError(format!("Failed to read import file: {}", e)))?;
        
        let imported_settings = serde_json::from_str::<BrowserSettings>(&contents)
            .map_err(|e| common::error::Error::ParseError(format!("Failed to parse import file: {}", e)))?;
        
        self.update_settings(imported_settings).await?;
        
        info!("Settings imported successfully");
        Ok(())
    }
    
    /// Validate settings
    pub async fn validate_settings(&self) -> Result<()> {
        // Validate homepage URL
        if !common::utils::is_valid_url(&self.settings.homepage) {
            return Err(common::error::Error::ConfigError(
                format!("Invalid homepage URL: {}", self.settings.homepage)
            ));
        }
        
        // Validate search engine URL
        if !self.settings.search_engine.contains("{}") {
            return Err(common::error::Error::ConfigError(
                "Search engine URL must contain '{}' placeholder".to_string()
            ));
        }
        
        // Validate user agent
        if self.settings.user_agent.is_empty() {
            return Err(common::error::Error::ConfigError(
                "User agent cannot be empty".to_string()
            ));
        }
        
        // Validate language
        if self.settings.language.is_empty() {
            return Err(common::error::Error::ConfigError(
                "Language cannot be empty".to_string()
            ));
        }
        
        Ok(())
    }
    
    /// Shutdown the settings manager
    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down settings manager");
        
        // Save settings before shutdown
        self.save_settings().await?;
        
        info!("Settings manager shutdown complete");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_settings_manager_creation() {
        let manager = SettingsManager::new().await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_get_settings() {
        let manager = SettingsManager::new().await.unwrap();
        let settings = manager.get_settings().await.unwrap();
        
        assert_eq!(settings.homepage, "https://www.google.com");
        assert!(settings.enable_javascript);
        assert!(settings.enable_tracking_protection);
    }

    #[tokio::test]
    async fn test_update_settings() {
        let mut manager = SettingsManager::new().await.unwrap();
        
        let mut new_settings = BrowserSettings::default();
        new_settings.homepage = "https://example.com".to_string();
        
        assert!(manager.update_settings(new_settings).await.is_ok());
        
        let updated_settings = manager.get_settings().await.unwrap();
        assert_eq!(updated_settings.homepage, "https://example.com");
    }

    #[tokio::test]
    async fn test_update_specific_setting() {
        let mut manager = SettingsManager::new().await.unwrap();
        
        assert!(manager.update_setting(|settings| {
            settings.homepage = "https://bing.com".to_string();
        }).await.is_ok());
        
        let updated_settings = manager.get_settings().await.unwrap();
        assert_eq!(updated_settings.homepage, "https://bing.com");
    }

    #[tokio::test]
    async fn test_reset_settings() {
        let mut manager = SettingsManager::new().await.unwrap();
        
        // Change a setting
        manager.update_setting(|settings| {
            settings.homepage = "https://example.com".to_string();
        }).await.unwrap();
        
        // Reset to defaults
        assert!(manager.reset_settings().await.is_ok());
        
        let reset_settings = manager.get_settings().await.unwrap();
        assert_eq!(reset_settings.homepage, "https://www.google.com");
    }

    #[tokio::test]
    async fn test_validate_settings() {
        let manager = SettingsManager::new().await.unwrap();
        assert!(manager.validate_settings().await.is_ok());
    }

    #[tokio::test]
    async fn test_export_import_settings() {
        let manager = SettingsManager::new().await.unwrap();
        let temp_file = PathBuf::from("/tmp/test_settings.json");
        
        // Export settings
        assert!(manager.export_settings(&temp_file).await.is_ok());
        
        // Import settings
        let mut new_manager = SettingsManager::new().await.unwrap();
        assert!(new_manager.import_settings(&temp_file).await.is_ok());
        
        // Clean up
        let _ = tokio::fs::remove_file(&temp_file).await;
    }
}
