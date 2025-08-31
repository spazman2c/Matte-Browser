use std::collections::HashMap;
use crate::typography::{FontFace, FontFamily, FontWeight, FontStyle, FontStretch};

/// Unicode character properties
#[derive(Debug, Clone, PartialEq)]
pub struct CharProperties {
    /// Unicode code point
    pub code_point: u32,
    /// Character category (letter, digit, punctuation, etc.)
    pub category: CharCategory,
    /// Bidirectional class
    pub bidi_class: BidiClass,
    /// Whether this character is a combining mark
    pub is_combining: bool,
    /// Whether this character is a whitespace
    pub is_whitespace: bool,
    /// Whether this character is a line break opportunity
    pub is_line_break_opportunity: bool,
}

/// Unicode character categories
#[derive(Debug, Clone, PartialEq)]
pub enum CharCategory {
    Letter,
    Digit,
    Punctuation,
    Symbol,
    Separator,
    Other,
}

/// Unicode bidirectional classes
#[derive(Debug, Clone, PartialEq)]
pub enum BidiClass {
    LeftToRight,
    RightToLeft,
    ArabicLetter,
    EuropeanNumber,
    ArabicNumber,
    EuropeanSeparator,
    CommonSeparator,
    ParagraphSeparator,
    SegmentSeparator,
    Whitespace,
    OtherNeutral,
}

/// Shaped glyph information
#[derive(Debug, Clone)]
pub struct ShapedGlyph {
    /// Unicode code point
    pub code_point: u32,
    /// Glyph ID in the font
    pub glyph_id: u16,
    /// X offset from previous glyph
    pub x_offset: f32,
    /// Y offset from previous glyph
    pub y_offset: f32,
    /// Advance width
    pub advance_width: f32,
    /// Whether this glyph is a ligature
    pub is_ligature: bool,
    /// Whether this glyph has kerning
    pub has_kerning: bool,
    /// Cluster start index
    pub cluster_start: usize,
    /// Cluster end index
    pub cluster_end: usize,
}

/// Text direction
#[derive(Debug, Clone, PartialEq)]
pub enum TextDirection {
    LeftToRight,
    RightToLeft,
    Auto,
}

/// Line break opportunity
#[derive(Debug, Clone)]
pub struct LineBreakOpportunity {
    /// Character index where break can occur
    pub index: usize,
    /// Break type
    pub break_type: LineBreakType,
}

/// Line break types
#[derive(Debug, Clone)]
pub enum LineBreakType {
    /// Mandatory break (newline)
    Mandatory,
    /// Allowed break
    Allowed,
    /// Prohibited break
    Prohibited,
}

/// Shaped text run
#[derive(Debug, Clone)]
pub struct ShapedTextRun {
    /// Font face used for this run
    pub font_face: FontFace,
    /// Shaped glyphs
    pub glyphs: Vec<ShapedGlyph>,
    /// Text direction
    pub direction: TextDirection,
    /// Start index in original text
    pub start_index: usize,
    /// End index in original text
    pub end_index: usize,
    /// Total width of the run
    pub width: f32,
    /// Total height of the run
    pub height: f32,
}

/// Text shaper for handling text layout
#[derive(Debug)]
pub struct TextShaper {
    /// Font cache for quick access
    font_cache: HashMap<(FontFamily, FontWeight, FontStyle, FontStretch), FontFace>,
    /// Kerning pairs cache
    kerning_cache: HashMap<(u16, u16), f32>,
    /// Ligature cache
    ligature_cache: HashMap<(u16, u16), u16>,
}

impl TextShaper {
    /// Create a new text shaper
    pub fn new() -> Self {
        Self {
            font_cache: HashMap::new(),
            kerning_cache: HashMap::new(),
            ligature_cache: HashMap::new(),
        }
    }
    
    /// Shape text into glyphs
    pub fn shape_text(&mut self, text: &str, font_face: &FontFace) -> Vec<ShapedGlyph> {
        let mut glyphs = Vec::new();
        let mut cluster_start = 0;
        
        for (i, char) in text.char_indices() {
            let code_point = char as u32;
            
            // Get character properties
            let properties = self.get_char_properties(code_point);
            
            // Create shaped glyph
            let glyph = ShapedGlyph {
                code_point,
                glyph_id: self.get_glyph_id(font_face, code_point),
                x_offset: 0.0,
                y_offset: 0.0,
                advance_width: self.get_advance_width(font_face, code_point),
                is_ligature: false,
                has_kerning: false,
                cluster_start,
                cluster_end: i + char.len_utf8(),
            };
            
            glyphs.push(glyph);
            cluster_start = i + char.len_utf8();
        }
        
        // Apply kerning
        self.apply_kerning(&mut glyphs, font_face);
        
        // Apply ligatures
        self.apply_ligatures(&mut glyphs, font_face);
        
        glyphs
    }
    
    /// Get character properties for a Unicode code point
    fn get_char_properties(&self, code_point: u32) -> CharProperties {
        // This is a simplified implementation
        // In a real browser, you would use Unicode data tables
        
        let category = match code_point {
            0x0030..=0x0039 => CharCategory::Digit,  // Digits
            0x0020 => CharCategory::Separator,       // Space
            0x000A => CharCategory::Separator,       // Line feed
            0x000D => CharCategory::Separator,       // Carriage return
            0x0020..=0x007F => CharCategory::Letter, // Basic Latin
            _ => CharCategory::Other,
        };
        
        let bidi_class = match code_point {
            0x0030..=0x0039 => BidiClass::EuropeanNumber, // Digits
            0x0020 => BidiClass::Whitespace,         // Space
            0x000A => BidiClass::ParagraphSeparator, // Line feed
            0x000D => BidiClass::ParagraphSeparator, // Carriage return
            0x0020..=0x007F => BidiClass::LeftToRight, // Basic Latin
            _ => BidiClass::OtherNeutral,
        };
        
        let is_combining = code_point >= 0x0300 && code_point <= 0x036F; // Combining diacritical marks
        let is_whitespace = code_point == 0x0020 || code_point == 0x000A || code_point == 0x000D;
        let is_line_break_opportunity = is_whitespace || code_point == 0x0020;
        
        CharProperties {
            code_point,
            category,
            bidi_class,
            is_combining,
            is_whitespace,
            is_line_break_opportunity,
        }
    }
    
    /// Get glyph ID for a code point
    fn get_glyph_id(&self, font_face: &FontFace, code_point: u32) -> u16 {
        // This is a simplified implementation
        // In a real browser, you would parse the font's cmap table
        code_point as u16
    }
    
    /// Get advance width for a code point
    fn get_advance_width(&self, font_face: &FontFace, code_point: u32) -> f32 {
        // This is a simplified implementation
        // In a real browser, you would use the font's hmtx table
        match code_point {
            0x0020 => 500.0, // Space
            0x000A => 0.0,   // Line feed
            0x000D => 0.0,   // Carriage return
            _ => 1000.0,     // Default width
        }
    }
    
    /// Apply kerning to glyphs
    fn apply_kerning(&mut self, glyphs: &mut Vec<ShapedGlyph>, font_face: &FontFace) {
        for i in 1..glyphs.len() {
            let prev_glyph_id = glyphs[i - 1].glyph_id;
            let curr_glyph_id = glyphs[i].glyph_id;
            
            let kerning_key = (prev_glyph_id, curr_glyph_id);
            if let Some(&kerning_value) = self.kerning_cache.get(&kerning_key) {
                glyphs[i].x_offset += kerning_value;
                glyphs[i].has_kerning = true;
            }
        }
    }
    
    /// Apply ligatures to glyphs
    fn apply_ligatures(&mut self, glyphs: &mut Vec<ShapedGlyph>, font_face: &FontFace) {
        // This is a simplified implementation
        // In a real browser, you would use the font's GSUB table
        
        let mut i = 0;
        while i < glyphs.len() - 1 {
            let curr_glyph_id = glyphs[i].glyph_id;
            let next_glyph_id = glyphs[i + 1].glyph_id;
            
            let ligature_key = (curr_glyph_id, next_glyph_id);
            if let Some(&ligature_glyph_id) = self.ligature_cache.get(&ligature_key) {
                // Replace current glyph with ligature
                glyphs[i].glyph_id = ligature_glyph_id;
                glyphs[i].is_ligature = true;
                glyphs[i].cluster_end = glyphs[i + 1].cluster_end;
                
                // Remove next glyph
                glyphs.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }
    
    /// Find line break opportunities
    pub fn find_line_breaks(&self, text: &str) -> Vec<LineBreakOpportunity> {
        let mut breaks = Vec::new();
        
        for (i, char) in text.char_indices() {
            let code_point = char as u32;
            let properties = self.get_char_properties(code_point);
            
            if properties.is_line_break_opportunity {
                let break_type = if code_point == 0x000A || code_point == 0x000D {
                    LineBreakType::Mandatory
                } else {
                    LineBreakType::Allowed
                };
                
                breaks.push(LineBreakOpportunity {
                    index: i,
                    break_type,
                });
            }
        }
        
        breaks
    }
    
    /// Determine text direction
    pub fn determine_text_direction(&self, text: &str) -> TextDirection {
        // This is a simplified implementation
        // In a real browser, you would use the Unicode Bidirectional Algorithm
        
        for char in text.chars() {
            let code_point = char as u32;
            let properties = self.get_char_properties(code_point);
            
            match properties.bidi_class {
                BidiClass::RightToLeft | BidiClass::ArabicLetter => {
                    return TextDirection::RightToLeft;
                }
                BidiClass::LeftToRight => {
                    return TextDirection::LeftToRight;
                }
                _ => continue,
            }
        }
        
        TextDirection::LeftToRight // Default
    }
    
    /// Create text runs with proper direction
    pub fn create_text_runs(
        &mut self,
        text: &str,
        font_face: &FontFace,
    ) -> Vec<ShapedTextRun> {
        let direction = self.determine_text_direction(text);
        let glyphs = self.shape_text(text, font_face);
        
        let width = glyphs.iter().map(|g| g.advance_width + g.x_offset).sum();
        vec![ShapedTextRun {
            font_face: font_face.clone(),
            glyphs,
            direction,
            start_index: 0,
            end_index: text.len(),
            width,
            height: font_face.line_height(),
        }]
    }
    
    /// Add kerning pair to cache
    pub fn add_kerning_pair(&mut self, glyph1: u16, glyph2: u16, kerning: f32) {
        self.kerning_cache.insert((glyph1, glyph2), kerning);
    }
    
    /// Add ligature to cache
    pub fn add_ligature(&mut self, glyph1: u16, glyph2: u16, ligature_glyph: u16) {
        self.ligature_cache.insert((glyph1, glyph2), ligature_glyph);
    }
    
    /// Get cache statistics
    pub fn get_cache_stats(&self) -> (usize, usize) {
        (self.kerning_cache.len(), self.ligature_cache.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_properties() {
        let shaper = TextShaper::new();
        
        // Test space character
        let space_props = shaper.get_char_properties(0x0020);
        assert_eq!(space_props.category, CharCategory::Separator);
        assert_eq!(space_props.bidi_class, BidiClass::Whitespace);
        assert!(space_props.is_whitespace);
        assert!(space_props.is_line_break_opportunity);
        
        // Test letter
        let letter_props = shaper.get_char_properties(0x0041); // 'A'
        assert_eq!(letter_props.category, CharCategory::Letter);
        assert_eq!(letter_props.bidi_class, BidiClass::LeftToRight);
        assert!(!letter_props.is_whitespace);
        assert!(!letter_props.is_line_break_opportunity);
    }

    #[test]
    fn test_text_direction() {
        let shaper = TextShaper::new();
        
        // Test LTR text
        let ltr_text = "Hello World";
        assert_eq!(shaper.determine_text_direction(ltr_text), TextDirection::LeftToRight);
        
        // Test RTL text (simplified)
        let rtl_text = "مرحبا بالعالم";
        assert_eq!(shaper.determine_text_direction(rtl_text), TextDirection::LeftToRight); // Simplified implementation
    }

    #[test]
    fn test_line_breaks() {
        let shaper = TextShaper::new();
        
        let text = "Hello\nWorld";
        let breaks = shaper.find_line_breaks(text);
        
        assert_eq!(breaks.len(), 1);
        assert_eq!(breaks[0].index, 5); // Position of '\n'
        assert!(matches!(breaks[0].break_type, LineBreakType::Mandatory));
    }

    #[test]
    fn test_text_shaping() {
        let mut shaper = TextShaper::new();
        let font_face = FontFace::new(
            FontFamily("Arial".to_string()),
            FontWeight(400),
            FontStyle::Normal,
            FontStretch::Normal,
        );
        
        let text = "Hello";
        let glyphs = shaper.shape_text(text, &font_face);
        
        assert_eq!(glyphs.len(), 5);
        assert_eq!(glyphs[0].code_point, 0x0048); // 'H'
        assert_eq!(glyphs[4].code_point, 0x006F); // 'o'
    }

    #[test]
    fn test_text_runs() {
        let mut shaper = TextShaper::new();
        let font_face = FontFace::new(
            FontFamily("Arial".to_string()),
            FontWeight(400),
            FontStyle::Normal,
            FontStretch::Normal,
        );
        
        let text = "Hello World";
        let runs = shaper.create_text_runs(text, &font_face);
        
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].glyphs.len(), 11);
        assert_eq!(runs[0].direction, TextDirection::LeftToRight);
        assert_eq!(runs[0].start_index, 0);
        assert_eq!(runs[0].end_index, text.len());
    }

    #[test]
    fn test_kerning_and_ligatures() {
        let mut shaper = TextShaper::new();
        
        // Add some test kerning
        shaper.add_kerning_pair(65, 86, -50.0); // A + V kerning
        shaper.add_ligature(102, 105, 1000); // f + i ligature
        
        let (kerning_count, ligature_count) = shaper.get_cache_stats();
        assert_eq!(kerning_count, 1);
        assert_eq!(ligature_count, 1);
    }
}
