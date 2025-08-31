//! Renderer process for the Matte browser

use common::{error::Result, TabId, WindowId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

pub mod site_isolation;
pub mod dom_integration;
pub mod style_engine;
pub mod js_vm;
pub mod rendering_pipeline;

use site_isolation::SiteIsolationManager;
use dom_integration::DomIntegrationManager;
use style_engine::StyleEngineManager;
use js_vm::JavaScriptVmManager;
use rendering_pipeline::RenderingPipeline;

/// Renderer process configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RendererConfig {
    /// Enable site isolation
    pub site_isolation_enabled: bool,
    
    /// Maximum number of renderer processes
    pub max_processes: usize,
    
    /// Memory limit per process (in MB)
    pub memory_limit_mb: usize,
    
    /// Enable hardware acceleration
    pub hardware_acceleration: bool,
    
    /// Enable JavaScript JIT compilation
    pub js_jit_enabled: bool,
    
    /// Enable WebAssembly
    pub wasm_enabled: bool,
    
    /// Enable WebGL
    pub webgl_enabled: bool,
    
    /// Enable WebGPU
    pub webgpu_enabled: bool,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            site_isolation_enabled: true,
            max_processes: 10,
            memory_limit_mb: 512,
            hardware_acceleration: true,
            js_jit_enabled: true,
            wasm_enabled: true,
            webgl_enabled: true,
            webgpu_enabled: false, // Disabled by default for security
        }
    }
}

/// Renderer process state
#[derive(Debug, Clone)]
pub enum RendererState {
    /// Process is starting up
    Starting,
    
    /// Process is ready to handle requests
    Ready,
    
    /// Process is busy rendering
    Rendering,
    
    /// Process is shutting down
    ShuttingDown,
    
    /// Process has crashed
    Crashed(String),
}

/// Renderer process instance
pub struct RendererProcess {
    /// Process ID
    pub process_id: u64,
    
    /// Associated tab ID
    pub tab_id: TabId,
    
    /// Process state
    pub state: RendererState,
    
    /// Site isolation manager
    pub site_isolation: Arc<RwLock<SiteIsolationManager>>,
    
    /// DOM integration manager
    pub dom_integration: Arc<RwLock<DomIntegrationManager>>,
    
    /// Style engine manager
    pub style_engine: Arc<RwLock<StyleEngineManager>>,
    
    /// JavaScript VM manager
    pub js_vm: Arc<RwLock<JavaScriptVmManager>>,
    
    /// Rendering pipeline
    pub rendering_pipeline: Arc<RwLock<RenderingPipeline>>,
    
    /// Process configuration
    pub config: RendererConfig,
    
    /// Memory usage (in bytes)
    pub memory_usage: usize,
    
    /// CPU usage (percentage)
    pub cpu_usage: f64,
}

/// Renderer process manager
pub struct RendererProcessManager {
    /// Active renderer processes
    processes: HashMap<u64, Arc<RwLock<RendererProcess>>>,
    
    /// Site to process mapping
    site_process_map: HashMap<String, u64>,
    
    /// Process configuration
    config: RendererConfig,
    
    /// Next process ID
    next_process_id: u64,
    
    /// Process statistics
    stats: RendererStats,
}

/// Renderer process statistics
#[derive(Debug, Default)]
pub struct RendererStats {
    /// Total processes created
    pub total_processes: usize,
    
    /// Current active processes
    pub active_processes: usize,
    
    /// Total crashes
    pub crashes: usize,
    
    /// Total memory usage (bytes)
    pub total_memory_usage: usize,
    
    /// Average CPU usage
    pub avg_cpu_usage: f64,
}

impl RendererProcessManager {
    /// Create a new renderer process manager
    pub async fn new(config: RendererConfig) -> Result<Self> {
        info!("Initializing renderer process manager");
        
        Ok(Self {
            processes: HashMap::new(),
            site_process_map: HashMap::new(),
            config,
            next_process_id: 1,
            stats: RendererStats::default(),
        })
    }
    
    /// Create a new renderer process for a tab
    pub async fn create_process(&mut self, tab_id: TabId, site_url: &str) -> Result<u64> {
        info!("Creating renderer process for tab {} and site {}", tab_id, site_url);
        
        // Check if we've reached the process limit
        if self.processes.len() >= self.config.max_processes {
            return Err(common::error::Error::ResourceError(
                "Maximum number of renderer processes reached".to_string()
            ));
        }
        
        let process_id = self.next_process_id;
        self.next_process_id += 1;
        
        // Create the renderer process
        let process = RendererProcess {
            process_id,
            tab_id,
            state: RendererState::Starting,
            site_isolation: Arc::new(RwLock::new(SiteIsolationManager::new(site_url).await?)),
            dom_integration: Arc::new(RwLock::new(DomIntegrationManager::new().await?)),
            style_engine: Arc::new(RwLock::new(StyleEngineManager::new().await?)),
            js_vm: Arc::new(RwLock::new(JavaScriptVmManager::new(&self.config).await?)),
            rendering_pipeline: Arc::new(RwLock::new(RenderingPipeline::new(&self.config).await?)),
            config: self.config.clone(),
            memory_usage: 0,
            cpu_usage: 0.0,
        };
        
        // Store the process
        self.processes.insert(process_id, Arc::new(RwLock::new(process)));
        
        // Map site to process (if site isolation is enabled)
        if self.config.site_isolation_enabled {
            let site_key = self.extract_site_key(site_url);
            self.site_process_map.insert(site_key, process_id);
        }
        
        // Update statistics
        self.stats.total_processes += 1;
        self.stats.active_processes += 1;
        
        info!("Renderer process {} created successfully", process_id);
        Ok(process_id)
    }
    
    /// Get or create a renderer process for a site
    pub async fn get_or_create_process(&mut self, tab_id: TabId, site_url: &str) -> Result<u64> {
        if self.config.site_isolation_enabled {
            let site_key = self.extract_site_key(site_url);
            
            // Check if we already have a process for this site
            if let Some(&process_id) = self.site_process_map.get(&site_key) {
                // Verify the process is still active
                if let Some(process) = self.processes.get(&process_id) {
                    let process_guard = process.read().await;
                    if matches!(process_guard.state, RendererState::Ready | RendererState::Rendering) {
                        return Ok(process_id);
                    }
                }
            }
        }
        
        // Create a new process
        self.create_process(tab_id, site_url).await
    }
    
    /// Get a renderer process by ID
    pub async fn get_process(&self, process_id: u64) -> Option<Arc<RwLock<RendererProcess>>> {
        self.processes.get(&process_id).cloned()
    }
    
    /// Terminate a renderer process
    pub async fn terminate_process(&mut self, process_id: u64) -> Result<()> {
        info!("Terminating renderer process {}", process_id);
        
        if let Some(process) = self.processes.remove(&process_id) {
            let mut process_guard = process.write().await;
            process_guard.state = RendererState::ShuttingDown;
            
            // Clean up site mapping
            let site_key = {
                let site_isolation = process_guard.site_isolation.read().await;
                site_isolation.site_url().to_string()
            };
            self.site_process_map.remove(&self.extract_site_key(&site_key));
            
            // Update statistics
            self.stats.active_processes -= 1;
            
            info!("Renderer process {} terminated", process_id);
        }
        
        Ok(())
    }
    
    /// Get all active processes
    pub async fn get_active_processes(&self) -> Vec<Arc<RwLock<RendererProcess>>> {
        self.processes.values().cloned().collect()
    }
    
    /// Get process statistics
    pub fn get_stats(&self) -> &RendererStats {
        &self.stats
    }
    
    /// Update process statistics
    pub async fn update_stats(&mut self) -> Result<()> {
        let mut total_memory = 0;
        let mut total_cpu = 0.0;
        let mut active_count = 0;
        
        for process in self.processes.values() {
            let process_guard = process.read().await;
            if matches!(process_guard.state, RendererState::Ready | RendererState::Rendering) {
                total_memory += process_guard.memory_usage;
                total_cpu += process_guard.cpu_usage;
                active_count += 1;
            }
        }
        
        self.stats.total_memory_usage = total_memory;
        self.stats.active_processes = active_count;
        self.stats.avg_cpu_usage = if active_count > 0 {
            total_cpu / active_count as f64
        } else {
            0.0
        };
        
        Ok(())
    }
    
    /// Extract site key from URL for site isolation
    fn extract_site_key(&self, url: &str) -> String {
        // Simple site key extraction - in a real implementation, this would be more sophisticated
        if let Ok(parsed_url) = url::Url::parse(url) {
            if let Some(host) = parsed_url.host_str() {
                return host.to_string();
            }
        }
        url.to_string()
    }
    
    /// Shutdown all processes
    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down renderer process manager");
        
        let process_ids: Vec<u64> = self.processes.keys().cloned().collect();
        
        for process_id in process_ids {
            if let Err(e) = self.terminate_process(process_id).await {
                warn!("Failed to terminate process {}: {}", process_id, e);
            }
        }
        
        info!("Renderer process manager shutdown complete");
        Ok(())
    }
}

impl RendererProcess {
    /// Initialize the renderer process
    pub async fn initialize(&mut self) -> Result<()> {
        info!("Initializing renderer process {}", self.process_id);
        
        // Initialize site isolation
        {
            let mut site_isolation = self.site_isolation.write().await;
            site_isolation.initialize().await?;
        }
        
        // Initialize DOM integration
        {
            let mut dom_integration = self.dom_integration.write().await;
            dom_integration.initialize().await?;
        }
        
        // Initialize style engine
        {
            let mut style_engine = self.style_engine.write().await;
            style_engine.initialize().await?;
        }
        
        // Initialize JavaScript VM
        {
            let mut js_vm = self.js_vm.write().await;
            js_vm.initialize().await?;
        }
        
        // Initialize rendering pipeline
        {
            let mut rendering_pipeline = self.rendering_pipeline.write().await;
            rendering_pipeline.initialize().await?;
        }
        
        self.state = RendererState::Ready;
        info!("Renderer process {} initialized successfully", self.process_id);
        
        Ok(())
    }
    
    /// Load a URL in the renderer process
    pub async fn load_url(&mut self, url: &str) -> Result<()> {
        info!("Loading URL {} in renderer process {}", url, self.process_id);
        
        self.state = RendererState::Rendering;
        
        // Load URL in site isolation
        {
            let mut site_isolation = self.site_isolation.write().await;
            site_isolation.load_url(url).await?;
        }
        
        // Parse HTML and create DOM
        {
            let mut dom_integration = self.dom_integration.write().await;
            dom_integration.parse_html(url).await?;
        }
        
        // Apply styles
        {
            let mut style_engine = self.style_engine.write().await;
            style_engine.apply_styles().await?;
        }
        
        // Execute JavaScript
        {
            let mut js_vm = self.js_vm.write().await;
            js_vm.execute_scripts().await?;
        }
        
        // Render the page
        {
            let mut rendering_pipeline = self.rendering_pipeline.write().await;
            rendering_pipeline.render_page().await?;
        }
        
        self.state = RendererState::Ready;
        info!("URL {} loaded successfully in renderer process {}", url, self.process_id);
        
        Ok(())
    }
    
    /// Execute JavaScript in the renderer process
    pub async fn execute_script(&self, script: &str) -> Result<serde_json::Value> {
        let js_vm = self.js_vm.read().await;
        js_vm.execute_script(script).await
    }
    
    /// Get the current DOM tree
    pub async fn get_dom_tree(&self) -> Result<serde_json::Value> {
        let dom_integration = self.dom_integration.read().await;
        dom_integration.get_dom_tree().await
    }
    
    /// Get computed styles for an element
    pub async fn get_computed_styles(&self, element_id: &str) -> Result<serde_json::Value> {
        let style_engine = self.style_engine.read().await;
        style_engine.get_computed_styles(element_id).await
    }
    
    /// Take a screenshot of the current page
    pub async fn take_screenshot(&self) -> Result<Vec<u8>> {
        let rendering_pipeline = self.rendering_pipeline.read().await;
        rendering_pipeline.take_screenshot().await
    }
    
    /// Update memory and CPU usage
    pub async fn update_usage_stats(&mut self) -> Result<()> {
        // TODO: Implement actual usage monitoring
        self.memory_usage = 1024 * 1024; // 1MB placeholder
        self.cpu_usage = 5.0; // 5% placeholder
        Ok(())
    }
}

/// Initialize the renderer process
pub async fn init(config: RendererConfig) -> Result<RendererProcessManager> {
    info!("Initializing renderer process");
    RendererProcessManager::new(config).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_renderer_process_manager_creation() {
        let config = RendererConfig::default();
        let manager = RendererProcessManager::new(config).await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_renderer_process_creation() {
        let config = RendererConfig::default();
        let mut manager = RendererProcessManager::new(config).await.unwrap();
        
        let tab_id = TabId::new();
        let process_id = manager.create_process(tab_id, "https://example.com").await;
        assert!(process_id.is_ok());
        
        let process_id = process_id.unwrap();
        assert!(manager.get_process(process_id).await.is_some());
    }

    #[tokio::test]
    async fn test_site_isolation() {
        let config = RendererConfig {
            site_isolation_enabled: true,
            ..Default::default()
        };
        let mut manager = RendererProcessManager::new(config).await.unwrap();
        
        let tab_id1 = TabId::new();
        let tab_id2 = TabId::new();
        
        let process_id1 = manager.get_or_create_process(tab_id1, "https://example.com").await.unwrap();
        let process_id2 = manager.get_or_create_process(tab_id2, "https://example.com").await.unwrap();
        
        // Should reuse the same process for the same site
        assert_eq!(process_id1, process_id2);
        
        let process_id3 = manager.get_or_create_process(tab_id2, "https://different.com").await.unwrap();
        
        // Should create a new process for a different site
        assert_ne!(process_id1, process_id3);
    }

    #[tokio::test]
    async fn test_process_limit() {
        let config = RendererConfig {
            max_processes: 1,
            ..Default::default()
        };
        let mut manager = RendererProcessManager::new(config).await.unwrap();
        
        let tab_id1 = TabId::new();
        let tab_id2 = TabId::new();
        
        let process_id1 = manager.create_process(tab_id1, "https://example.com").await;
        assert!(process_id1.is_ok());
        
        let process_id2 = manager.create_process(tab_id2, "https://different.com").await;
        assert!(process_id2.is_err());
    }
}
