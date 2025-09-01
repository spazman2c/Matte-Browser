use crate::css_tokenizer::{CssTokenizer, CssToken};
use crate::cssom::{CssRuleVariant, CssStyleSheet};
use crate::error::Result;
use std::collections::HashMap;

/// Represents different types of CSS at-rules
#[derive(Debug, Clone, PartialEq)]
pub enum AtRule {
    /// @import rule
    Import {
        url: String,
        media_list: Vec<String>,
    },
    /// @media rule
    Media {
        media_query: String,
        rules: Vec<CssRuleVariant>,
    },
    /// @font-face rule
    FontFace {
        declarations: HashMap<String, String>,
    },
    /// @keyframes rule
    Keyframes {
        name: String,
        keyframes: Vec<KeyframeRule>,
    },
    /// @page rule
    Page {
        selector: String,
        declarations: HashMap<String, String>,
    },
    /// @supports rule
    Supports {
        condition: String,
        rules: Vec<CssRuleVariant>,
    },
    /// @charset rule
    Charset {
        encoding: String,
    },
    /// @namespace rule
    Namespace {
        prefix: Option<String>,
        uri: String,
    },
    /// @viewport rule (deprecated but still supported)
    Viewport {
        declarations: HashMap<String, String>,
    },
    /// @document rule (deprecated)
    Document {
        url_pattern: String,
        rules: Vec<CssRuleVariant>,
    },
    /// @counter-style rule
    CounterStyle {
        name: String,
        declarations: HashMap<String, String>,
    },
    /// @font-feature-values rule
    FontFeatureValues {
        font_family: String,
        feature_values: HashMap<String, Vec<String>>,
    },
}

/// Represents a keyframe rule within @keyframes
#[derive(Debug, Clone, PartialEq)]
pub struct KeyframeRule {
    /// Keyframe selectors (e.g., "0%", "50%", "100%")
    pub selectors: Vec<String>,
    /// CSS declarations for this keyframe
    pub declarations: HashMap<String, String>,
}

/// Parser for CSS at-rules
pub struct AtRuleParser {
    /// Current position in the token stream
    position: usize,
    /// Tokens to parse
    tokens: Vec<CssToken>,
}

impl AtRuleParser {
    /// Create a new at-rule parser
    pub fn new() -> Self {
        Self {
            position: 0,
            tokens: Vec::new(),
        }
    }

    /// Parse an at-rule from CSS text
    pub fn parse_at_rule(&mut self, input: &str) -> Result<AtRule> {
        let mut tokenizer = CssTokenizer::new(input);
        self.tokens = tokenizer.tokenize()?;
        self.position = 0;
        
        // Expect @ symbol and get the rule name
        let rule_name = self.expect_at_symbol()?;
        
        match rule_name.as_str() {
            "import" => self.parse_import_rule(),
            "media" => self.parse_media_rule(),
            "font-face" => self.parse_font_face_rule(),
            "keyframes" => self.parse_keyframes_rule(),
            "page" => self.parse_page_rule(),
            "supports" => self.parse_supports_rule(),
            "charset" => self.parse_charset_rule(),
            "namespace" => self.parse_namespace_rule(),
            "viewport" => self.parse_viewport_rule(),
            "document" => self.parse_document_rule(),
            "counter-style" => self.parse_counter_style_rule(),
            "font-feature-values" => self.parse_font_feature_values_rule(),
            _ => Err(crate::error::Error::ParseError(format!("Unknown at-rule: @{}", rule_name))),
        }
    }

    /// Expect and consume an @ symbol, returning the rule name
    fn expect_at_symbol(&mut self) -> Result<String> {
        if self.position >= self.tokens.len() {
            return Err(crate::error::Error::ParseError("Unexpected end of input".to_string()));
        }
        
        match &self.tokens[self.position] {
            CssToken::AtKeyword(name) => {
                self.position += 1;
                Ok(name.clone())
            }
            _ => Err(crate::error::Error::ParseError("Expected @ symbol".to_string())),
        }
    }

    /// Parse an identifier
    fn parse_identifier(&mut self) -> Result<String> {
        if self.position >= self.tokens.len() {
            return Err(crate::error::Error::ParseError("Unexpected end of input".to_string()));
        }
        
        match &self.tokens[self.position] {
            CssToken::Ident(name) => {
                self.position += 1;
                Ok(name.clone())
            }
            _ => Err(crate::error::Error::ParseError("Expected identifier".to_string())),
        }
    }

    /// Parse @import rule
    fn parse_import_rule(&mut self) -> Result<AtRule> {
        // Parse URL or string
        let url = self.parse_url_or_string()?;
        
        // Parse optional media list
        let media_list = self.parse_media_list()?;
        
        // Expect semicolon
        self.expect_semicolon()?;
        
        Ok(AtRule::Import { url, media_list })
    }

    /// Parse @media rule
    fn parse_media_rule(&mut self) -> Result<AtRule> {
        // Parse media query
        let media_query = self.parse_media_query()?;
        
        // Expect opening brace
        self.expect_brace('{')?;
        
        // Parse rules inside media block
        let rules = self.parse_rule_list()?;
        
        // Expect closing brace
        self.expect_brace('}')?;
        
        Ok(AtRule::Media { media_query, rules })
    }

    /// Parse @font-face rule
    fn parse_font_face_rule(&mut self) -> Result<AtRule> {
        // Expect opening brace
        self.expect_brace('{')?;
        
        // Parse declarations
        let declarations = self.parse_declaration_list()?;
        
        // Expect closing brace
        self.expect_brace('}')?;
        
        Ok(AtRule::FontFace { declarations })
    }

    /// Parse @keyframes rule
    fn parse_keyframes_rule(&mut self) -> Result<AtRule> {
        // Parse keyframes name
        let name = self.parse_identifier()?;
        
        // Expect opening brace
        self.expect_brace('{')?;
        
        // Parse keyframe rules
        let keyframes = self.parse_keyframe_list()?;
        
        // Expect closing brace
        self.expect_brace('}')?;
        
        Ok(AtRule::Keyframes { name, keyframes })
    }

    /// Parse @page rule
    fn parse_page_rule(&mut self) -> Result<AtRule> {
        // Parse optional page selector
        let selector = self.parse_page_selector()?;
        
        // Expect opening brace
        self.expect_brace('{')?;
        
        // Parse declarations
        let declarations = self.parse_declaration_list()?;
        
        // Expect closing brace
        self.expect_brace('}')?;
        
        Ok(AtRule::Page { selector, declarations })
    }

    /// Parse @supports rule
    fn parse_supports_rule(&mut self) -> Result<AtRule> {
        // Parse supports condition
        let condition = self.parse_supports_condition()?;
        
        // Expect opening brace
        self.expect_brace('{')?;
        
        // Parse rules inside supports block
        let rules = self.parse_rule_list()?;
        
        // Expect closing brace
        self.expect_brace('}')?;
        
        Ok(AtRule::Supports { condition, rules })
    }

    /// Parse @charset rule
    fn parse_charset_rule(&mut self) -> Result<AtRule> {
        // Parse encoding string
        let encoding = self.parse_string()?;
        
        // Expect semicolon
        self.expect_semicolon()?;
        
        Ok(AtRule::Charset { encoding })
    }

    /// Parse @namespace rule
    fn parse_namespace_rule(&mut self) -> Result<AtRule> {
        // Parse optional prefix
        let prefix = if self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                CssToken::Ident(_) => Some(self.parse_identifier()?),
                _ => None,
            }
        } else {
            None
        };
        
        // Parse URI
        let uri = self.parse_url_or_string()?;
        
        // Expect semicolon
        self.expect_semicolon()?;
        
        Ok(AtRule::Namespace { prefix, uri })
    }

    /// Parse @viewport rule
    fn parse_viewport_rule(&mut self) -> Result<AtRule> {
        // Expect opening brace
        self.expect_brace('{')?;
        
        // Parse declarations
        let declarations = self.parse_declaration_list()?;
        
        // Expect closing brace
        self.expect_brace('}')?;
        
        Ok(AtRule::Viewport { declarations })
    }

    /// Parse @document rule
    fn parse_document_rule(&mut self) -> Result<AtRule> {
        // Parse URL pattern
        let url_pattern = self.parse_url_pattern()?;
        
        // Expect opening brace
        self.expect_brace('{')?;
        
        // Parse rules inside document block
        let rules = self.parse_rule_list()?;
        
        // Expect closing brace
        self.expect_brace('}')?;
        
        Ok(AtRule::Document { url_pattern, rules })
    }

    /// Parse @counter-style rule
    fn parse_counter_style_rule(&mut self) -> Result<AtRule> {
        // Parse counter style name
        let name = self.parse_identifier()?;
        
        // Expect opening brace
        self.expect_brace('{')?;
        
        // Parse declarations
        let declarations = self.parse_declaration_list()?;
        
        // Expect closing brace
        self.expect_brace('}')?;
        
        Ok(AtRule::CounterStyle { name, declarations })
    }

    /// Parse @font-feature-values rule
    fn parse_font_feature_values_rule(&mut self) -> Result<AtRule> {
        // Parse font family
        let font_family = self.parse_identifier()?;
        
        // Expect opening brace
        self.expect_brace('{')?;
        
        // Parse feature values
        let feature_values = self.parse_feature_values()?;
        
        // Expect closing brace
        self.expect_brace('}')?;
        
        Ok(AtRule::FontFeatureValues { font_family, feature_values })
    }

    /// Parse URL or string
    fn parse_url_or_string(&mut self) -> Result<String> {
        if self.position >= self.tokens.len() {
            return Err(crate::error::Error::ParseError("Unexpected end of input".to_string()));
        }
        
        match &self.tokens[self.position] {
            CssToken::Url(url) => {
                self.position += 1;
                Ok(url.clone())
            }
            CssToken::String(s) => {
                self.position += 1;
                Ok(s.clone())
            }
            CssToken::Function(name) if name == "url" => {
                // Parse url() function
                self.position += 1; // Skip the function token
                
                // Parse the URL string (the Function token already includes the opening parenthesis)
                let url = if self.position < self.tokens.len() {
                    match &self.tokens[self.position] {
                        CssToken::String(s) => {
                            self.position += 1;
                            s.clone()
                        }
                        _ => {
                            return Err(crate::error::Error::ParseError("Expected string in url() function".to_string()));
                        }
                    }
                } else {
                    return Err(crate::error::Error::ParseError("Unexpected end of input".to_string()));
                };
                
                // Expect closing parenthesis
                if self.position < self.tokens.len() {
                    match &self.tokens[self.position] {
                        CssToken::RightParen => {
                            self.position += 1;
                        }
                        _ => {
                            return Err(crate::error::Error::ParseError("Expected closing parenthesis".to_string()));
                        }
                    }
                }
                
                Ok(url)
            }
            _ => Err(crate::error::Error::ParseError("Expected URL or string".to_string())),
        }
    }

    /// Parse string
    fn parse_string(&mut self) -> Result<String> {
        if self.position >= self.tokens.len() {
            return Err(crate::error::Error::ParseError("Unexpected end of input".to_string()));
        }
        
        match &self.tokens[self.position] {
            CssToken::String(s) => {
                self.position += 1;
                Ok(s.clone())
            }
            _ => Err(crate::error::Error::ParseError("Expected string".to_string())),
        }
    }

    /// Parse media list
    fn parse_media_list(&mut self) -> Result<Vec<String>> {
        let mut media_list = Vec::new();
        
        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                CssToken::Ident(name) => {
                    media_list.push(name.clone());
                    self.position += 1;
                }
                CssToken::Delim(',') => {
                    self.position += 1;
                }
                _ => break,
            }
        }
        
        Ok(media_list)
    }

    /// Parse media query
    fn parse_media_query(&mut self) -> Result<String> {
        let mut query = String::new();
        
        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                CssToken::Delim('{') => break,
                _ => {
                    query.push_str(&self.tokens[self.position].to_string());
                    self.position += 1;
                }
            }
        }
        
        Ok(query.trim().to_string())
    }

    /// Parse rule list
    fn parse_rule_list(&mut self) -> Result<Vec<CssRuleVariant>> {
        let mut rules = Vec::new();
        
        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                CssToken::Delim('}') => break,
                _ => {
                    // For now, we'll skip individual rules and just consume tokens
                    // until we find a closing brace or another at-rule
                    if let CssToken::AtKeyword(_) = &self.tokens[self.position] {
                        // Parse at-rule
                        let at_rule = self.parse_at_rule_inline()?;
                        rules.push(CssRuleVariant::AtRule(at_rule));
                    } else {
                        // Skip non-at-rule tokens for now
                        self.position += 1;
                    }
                }
            }
        }
        
        Ok(rules)
    }

    /// Parse at-rule inline (when already inside a rule list)
    fn parse_at_rule_inline(&mut self) -> Result<AtRule> {
        // Get the at-rule name from the current AtKeyword token
        let rule_name = if let CssToken::AtKeyword(name) = &self.tokens[self.position] {
            name.clone()
        } else {
            return Err(crate::error::Error::ParseError("Expected @ symbol".to_string()));
        };
        
        // Skip the @ symbol
        self.position += 1;
        
        match rule_name.as_str() {
            "import" => self.parse_import_rule(),
            "media" => self.parse_media_rule(),
            "font-face" => self.parse_font_face_rule(),
            "keyframes" => self.parse_keyframes_rule(),
            "page" => self.parse_page_rule(),
            "supports" => self.parse_supports_rule(),
            "charset" => self.parse_charset_rule(),
            "namespace" => self.parse_namespace_rule(),
            "viewport" => self.parse_viewport_rule(),
            "document" => self.parse_document_rule(),
            "counter-style" => self.parse_counter_style_rule(),
            "font-feature-values" => self.parse_font_feature_values_rule(),
            _ => Err(crate::error::Error::ParseError(format!("Unknown at-rule: @{}", rule_name))),
        }
    }

    /// Parse declaration list
    fn parse_declaration_list(&mut self) -> Result<HashMap<String, String>> {
        let mut declarations = HashMap::new();
        
        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                CssToken::Delim('}') => break,
                CssToken::Ident(property) => {
                    let property_name = property.clone();
                    self.position += 1;
                    
                    // Expect colon
                    if self.position < self.tokens.len() {
                        match &self.tokens[self.position] {
                            CssToken::Delim(':') => {
                                self.position += 1;
                            }
                            _ => {
                                return Err(crate::error::Error::ParseError("Expected colon".to_string()));
                            }
                        }
                    }
                    
                    // Parse value
                    let value = self.parse_declaration_value()?;
                    declarations.insert(property_name, value);
                    
                    // Expect semicolon
                    self.expect_semicolon()?;
                }
                _ => {
                    self.position += 1;
                }
            }
        }
        
        Ok(declarations)
    }

    /// Parse declaration value
    fn parse_declaration_value(&mut self) -> Result<String> {
        let mut value = String::new();
        
        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                CssToken::Delim(';') | CssToken::Delim('}') => break,
                _ => {
                    value.push_str(&self.tokens[self.position].to_string());
                    self.position += 1;
                }
            }
        }
        
        Ok(value.trim().to_string())
    }

    /// Parse keyframe list
    fn parse_keyframe_list(&mut self) -> Result<Vec<KeyframeRule>> {
        let mut keyframes = Vec::new();
        
        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                CssToken::Delim('}') => break,
                CssToken::Percentage(_) | CssToken::Ident(_) => {
                    // Parse keyframe selectors
                    let selectors = self.parse_keyframe_selectors()?;
                    
                    // Expect opening brace
                    self.expect_brace('{')?;
                    
                    // Parse declarations
                    let declarations = self.parse_declaration_list()?;
                    
                    // Expect closing brace
                    self.expect_brace('}')?;
                    
                    keyframes.push(KeyframeRule { selectors, declarations });
                }
                _ => {
                    self.position += 1;
                }
            }
        }
        
        Ok(keyframes)
    }

    /// Parse keyframe selectors
    fn parse_keyframe_selectors(&mut self) -> Result<Vec<String>> {
        let mut selectors = Vec::new();
        
        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                CssToken::Delim('{') => break,
                CssToken::Percentage(p) => {
                    selectors.push(format!("{}%", p));
                    self.position += 1;
                }
                CssToken::Ident(name) => {
                    selectors.push(name.clone());
                    self.position += 1;
                }
                CssToken::Delim(',') => {
                    self.position += 1;
                }
                _ => break,
            }
        }
        
        Ok(selectors)
    }

    /// Parse page selector
    fn parse_page_selector(&mut self) -> Result<String> {
        if self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                CssToken::Ident(name) => {
                    self.position += 1;
                    Ok(name.clone())
                }
                CssToken::Delim('{') => Ok("".to_string()),
                _ => Ok("".to_string()),
            }
        } else {
            Ok("".to_string())
        }
    }

    /// Parse supports condition
    fn parse_supports_condition(&mut self) -> Result<String> {
        let mut condition = String::new();
        
        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                CssToken::Delim('{') => break,
                _ => {
                    condition.push_str(&self.tokens[self.position].to_string());
                    self.position += 1;
                }
            }
        }
        
        Ok(condition.trim().to_string())
    }

    /// Parse URL pattern
    fn parse_url_pattern(&mut self) -> Result<String> {
        let mut pattern = String::new();
        
        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                CssToken::Delim('{') => break,
                _ => {
                    pattern.push_str(&self.tokens[self.position].to_string());
                    self.position += 1;
                }
            }
        }
        
        Ok(pattern.trim().to_string())
    }

    /// Parse feature values
    fn parse_feature_values(&mut self) -> Result<HashMap<String, Vec<String>>> {
        let mut feature_values = HashMap::new();
        
        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                CssToken::Delim('}') => break,
                CssToken::Ident(feature) => {
                    let feature_name = feature.clone();
                    self.position += 1;
                    
                    // Expect opening brace
                    self.expect_brace('{')?;
                    
                    // Parse values
                    let values = self.parse_feature_value_list()?;
                    
                    // Expect closing brace
                    self.expect_brace('}')?;
                    
                    feature_values.insert(feature_name, values);
                }
                _ => {
                    self.position += 1;
                }
            }
        }
        
        Ok(feature_values)
    }

    /// Parse feature value list
    fn parse_feature_value_list(&mut self) -> Result<Vec<String>> {
        let mut values = Vec::new();
        
        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                CssToken::Delim('}') => break,
                CssToken::Ident(name) => {
                    values.push(name.clone());
                    self.position += 1;
                }
                CssToken::Delim(',') => {
                    self.position += 1;
                }
                _ => {
                    self.position += 1;
                }
            }
        }
        
        Ok(values)
    }

    /// Expect and consume a semicolon
    fn expect_semicolon(&mut self) -> Result<()> {
        if self.position >= self.tokens.len() {
            return Err(crate::error::Error::ParseError("Unexpected end of input".to_string()));
        }
        
        match &self.tokens[self.position] {
            CssToken::Delim(';') | CssToken::Semicolon => {
                self.position += 1;
                Ok(())
            }
            _ => Err(crate::error::Error::ParseError("Expected semicolon".to_string())),
        }
    }

    /// Expect and consume a brace
    fn expect_brace(&mut self, brace: char) -> Result<()> {
        if self.position >= self.tokens.len() {
            return Err(crate::error::Error::ParseError("Unexpected end of input".to_string()));
        }
        
        match &self.tokens[self.position] {
            CssToken::Delim(c) if *c == brace => {
                self.position += 1;
                Ok(())
            }
            _ => Err(crate::error::Error::ParseError(format!("Expected {}", brace))),
        }
    }
}

/// Manager for handling at-rules in a stylesheet
pub struct AtRuleManager {
    /// Registered at-rule handlers
    handlers: HashMap<String, Box<dyn AtRuleHandler>>,
}

/// Trait for handling specific at-rules
pub trait AtRuleHandler {
    /// Process an at-rule
    fn process(&self, rule: &AtRule, stylesheet: &mut CssStyleSheet) -> Result<()>;
}

impl AtRuleManager {
    /// Create a new at-rule manager
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Register an at-rule handler
    pub fn register_handler(&mut self, rule_name: &str, handler: Box<dyn AtRuleHandler>) {
        self.handlers.insert(rule_name.to_string(), handler);
    }

    /// Process an at-rule
    pub fn process_at_rule(&self, rule: &AtRule, stylesheet: &mut CssStyleSheet) -> Result<()> {
        let rule_name = match rule {
            AtRule::Import { .. } => "import",
            AtRule::Media { .. } => "media",
            AtRule::FontFace { .. } => "font-face",
            AtRule::Keyframes { .. } => "keyframes",
            AtRule::Page { .. } => "page",
            AtRule::Supports { .. } => "supports",
            AtRule::Charset { .. } => "charset",
            AtRule::Namespace { .. } => "namespace",
            AtRule::Viewport { .. } => "viewport",
            AtRule::Document { .. } => "document",
            AtRule::CounterStyle { .. } => "counter-style",
            AtRule::FontFeatureValues { .. } => "font-feature-values",
        };

        if let Some(handler) = self.handlers.get(rule_name) {
            handler.process(rule, stylesheet)
        } else {
            // Default handling: add to stylesheet
            stylesheet.add_at_rule(rule.clone());
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_import_rule() {
        let mut parser = AtRuleParser::new();
        let result = parser.parse_at_rule("@import url('style.css');");
        assert!(result.is_ok());
        
        if let AtRule::Import { url, media_list } = result.unwrap() {
            assert_eq!(url, "style.css");
            assert!(media_list.is_empty());
        } else {
            panic!("Expected import rule");
        }
    }

    #[test]
    fn test_parse_media_rule() {
        let mut parser = AtRuleParser::new();
        let result = parser.parse_at_rule("@media screen and (max-width: 600px) { body { color: red; } }");
        assert!(result.is_ok());
        
        if let AtRule::Media { media_query, rules } = result.unwrap() {
            assert_eq!(media_query, "screen and (max-width: 600px)");
            // Rules parsing is simplified for now
        } else {
            panic!("Expected media rule");
        }
    }

    #[test]
    fn test_parse_font_face_rule() {
        let mut parser = AtRuleParser::new();
        let result = parser.parse_at_rule("@font-face { font-family: 'Arial'; src: url('arial.ttf'); }");
        assert!(result.is_ok());
        
        if let AtRule::FontFace { declarations } = result.unwrap() {
            assert!(declarations.contains_key("font-family"));
            assert!(declarations.contains_key("src"));
        } else {
            panic!("Expected font-face rule");
        }
    }

    #[test]
    fn test_parse_keyframes_rule() {
        let mut parser = AtRuleParser::new();
        let result = parser.parse_at_rule("@keyframes fade { 0% { opacity: 0; } 100% { opacity: 1; } }");
        assert!(result.is_ok());
        
        if let AtRule::Keyframes { name, keyframes } = result.unwrap() {
            assert_eq!(name, "fade");
            assert_eq!(keyframes.len(), 2);
        } else {
            panic!("Expected keyframes rule");
        }
    }

    #[test]
    fn test_parse_charset_rule() {
        let mut parser = AtRuleParser::new();
        let result = parser.parse_at_rule("@charset \"UTF-8\";");
        assert!(result.is_ok());
        
        if let AtRule::Charset { encoding } = result.unwrap() {
            assert_eq!(encoding, "UTF-8");
        } else {
            panic!("Expected charset rule");
        }
    }

    #[test]
    fn test_parse_namespace_rule() {
        let mut parser = AtRuleParser::new();
        let result = parser.parse_at_rule("@namespace svg \"http://www.w3.org/2000/svg\";");
        assert!(result.is_ok());
        
        if let AtRule::Namespace { prefix, uri } = result.unwrap() {
            assert_eq!(prefix, Some("svg".to_string()));
            assert_eq!(uri, "http://www.w3.org/2000/svg");
        } else {
            panic!("Expected namespace rule");
        }
    }
}
