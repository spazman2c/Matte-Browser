use crate::error::{Error, Result};
use crate::style::{FontFamily, FontWeight, FontStyleType, FontStretch};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Font loading manager
pub struct FontLoadingManager {
    /// Font registry
    fonts: Arc<RwLock<HashMap<String, FontFamily>>>,
    /// Font face registry
    font_faces: Arc<RwLock<HashMap<String, FontFace>>>,
    /// Font loading queue
    loading_queue: Arc<RwLock<Vec<FontLoadingTask>>>,
    /// Font cache
    font_cache: Arc<RwLock<HashMap<String, Arc<LoadedFont>>>>,
    /// Font loading callbacks
    loading_callbacks: Arc<RwLock<HashMap<String, Vec<Box<dyn Fn(Arc<LoadedFont>) + Send>>>>>,
}

/// Font face
#[derive(Debug, Clone)]
pub struct FontFace {
    /// Font family name
    pub family: String,
    /// Font source
    pub source: FontSource,
    /// Font weight
    pub weight: FontWeight,
    /// Font style
    pub style: FontStyleType,
    /// Font stretch
    pub stretch: FontStretch,
    /// Unicode range
    pub unicode_range: Vec<UnicodeRange>,
    /// Font display
    pub display: FontDisplay,
    /// Font feature settings
    pub feature_settings: Vec<FontFeatureSetting>,
    /// Font variation settings
    pub variation_settings: Vec<FontVariationSetting>,
    /// Font ascent override
    pub ascent_override: Option<f32>,
    /// Font descent override
    pub descent_override: Option<f32>,
    /// Font line gap override
    pub line_gap_override: Option<f32>,
    /// Font advance override
    pub advance_override: Option<f32>,
}

/// Font source
#[derive(Debug, Clone)]
pub enum FontSource {
    /// Local font
    Local(String),
    /// URL font
    Url(String),
    /// Format hint
    FormatHint(String),
}

/// Unicode range
#[derive(Debug, Clone)]
pub struct UnicodeRange {
    /// Start code point
    pub start: u32,
    /// End code point
    pub end: u32,
}

/// Font display
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FontDisplay {
    /// Auto display
    Auto,
    /// Block display
    Block,
    /// Swap display
    Swap,
    /// Fallback display
    Fallback,
    /// Optional display
    Optional,
}

/// Font feature setting
#[derive(Debug, Clone)]
pub struct FontFeatureSetting {
    /// Feature tag
    pub tag: String,
    /// Feature value
    pub value: u32,
}

/// Font variation setting
#[derive(Debug, Clone)]
pub struct FontVariationSetting {
    /// Variation axis tag
    pub axis: String,
    /// Variation value
    pub value: f32,
}

/// Font loading task
#[derive(Debug, Clone)]
pub struct FontLoadingTask {
    /// Task ID
    pub id: String,
    /// Font face
    pub font_face: FontFace,
    /// Loading state
    pub state: FontLoadingState,
    /// Created time
    pub created: Instant,
    /// Priority
    pub priority: FontLoadingPriority,
}

/// Font loading state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontLoadingState {
    /// Pending
    Pending,
    /// Loading
    Loading,
    /// Loaded
    Loaded,
    /// Failed
    Failed,
}

/// Font loading priority
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontLoadingPriority {
    /// High priority
    High,
    /// Normal priority
    Normal,
    /// Low priority
    Low,
}

/// Loaded font
#[derive(Debug, Clone)]
pub struct LoadedFont {
    /// Font face
    pub font_face: FontFace,
    /// Font data
    pub data: Vec<u8>,
    /// Font format
    pub format: FontFormat,
    /// Loaded time
    pub loaded: Instant,
    /// Font metrics
    pub metrics: FontMetrics,
}

/// Font format
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FontFormat {
    /// TrueType
    TrueType,
    /// OpenType
    OpenType,
    /// Web Open Font Format
    WOFF,
    /// Web Open Font Format 2
    WOFF2,
    /// Embedded OpenType
    EOT,
    /// Scalable Vector Graphics
    SVG,
}

/// Font metrics
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FontMetrics {
    /// Units per em
    pub units_per_em: u16,
    /// Ascent
    pub ascent: i16,
    /// Descent
    pub descent: i16,
    /// Line gap
    pub line_gap: i16,
    /// Cap height
    pub cap_height: i16,
    /// X height
    pub x_height: i16,
    /// Underline position
    pub underline_position: i16,
    /// Underline thickness
    pub underline_thickness: u16,
    /// Strikeout position
    pub strikeout_position: i16,
    /// Strikeout thickness
    pub strikeout_thickness: u16,
}

/// Font loading API
pub struct FontLoadingAPI {
    /// Loading manager
    manager: Arc<FontLoadingManager>,
    /// Network client
    network_client: Arc<dyn NetworkClient>,
    /// File system
    file_system: Arc<dyn FileSystem>,
}

/// Network client trait
pub trait NetworkClient: Send + Sync {
    /// Fetch resource
    fn fetch(&self, url: &str) -> Result<Vec<u8>>;
    /// Check if URL is supported
    fn is_supported(&self, url: &str) -> bool;
}

/// File system trait
pub trait FileSystem: Send + Sync {
    /// Read file
    fn read_file(&self, path: &PathBuf) -> Result<Vec<u8>>;
    /// Check if file exists
    fn exists(&self, path: &PathBuf) -> bool;
}

/// Font face set
pub struct FontFaceSet {
    /// Font faces
    faces: Vec<FontFace>,
    /// Loading state
    loading_state: FontFaceSetLoadingState,
    /// Ready promise
    ready_promise: Option<FontFaceSetReadyPromise>,
}

/// Font face set loading state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontFaceSetLoadingState {
    /// Loading
    Loading,
    /// Loaded
    Loaded,
}

/// Font face set ready promise
#[derive(Debug, Clone)]
pub struct FontFaceSetReadyPromise {
    /// Promise ID
    pub id: String,
    /// Resolve callback
    pub resolve: Box<dyn Fn() + Send>,
    /// Reject callback
    pub reject: Box<dyn Fn(Error) + Send>,
}

impl FontLoadingManager {
    /// Create new font loading manager
    pub fn new() -> Self {
        Self {
            fonts: Arc::new(RwLock::new(HashMap::new())),
            font_faces: Arc::new(RwLock::new(HashMap::new())),
            loading_queue: Arc::new(RwLock::new(Vec::new())),
            font_cache: Arc::new(RwLock::new(HashMap::new())),
            loading_callbacks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add font face
    pub fn add_font_face(&self, font_face: FontFace) -> Result<()> {
        let key = self.generate_font_face_key(&font_face);
        
        // Add to font faces registry
        self.font_faces.write().insert(key.clone(), font_face.clone());
        
        // Add to loading queue
        let task = FontLoadingTask {
            id: key.clone(),
            font_face,
            state: FontLoadingState::Pending,
            created: Instant::now(),
            priority: FontLoadingPriority::Normal,
        };
        
        self.loading_queue.write().push(task);
        
        Ok(())
    }

    /// Load font face
    pub fn load_font_face(&self, font_face: &FontFace) -> Result<Arc<LoadedFont>> {
        let key = self.generate_font_face_key(font_face);
        
        // Check cache first
        if let Some(font) = self.font_cache.read().get(&key) {
            return Ok(font.clone());
        }
        
        // Load font data
        let font_data = self.load_font_data(font_face)?;
        
        // Parse font format
        let format = self.detect_font_format(&font_data)?;
        
        // Parse font metrics
        let metrics = self.parse_font_metrics(&font_data, format)?;
        
        let loaded_font = Arc::new(LoadedFont {
            font_face: font_face.clone(),
            data: font_data,
            format,
            loaded: Instant::now(),
            metrics,
        });
        
        // Cache the font
        self.font_cache.write().insert(key, loaded_font.clone());
        
        Ok(loaded_font)
    }

    /// Get font face by family and style
    pub fn get_font_face(&self, family: &str, weight: FontWeight, style: FontStyleType, stretch: FontStretch) -> Option<FontFace> {
        let key = self.generate_font_face_key_from_style(family, weight, style, stretch);
        self.font_faces.read().get(&key).cloned()
    }

    /// Check if character is supported
    pub fn is_character_supported(&self, font_face: &FontFace, ch: char) -> bool {
        let code_point = ch as u32;
        
        for range in &font_face.unicode_range {
            if code_point >= range.start && code_point <= range.end {
                return true;
            }
        }
        
        false
    }

    /// Add loading callback
    pub fn add_loading_callback<F>(&self, font_face: &FontFace, callback: F)
    where
        F: Fn(Arc<LoadedFont>) + Send + 'static,
    {
        let key = self.generate_font_face_key(font_face);
        self.loading_callbacks.write()
            .entry(key)
            .or_insert_with(Vec::new)
            .push(Box::new(callback));
    }

    /// Generate font face key
    fn generate_font_face_key(&self, font_face: &FontFace) -> String {
        format!("{}:{}:{}:{}", font_face.family, font_face.weight as u16, font_face.style as u8, font_face.stretch as u16)
    }

    /// Generate font face key from style
    fn generate_font_face_key_from_style(&self, family: &str, weight: FontWeight, style: FontStyleType, stretch: FontStretch) -> String {
        format!("{}:{}:{}:{}", family, weight as u16, style as u8, stretch as u16)
    }

    /// Load font data
    fn load_font_data(&self, font_face: &FontFace) -> Result<Vec<u8>> {
        match &font_face.source {
            FontSource::Local(name) => {
                // Load local font
                self.load_local_font(name)
            }
            FontSource::Url(url) => {
                // Load remote font
                self.load_remote_font(url)
            }
            FontSource::FormatHint(_) => {
                // Format hint is not used for loading
                Err(Error::typography("Format hint cannot be used for loading".to_string()))
            }
        }
    }

    /// Load local font
    fn load_local_font(&self, name: &str) -> Result<Vec<u8>> {
        // This is a simplified implementation
        // In a real implementation, you would search system font directories
        let font_paths = [
            format!("/System/Library/Fonts/{}.ttf", name),
            format!("/System/Library/Fonts/{}.otf", name),
            format!("/Library/Fonts/{}.ttf", name),
            format!("/Library/Fonts/{}.otf", name),
            format!("~/.fonts/{}.ttf", name),
            format!("~/.fonts/{}.otf", name),
        ];
        
        for path_str in &font_paths {
            let path = PathBuf::from(path_str);
            if path.exists() {
                return std::fs::read(&path)
                    .map_err(|e| Error::typography(format!("Failed to read font file: {}", e)));
            }
        }
        
        Err(Error::typography(format!("Font not found: {}", name)))
    }

    /// Load remote font
    fn load_remote_font(&self, url: &str) -> Result<Vec<u8>> {
        // This is a simplified implementation
        // In a real implementation, you would use a proper HTTP client
        let response = ureq::get(url)
            .call()
            .map_err(|e| Error::typography(format!("Failed to fetch font: {}", e)))?;
        
        let mut bytes = Vec::new();
        response.into_reader()
            .read_to_end(&mut bytes)
            .map_err(|e| Error::typography(format!("Failed to read font data: {}", e)))?;
        
        Ok(bytes)
    }

    /// Detect font format
    fn detect_font_format(&self, data: &[u8]) -> Result<FontFormat> {
        if data.len() < 4 {
            return Err(Error::typography("Invalid font data".to_string()));
        }
        
        let signature = &data[0..4];
        
        match signature {
            b"OTTO" => Ok(FontFormat::OpenType),
            b"ttcf" => Ok(FontFormat::TrueType),
            b"wOFF" => Ok(FontFormat::WOFF),
            b"wO2F" => Ok(FontFormat::WOFF2),
            _ => {
                // Check for TrueType signature
                if data.len() >= 12 {
                    let ttf_signature = &data[0..4];
                    if ttf_signature == b"\x00\x01\x00\x00" {
                        return Ok(FontFormat::TrueType);
                    }
                }
                
                Err(Error::typography("Unknown font format".to_string()))
            }
        }
    }

    /// Parse font metrics
    fn parse_font_metrics(&self, data: &[u8], format: FontFormat) -> Result<FontMetrics> {
        match format {
            FontFormat::TrueType | FontFormat::OpenType => {
                self.parse_opentype_metrics(data)
            }
            FontFormat::WOFF => {
                self.parse_woff_metrics(data)
            }
            FontFormat::WOFF2 => {
                self.parse_woff2_metrics(data)
            }
            _ => {
                // Return default metrics for unsupported formats
                Ok(FontMetrics::default())
            }
        }
    }

    /// Parse OpenType metrics
    fn parse_opentype_metrics(&self, data: &[u8]) -> Result<FontMetrics> {
        // This is a simplified implementation
        // In a real implementation, you would parse the OpenType tables
        
        if data.len() < 12 {
            return Err(Error::typography("Invalid OpenType font data".to_string()));
        }
        
        // Read units per em from head table
        let units_per_em = u16::from_be_bytes([data[18], data[19]]);
        
        // For now, return default metrics
        Ok(FontMetrics {
            units_per_em,
            ascent: 1900,
            descent: -500,
            line_gap: 0,
            cap_height: 1456,
            x_height: 1062,
            underline_position: -100,
            underline_thickness: 50,
            strikeout_position: 600,
            strikeout_thickness: 50,
        })
    }

    /// Parse WOFF metrics
    fn parse_woff_metrics(&self, data: &[u8]) -> Result<FontMetrics> {
        // This is a simplified implementation
        // In a real implementation, you would parse the WOFF format
        
        // For now, return default metrics
        Ok(FontMetrics::default())
    }

    /// Parse WOFF2 metrics
    fn parse_woff2_metrics(&self, data: &[u8]) -> Result<FontMetrics> {
        // This is a simplified implementation
        // In a real implementation, you would parse the WOFF2 format
        
        // For now, return default metrics
        Ok(FontMetrics::default())
    }
}

impl FontLoadingAPI {
    /// Create new font loading API
    pub fn new(manager: Arc<FontLoadingManager>, network_client: Arc<dyn NetworkClient>, file_system: Arc<dyn FileSystem>) -> Self {
        Self {
            manager,
            network_client,
            file_system,
        }
    }

    /// Load font face
    pub fn load_font_face(&self, font_face: &FontFace) -> Result<Arc<LoadedFont>> {
        self.manager.load_font_face(font_face)
    }

    /// Load font face with display strategy
    pub fn load_font_face_with_display(&self, font_face: &FontFace) -> Result<FontLoadingResult> {
        match font_face.display {
            FontDisplay::Block => {
                // Block until font is loaded
                let font = self.load_font_face(font_face)?;
                Ok(FontLoadingResult::Loaded(font))
            }
            FontDisplay::Swap => {
                // Return immediately, swap when loaded
                let font = self.load_font_face(font_face);
                match font {
                    Ok(font) => Ok(FontLoadingResult::Loaded(font)),
                    Err(_) => Ok(FontLoadingResult::Fallback),
                }
            }
            FontDisplay::Fallback => {
                // Use fallback font immediately
                Ok(FontLoadingResult::Fallback)
            }
            FontDisplay::Optional => {
                // Use fallback font, don't swap
                Ok(FontLoadingResult::Fallback)
            }
            FontDisplay::Auto => {
                // Use browser's default behavior (usually block)
                let font = self.load_font_face(font_face)?;
                Ok(FontLoadingResult::Loaded(font))
            }
        }
    }

    /// Check if font is loaded
    pub fn is_font_loaded(&self, font_face: &FontFace) -> bool {
        let key = self.manager.generate_font_face_key(font_face);
        self.manager.font_cache.read().contains_key(&key)
    }

    /// Get font loading state
    pub fn get_font_loading_state(&self, font_face: &FontFace) -> FontLoadingState {
        let key = self.manager.generate_font_face_key(font_face);
        
        if self.manager.font_cache.read().contains_key(&key) {
            FontLoadingState::Loaded
        } else if self.manager.loading_queue.read().iter().any(|task| task.id == key) {
            FontLoadingState::Loading
        } else {
            FontLoadingState::Pending
        }
    }
}

/// Font loading result
#[derive(Debug, Clone)]
pub enum FontLoadingResult {
    /// Font loaded
    Loaded(Arc<LoadedFont>),
    /// Use fallback font
    Fallback,
    /// Loading in progress
    Loading,
}

impl FontFaceSet {
    /// Create new font face set
    pub fn new() -> Self {
        Self {
            faces: Vec::new(),
            loading_state: FontFaceSetLoadingState::Loaded,
            ready_promise: None,
        }
    }

    /// Add font face
    pub fn add(&mut self, font_face: FontFace) {
        self.faces.push(font_face);
        self.loading_state = FontFaceSetLoadingState::Loading;
    }

    /// Get loading state
    pub fn loading_state(&self) -> FontFaceSetLoadingState {
        self.loading_state
    }

    /// Check if ready
    pub fn ready(&self) -> bool {
        self.loading_state == FontFaceSetLoadingState::Loaded
    }

    /// Get ready promise
    pub fn ready_promise(&self) -> Option<&FontFaceSetReadyPromise> {
        self.ready_promise.as_ref()
    }

    /// Set ready promise
    pub fn set_ready_promise(&mut self, promise: FontFaceSetReadyPromise) {
        self.ready_promise = Some(promise);
    }
}

impl UnicodeRange {
    /// Create new unicode range
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    /// Parse unicode range from string
    pub fn from_string(range_str: &str) -> Result<Self> {
        // Parse unicode range in format "U+XXXX" or "U+XXXX-YYYY"
        let range_str = range_str.trim();
        
        if !range_str.starts_with("U+") {
            return Err(Error::typography("Invalid unicode range format".to_string()));
        }
        
        let range_part = &range_str[2..];
        
        if let Some(dash_pos) = range_part.find('-') {
            let start_str = &range_part[..dash_pos];
            let end_str = &range_part[dash_pos + 1..];
            
            let start = u32::from_str_radix(start_str, 16)
                .map_err(|_| Error::typography("Invalid unicode range start".to_string()))?;
            let end = u32::from_str_radix(end_str, 16)
                .map_err(|_| Error::typography("Invalid unicode range end".to_string()))?;
            
            Ok(UnicodeRange { start, end })
        } else {
            let code_point = u32::from_str_radix(range_part, 16)
                .map_err(|_| Error::typography("Invalid unicode range".to_string()))?;
            
            Ok(UnicodeRange { start: code_point, end: code_point })
        }
    }

    /// Check if code point is in range
    pub fn contains(&self, code_point: u32) -> bool {
        code_point >= self.start && code_point <= self.end
    }
}

impl FontMetrics {
    /// Create default font metrics
    pub fn default() -> Self {
        Self {
            units_per_em: 1000,
            ascent: 800,
            descent: -200,
            line_gap: 0,
            cap_height: 700,
            x_height: 500,
            underline_position: -100,
            underline_thickness: 50,
            strikeout_position: 400,
            strikeout_thickness: 50,
        }
    }

    /// Get em size
    pub fn em_size(&self) -> f32 {
        self.units_per_em as f32
    }

    /// Get ascent ratio
    pub fn ascent_ratio(&self) -> f32 {
        self.ascent as f32 / self.em_size()
    }

    /// Get descent ratio
    pub fn descent_ratio(&self) -> f32 {
        self.descent as f32 / self.em_size()
    }

    /// Get line gap ratio
    pub fn line_gap_ratio(&self) -> f32 {
        self.line_gap as f32 / self.em_size()
    }

    /// Get cap height ratio
    pub fn cap_height_ratio(&self) -> f32 {
        self.cap_height as f32 / self.em_size()
    }

    /// Get x height ratio
    pub fn x_height_ratio(&self) -> f32 {
        self.x_height as f32 / self.em_size()
    }
}

impl FontFace {
    /// Create new font face
    pub fn new(family: String, source: FontSource) -> Self {
        Self {
            family,
            source,
            weight: FontWeight::Normal,
            style: FontStyleType::Normal,
            stretch: FontStretch::Normal,
            unicode_range: Vec::new(),
            display: FontDisplay::Auto,
            feature_settings: Vec::new(),
            variation_settings: Vec::new(),
            ascent_override: None,
            descent_override: None,
            line_gap_override: None,
            advance_override: None,
        }
    }

    /// Set font weight
    pub fn with_weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }

    /// Set font style
    pub fn with_style(mut self, style: FontStyleType) -> Self {
        self.style = style;
        self
    }

    /// Set font stretch
    pub fn with_stretch(mut self, stretch: FontStretch) -> Self {
        self.stretch = stretch;
        self
    }

    /// Set unicode range
    pub fn with_unicode_range(mut self, range: UnicodeRange) -> Self {
        self.unicode_range.push(range);
        self
    }

    /// Set font display
    pub fn with_display(mut self, display: FontDisplay) -> Self {
        self.display = display;
        self
    }

    /// Add feature setting
    pub fn with_feature_setting(mut self, tag: String, value: u32) -> Self {
        self.feature_settings.push(FontFeatureSetting { tag, value });
        self
    }

    /// Add variation setting
    pub fn with_variation_setting(mut self, axis: String, value: f32) -> Self {
        self.variation_settings.push(FontVariationSetting { axis, value });
        self
    }
}
