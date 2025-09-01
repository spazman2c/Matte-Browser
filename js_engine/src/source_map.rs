use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Source map version
pub const SOURCE_MAP_VERSION: u32 = 3;

/// Source map for mapping compiled code back to original source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMap {
    /// Source map version (should be 3)
    pub version: u32,
    /// Optional name of the generated file
    pub file: Option<String>,
    /// Root path for resolving relative source paths
    pub source_root: Option<String>,
    /// List of source file paths
    pub sources: Vec<String>,
    /// List of source content (optional, for embedded source maps)
    pub sources_content: Option<Vec<Option<String>>>,
    /// List of symbol names
    pub names: Vec<String>,
    /// Encoded mapping data
    pub mappings: String,
}

impl SourceMap {
    /// Create a new source map
    pub fn new() -> Self {
        Self {
            version: SOURCE_MAP_VERSION,
            file: None,
            source_root: None,
            sources: Vec::new(),
            sources_content: None,
            names: Vec::new(),
            mappings: String::new(),
        }
    }

    /// Set the generated file name
    pub fn set_file(&mut self, file: String) {
        self.file = Some(file);
    }

    /// Set the source root path
    pub fn set_source_root(&mut self, source_root: String) {
        self.source_root = Some(source_root);
    }

    /// Add a source file
    pub fn add_source(&mut self, source: String) -> usize {
        let index = self.sources.len();
        self.sources.push(source);
        index
    }

    /// Add source content
    pub fn add_source_content(&mut self, content: Option<String>) {
        if self.sources_content.is_none() {
            self.sources_content = Some(Vec::new());
        }
        self.sources_content.as_mut().unwrap().push(content);
    }

    /// Add a symbol name
    pub fn add_name(&mut self, name: String) -> usize {
        let index = self.names.len();
        self.names.push(name);
        index
    }

    /// Set the mappings data
    pub fn set_mappings(&mut self, mappings: String) {
        self.mappings = mappings;
    }

    /// Generate the source map as JSON
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(|e| {
            Error::source_map(format!("Failed to serialize source map: {}", e))
        })
    }

    /// Parse a source map from JSON
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(|e| {
            Error::source_map(format!("Failed to parse source map: {}", e))
        })
    }
}

/// Source map generator for building source maps incrementally
pub struct SourceMapGenerator {
    source_map: SourceMap,
    mappings: Vec<Mapping>,
    current_line: i32,
    current_column: i32,
    current_source: i32,
    current_source_line: i32,
    current_source_column: i32,
    current_name: i32,
}

impl SourceMapGenerator {
    /// Create a new source map generator
    pub fn new() -> Self {
        Self {
            source_map: SourceMap::new(),
            mappings: Vec::new(),
            current_line: 0,
            current_column: 0,
            current_source: 0,
            current_source_line: 0,
            current_source_column: 0,
            current_name: 0,
        }
    }

    /// Add a source file
    pub fn add_source(&mut self, source: String) -> usize {
        self.source_map.add_source(source)
    }

    /// Add source content
    pub fn add_source_content(&mut self, content: Option<String>) {
        self.source_map.add_source_content(content);
    }

    /// Add a symbol name
    pub fn add_name(&mut self, name: String) -> usize {
        self.source_map.add_name(name)
    }

    /// Add a mapping
    pub fn add_mapping(
        &mut self,
        generated_line: i32,
        generated_column: i32,
        source: Option<i32>,
        source_line: Option<i32>,
        source_column: Option<i32>,
        name: Option<i32>,
    ) {
        let mapping = Mapping {
            generated_line,
            generated_column,
            source,
            source_line,
            source_column,
            name,
        };
        self.mappings.push(mapping);
    }

    /// Generate the final source map
    pub fn generate(&mut self) -> Result<SourceMap> {
        // Sort mappings by generated position
        self.mappings.sort_by(|a, b| {
            a.generated_line
                .cmp(&b.generated_line)
                .then(a.generated_column.cmp(&b.generated_column))
        });

        // Encode mappings
        let mappings = self.encode_mappings()?;
        self.source_map.set_mappings(mappings);

        Ok(self.source_map.clone())
    }

    /// Encode mappings using VLQ encoding
    fn encode_mappings(&self) -> Result<String> {
        let mut result = String::new();
        let mut current_line = 0;
        let mut current_column = 0;
        let mut current_source = 0;
        let mut current_source_line = 0;
        let mut current_source_column = 0;
        let mut current_name = 0;

        for mapping in &self.mappings {
            // Add line separators
            while current_line < mapping.generated_line {
                result.push(';');
                current_line += 1;
                current_column = 0;
            }

            if !result.is_empty() && !result.ends_with(';') {
                result.push(',');
            }

            // Encode generated column
            let column_diff = mapping.generated_column - current_column;
            result.push_str(&self.encode_vlq(column_diff)?);
            current_column = mapping.generated_column;

            // Encode source index
            if let Some(source) = mapping.source {
                let source_diff = source - current_source;
                result.push_str(&self.encode_vlq(source_diff)?);
                current_source = source;

                // Encode source line
                if let Some(source_line) = mapping.source_line {
                    let source_line_diff = source_line - current_source_line;
                    result.push_str(&self.encode_vlq(source_line_diff)?);
                    current_source_line = source_line;

                    // Encode source column
                    if let Some(source_column) = mapping.source_column {
                        let source_column_diff = source_column - current_source_column;
                        result.push_str(&self.encode_vlq(source_column_diff)?);
                        current_source_column = source_column;

                        // Encode name index
                        if let Some(name) = mapping.name {
                            let name_diff = name - current_name;
                            result.push_str(&self.encode_vlq(name_diff)?);
                            current_name = name;
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    /// Encode a value using VLQ (Variable Length Quantity) encoding
    fn encode_vlq(&self, mut value: i32) -> Result<String> {
        // Convert to zigzag encoding
        let zigzag = if value < 0 { (-value << 1) | 1 } else { value << 1 };

        let mut result = String::new();
        let mut remaining = zigzag;

        loop {
            let mut digit = remaining & 0x1F;
            remaining >>= 5;

            if remaining != 0 {
                digit |= 0x20;
            }

            let encoded = self.base64_encode(digit as u8)?;
            result.push(encoded);

            if remaining == 0 {
                break;
            }
        }

        Ok(result)
    }

    /// Encode a value to base64
    fn base64_encode(&self, value: u8) -> Result<char> {
        const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        
        if value >= 64 {
            return Err(Error::source_map(format!("Invalid base64 value: {}", value)));
        }

        Ok(BASE64_CHARS[value as usize] as char)
    }
}

/// Individual mapping entry
#[derive(Debug, Clone)]
struct Mapping {
    generated_line: i32,
    generated_column: i32,
    source: Option<i32>,
    source_line: Option<i32>,
    source_column: Option<i32>,
    name: Option<i32>,
}

/// Source map consumer for reading and using source maps
pub struct SourceMapConsumer {
    source_map: SourceMap,
    mappings: Vec<DecodedMapping>,
}

impl SourceMapConsumer {
    /// Create a new source map consumer
    pub fn new(source_map: SourceMap) -> Result<Self> {
        let mappings = Self::decode_mappings(&source_map.mappings)?;
        
        Ok(Self {
            source_map,
            mappings,
        })
    }

    /// Get the original position for a generated position
    pub fn original_position_for(&self, line: u32, column: u32) -> Option<OriginalPosition> {
        let line = line as i32;
        let column = column as i32;

        // Binary search for the mapping
        let mut left = 0;
        let mut right = self.mappings.len();

        while left < right {
            let mid = (left + right) / 2;
            let mapping = &self.mappings[mid];

            if mapping.generated_line < line || 
               (mapping.generated_line == line && mapping.generated_column < column) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        // Find the closest mapping
        if left > 0 {
            let mapping = &self.mappings[left - 1];
            if mapping.generated_line == line && mapping.generated_column <= column {
                return Some(OriginalPosition {
                    source: mapping.source.map(|i| self.source_map.sources[i as usize].clone()),
                    line: mapping.source_line.map(|l| l as u32),
                    column: mapping.source_column.map(|c| c as u32),
                    name: mapping.name.map(|i| self.source_map.names[i as usize].clone()),
                });
            }
        }

        None
    }

    /// Get the generated position for an original position
    pub fn generated_position_for(&self, source: &str, line: u32, column: u32) -> Option<GeneratedPosition> {
        let line = line as i32;
        let column = column as i32;

        // Find the source index
        let source_index = self.source_map.sources.iter().position(|s| s == source)? as i32;

        // Find the mapping
        for mapping in &self.mappings {
            if mapping.source == Some(source_index) && 
               mapping.source_line == Some(line) && 
               mapping.source_column == Some(column) {
                return Some(GeneratedPosition {
                    line: mapping.generated_line as u32,
                    column: mapping.generated_column as u32,
                });
            }
        }

        None
    }

    /// Decode the mappings string
    fn decode_mappings(mappings: &str) -> Result<Vec<DecodedMapping>> {
        let mut result = Vec::new();
        let mut current_line = 0;
        let mut current_column = 0;
        let mut current_source = 0;
        let mut current_source_line = 0;
        let mut current_source_column = 0;
        let mut current_name = 0;

        for line in mappings.split(';') {
            current_line += 1;
            current_column = 0;

            for segment in line.split(',') {
                if segment.is_empty() {
                    continue;
                }

                let values = Self::decode_vlq_segment(segment)?;
                if values.is_empty() {
                    continue;
                }

                let generated_column = current_column + values[0];
                current_column = generated_column;

                let mut mapping = DecodedMapping {
                    generated_line: current_line,
                    generated_column,
                    source: None,
                    source_line: None,
                    source_column: None,
                    name: None,
                };

                if values.len() > 1 {
                    let source = current_source + values[1];
                    current_source = source;
                    mapping.source = Some(source);

                    if values.len() > 2 {
                        let source_line = current_source_line + values[2];
                        current_source_line = source_line;
                        mapping.source_line = Some(source_line);

                        if values.len() > 3 {
                            let source_column = current_source_column + values[3];
                            current_source_column = source_column;
                            mapping.source_column = Some(source_column);

                            if values.len() > 4 {
                                let name = current_name + values[4];
                                current_name = name;
                                mapping.name = Some(name);
                            }
                        }
                    }
                }

                result.push(mapping);
            }
        }

        Ok(result)
    }

    /// Decode a VLQ segment
    fn decode_vlq_segment(segment: &str) -> Result<Vec<i32>> {
        let mut result = Vec::new();
        let mut value = 0;
        let mut shift = 0;

        for c in segment.chars() {
            let digit = Self::base64_decode(c)?;
            let has_more = (digit & 0x20) != 0;
            let digit_value = digit & 0x1F;

            value += (digit_value as i32) << shift;
            shift += 5;

            if !has_more {
                // Convert from zigzag encoding
                let zigzag = value;
                let decoded = if (zigzag & 1) != 0 {
                    -(zigzag >> 1)
                } else {
                    zigzag >> 1
                };
                result.push(decoded);
                value = 0;
                shift = 0;
            }
        }

        Ok(result)
    }

    /// Decode a base64 character
    fn base64_decode(c: char) -> Result<u8> {
        match c {
            'A'..='Z' => Ok((c as u8) - b'A'),
            'a'..='z' => Ok((c as u8) - b'a' + 26),
            '0'..='9' => Ok((c as u8) - b'0' + 52),
            '+' => Ok(62),
            '/' => Ok(63),
            _ => Err(Error::source_map(format!("Invalid base64 character: {}", c))),
        }
    }
}

/// Decoded mapping entry
#[derive(Debug, Clone)]
struct DecodedMapping {
    generated_line: i32,
    generated_column: i32,
    source: Option<i32>,
    source_line: Option<i32>,
    source_column: Option<i32>,
    name: Option<i32>,
}

/// Original position information
#[derive(Debug, Clone)]
pub struct OriginalPosition {
    pub source: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub name: Option<String>,
}

/// Generated position information
#[derive(Debug, Clone)]
pub struct GeneratedPosition {
    pub line: u32,
    pub column: u32,
}
