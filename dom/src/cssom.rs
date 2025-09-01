//! CSSOM (CSS Object Model) implementation.
//! 
//! This module provides the CSS Object Model for managing CSS rules,
//! stylesheets, and computed values according to the CSS specification.

use crate::error::{Error, Result};
use crate::css_selector::SelectorList;
use crate::css_at_rules::AtRule;

/// CSS rule types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CssRuleType {
    /// Style rule (e.g., `div { color: red; }`)
    Style,
    /// Media rule (e.g., `@media screen { ... }`)
    Media,
    /// Import rule (e.g., `@import url("style.css");`)
    Import,
    /// Font-face rule (e.g., `@font-face { ... }`)
    FontFace,
    /// Page rule (e.g., `@page { ... }`)
    Page,
    /// Keyframes rule (e.g., `@keyframes animation { ... }`)
    Keyframes,
    /// Namespace rule (e.g., `@namespace svg "http://www.w3.org/2000/svg";`)
    Namespace,
    /// Supports rule (e.g., `@supports (display: flex) { ... }`)
    Supports,
    /// Document rule (e.g., `@document url("...") { ... }`)
    Document,
    /// Viewport rule (e.g., `@viewport { ... }`)
    Viewport,
    /// Counter-style rule (e.g., `@counter-style { ... }`)
    CounterStyle,
    /// Font-feature-values rule (e.g., `@font-feature-values { ... }`)
    FontFeatureValues,
    /// Region-style rule (e.g., `@region-style { ... }`)
    RegionStyle,
}

/// CSS property value types
#[derive(Debug, Clone, PartialEq)]
pub enum CssValue {
    /// Keyword value (e.g., "auto", "none")
    Keyword(String),
    /// Number value (e.g., 12.5)
    Number(f64),
    /// Length value (e.g., "12px", "2em")
    Length(f64, String),
    /// Percentage value (e.g., "50%")
    Percentage(f64),
    /// Color value (e.g., "#ff0000", "rgb(255, 0, 0)")
    Color(String),
    /// String value (e.g., "Arial")
    String(String),
    /// URL value (e.g., "url(image.png)")
    Url(String),
    /// Function value (e.g., "calc(100% - 20px)")
    Function(String, Vec<CssValue>),
    /// List of values (e.g., "1px 2px 3px 4px")
    List(Vec<CssValue>),
    /// Initial value
    Initial,
    /// Inherit value
    Inherit,
    /// Unset value
    Unset,
    /// Revert value
    Revert,
}

impl CssValue {
    /// Check if the value is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(self, CssValue::Keyword(_))
    }
    
    /// Get the keyword value if this is a keyword
    pub fn as_keyword(&self) -> Option<&str> {
        if let CssValue::Keyword(kw) = self {
            Some(kw)
        } else {
            None
        }
    }
    
    /// Check if the value is a number
    pub fn is_number(&self) -> bool {
        matches!(self, CssValue::Number(_))
    }
    
    /// Get the number value if this is a number
    pub fn as_number(&self) -> Option<f64> {
        if let CssValue::Number(n) = self {
            Some(*n)
        } else {
            None
        }
    }
    
    /// Check if the value is a length
    pub fn is_length(&self) -> bool {
        matches!(self, CssValue::Length(_, _))
    }
    
    /// Get the length value and unit if this is a length
    pub fn as_length(&self) -> Option<(f64, &str)> {
        if let CssValue::Length(value, unit) = self {
            Some((*value, unit))
        } else {
            None
        }
    }
    
    /// Check if the value is a percentage
    pub fn is_percentage(&self) -> bool {
        matches!(self, CssValue::Percentage(_))
    }
    
    /// Get the percentage value if this is a percentage
    pub fn as_percentage(&self) -> Option<f64> {
        if let CssValue::Percentage(p) = self {
            Some(*p)
        } else {
            None
        }
    }
    
    /// Check if the value is a color
    pub fn is_color(&self) -> bool {
        matches!(self, CssValue::Color(_))
    }
    
    /// Get the color value if this is a color
    pub fn as_color(&self) -> Option<&str> {
        if let CssValue::Color(c) = self {
            Some(c)
        } else {
            None
        }
    }
}

/// CSS declaration (property-value pair)
#[derive(Debug, Clone, PartialEq)]
pub struct CssDeclaration {
    /// Property name
    pub property: String,
    /// Property value
    pub value: CssValue,
    /// Whether the declaration is important
    pub important: bool,
}

impl CssDeclaration {
    /// Create a new CSS declaration
    pub fn new(property: String, value: CssValue, important: bool) -> Self {
        Self {
            property,
            value,
            important,
        }
    }
    
    /// Get the property name
    pub fn property(&self) -> &str {
        &self.property
    }
    
    /// Get the property value
    pub fn value(&self) -> &CssValue {
        &self.value
    }
    
    /// Check if the declaration is important
    pub fn is_important(&self) -> bool {
        self.important
    }
}

/// CSS style rule
#[derive(Debug, Clone, PartialEq)]
pub struct CssStyleRule {
    /// Selector list for this rule
    pub selectors: SelectorList,
    /// Declarations in this rule
    pub declarations: Vec<CssDeclaration>,
    /// Rule type
    pub rule_type: CssRuleType,
}

impl CssStyleRule {
    /// Create a new CSS style rule
    pub fn new(selectors: SelectorList) -> Self {
        Self {
            selectors,
            declarations: Vec::new(),
            rule_type: CssRuleType::Style,
        }
    }
    
    /// Add a declaration to this rule
    pub fn add_declaration(&mut self, declaration: CssDeclaration) {
        self.declarations.push(declaration);
    }
    
    /// Get all declarations
    pub fn declarations(&self) -> &[CssDeclaration] {
        &self.declarations
    }
    
    /// Get a specific declaration by property name
    pub fn get_declaration(&self, property: &str) -> Option<&CssDeclaration> {
        self.declarations.iter().find(|d| d.property == property)
    }
    
    /// Get the value of a specific property
    pub fn get_property_value(&self, property: &str) -> Option<&CssValue> {
        self.get_declaration(property).map(|d| &d.value)
    }
    
    /// Set a property value
    pub fn set_property_value(&mut self, property: String, value: CssValue, important: bool) {
        // Remove existing declaration for this property
        self.declarations.retain(|d| d.property != property);
        
        // Add new declaration
        self.add_declaration(CssDeclaration::new(property, value, important));
    }
    
    /// Remove a property
    pub fn remove_property(&mut self, property: &str) -> Option<CssDeclaration> {
        if let Some(index) = self.declarations.iter().position(|d| d.property == property) {
            Some(self.declarations.remove(index))
        } else {
            None
        }
    }
}

/// CSS rule trait
pub trait CssRule {
    /// Get the rule type
    fn rule_type(&self) -> CssRuleType;
    
    /// Get the CSS text representation
    fn css_text(&self) -> String;
    
    /// Get the parent stylesheet
    fn parent_stylesheet(&self) -> Option<&CssStyleSheet>;
    
    /// Set the parent stylesheet
    fn set_parent_stylesheet(&mut self, stylesheet: &CssStyleSheet);
}

/// CSS rule enum that can represent both style rules and at-rules
#[derive(Debug, Clone, PartialEq)]
pub enum CssRuleVariant {
    /// Style rule
    StyleRule(CssStyleRule),
    /// At-rule
    AtRule(AtRule),
}

impl CssRule for CssStyleRule {
    fn rule_type(&self) -> CssRuleType {
        self.rule_type.clone()
    }
    
    fn css_text(&self) -> String {
        let mut css = String::new();
        
        // Add selectors
        for (i, _selector) in self.selectors.selectors.iter().enumerate() {
            if i > 0 {
                css.push_str(", ");
            }
            // For now, just add a placeholder for selector text
            css.push_str("selector");
        }
        
        css.push_str(" { ");
        
        // Add declarations
        for (i, declaration) in self.declarations.iter().enumerate() {
            if i > 0 {
                css.push_str("; ");
            }
            css.push_str(&declaration.property);
            css.push_str(": ");
            // For now, just add a placeholder for value text
            css.push_str("value");
            if declaration.important {
                css.push_str(" !important");
            }
        }
        
        css.push_str(" }");
        css
    }
    
    fn parent_stylesheet(&self) -> Option<&CssStyleSheet> {
        None // Placeholder
    }
    
    fn set_parent_stylesheet(&mut self, _stylesheet: &CssStyleSheet) {
        // Placeholder
    }
}

impl CssRuleVariant {
    /// Get the rule type
    pub fn rule_type(&self) -> CssRuleType {
        match self {
            CssRuleVariant::StyleRule(rule) => rule.rule_type(),
            CssRuleVariant::AtRule(at_rule) => match at_rule {
                AtRule::Import { .. } => CssRuleType::Import,
                AtRule::Media { .. } => CssRuleType::Media,
                AtRule::FontFace { .. } => CssRuleType::FontFace,
                AtRule::Keyframes { .. } => CssRuleType::Keyframes,
                AtRule::Page { .. } => CssRuleType::Page,
                AtRule::Supports { .. } => CssRuleType::Supports,
                AtRule::Charset { .. } => CssRuleType::Import, // Charset is handled like import
                AtRule::Namespace { .. } => CssRuleType::Namespace,
                AtRule::Viewport { .. } => CssRuleType::Viewport,
                AtRule::Document { .. } => CssRuleType::Document,
                AtRule::CounterStyle { .. } => CssRuleType::CounterStyle,
                AtRule::FontFeatureValues { .. } => CssRuleType::FontFeatureValues,
            },
        }
    }
    
    /// Get the CSS text representation
    pub fn css_text(&self) -> String {
        match self {
            CssRuleVariant::StyleRule(rule) => rule.css_text(),
            CssRuleVariant::AtRule(at_rule) => match at_rule {
                AtRule::Import { url, media_list } => {
                    let mut css = format!("@import url('{}')", url);
                    if !media_list.is_empty() {
                        css.push_str(&format!(" {}", media_list.join(", ")));
                    }
                    css.push(';');
                    css
                }
                AtRule::Media { media_query, rules } => {
                    let mut css = format!("@media {} {{", media_query);
                    for rule in rules {
                        css.push_str(&format!(" {}", rule.css_text()));
                    }
                    css.push_str(" }");
                    css
                }
                AtRule::FontFace { declarations } => {
                    let mut css = "@font-face {".to_string();
                    for (property, value) in declarations {
                        css.push_str(&format!(" {}: {};", property, value));
                    }
                    css.push_str(" }");
                    css
                }
                AtRule::Keyframes { name, keyframes } => {
                    let mut css = format!("@keyframes {} {{", name);
                    for keyframe in keyframes {
                        css.push_str(&format!(" {} {{", keyframe.selectors.join(", ")));
                        for (property, value) in &keyframe.declarations {
                            css.push_str(&format!(" {}: {};", property, value));
                        }
                        css.push_str(" }");
                    }
                    css.push_str(" }");
                    css
                }
                AtRule::Page { selector, declarations } => {
                    let mut css = if selector.is_empty() {
                        "@page {".to_string()
                    } else {
                        format!("@page {} {{", selector)
                    };
                    for (property, value) in declarations {
                        css.push_str(&format!(" {}: {};", property, value));
                    }
                    css.push_str(" }");
                    css
                }
                AtRule::Supports { condition, rules } => {
                    let mut css = format!("@supports {} {{", condition);
                    for rule in rules {
                        css.push_str(&format!(" {}", rule.css_text()));
                    }
                    css.push_str(" }");
                    css
                }
                AtRule::Charset { encoding } => {
                    format!("@charset \"{}\";", encoding)
                }
                AtRule::Namespace { prefix, uri } => {
                    if let Some(prefix) = prefix {
                        format!("@namespace {} \"{}\";", prefix, uri)
                    } else {
                        format!("@namespace \"{}\";", uri)
                    }
                }
                AtRule::Viewport { declarations } => {
                    let mut css = "@viewport {".to_string();
                    for (property, value) in declarations {
                        css.push_str(&format!(" {}: {};", property, value));
                    }
                    css.push_str(" }");
                    css
                }
                AtRule::Document { url_pattern, rules } => {
                    let mut css = format!("@document url(\"{}\") {{", url_pattern);
                    for rule in rules {
                        css.push_str(&format!(" {}", rule.css_text()));
                    }
                    css.push_str(" }");
                    css
                }
                AtRule::CounterStyle { name, declarations } => {
                    let mut css = format!("@counter-style {} {{", name);
                    for (property, value) in declarations {
                        css.push_str(&format!(" {}: {};", property, value));
                    }
                    css.push_str(" }");
                    css
                }
                AtRule::FontFeatureValues { font_family, feature_values } => {
                    let mut css = format!("@font-feature-values {} {{", font_family);
                    for (feature, values) in feature_values {
                        css.push_str(&format!(" @{} {{", feature));
                        css.push_str(&format!(" {};", values.join(", ")));
                        css.push_str(" }");
                    }
                    css.push_str(" }");
                    css
                }
            },
        }
    }
}

/// CSS stylesheet
pub struct CssStyleSheet {
    /// Rules in this stylesheet
    pub rules: Vec<CssRuleVariant>,
    /// Whether the stylesheet is disabled
    pub disabled: bool,
    /// Href of the stylesheet (if external)
    pub href: Option<String>,
    /// Title of the stylesheet
    pub title: Option<String>,
    /// Media list
    pub media: Vec<String>,
}

impl CssStyleSheet {
    /// Create a new CSS stylesheet
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            disabled: false,
            href: None,
            title: None,
            media: Vec::new(),
        }
    }
    
    /// Add a rule to the stylesheet
    pub fn add_rule(&mut self, rule: CssRuleVariant) {
        self.rules.push(rule);
    }
    
    /// Add an at-rule to the stylesheet
    pub fn add_at_rule(&mut self, at_rule: AtRule) {
        self.rules.push(CssRuleVariant::AtRule(at_rule));
    }
    
    /// Insert a rule at a specific index
    pub fn insert_rule(&mut self, rule: CssRuleVariant, index: usize) -> Result<()> {
        if index > self.rules.len() {
            return Err(Error::ConfigError("Index out of bounds".to_string()));
        }
        self.rules.insert(index, rule);
        Ok(())
    }
    
    /// Remove a rule at a specific index
    pub fn remove_rule(&mut self, index: usize) -> Result<CssRuleVariant> {
        if index >= self.rules.len() {
            return Err(Error::ConfigError("Index out of bounds".to_string()));
        }
        Ok(self.rules.remove(index))
    }
    
    /// Get a rule at a specific index
    pub fn get_rule(&self, index: usize) -> Option<&CssRuleVariant> {
        self.rules.get(index)
    }
    
    /// Get the number of rules
    pub fn length(&self) -> usize {
        self.rules.len()
    }
    
    /// Get all rules
    pub fn rules(&self) -> &[CssRuleVariant] {
        &self.rules
    }
    
    /// Check if the stylesheet is disabled
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }
    
    /// Set the disabled state
    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }
    
    /// Get the href
    pub fn href(&self) -> Option<&str> {
        self.href.as_deref()
    }
    
    /// Set the href
    pub fn set_href(&mut self, href: Option<String>) {
        self.href = href;
    }
    
    /// Get the title
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }
    
    /// Set the title
    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }
    
    /// Get the media list
    pub fn media(&self) -> &[String] {
        &self.media
    }
    
    /// Add a media query
    pub fn add_media(&mut self, media: String) {
        self.media.push(media);
    }
}

/// CSS computed value
#[derive(Debug, Clone, PartialEq)]
pub struct ComputedValue {
    /// The computed value
    pub value: CssValue,
    /// Whether the value is inherited
    pub inherited: bool,
    /// Whether the value is specified
    pub specified: bool,
}

impl ComputedValue {
    /// Create a new computed value
    pub fn new(value: CssValue, inherited: bool, specified: bool) -> Self {
        Self {
            value,
            inherited,
            specified,
        }
    }
    
    /// Get the computed value
    pub fn value(&self) -> &CssValue {
        &self.value
    }
    
    /// Check if the value is inherited
    pub fn is_inherited(&self) -> bool {
        self.inherited
    }
    
    /// Check if the value is specified
    pub fn is_specified(&self) -> bool {
        self.specified
    }
}

/// CSS cascade manager
pub struct CssCascade {
    /// Stylesheets in cascade order
    stylesheets: Vec<CssStyleSheet>,
}

impl CssCascade {
    /// Create a new CSS cascade manager
    pub fn new() -> Self {
        Self {
            stylesheets: Vec::new(),
        }
    }
    
    /// Add a stylesheet to the cascade
    pub fn add_stylesheet(&mut self, stylesheet: CssStyleSheet) {
        self.stylesheets.push(stylesheet);
    }
    
    /// Get all stylesheets
    pub fn stylesheets(&self) -> &[CssStyleSheet] {
        &self.stylesheets
    }
    
    /// Compute the final value for a property on an element
    pub fn compute_property_value(&self, _element: &str, _property: &str) -> Option<ComputedValue> {
        // This is a placeholder implementation
        // In a real implementation, this would:
        // 1. Find all matching rules for the element
        // 2. Sort them by specificity and order
        // 3. Apply inheritance and cascade
        // 4. Return the final computed value
        None
    }
    
    /// Get all matching rules for an element
    pub fn get_matching_rules(&self, _element: &str) -> Vec<&CssStyleRule> {
        // This is a placeholder implementation
        // In a real implementation, this would:
        // 1. Iterate through all stylesheets
        // 2. Find rules that match the element
        // 3. Return them sorted by specificity
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css_selector::CssSelectorParser;

    #[test]
    fn test_css_declaration_creation() {
        let declaration = CssDeclaration::new(
            "color".to_string(),
            CssValue::Color("#ff0000".to_string()),
            false,
        );
        
        assert_eq!(declaration.property(), "color");
        assert!(declaration.value().is_color());
        assert!(!declaration.is_important());
    }

    #[test]
    fn test_css_style_rule_creation() {
        let mut parser = CssSelectorParser::new("div").unwrap();
        let selector_list = parser.parse_selector_list().unwrap();
        
        let mut rule = CssStyleRule::new(selector_list);
        
        let declaration = CssDeclaration::new(
            "color".to_string(),
            CssValue::Color("#ff0000".to_string()),
            false,
        );
        
        rule.add_declaration(declaration);
        
        assert_eq!(rule.declarations().len(), 1);
        assert!(rule.get_property_value("color").is_some());
    }

    #[test]
    fn test_css_stylesheet_creation() {
        let mut stylesheet = CssStyleSheet::new();
        
        assert_eq!(stylesheet.length(), 0);
        assert!(!stylesheet.is_disabled());
        
        stylesheet.set_disabled(true);
        assert!(stylesheet.is_disabled());
    }

    #[test]
    fn test_css_value_types() {
        let keyword = CssValue::Keyword("auto".to_string());
        let number = CssValue::Number(12.5);
        let length = CssValue::Length(10.0, "px".to_string());
        let percentage = CssValue::Percentage(50.0);
        let color = CssValue::Color("#ff0000".to_string());
        
        assert!(keyword.is_keyword());
        assert_eq!(keyword.as_keyword(), Some("auto"));
        
        assert!(number.is_number());
        assert_eq!(number.as_number(), Some(12.5));
        
        assert!(length.is_length());
        assert_eq!(length.as_length(), Some((10.0, "px")));
        
        assert!(percentage.is_percentage());
        assert_eq!(percentage.as_percentage(), Some(50.0));
        
        assert!(color.is_color());
        assert_eq!(color.as_color(), Some("#ff0000"));
    }

    #[test]
    fn test_css_cascade_creation() {
        let cascade = CssCascade::new();
        
        assert_eq!(cascade.stylesheets().len(), 0);
    }
}
