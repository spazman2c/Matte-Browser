//! Crash reporting and diagnostics for the Matte browser.
//! 
//! This module provides crash reporting functionality including
//! minidump generation, symbol server integration, and crash upload.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Crash report information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrashReport {
    pub crash_id: String,
    pub timestamp: std::time::SystemTime,
    pub process_type: String,
    pub process_id: u32,
    pub thread_id: u32,
    pub exception_type: String,
    pub exception_address: u64,
    pub stack_trace: Vec<StackFrame>,
    pub registers: HashMap<String, u64>,
    pub system_info: SystemInfo,
    pub browser_info: BrowserInfo,
    pub minidump_path: Option<PathBuf>,
    pub is_privacy_scrubbed: bool,
}

/// Stack frame information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrame {
    pub address: u64,
    pub module_name: Option<String>,
    pub function_name: Option<String>,
    pub source_file: Option<String>,
    pub line_number: Option<u32>,
    pub offset: u64,
}

/// System information for crash reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub architecture: String,
    pub cpu_count: u32,
    pub memory_total: u64,
    pub memory_available: u64,
}

/// Browser information for crash reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserInfo {
    pub version: String,
    pub build_id: String,
    pub channel: String,
    pub uptime: std::time::Duration,
    pub tab_count: u32,
    pub window_count: u32,
}

/// Crash reporter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrashReporterConfig {
    pub enabled: bool,
    pub upload_url: Option<String>,
    pub symbol_server_url: Option<String>,
    pub privacy_scrub_enabled: bool,
    pub max_crash_size: usize,
    pub crash_directory: PathBuf,
}

impl Default for CrashReporterConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            upload_url: None,
            symbol_server_url: None,
            privacy_scrub_enabled: true,
            max_crash_size: 10 * 1024 * 1024, // 10MB
            crash_directory: PathBuf::from("crashes"),
        }
    }
}

/// Crash reporter for handling browser crashes
pub struct CrashReporter {
    config: CrashReporterConfig,
    reports: Arc<RwLock<Vec<CrashReport>>>,
    symbol_cache: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl CrashReporter {
    /// Create a new crash reporter
    pub fn new(config: CrashReporterConfig) -> Result<Self> {
        // Create crash directory if it doesn't exist
        if !config.crash_directory.exists() {
            std::fs::create_dir_all(&config.crash_directory)
                .map_err(|e| Error::IoError(format!("Failed to create crash directory: {}", e)))?;
        }

        Ok(Self {
            config,
            reports: Arc::new(RwLock::new(Vec::new())),
            symbol_cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Generate a crash report
    pub async fn generate_crash_report(
        &self,
        process_type: String,
        process_id: u32,
        thread_id: u32,
        exception_type: String,
        exception_address: u64,
    ) -> Result<CrashReport> {
        let crash_id = crate::utils::generate_uuid();
        let timestamp = std::time::SystemTime::now();
        
        // Collect stack trace
        let stack_trace = self.collect_stack_trace().await?;
        
        // Collect register information
        let registers = self.collect_registers().await?;
        
        // Collect system information
        let system_info = self.collect_system_info().await?;
        
        // Collect browser information
        let browser_info = self.collect_browser_info().await?;
        
        // Generate minidump
        let minidump_path = if self.config.enabled {
            Some(self.generate_minidump(&crash_id).await?)
        } else {
            None
        };

        let mut report = CrashReport {
            crash_id,
            timestamp,
            process_type,
            process_id,
            thread_id,
            exception_type,
            exception_address,
            stack_trace,
            registers,
            system_info,
            browser_info,
            minidump_path,
            is_privacy_scrubbed: false,
        };

        // Privacy scrub the report if enabled
        if self.config.privacy_scrub_enabled {
            self.privacy_scrub_report(&mut report).await?;
        }

        // Store the report
        {
            let mut reports = self.reports.write().await;
            reports.push(report.clone());
        }

        info!("Generated crash report: {}", report.crash_id);
        Ok(report)
    }

    /// Collect stack trace information
    async fn collect_stack_trace(&self) -> Result<Vec<StackFrame>> {
        // This is a simplified implementation
        // In a real implementation, this would use platform-specific APIs
        // like StackWalk64 on Windows, backtrace on Linux, etc.
        
        let mut frames = Vec::new();
        
        // For now, return a placeholder stack trace
        frames.push(StackFrame {
            address: 0x12345678,
            module_name: Some("matte-browser.exe".to_string()),
            function_name: Some("main".to_string()),
            source_file: Some("main.rs".to_string()),
            line_number: Some(42),
            offset: 0x1000,
        });

        Ok(frames)
    }

    /// Collect register information
    async fn collect_registers(&self) -> Result<HashMap<String, u64>> {
        // This is a simplified implementation
        // In a real implementation, this would use platform-specific APIs
        // to capture CPU registers at the time of the crash
        
        let mut registers = HashMap::new();
        registers.insert("rax".to_string(), 0x0);
        registers.insert("rbx".to_string(), 0x0);
        registers.insert("rcx".to_string(), 0x0);
        registers.insert("rdx".to_string(), 0x0);
        registers.insert("rsi".to_string(), 0x0);
        registers.insert("rdi".to_string(), 0x0);
        registers.insert("rsp".to_string(), 0x0);
        registers.insert("rbp".to_string(), 0x0);
        registers.insert("rip".to_string(), 0x0);

        Ok(registers)
    }

    /// Collect system information
    async fn collect_system_info(&self) -> Result<SystemInfo> {
        use crate::platform::PlatformSystem;
        
        let cpu_info = PlatformSystem::get_cpu_info()?;
        let memory_info = PlatformSystem::get_memory_info()?;
        
        Ok(SystemInfo {
            os_name: std::env::consts::OS.to_string(),
            os_version: std::env::consts::OS.to_string(), // Simplified
            architecture: std::env::consts::ARCH.to_string(),
            cpu_count: cpu_info.cores,
            memory_total: memory_info.total_physical,
            memory_available: memory_info.available_physical,
        })
    }

    /// Collect browser information
    async fn collect_browser_info(&self) -> Result<BrowserInfo> {
        Ok(BrowserInfo {
            version: env!("CARGO_PKG_VERSION").to_string(),
            build_id: env!("CARGO_PKG_VERSION").to_string(), // Simplified
            channel: "dev".to_string(),
            uptime: std::time::Duration::from_secs(0), // Would be calculated from start time
            tab_count: 0, // Would be retrieved from tab manager
            window_count: 0, // Would be retrieved from window manager
        })
    }

    /// Generate minidump file
    async fn generate_minidump(&self, crash_id: &str) -> Result<PathBuf> {
        let dump_path = self.config.crash_directory.join(format!("{}.dmp", crash_id));
        
        // This is a simplified implementation
        // In a real implementation, this would use platform-specific APIs
        // like MiniDumpWriteDump on Windows, or breakpad on other platforms
        
        // Create a placeholder minidump file
        let dump_content = format!(
            "Minidump for crash {}\nTimestamp: {:?}\nThis is a placeholder minidump file.",
            crash_id,
            std::time::SystemTime::now()
        );
        
        tokio::fs::write(&dump_path, dump_content).await
            .map_err(|e| Error::IoError(format!("Failed to write minidump: {}", e)))?;
        
        Ok(dump_path)
    }

    /// Privacy scrub a crash report
    async fn privacy_scrub_report(&self, report: &mut CrashReport) -> Result<()> {
        // Remove potentially sensitive information
        // This is a simplified implementation
        
        // Clear function names that might contain sensitive data
        for frame in &mut report.stack_trace {
            if let Some(func_name) = &frame.function_name {
                if func_name.contains("password") || func_name.contains("token") || func_name.contains("key") {
                    frame.function_name = Some("[REDACTED]".to_string());
                }
            }
        }
        
        // Clear register values that might contain sensitive data
        report.registers.clear();
        
        report.is_privacy_scrubbed = true;
        Ok(())
    }

    /// Upload a crash report
    pub async fn upload_crash_report(&self, report: &CrashReport) -> Result<()> {
        if let Some(upload_url) = &self.config.upload_url {
            // Serialize the report
            let report_json = serde_json::to_string(report)
                .map_err(|e| Error::ParseError(format!("Failed to serialize crash report: {}", e)))?;
            
            // Create HTTP client and upload
            // This is a simplified implementation
            // In a real implementation, this would use a proper HTTP client
            warn!("Would upload crash report to: {}", upload_url);
            debug!("Crash report size: {} bytes", report_json.len());
            
            Ok(())
        } else {
            warn!("No upload URL configured, skipping crash report upload");
            Ok(())
        }
    }

    /// Get symbol information for an address
    pub async fn get_symbol_info(&self, module_name: &str, address: u64) -> Result<Option<String>> {
        if let Some(symbol_server_url) = &self.config.symbol_server_url {
            // Check cache first
            {
                let cache = self.symbol_cache.read().await;
                if let Some(_symbols) = cache.get(module_name) {
                    // Parse symbols and find the function name
                    // This is a simplified implementation
                    return Ok(Some(format!("{}+0x{:x}", module_name, address)));
                }
            }
            
            // Download symbols from server
            // This is a simplified implementation
            warn!("Would download symbols for {} from {}", module_name, symbol_server_url);
            
            Ok(Some(format!("{}+0x{:x}", module_name, address)))
        } else {
            Ok(None)
        }
    }

    /// Get all crash reports
    pub async fn get_crash_reports(&self) -> Vec<CrashReport> {
        let reports = self.reports.read().await;
        reports.clone()
    }

    /// Clear old crash reports
    pub async fn clear_old_reports(&self, older_than: std::time::Duration) -> Result<usize> {
        let now = std::time::SystemTime::now();
        let mut reports = self.reports.write().await;
        let initial_count = reports.len();
        
        reports.retain(|report| {
            if let Ok(age) = now.duration_since(report.timestamp) {
                age < older_than
            } else {
                true // Keep reports with invalid timestamps
            }
        });
        
        let removed_count = initial_count - reports.len();
        info!("Cleared {} old crash reports", removed_count);
        
        Ok(removed_count)
    }

    /// Get crash reporter configuration
    pub fn config(&self) -> &CrashReporterConfig {
        &self.config
    }

    /// Update crash reporter configuration
    pub fn update_config(&mut self, new_config: CrashReporterConfig) -> Result<()> {
        // Create crash directory if it doesn't exist
        if !new_config.crash_directory.exists() {
            std::fs::create_dir_all(&new_config.crash_directory)
                .map_err(|e| Error::IoError(format!("Failed to create crash directory: {}", e)))?;
        }
        
        self.config = new_config;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_crash_reporter_creation() {
        let config = CrashReporterConfig::default();
        let reporter = CrashReporter::new(config).unwrap();
        
        assert!(reporter.config().enabled);
        assert_eq!(reporter.config().max_crash_size, 10 * 1024 * 1024);
    }

    #[tokio::test]
    async fn test_crash_report_generation() {
        let config = CrashReporterConfig::default();
        let reporter = CrashReporter::new(config).unwrap();
        
        let report = reporter.generate_crash_report(
            "browser".to_string(),
            1234,
            5678,
            "SIGSEGV".to_string(),
            0x12345678,
        ).await.unwrap();
        
        assert!(!report.crash_id.is_empty());
        assert_eq!(report.process_type, "browser");
        assert_eq!(report.process_id, 1234);
        assert_eq!(report.thread_id, 5678);
        assert_eq!(report.exception_type, "SIGSEGV");
        assert_eq!(report.exception_address, 0x12345678);
        assert!(!report.stack_trace.is_empty());
        // Registers might be empty if privacy scrubbing is enabled
        if !report.is_privacy_scrubbed {
            assert!(!report.registers.is_empty());
        }
    }

    #[tokio::test]
    async fn test_privacy_scrubbing() {
        let mut config = CrashReporterConfig::default();
        config.privacy_scrub_enabled = true;
        let reporter = CrashReporter::new(config).unwrap();
        
        let mut report = reporter.generate_crash_report(
            "browser".to_string(),
            1234,
            5678,
            "SIGSEGV".to_string(),
            0x12345678,
        ).await.unwrap();
        
        // Add some sensitive data to test scrubbing
        report.stack_trace.push(StackFrame {
            address: 0x12345678,
            module_name: Some("password_manager.dll".to_string()),
            function_name: Some("get_user_password".to_string()),
            source_file: Some("password.rs".to_string()),
            line_number: Some(42),
            offset: 0x1000,
        });
        
        reporter.privacy_scrub_report(&mut report).await.unwrap();
        
        assert!(report.is_privacy_scrubbed);
        assert!(report.registers.is_empty());
    }

    #[tokio::test]
    async fn test_crash_report_cleanup() {
        let config = CrashReporterConfig::default();
        let reporter = CrashReporter::new(config).unwrap();
        
        // Generate a few reports
        for i in 0..5 {
            reporter.generate_crash_report(
                "browser".to_string(),
                i,
                i * 2,
                "SIGSEGV".to_string(),
                0x12345678,
            ).await.unwrap();
        }
        
        let initial_reports = reporter.get_crash_reports().await;
        assert_eq!(initial_reports.len(), 5);
        
        // Clear reports older than 1 second (should keep all)
        let cleared = reporter.clear_old_reports(std::time::Duration::from_secs(1)).await.unwrap();
        assert_eq!(cleared, 0);
        
        let remaining_reports = reporter.get_crash_reports().await;
        assert_eq!(remaining_reports.len(), 5);
    }
}
