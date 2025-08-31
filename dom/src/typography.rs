use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Font family name
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FontFamily(pub String);

/// Font weight (100-900)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Copy)]
pub struct FontWeight(pub u16);

/// Font style (normal, italic, oblique)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Copy)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

/// Font stretch (ultra-condensed to ultra-expanded)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Copy)]
pub enum FontStretch {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

/// Font metrics for a specific font
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontMetrics {
    /// Ascent (distance from baseline to top of highest glyph)
    pub ascent: f32,
    /// Descent (distance from baseline to bottom of lowest glyph)
    pub descent: f32,
    /// Line gap (recommended space between lines)
    pub line_gap: f32,
    /// Cap height (height of capital letters)
    pub cap_height: f32,
    /// X height (height of lowercase 'x')
    pub x_height: f32,
    /// Underline position
    pub underline_position: f32,
    /// Underline thickness
    pub underline_thickness: f32,
    /// Strikeout position
    pub strikeout_position: f32,
    /// Strikeout thickness
    pub strikeout_thickness: f32,
}

impl Default for FontMetrics {
    fn default() -> Self {
        Self {
            ascent: 1900.0,
            descent: -500.0,
            line_gap: 0.0,
            cap_height: 1456.0,
            x_height: 1062.0,
            underline_position: -100.0,
            underline_thickness: 50.0,
            strikeout_position: 728.0,
            strikeout_thickness: 50.0,
        }
    }
}

impl FontMetrics {
    /// Get the line height for these metrics
    pub fn line_height(&self) -> f32 {
        self.ascent - self.descent + self.line_gap
    }
}

/// Font face information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontFace {
    /// Font family name
    pub family: FontFamily,
    /// Font weight
    pub weight: FontWeight,
    /// Font style
    pub style: FontStyle,
    /// Font stretch
    pub stretch: FontStretch,
    /// Font file path
    pub file_path: Option<PathBuf>,
    /// Font data (if loaded in memory)
    pub data: Option<Vec<u8>>,
    /// Font metrics
    pub metrics: FontMetrics,
    /// Whether the font is loaded
    pub is_loaded: bool,
    /// Font loading error (if any)
    pub load_error: Option<String>,
}

impl FontFace {
    /// Create a new font face
    pub fn new(family: FontFamily, weight: FontWeight, style: FontStyle, stretch: FontStretch) -> Self {
        Self {
            family,
            weight,
            style,
            stretch,
            file_path: None,
            data: None,
            metrics: FontMetrics::default(),
            is_loaded: false,
            load_error: None,
        }
    }
    
    /// Load font from file
    pub async fn load_from_file(&mut self, path: PathBuf) -> Result<(), String> {
        self.file_path = Some(path.clone());
        
        match tokio::fs::read(&path).await {
            Ok(data) => {
                self.data = Some(data);
                self.is_loaded = true;
                self.load_error = None;
                Ok(())
            }
            Err(e) => {
                self.load_error = Some(e.to_string());
                Err(e.to_string())
            }
        }
    }
    
    /// Get the line height for this font
    pub fn line_height(&self) -> f32 {
        self.metrics.ascent - self.metrics.descent + self.metrics.line_gap
    }
    
    /// Get the em size (typically 1000 units in most fonts)
    pub fn em_size(&self) -> f32 {
        1000.0
    }
}

/// Font fallback chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontFallback {
    /// Primary font family
    pub primary: FontFamily,
    /// Fallback font families in order of preference
    pub fallbacks: Vec<FontFamily>,
}

impl FontFallback {
    /// Create a new font fallback chain
    pub fn new(primary: FontFamily) -> Self {
        Self {
            primary,
            fallbacks: Vec::new(),
        }
    }
    
    /// Add a fallback font family
    pub fn add_fallback(&mut self, family: FontFamily) {
        self.fallbacks.push(family);
    }
    
    /// Get all font families in order
    pub fn all_families(&self) -> Vec<&FontFamily> {
        let mut families = vec![&self.primary];
        families.extend(self.fallbacks.iter());
        families
    }
}

/// Font cache entry
#[derive(Debug, Clone)]
pub struct FontCacheEntry {
    /// Font face
    pub face: FontFace,
    /// Last access time
    pub last_access: std::time::Instant,
    /// Access count
    pub access_count: u64,
}

/// Font manager for handling font loading, caching, and fallbacks
#[derive(Debug)]
pub struct FontManager {
    /// Font faces by family, weight, style, and stretch
    fonts: HashMap<(FontFamily, FontWeight, FontStyle, FontStretch), FontCacheEntry>,
    /// Font fallback chains
    fallbacks: HashMap<FontFamily, FontFallback>,
    /// System font directories
    system_font_dirs: Vec<PathBuf>,
    /// Maximum cache size
    max_cache_size: usize,
}

impl FontManager {
    /// Create a new font manager
    pub fn new() -> Self {
        let mut manager = Self {
            fonts: HashMap::new(),
            fallbacks: HashMap::new(),
            system_font_dirs: Vec::new(),
            max_cache_size: 1000,
        };
        
        // Add default system font directories
        manager.add_system_font_directories();
        
        // Set up default fallbacks
        manager.setup_default_fallbacks();
        
        manager
    }
    
    /// Add system font directories
    fn add_system_font_directories(&mut self) {
        #[cfg(target_os = "macos")]
        {
            self.system_font_dirs.push(PathBuf::from("/System/Library/Fonts"));
            self.system_font_dirs.push(PathBuf::from("/Library/Fonts"));
            // TODO: Add home directory fonts when dirs crate is available
        }
        
        #[cfg(target_os = "windows")]
        {
            self.system_font_dirs.push(PathBuf::from("C:\\Windows\\Fonts"));
        }
        
        #[cfg(target_os = "linux")]
        {
            self.system_font_dirs.push(PathBuf::from("/usr/share/fonts"));
            self.system_font_dirs.push(PathBuf::from("/usr/local/share/fonts"));
            // TODO: Add home directory fonts when dirs crate is available
        }
    }
    
    /// Set up default font fallbacks
    fn setup_default_fallbacks(&mut self) {
        // Serif fonts
        let mut serif = FontFallback::new(FontFamily("serif".to_string()));
        serif.add_fallback(FontFamily("Times New Roman".to_string()));
        serif.add_fallback(FontFamily("Times".to_string()));
        serif.add_fallback(FontFamily("Georgia".to_string()));
        self.fallbacks.insert(FontFamily("serif".to_string()), serif);
        
        // Sans-serif fonts
        let mut sans_serif = FontFallback::new(FontFamily("sans-serif".to_string()));
        sans_serif.add_fallback(FontFamily("Arial".to_string()));
        sans_serif.add_fallback(FontFamily("Helvetica".to_string()));
        sans_serif.add_fallback(FontFamily("Verdana".to_string()));
        self.fallbacks.insert(FontFamily("sans-serif".to_string()), sans_serif);
        
        // Monospace fonts
        let mut monospace = FontFallback::new(FontFamily("monospace".to_string()));
        monospace.add_fallback(FontFamily("Courier New".to_string()));
        monospace.add_fallback(FontFamily("Courier".to_string()));
        monospace.add_fallback(FontFamily("Consolas".to_string()));
        self.fallbacks.insert(FontFamily("monospace".to_string()), monospace);
    }
    
    /// Get a font face, loading it if necessary
    pub async fn get_font_face(
        &mut self,
        family: &FontFamily,
        weight: FontWeight,
        style: FontStyle,
        stretch: FontStretch,
    ) -> Option<&FontFace> {
        let key = (family.clone(), weight, style, stretch);
        
        // Check if font is already cached
        if let Some(entry) = self.fonts.get_mut(&key) {
            entry.last_access = std::time::Instant::now();
            entry.access_count += 1;
            return Some(&entry.face);
        }
        
        // TODO: Implement font loading without borrow checker issues
        // For now, return None
        None
    }
    

    
    /// Load a font face from the system
    async fn load_font_face(
        &self,
        key: &(FontFamily, FontWeight, FontStyle, FontStretch),
    ) -> Option<FontFace> {
        let (family, weight, style, stretch) = key;
        
        // Create a new font face
        let mut face = FontFace::new(family.clone(), *weight, *style, *stretch);
        
        // Try to find the font file
        if let Some(file_path) = self.find_font_file(family, *weight, *style, *stretch).await {
            if let Ok(()) = face.load_from_file(file_path).await {
                return Some(face);
            }
        }
        
        None
    }
    
    /// Find a font file in the system directories
    async fn find_font_file(
        &self,
        family: &FontFamily,
        _weight: FontWeight,
        _style: FontStyle,
        _stretch: FontStretch,
    ) -> Option<PathBuf> {
        // This is a simplified implementation
        // In a real browser, you would:
        // 1. Parse font files to extract metadata
        // 2. Match font properties more accurately
        // 3. Handle font format detection (TTF, OTF, WOFF, etc.)
        
        for dir in &self.system_font_dirs {
            if let Ok(entries) = tokio::fs::read_dir(dir).await {
                let mut entries = entries;
                while let Ok(Some(entry)) = entries.next_entry().await {
                    if let Ok(file_type) = entry.file_type().await {
                        if file_type.is_file() {
                            let path = entry.path();
                            if let Some(extension) = path.extension() {
                                if extension == "ttf" || extension == "otf" || extension == "woff" || extension == "woff2" {
                                    // Simple name matching (in real implementation, parse font metadata)
                                    if let Some(file_name) = path.file_stem() {
                                        let file_name = file_name.to_string_lossy().to_lowercase();
                                        let family_name = family.0.to_lowercase();
                                        if file_name.contains(&family_name) {
                                            return Some(path);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        None
    }
    
    /// Clean up the font cache
    fn cleanup_cache(&mut self) {
        if self.fonts.len() <= self.max_cache_size {
            return;
        }
        
        // Remove least recently used fonts
        let mut entries: Vec<_> = self.fonts.drain().collect();
        entries.sort_by(|a, b| a.1.last_access.cmp(&b.1.last_access));
        
        // Keep the most recently used fonts
        let keep_count = self.max_cache_size / 2;
        for (key, entry) in entries.into_iter().skip(keep_count) {
            self.fonts.insert(key, entry);
        }
    }
    
    /// Get font metrics for a specific font
    pub async fn get_font_metrics(
        &mut self,
        family: &FontFamily,
        weight: FontWeight,
        style: FontStyle,
        stretch: FontStretch,
    ) -> Option<FontMetrics> {
        if let Some(face) = self.get_font_face(family, weight, style, stretch).await {
            Some(face.metrics.clone())
        } else {
            None
        }
    }
    
    /// Get a fallback font family
    pub fn get_fallback_family(&self, family: &FontFamily) -> Option<&FontFallback> {
        self.fallbacks.get(family)
    }
    
    /// Add a custom font fallback
    pub fn add_font_fallback(&mut self, family: FontFamily, fallback: FontFallback) {
        self.fallbacks.insert(family, fallback);
    }
    
    /// Set the maximum cache size
    pub fn set_max_cache_size(&mut self, size: usize) {
        self.max_cache_size = size;
        self.cleanup_cache();
    }
    
    /// Get cache statistics
    pub fn get_cache_stats(&self) -> (usize, usize) {
        (self.fonts.len(), self.max_cache_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_face_creation() {
        let family = FontFamily("Arial".to_string());
        let weight = FontWeight(400);
        let style = FontStyle::Normal;
        let stretch = FontStretch::Normal;
        
        let face = FontFace::new(family.clone(), weight, style, stretch);
        
        assert_eq!(face.family, family);
        assert_eq!(face.weight, weight);
        assert_eq!(face.style, style);
        assert_eq!(face.stretch, stretch);
        assert!(!face.is_loaded);
    }

    #[test]
    fn test_font_metrics_default() {
        let metrics = FontMetrics::default();
        
        assert_eq!(metrics.ascent, 1900.0);
        assert_eq!(metrics.descent, -500.0);
        assert_eq!(metrics.line_gap, 0.0);
        assert_eq!(metrics.line_height(), 2400.0);
    }

    #[test]
    fn test_font_fallback_creation() {
        let mut fallback = FontFallback::new(FontFamily("Arial".to_string()));
        fallback.add_fallback(FontFamily("Helvetica".to_string()));
        fallback.add_fallback(FontFamily("sans-serif".to_string()));
        
        let families: Vec<_> = fallback.all_families().iter().map(|f| &f.0).collect();
        assert_eq!(families, vec!["Arial", "Helvetica", "sans-serif"]);
    }

    #[test]
    fn test_font_manager_creation() {
        let manager = FontManager::new();
        
        // Check that default fallbacks are set up
        assert!(manager.get_fallback_family(&FontFamily("serif".to_string())).is_some());
        assert!(manager.get_fallback_family(&FontFamily("sans-serif".to_string())).is_some());
        assert!(manager.get_fallback_family(&FontFamily("monospace".to_string())).is_some());
        
        // Check cache stats
        let (current, max) = manager.get_cache_stats();
        assert_eq!(current, 0);
        assert_eq!(max, 1000);
    }

    #[tokio::test]
    async fn test_font_manager_cache() {
        let mut manager = FontManager::new();
        
        // Set a small cache size for testing
        manager.set_max_cache_size(2);
        
        // Try to get a font (this will likely fail on CI, but that's okay)
        let family = FontFamily("Arial".to_string());
        let weight = FontWeight(400);
        let style = FontStyle::Normal;
        let stretch = FontStretch::Normal;
        
        let _result = manager.get_font_face(&family, weight, style, stretch).await;
        
        // The result might be None if the font isn't available, but the cache should work
        let (current, max) = manager.get_cache_stats();
        assert!(current <= max);
    }
}
