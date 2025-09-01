use crate::error::{Error, Result};
use crate::rendering::{Color, Point, Rectangle, Transform, Image};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use parking_lot::{RwLock, Mutex};
use serde::{Serialize, Deserialize};
use std::time::{Duration, Instant, SystemTime};
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Condvar;

/// Layer type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LayerType {
    Background,
    Content,
    Overlay,
    UI,
    Cursor,
    Debug,
}

/// Layer blend mode
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LayerBlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

/// Layer state
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LayerState {
    Hidden,
    Visible,
    Animating,
    Dirty,
}

/// Layer
#[derive(Debug, Clone)]
pub struct Layer {
    /// Layer ID
    pub id: u64,
    /// Layer name
    pub name: String,
    /// Layer type
    pub layer_type: LayerType,
    /// Layer state
    pub state: LayerState,
    /// Layer bounds
    pub bounds: Rectangle,
    /// Layer transform
    pub transform: Transform,
    /// Layer opacity
    pub opacity: f32,
    /// Layer blend mode
    pub blend_mode: LayerBlendMode,
    /// Layer content
    pub content: Option<Arc<Image>>,
    /// Layer dirty region
    pub dirty_region: Option<Rectangle>,
    /// Layer children
    pub children: Vec<u64>,
    /// Layer parent
    pub parent: Option<u64>,
    /// Layer z-index
    pub z_index: i32,
    /// Layer creation time
    pub created: SystemTime,
    /// Layer last update time
    pub last_update: SystemTime,
}

/// Frame timing information
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FrameTiming {
    /// Frame number
    pub frame_number: u64,
    /// Frame start time
    pub start_time: Instant,
    /// Frame end time
    pub end_time: Instant,
    /// Frame duration
    pub duration: Duration,
    /// Target frame duration
    pub target_duration: Duration,
    /// Vsync time
    pub vsync_time: Option<Instant>,
    /// Frame drop count
    pub dropped_frames: u32,
}

/// Vsync mode
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum VsyncMode {
    Disabled,
    Enabled,
    Adaptive,
}

/// Hardware acceleration type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum HardwareAcceleration {
    None,
    OpenGL,
    Vulkan,
    Metal,
    DirectX,
}

/// Window state
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
    Hidden,
}

/// Window event
#[derive(Debug, Clone)]
pub enum WindowEvent {
    Resize { width: u32, height: u32 },
    Move { x: i32, y: i32 },
    Focus { focused: bool },
    Close,
    Minimize,
    Maximize,
    Restore,
    KeyPress { key: String, modifiers: Vec<String> },
    KeyRelease { key: String, modifiers: Vec<String> },
    MouseMove { x: f32, y: f32 },
    MousePress { button: u8, x: f32, y: f32 },
    MouseRelease { button: u8, x: f32, y: f32 },
    MouseWheel { delta_x: f32, delta_y: f32 },
    Touch { id: u64, x: f32, y: f32, phase: TouchPhase },
}

/// Touch phase
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TouchPhase {
    Began,
    Moved,
    Ended,
    Cancelled,
}

/// Window configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    /// Window title
    pub title: String,
    /// Window width
    pub width: u32,
    /// Window height
    pub height: u32,
    /// Window position
    pub x: Option<i32>,
    pub y: Option<i32>,
    /// Window state
    pub state: WindowState,
    /// Window resizable
    pub resizable: bool,
    /// Window decorations
    pub decorations: bool,
    /// Window transparency
    pub transparent: bool,
    /// Window always on top
    pub always_on_top: bool,
    /// Window vsync mode
    pub vsync_mode: VsyncMode,
    /// Window hardware acceleration
    pub hardware_acceleration: HardwareAcceleration,
}

/// Compositor statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositorStats {
    /// Total frames rendered
    pub total_frames: u64,
    /// Average frame time
    pub avg_frame_time: Duration,
    /// Frame rate
    pub frame_rate: f64,
    /// Dropped frames
    pub dropped_frames: u64,
    /// Total layers
    pub total_layers: usize,
    /// Visible layers
    pub visible_layers: usize,
    /// Dirty layers
    pub dirty_layers: usize,
    /// Memory usage
    pub memory_usage: usize,
    /// GPU memory usage
    pub gpu_memory_usage: usize,
    /// Last update time
    pub last_update: SystemTime,
}

/// Layer manager
pub struct LayerManager {
    /// Layers
    layers: Arc<RwLock<HashMap<u64, Layer>>>,
    /// Layer tree root
    root_layer: Option<u64>,
    /// Next layer ID
    next_layer_id: Arc<Mutex<u64>>,
    /// Layer event sender
    layer_event_sender: Sender<LayerEvent>,
}

/// Layer event
#[derive(Debug, Clone)]
pub enum LayerEvent {
    LayerCreated { id: u64, layer: Layer },
    LayerDestroyed { id: u64 },
    LayerUpdated { id: u64, layer: Layer },
    LayerMoved { id: u64, new_parent: Option<u64> },
    LayerVisibilityChanged { id: u64, visible: bool },
}

/// Hardware accelerator
pub struct HardwareAccelerator {
    /// Acceleration type
    acceleration_type: HardwareAcceleration,
    /// GPU context
    gpu_context: Option<Arc<dyn GpuContext>>,
    /// Shader cache
    shader_cache: Arc<RwLock<HashMap<String, Arc<dyn Shader>>>>,
    /// Texture cache
    texture_cache: Arc<RwLock<HashMap<String, Arc<dyn Texture>>>>,
    /// Buffer cache
    buffer_cache: Arc<RwLock<HashMap<String, Arc<dyn Buffer>>>>,
}

/// GPU context trait
pub trait GpuContext: Send + Sync {
    /// Initialize GPU context
    fn initialize(&mut self) -> Result<()>;
    /// Create shader
    fn create_shader(&self, source: &str, shader_type: ShaderType) -> Result<Arc<dyn Shader>>;
    /// Create texture
    fn create_texture(&self, width: u32, height: u32, format: TextureFormat) -> Result<Arc<dyn Texture>>;
    /// Create buffer
    fn create_buffer(&self, data: &[u8], buffer_type: BufferType) -> Result<Arc<dyn Buffer>>;
    /// Begin frame
    fn begin_frame(&mut self) -> Result<()>;
    /// End frame
    fn end_frame(&mut self) -> Result<()>;
    /// Clear
    fn clear(&mut self, color: Color) -> Result<()>;
    /// Draw
    fn draw(&mut self, draw_call: DrawCall) -> Result<()>;
    /// Get capabilities
    fn get_capabilities(&self) -> GpuCapabilities;
}

/// Shader trait
pub trait Shader: Send + Sync {
    /// Get shader type
    fn get_type(&self) -> ShaderType;
    /// Compile shader
    fn compile(&mut self, source: &str) -> Result<()>;
    /// Get uniform location
    fn get_uniform_location(&self, name: &str) -> Option<u32>;
    /// Set uniform value
    fn set_uniform(&self, location: u32, value: UniformValue) -> Result<()>;
}

/// Texture trait
pub trait Texture: Send + Sync {
    /// Get texture dimensions
    fn get_dimensions(&self) -> (u32, u32);
    /// Get texture format
    fn get_format(&self) -> TextureFormat;
    /// Update texture data
    fn update(&mut self, data: &[u8], x: u32, y: u32, width: u32, height: u32) -> Result<()>;
    /// Bind texture
    fn bind(&self, unit: u32) -> Result<()>;
}

/// Buffer trait
pub trait Buffer: Send + Sync {
    /// Get buffer type
    fn get_type(&self) -> BufferType;
    /// Get buffer size
    fn get_size(&self) -> usize;
    /// Update buffer data
    fn update(&mut self, data: &[u8], offset: usize) -> Result<()>;
    /// Bind buffer
    fn bind(&self) -> Result<()>;
}

/// Shader type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ShaderType {
    Vertex,
    Fragment,
    Compute,
}

/// Texture format
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TextureFormat {
    RGBA8,
    RGB8,
    RG8,
    R8,
    RGBA16F,
    RGB16F,
    RG16F,
    R16F,
    RGBA32F,
    RGB32F,
    RG32F,
    R32F,
}

/// Buffer type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BufferType {
    Vertex,
    Index,
    Uniform,
    Storage,
}

/// Uniform value
#[derive(Debug, Clone)]
pub enum UniformValue {
    Float(f32),
    Vec2(f32, f32),
    Vec3(f32, f32, f32),
    Vec4(f32, f32, f32, f32),
    Int(i32),
    Vec2i(i32, i32),
    Vec3i(i32, i32, i32),
    Vec4i(i32, i32, i32, i32),
    Mat3([f32; 9]),
    Mat4([f32; 16]),
}

/// Draw call
#[derive(Debug, Clone)]
pub struct DrawCall {
    /// Shader program
    pub shader: Arc<dyn Shader>,
    /// Vertex buffer
    pub vertex_buffer: Arc<dyn Buffer>,
    /// Index buffer
    pub index_buffer: Option<Arc<dyn Buffer>>,
    /// Textures
    pub textures: Vec<Arc<dyn Texture>>,
    /// Uniforms
    pub uniforms: HashMap<String, UniformValue>,
    /// Draw mode
    pub draw_mode: DrawMode,
    /// Vertex count
    pub vertex_count: u32,
    /// Index count
    pub index_count: Option<u32>,
    /// Instance count
    pub instance_count: u32,
}

/// Draw mode
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DrawMode {
    Points,
    Lines,
    LineStrip,
    LineLoop,
    Triangles,
    TriangleStrip,
    TriangleFan,
}

/// GPU capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuCapabilities {
    /// Maximum texture size
    pub max_texture_size: u32,
    /// Maximum vertex attributes
    pub max_vertex_attributes: u32,
    /// Maximum uniform buffer size
    pub max_uniform_buffer_size: usize,
    /// Maximum storage buffer size
    pub max_storage_buffer_size: usize,
    /// Maximum compute work group size
    pub max_compute_work_group_size: [u32; 3],
    /// Supported texture formats
    pub supported_texture_formats: Vec<TextureFormat>,
    /// Supported shader versions
    pub supported_shader_versions: Vec<String>,
}

/// Vsync manager
pub struct VsyncManager {
    /// Vsync mode
    vsync_mode: VsyncMode,
    /// Target frame rate
    target_frame_rate: f64,
    /// Target frame duration
    target_frame_duration: Duration,
    /// Last vsync time
    last_vsync: Option<Instant>,
    /// Frame timing history
    frame_timing_history: VecDeque<FrameTiming>,
    /// Vsync callback
    vsync_callback: Option<Box<dyn Fn() + Send>>,
    /// Running flag
    running: Arc<AtomicBool>,
    /// Vsync thread
    vsync_thread: Option<thread::JoinHandle<()>>,
}

/// Window manager
pub struct WindowManager {
    /// Windows
    windows: Arc<RwLock<HashMap<u64, Window>>>,
    /// Next window ID
    next_window_id: Arc<Mutex<u64>>,
    /// Window event sender
    window_event_sender: Sender<WindowEvent>,
    /// Window event receiver
    window_event_receiver: Receiver<WindowEvent>,
    /// Active window
    active_window: Arc<RwLock<Option<u64>>>,
    /// Window event callbacks
    window_event_callbacks: Arc<RwLock<HashMap<u64, Vec<Box<dyn Fn(WindowEvent) + Send>>>>>,
}

/// Window
#[derive(Debug, Clone)]
pub struct Window {
    /// Window ID
    pub id: u64,
    /// Window configuration
    pub config: WindowConfig,
    /// Window state
    pub state: WindowState,
    /// Window bounds
    pub bounds: Rectangle,
    /// Window content
    pub content: Option<Arc<Image>>,
    /// Window dirty region
    pub dirty_region: Option<Rectangle>,
    /// Window creation time
    pub created: SystemTime,
    /// Window last update time
    pub last_update: SystemTime,
}

/// Compositor
pub struct Compositor {
    /// Layer manager
    layer_manager: LayerManager,
    /// Hardware accelerator
    hardware_accelerator: HardwareAccelerator,
    /// Vsync manager
    vsync_manager: VsyncManager,
    /// Window manager
    window_manager: WindowManager,
    /// Compositor statistics
    stats: Arc<RwLock<CompositorStats>>,
    /// Running flag
    running: Arc<AtomicBool>,
    /// Compositor thread
    compositor_thread: Option<thread::JoinHandle<()>>,
    /// Frame event sender
    frame_event_sender: Sender<FrameEvent>,
}

/// Frame event
#[derive(Debug, Clone)]
pub enum FrameEvent {
    FrameStart { frame_number: u64, timestamp: Instant },
    FrameEnd { frame_number: u64, timestamp: Instant, duration: Duration },
    Vsync { timestamp: Instant },
    LayerDirty { layer_id: u64, region: Rectangle },
    WindowResized { window_id: u64, width: u32, height: u32 },
}

impl Layer {
    /// Create new layer
    pub fn new(id: u64, name: String, layer_type: LayerType, bounds: Rectangle) -> Self {
        let now = SystemTime::now();
        Self {
            id,
            name,
            layer_type,
            state: LayerState::Visible,
            bounds,
            transform: Transform::identity(),
            opacity: 1.0,
            blend_mode: LayerBlendMode::Normal,
            content: None,
            dirty_region: None,
            children: Vec::new(),
            parent: None,
            z_index: 0,
            created: now,
            last_update: now,
        }
    }

    /// Check if layer is visible
    pub fn is_visible(&self) -> bool {
        self.state == LayerState::Visible && self.opacity > 0.0
    }

    /// Check if layer is dirty
    pub fn is_dirty(&self) -> bool {
        self.state == LayerState::Dirty || self.dirty_region.is_some()
    }

    /// Mark layer as dirty
    pub fn mark_dirty(&mut self, region: Option<Rectangle>) {
        self.state = LayerState::Dirty;
        self.dirty_region = region;
        self.last_update = SystemTime::now();
    }

    /// Clear dirty state
    pub fn clear_dirty(&mut self) {
        self.state = LayerState::Visible;
        self.dirty_region = None;
    }

    /// Get world transform
    pub fn get_world_transform(&self, layer_manager: &LayerManager) -> Transform {
        let mut transform = self.transform;
        let mut current_parent = self.parent;
        
        while let Some(parent_id) = current_parent {
            if let Some(parent_layer) = layer_manager.get_layer(parent_id) {
                transform = parent_layer.transform.multiply(&transform);
                current_parent = parent_layer.parent;
            } else {
                break;
            }
        }
        
        transform
    }

    /// Get world bounds
    pub fn get_world_bounds(&self, layer_manager: &LayerManager) -> Rectangle {
        let world_transform = self.get_world_transform(layer_manager);
        
        // Transform the four corners of the layer
        let corners = [
            Point::new(self.bounds.x, self.bounds.y),
            Point::new(self.bounds.x + self.bounds.width, self.bounds.y),
            Point::new(self.bounds.x + self.bounds.width, self.bounds.y + self.bounds.height),
            Point::new(self.bounds.x, self.bounds.y + self.bounds.height),
        ];
        
        let transformed_corners: Vec<Point> = corners.iter()
            .map(|p| p.transform(&world_transform))
            .collect();
        
        // Calculate bounding rectangle
        let min_x = transformed_corners.iter().map(|p| p.x).fold(f32::INFINITY, f32::min);
        let min_y = transformed_corners.iter().map(|p| p.y).fold(f32::INFINITY, f32::min);
        let max_x = transformed_corners.iter().map(|p| p.x).fold(f32::NEG_INFINITY, f32::max);
        let max_y = transformed_corners.iter().map(|p| p.y).fold(f32::NEG_INFINITY, f32::max);
        
        Rectangle::new(min_x, min_y, max_x - min_x, max_y - min_y)
    }
}

impl LayerManager {
    /// Create new layer manager
    pub fn new() -> Self {
        let (sender, _) = mpsc::channel();
        Self {
            layers: Arc::new(RwLock::new(HashMap::new())),
            root_layer: None,
            next_layer_id: Arc::new(Mutex::new(1)),
            layer_event_sender: sender,
        }
    }

    /// Create new layer
    pub fn create_layer(&self, name: String, layer_type: LayerType, bounds: Rectangle) -> u64 {
        let id = {
            let mut next_id = self.next_layer_id.lock();
            let id = *next_id;
            *next_id += 1;
            id
        };
        
        let layer = Layer::new(id, name, layer_type, bounds);
        
        {
            let mut layers = self.layers.write();
            layers.insert(id, layer.clone());
            
            // Set as root layer if it's the first layer
            if self.root_layer.is_none() {
                self.root_layer = Some(id);
            }
        }
        
        // Send layer created event
        let _ = self.layer_event_sender.send(LayerEvent::LayerCreated {
            id,
            layer,
        });
        
        id
    }

    /// Get layer by ID
    pub fn get_layer(&self, id: u64) -> Option<Layer> {
        self.layers.read().get(&id).cloned()
    }

    /// Update layer
    pub fn update_layer(&self, id: u64, updates: impl FnOnce(&mut Layer)) -> Result<()> {
        let mut layers = self.layers.write();
        if let Some(layer) = layers.get_mut(&id) {
            updates(layer);
            layer.last_update = SystemTime::now();
            
            // Send layer updated event
            let _ = self.layer_event_sender.send(LayerEvent::LayerUpdated {
                id,
                layer: layer.clone(),
            });
            
            Ok(())
        } else {
            Err(Error::graphics(format!("Layer {} not found", id)))
        }
    }

    /// Destroy layer
    pub fn destroy_layer(&self, id: u64) -> Result<()> {
        let mut layers = self.layers.write();
        if layers.remove(&id).is_some() {
            // Send layer destroyed event
            let _ = self.layer_event_sender.send(LayerEvent::LayerDestroyed { id });
            Ok(())
        } else {
            Err(Error::graphics(format!("Layer {} not found", id)))
        }
    }

    /// Set layer parent
    pub fn set_layer_parent(&self, layer_id: u64, parent_id: Option<u64>) -> Result<()> {
        // Validate parent exists
        if let Some(parent_id) = parent_id {
            if !self.layers.read().contains_key(&parent_id) {
                return Err(Error::graphics(format!("Parent layer {} not found", parent_id)));
            }
        }
        
        // Update layer parent
        self.update_layer(layer_id, |layer| {
            // Remove from old parent's children
            if let Some(old_parent_id) = layer.parent {
                if let Some(old_parent) = self.layers.write().get_mut(&old_parent_id) {
                    old_parent.children.retain(|&x| x != layer_id);
                }
            }
            
            // Add to new parent's children
            if let Some(new_parent_id) = parent_id {
                if let Some(new_parent) = self.layers.write().get_mut(&new_parent_id) {
                    new_parent.children.push(layer_id);
                }
            }
            
            layer.parent = parent_id;
        })?;
        
        // Send layer moved event
        let _ = self.layer_event_sender.send(LayerEvent::LayerMoved {
            id: layer_id,
            new_parent: parent_id,
        });
        
        Ok(())
    }

    /// Get visible layers
    pub fn get_visible_layers(&self) -> Vec<Layer> {
        self.layers.read().values()
            .filter(|layer| layer.is_visible())
            .cloned()
            .collect()
    }

    /// Get dirty layers
    pub fn get_dirty_layers(&self) -> Vec<Layer> {
        self.layers.read().values()
            .filter(|layer| layer.is_dirty())
            .cloned()
            .collect()
    }

    /// Get layers in render order
    pub fn get_layers_in_render_order(&self) -> Vec<Layer> {
        let mut layers: Vec<Layer> = self.layers.read().values().cloned().collect();
        layers.sort_by(|a, b| a.z_index.cmp(&b.z_index));
        layers
    }
}

impl HardwareAccelerator {
    /// Create new hardware accelerator
    pub fn new(acceleration_type: HardwareAcceleration) -> Self {
        Self {
            acceleration_type,
            gpu_context: None,
            shader_cache: Arc::new(RwLock::new(HashMap::new())),
            texture_cache: Arc::new(RwLock::new(HashMap::new())),
            buffer_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Initialize hardware acceleration
    pub fn initialize(&mut self) -> Result<()> {
        // TODO: Implement hardware acceleration initialization
        // This would create the appropriate GPU context based on the acceleration type
        match self.acceleration_type {
            HardwareAcceleration::None => {
                // Software rendering
                Ok(())
            }
            HardwareAcceleration::OpenGL => {
                // TODO: Initialize OpenGL context
                Ok(())
            }
            HardwareAcceleration::Vulkan => {
                // TODO: Initialize Vulkan context
                Ok(())
            }
            HardwareAcceleration::Metal => {
                // TODO: Initialize Metal context
                Ok(())
            }
            HardwareAcceleration::DirectX => {
                // TODO: Initialize DirectX context
                Ok(())
            }
        }
    }

    /// Check if hardware acceleration is available
    pub fn is_available(&self) -> bool {
        self.gpu_context.is_some()
    }

    /// Get GPU capabilities
    pub fn get_capabilities(&self) -> Option<GpuCapabilities> {
        self.gpu_context.as_ref().map(|ctx| ctx.get_capabilities())
    }

    /// Create shader
    pub fn create_shader(&self, source: &str, shader_type: ShaderType) -> Result<Arc<dyn Shader>> {
        let cache_key = format!("{}:{}", shader_type as u8, source);
        
        // Check cache first
        if let Some(shader) = self.shader_cache.read().get(&cache_key) {
            return Ok(shader.clone());
        }
        
        // Create new shader
        if let Some(gpu_context) = &self.gpu_context {
            let shader = gpu_context.create_shader(source, shader_type)?;
            
            // Cache the shader
            self.shader_cache.write().insert(cache_key, shader.clone());
            
            Ok(shader)
        } else {
            Err(Error::graphics("Hardware acceleration not available".to_string()))
        }
    }

    /// Create texture
    pub fn create_texture(&self, width: u32, height: u32, format: TextureFormat) -> Result<Arc<dyn Texture>> {
        let cache_key = format!("{}x{}:{}", width, height, format as u8);
        
        // Check cache first
        if let Some(texture) = self.texture_cache.read().get(&cache_key) {
            return Ok(texture.clone());
        }
        
        // Create new texture
        if let Some(gpu_context) = &self.gpu_context {
            let texture = gpu_context.create_texture(width, height, format)?;
            
            // Cache the texture
            self.texture_cache.write().insert(cache_key, texture.clone());
            
            Ok(texture)
        } else {
            Err(Error::graphics("Hardware acceleration not available".to_string()))
        }
    }

    /// Create buffer
    pub fn create_buffer(&self, data: &[u8], buffer_type: BufferType) -> Result<Arc<dyn Buffer>> {
        let cache_key = format!("{}:{}", buffer_type as u8, data.len());
        
        // Check cache first
        if let Some(buffer) = self.buffer_cache.read().get(&cache_key) {
            return Ok(buffer.clone());
        }
        
        // Create new buffer
        if let Some(gpu_context) = &self.gpu_context {
            let buffer = gpu_context.create_buffer(data, buffer_type)?;
            
            // Cache the buffer
            self.buffer_cache.write().insert(cache_key, buffer.clone());
            
            Ok(buffer)
        } else {
            Err(Error::graphics("Hardware acceleration not available".to_string()))
        }
    }

    /// Begin frame
    pub fn begin_frame(&mut self) -> Result<()> {
        if let Some(gpu_context) = &mut self.gpu_context {
            gpu_context.begin_frame()
        } else {
            Ok(())
        }
    }

    /// End frame
    pub fn end_frame(&mut self) -> Result<()> {
        if let Some(gpu_context) = &mut self.gpu_context {
            gpu_context.end_frame()
        } else {
            Ok(())
        }
    }

    /// Clear
    pub fn clear(&mut self, color: Color) -> Result<()> {
        if let Some(gpu_context) = &mut self.gpu_context {
            gpu_context.clear(color)
        } else {
            Ok(())
        }
    }

    /// Draw
    pub fn draw(&mut self, draw_call: DrawCall) -> Result<()> {
        if let Some(gpu_context) = &mut self.gpu_context {
            gpu_context.draw(draw_call)
        } else {
            Ok(())
        }
    }
}

impl VsyncManager {
    /// Create new vsync manager
    pub fn new(vsync_mode: VsyncMode, target_frame_rate: f64) -> Self {
        let target_frame_duration = Duration::from_secs_f64(1.0 / target_frame_rate);
        Self {
            vsync_mode,
            target_frame_rate,
            target_frame_duration,
            last_vsync: None,
            frame_timing_history: VecDeque::with_capacity(60), // Keep last 60 frames
            vsync_callback: None,
            running: Arc::new(AtomicBool::new(false)),
            vsync_thread: None,
        }
    }

    /// Set vsync callback
    pub fn set_vsync_callback<F>(&mut self, callback: F)
    where
        F: Fn() + Send + 'static,
    {
        self.vsync_callback = Some(Box::new(callback));
    }

    /// Start vsync manager
    pub fn start(&mut self) -> Result<()> {
        if self.running.load(Ordering::Relaxed) {
            return Ok(());
        }
        
        self.running.store(true, Ordering::Relaxed);
        
        let running = self.running.clone();
        let target_frame_duration = self.target_frame_duration;
        let vsync_callback = self.vsync_callback.clone();
        
        self.vsync_thread = Some(thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                let frame_start = Instant::now();
                
                // Call vsync callback
                if let Some(callback) = &vsync_callback {
                    callback();
                }
                
                // Wait for next frame
                let frame_end = Instant::now();
                let frame_duration = frame_end.duration_since(frame_start);
                
                if frame_duration < target_frame_duration {
                    thread::sleep(target_frame_duration - frame_duration);
                }
            }
        }));
        
        Ok(())
    }

    /// Stop vsync manager
    pub fn stop(&mut self) -> Result<()> {
        self.running.store(false, Ordering::Relaxed);
        
        if let Some(thread) = self.vsync_thread.take() {
            thread.join().map_err(|_| Error::graphics("Failed to join vsync thread".to_string()))?;
        }
        
        Ok(())
    }

    /// Record frame timing
    pub fn record_frame(&mut self, frame_number: u64, start_time: Instant, end_time: Instant) {
        let duration = end_time.duration_since(start_time);
        let frame_timing = FrameTiming {
            frame_number,
            start_time,
            end_time,
            duration,
            target_duration: self.target_frame_duration,
            vsync_time: self.last_vsync,
            dropped_frames: if duration > self.target_frame_duration * 2 {
                1
            } else {
                0
            },
        };
        
        self.frame_timing_history.push_back(frame_timing);
        
        // Keep only last 60 frames
        if self.frame_timing_history.len() > 60 {
            self.frame_timing_history.pop_front();
        }
    }

    /// Get frame rate
    pub fn get_frame_rate(&self) -> f64 {
        if self.frame_timing_history.len() < 2 {
            return 0.0;
        }
        
        let total_duration: Duration = self.frame_timing_history.iter()
            .map(|ft| ft.duration)
            .sum();
        
        let avg_duration = total_duration.as_secs_f64() / self.frame_timing_history.len() as f64;
        
        if avg_duration > 0.0 {
            1.0 / avg_duration
        } else {
            0.0
        }
    }

    /// Get dropped frame count
    pub fn get_dropped_frames(&self) -> u64 {
        self.frame_timing_history.iter()
            .map(|ft| ft.dropped_frames as u64)
            .sum()
    }
}

impl WindowManager {
    /// Create new window manager
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            windows: Arc::new(RwLock::new(HashMap::new())),
            next_window_id: Arc::new(Mutex::new(1)),
            window_event_sender: sender,
            window_event_receiver: receiver,
            active_window: Arc::new(RwLock::new(None)),
            window_event_callbacks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create new window
    pub fn create_window(&self, config: WindowConfig) -> u64 {
        let id = {
            let mut next_id = self.next_window_id.lock();
            let id = *next_id;
            *next_id += 1;
            id
        };
        
        let window = Window {
            id,
            config: config.clone(),
            state: config.state,
            bounds: Rectangle::new(
                config.x.unwrap_or(0) as f32,
                config.y.unwrap_or(0) as f32,
                config.width as f32,
                config.height as f32,
            ),
            content: None,
            dirty_region: None,
            created: SystemTime::now(),
            last_update: SystemTime::now(),
        };
        
        {
            let mut windows = self.windows.write();
            windows.insert(id, window);
            
            // Set as active window if it's the first window
            if self.active_window.read().is_none() {
                *self.active_window.write() = Some(id);
            }
        }
        
        id
    }

    /// Get window by ID
    pub fn get_window(&self, id: u64) -> Option<Window> {
        self.windows.read().get(&id).cloned()
    }

    /// Update window
    pub fn update_window(&self, id: u64, updates: impl FnOnce(&mut Window)) -> Result<()> {
        let mut windows = self.windows.write();
        if let Some(window) = windows.get_mut(&id) {
            updates(window);
            window.last_update = SystemTime::now();
            Ok(())
        } else {
            Err(Error::graphics(format!("Window {} not found", id)))
        }
    }

    /// Destroy window
    pub fn destroy_window(&self, id: u64) -> Result<()> {
        let mut windows = self.windows.write();
        if windows.remove(&id).is_some() {
            // Update active window if needed
            if self.active_window.read() == Some(id) {
                *self.active_window.write() = windows.keys().next().copied();
            }
            Ok(())
        } else {
            Err(Error::graphics(format!("Window {} not found", id)))
        }
    }

    /// Set active window
    pub fn set_active_window(&self, id: u64) -> Result<()> {
        if self.windows.read().contains_key(&id) {
            *self.active_window.write() = Some(id);
            Ok(())
        } else {
            Err(Error::graphics(format!("Window {} not found", id)))
        }
    }

    /// Get active window
    pub fn get_active_window(&self) -> Option<u64> {
        self.active_window.read().clone()
    }

    /// Add window event callback
    pub fn add_event_callback<F>(&self, window_id: u64, callback: F)
    where
        F: Fn(WindowEvent) + Send + 'static,
    {
        self.window_event_callbacks.write()
            .entry(window_id)
            .or_insert_with(Vec::new)
            .push(Box::new(callback));
    }

    /// Process window events
    pub fn process_events(&self) {
        while let Ok(event) = self.window_event_receiver.try_recv() {
            // Call event callbacks
            let callbacks = self.window_event_callbacks.read();
            if let Some(window_callbacks) = callbacks.get(&0) { // Global callbacks
                for callback in window_callbacks {
                    callback(event.clone());
                }
            }
        }
    }

    /// Send window event
    pub fn send_event(&self, event: WindowEvent) -> Result<()> {
        self.window_event_sender.send(event)
            .map_err(|_| Error::graphics("Failed to send window event".to_string()))
    }
}

impl Compositor {
    /// Create new compositor
    pub fn new(config: WindowConfig) -> Self {
        let (frame_sender, _) = mpsc::channel();
        Self {
            layer_manager: LayerManager::new(),
            hardware_accelerator: HardwareAccelerator::new(config.hardware_acceleration),
            vsync_manager: VsyncManager::new(config.vsync_mode, 60.0),
            window_manager: WindowManager::new(),
            stats: Arc::new(RwLock::new(CompositorStats::new())),
            running: Arc::new(AtomicBool::new(false)),
            compositor_thread: None,
            frame_event_sender: frame_sender,
        }
    }

    /// Initialize compositor
    pub fn initialize(&mut self) -> Result<()> {
        // Initialize hardware acceleration
        self.hardware_accelerator.initialize()?;
        
        // Start vsync manager
        self.vsync_manager.start()?;
        
        Ok(())
    }

    /// Start compositor
    pub fn start(&mut self) -> Result<()> {
        if self.running.load(Ordering::Relaxed) {
            return Ok(());
        }
        
        self.running.store(true, Ordering::Relaxed);
        
        let running = self.running.clone();
        let layer_manager = self.layer_manager.clone();
        let hardware_accelerator = self.hardware_accelerator.clone();
        let vsync_manager = self.vsync_manager.clone();
        let stats = self.stats.clone();
        let frame_sender = self.frame_event_sender.clone();
        
        self.compositor_thread = Some(thread::spawn(move || {
            let mut frame_number = 0;
            
            while running.load(Ordering::Relaxed) {
                let frame_start = Instant::now();
                
                // Send frame start event
                let _ = frame_sender.send(FrameEvent::FrameStart {
                    frame_number,
                    timestamp: frame_start,
                });
                
                // Begin frame
                if let Err(e) = hardware_accelerator.begin_frame() {
                    eprintln!("Failed to begin frame: {}", e);
                    continue;
                }
                
                // Clear background
                if let Err(e) = hardware_accelerator.clear(Color::rgb(255, 255, 255)) {
                    eprintln!("Failed to clear frame: {}", e);
                    continue;
                }
                
                // Render layers
                let layers = layer_manager.get_layers_in_render_order();
                for layer in layers {
                    if layer.is_visible() {
                        // TODO: Render layer content
                        // This would involve creating draw calls for the layer
                    }
                }
                
                // End frame
                if let Err(e) = hardware_accelerator.end_frame() {
                    eprintln!("Failed to end frame: {}", e);
                    continue;
                }
                
                let frame_end = Instant::now();
                
                // Record frame timing
                vsync_manager.record_frame(frame_number, frame_start, frame_end);
                
                // Update statistics
                {
                    let mut stats = stats.write();
                    stats.total_frames += 1;
                    stats.frame_rate = vsync_manager.get_frame_rate();
                    stats.dropped_frames = vsync_manager.get_dropped_frames();
                    stats.total_layers = layer_manager.layers.read().len();
                    stats.visible_layers = layer_manager.get_visible_layers().len();
                    stats.dirty_layers = layer_manager.get_dirty_layers().len();
                    stats.last_update = SystemTime::now();
                }
                
                // Send frame end event
                let _ = frame_sender.send(FrameEvent::FrameEnd {
                    frame_number,
                    timestamp: frame_end,
                    duration: frame_end.duration_since(frame_start),
                });
                
                frame_number += 1;
            }
        }));
        
        Ok(())
    }

    /// Stop compositor
    pub fn stop(&mut self) -> Result<()> {
        self.running.store(false, Ordering::Relaxed);
        
        if let Some(thread) = self.compositor_thread.take() {
            thread.join().map_err(|_| Error::graphics("Failed to join compositor thread".to_string()))?;
        }
        
        self.vsync_manager.stop()?;
        
        Ok(())
    }

    /// Get layer manager
    pub fn layer_manager(&self) -> &LayerManager {
        &self.layer_manager
    }

    /// Get window manager
    pub fn window_manager(&self) -> &WindowManager {
        &self.window_manager
    }

    /// Get compositor statistics
    pub fn get_stats(&self) -> CompositorStats {
        self.stats.read().clone()
    }
}

impl CompositorStats {
    /// Create new compositor statistics
    pub fn new() -> Self {
        Self {
            total_frames: 0,
            avg_frame_time: Duration::ZERO,
            frame_rate: 0.0,
            dropped_frames: 0,
            total_layers: 0,
            visible_layers: 0,
            dirty_layers: 0,
            memory_usage: 0,
            gpu_memory_usage: 0,
            last_update: SystemTime::now(),
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Matte Browser".to_string(),
            width: 1024,
            height: 768,
            x: None,
            y: None,
            state: WindowState::Normal,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            vsync_mode: VsyncMode::Enabled,
            hardware_acceleration: HardwareAcceleration::None,
        }
    }
}
