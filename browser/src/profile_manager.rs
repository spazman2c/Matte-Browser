//! Profile manager for the Matte browser

use common::{error::Result, ProfileInfo};
use tracing::{debug, error, info, warn};
use std::collections::HashMap;
use std::path::PathBuf;

/// Profile manager for handling browser profiles
pub struct ProfileManager {
    /// Map of profile name to profile info
    profiles: HashMap<String, ProfileInfo>,
    
    /// Current active profile
    active_profile: Option<String>,
    
    /// Default profile name
    default_profile_name: String,
}

impl ProfileManager {
    /// Create a new profile manager
    pub async fn new() -> Result<Self> {
        info!("Initializing profile manager");
        
        let default_profile_name = "Default".to_string();
        let mut profiles = HashMap::new();
        
        // Create default profile
        let default_profile = ProfileInfo::new(
            default_profile_name.clone(),
            Self::get_profile_directory(&default_profile_name).await?,
        );
        profiles.insert(default_profile_name.clone(), default_profile);
        
        Ok(Self {
            profiles,
            active_profile: Some(default_profile_name.clone()),
            default_profile_name,
        })
    }
    
    /// Create a new profile
    pub async fn create_profile(&mut self, name: String) -> Result<()> {
        info!("Creating profile: {}", name);
        
        if self.profiles.contains_key(&name) {
            return Err(common::error::Error::InvalidState(
                format!("Profile '{}' already exists", name)
            ));
        }
        
        let profile_dir = Self::get_profile_directory(&name).await?;
        let profile = ProfileInfo::new(name.clone(), profile_dir);
        
        self.profiles.insert(name.clone(), profile);
        
        info!("Created profile '{}' successfully", name);
        Ok(())
    }
    
    /// Delete a profile
    pub async fn delete_profile(&mut self, name: &str) -> Result<()> {
        info!("Deleting profile: {}", name);
        
        if name == self.default_profile_name {
            return Err(common::error::Error::InvalidState(
                "Cannot delete the default profile".to_string()
            ));
        }
        
        if let Some(profile) = self.profiles.remove(name) {
            // Clean up profile directory
            if let Err(e) = tokio::fs::remove_dir_all(&profile.data_directory).await {
                warn!("Failed to remove profile directory: {}", e);
            }
            
            // If this was the active profile, switch to default
            if self.active_profile.as_ref() == Some(&name.to_string()) {
                self.active_profile = Some(self.default_profile_name.clone());
            }
            
            info!("Deleted profile '{}' successfully", name);
            Ok(())
        } else {
            Err(common::error::Error::NotFound(
                format!("Profile '{}' not found", name)
            ))
        }
    }
    
    /// Get profile info
    pub async fn get_profile(&self, name: &str) -> Result<&ProfileInfo> {
        self.profiles.get(name)
            .ok_or_else(|| common::error::Error::NotFound(
                format!("Profile '{}' not found", name)
            ))
    }
    
    /// Get profile info mutably
    pub async fn get_profile_mut(&mut self, name: &str) -> Result<&mut ProfileInfo> {
        self.profiles.get_mut(name)
            .ok_or_else(|| common::error::Error::NotFound(
                format!("Profile '{}' not found", name)
            ))
    }
    
    /// Get active profile
    pub async fn get_active_profile(&self) -> Result<&ProfileInfo> {
        if let Some(ref active_name) = self.active_profile {
            self.get_profile(active_name).await
        } else {
            Err(common::error::Error::InvalidState(
                "No active profile".to_string()
            ))
        }
    }
    
    /// Set active profile
    pub async fn set_active_profile(&mut self, name: &str) -> Result<()> {
        info!("Setting active profile to: {}", name);
        
        if !self.profiles.contains_key(name) {
            return Err(common::error::Error::NotFound(
                format!("Profile '{}' not found", name)
            ));
        }
        
        self.active_profile = Some(name.to_string());
        info!("Set active profile to '{}' successfully", name);
        Ok(())
    }
    
    /// Get all profiles
    pub async fn get_all_profiles(&self) -> Vec<&ProfileInfo> {
        self.profiles.values().collect()
    }
    
    /// Get profile count
    pub async fn profile_count(&self) -> usize {
        self.profiles.len()
    }
    
    /// Check if profile exists
    pub async fn has_profile(&self, name: &str) -> bool {
        self.profiles.contains_key(name)
    }
    
    /// Get profile directory path
    async fn get_profile_directory(profile_name: &str) -> Result<PathBuf> {
        let base_dir = common::platform::PlatformPaths::data_directory()?;
        let profile_dir = base_dir.join("Profiles").join(profile_name);
        
        // Create profile directory if it doesn't exist
        if !profile_dir.exists() {
            tokio::fs::create_dir_all(&profile_dir).await
                .map_err(|e| common::error::Error::IoError(format!("Failed to create profile directory: {}", e)))?;
        }
        
        Ok(profile_dir)
    }
    
    /// Create incognito profile
    pub async fn create_incognito_profile(&mut self) -> Result<String> {
        let incognito_name = "Incognito".to_string();
        
        if self.profiles.contains_key(&incognito_name) {
            // Remove existing incognito profile
            self.delete_profile(&incognito_name).await?;
        }
        
        let incognito_profile = ProfileInfo::incognito();
        self.profiles.insert(incognito_name.clone(), incognito_profile);
        
        info!("Created incognito profile successfully");
        Ok(incognito_name)
    }
    
    /// Shutdown the profile manager
    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down profile manager");
        
        let profile_count = self.profiles.len();
        self.profiles.clear();
        self.active_profile = None;
        
        info!("Profile manager shutdown complete (managed {} profiles)", profile_count);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_profile_manager_creation() {
        let manager = ProfileManager::new().await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_create_profile() {
        let mut manager = ProfileManager::new().await.unwrap();
        
        assert!(manager.create_profile("TestProfile".to_string()).await.is_ok());
        assert_eq!(manager.profile_count().await, 2); // Default + TestProfile
        assert!(manager.has_profile("TestProfile").await);
    }

    #[tokio::test]
    async fn test_delete_profile() {
        let mut manager = ProfileManager::new().await.unwrap();
        
        manager.create_profile("TestProfile".to_string()).await.unwrap();
        assert_eq!(manager.profile_count().await, 2);
        
        assert!(manager.delete_profile("TestProfile").await.is_ok());
        assert_eq!(manager.profile_count().await, 1); // Only Default remains
        assert!(!manager.has_profile("TestProfile").await);
    }

    #[tokio::test]
    async fn test_cannot_delete_default_profile() {
        let mut manager = ProfileManager::new().await.unwrap();
        
        let result = manager.delete_profile("Default").await;
        assert!(result.is_err());
        assert_eq!(manager.profile_count().await, 1); // Default still exists
    }

    #[tokio::test]
    async fn test_set_active_profile() {
        let mut manager = ProfileManager::new().await.unwrap();
        
        manager.create_profile("TestProfile".to_string()).await.unwrap();
        assert!(manager.set_active_profile("TestProfile").await.is_ok());
        
        let active_profile = manager.get_active_profile().await.unwrap();
        assert_eq!(active_profile.name, "TestProfile");
    }

    #[tokio::test]
    async fn test_create_incognito_profile() {
        let mut manager = ProfileManager::new().await.unwrap();
        
        let incognito_name = manager.create_incognito_profile().await.unwrap();
        assert_eq!(incognito_name, "Incognito");
        
        let incognito_profile = manager.get_profile("Incognito").await.unwrap();
        assert!(incognito_profile.is_incognito);
    }
}
