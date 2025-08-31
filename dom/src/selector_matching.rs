//! CSS Selector Matching implementation.
//! 
//! This module provides efficient CSS selector matching algorithms
//! including O(N) matching, bloom-filter ancestor hints, and selector
//! optimization techniques.

use std::collections::{HashMap, HashSet};
use tracing::debug;
use crate::css_selector::{ComplexSelector, ComplexSelectorPart, SimpleSelector, Specificity, PseudoClass, PseudoElement, AttributeSelector, AttributeOperator};
use crate::dom::Element;
use crate::pseudo_classes::PseudoClassEvaluator;

/// Selector matching result
#[derive(Debug, Clone, PartialEq)]
pub struct MatchResult {
    /// Whether the selector matches
    pub matches: bool,
    /// Specificity of the match
    pub specificity: Specificity,
    /// Matching element
    pub element: String,
}

/// Bloom filter for ancestor hints
#[derive(Debug, Clone)]
pub struct AncestorBloomFilter {
    /// Bloom filter bits
    bits: Vec<bool>,
    /// Number of hash functions
    hash_count: usize,
    /// Filter size
    size: usize,
}

impl AncestorBloomFilter {
    /// Create a new bloom filter
    pub fn new(size: usize, hash_count: usize) -> Self {
        Self {
            bits: vec![false; size],
            hash_count,
            size,
        }
    }
    
    /// Add an element to the bloom filter
    pub fn add(&mut self, element: &str) {
        for i in 0..self.hash_count {
            let hash = self.hash(element, i);
            self.bits[hash % self.size] = true;
        }
    }
    
    /// Check if an element might be in the bloom filter
    pub fn might_contain(&self, element: &str) -> bool {
        for i in 0..self.hash_count {
            let hash = self.hash(element, i);
            if !self.bits[hash % self.size] {
                return false;
            }
        }
        true
    }
    
    /// Hash function for bloom filter
    fn hash(&self, element: &str, seed: usize) -> usize {
        let mut hash = seed as u64;
        for byte in element.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        hash as usize
    }
}

/// Selector matcher for efficient CSS selector matching
pub struct SelectorMatcher {
    /// Index of selectors by tag name
    tag_index: HashMap<String, Vec<ComplexSelector>>,
    /// Index of selectors by class name
    class_index: HashMap<String, Vec<ComplexSelector>>,
    /// Index of selectors by ID
    id_index: HashMap<String, Vec<ComplexSelector>>,
    /// All selectors (for fallback matching)
    all_selectors: Vec<ComplexSelector>,
    /// Bloom filter for ancestor hints
    bloom_filter: AncestorBloomFilter,
    /// Pseudo-class evaluator
    pseudo_class_evaluator: PseudoClassEvaluator,
}

impl SelectorMatcher {
    /// Create a new selector matcher
    pub fn new() -> Self {
        Self {
            tag_index: HashMap::new(),
            class_index: HashMap::new(),
            id_index: HashMap::new(),
            all_selectors: Vec::new(),
            bloom_filter: AncestorBloomFilter::new(1024, 3),
            pseudo_class_evaluator: PseudoClassEvaluator::new(),
        }
    }
    
    /// Add a selector to the matcher
    pub fn add_selector(&mut self, selector: ComplexSelector) {
        // Index by tag name
        if let Some(tag_name) = self.extract_tag_name(&selector) {
            self.tag_index.entry(tag_name).or_insert_with(Vec::new).push(selector.clone());
        }
        
        // Index by class names
        for class_name in self.extract_class_names(&selector) {
            self.class_index.entry(class_name).or_insert_with(Vec::new).push(selector.clone());
        }
        
        // Index by ID
        if let Some(id) = self.extract_id(&selector) {
            self.id_index.entry(id).or_insert_with(Vec::new).push(selector.clone());
        }
        
        // Add to all selectors
        self.all_selectors.push(selector);
    }
    
    /// Match an element against all selectors
    pub fn match_element(&self, element: &Element) -> Vec<MatchResult> {
        let mut results = Vec::new();
        
        // Get candidate selectors based on element properties
        let candidates = self.get_candidate_selectors(element);
        
        // Test each candidate selector
        for selector in candidates {
            if self.matches_selector(element, selector) {
                let specificity = selector.specificity();
                results.push(MatchResult {
                    matches: true,
                    specificity,
                    element: element.id.clone(),
                });
            }
        }
        
        // Sort by specificity (highest first)
        results.sort_by(|a, b| b.specificity.cmp(&a.specificity));
        
        debug!("Matched {} selectors for element {}", results.len(), element.id);
        results
    }
    
    /// Get candidate selectors for an element
    fn get_candidate_selectors(&self, element: &Element) -> Vec<&ComplexSelector> {
        let mut candidates = HashSet::new();
        
        // Get selectors by tag name
        if let Some(selectors) = self.tag_index.get(&element.tag_name) {
            for selector in selectors {
                candidates.insert(selector);
            }
        }
        
        // Get selectors by class names
        for class_name in self.get_element_classes(element) {
            if let Some(selectors) = self.class_index.get(class_name) {
                for selector in selectors {
                    candidates.insert(selector);
                }
            }
        }
        
        // Get selectors by ID
        if let Some(id) = self.get_element_id(element) {
            if let Some(selectors) = self.id_index.get(id) {
                for selector in selectors {
                    candidates.insert(selector);
                }
            }
        }
        
        candidates.into_iter().collect()
    }
    
    /// Check if a selector matches an element
    fn matches_selector(&self, element: &Element, selector: &ComplexSelector) -> bool {
        // For now, implement a simple matching algorithm
        // In a real implementation, this would handle complex selectors with combinators
        
        if selector.parts.is_empty() {
            return false;
        }
        
        // Check if the first part (compound selector) matches
        if let Some(ComplexSelectorPart::Compound(compound)) = selector.parts.first() {
            self.matches_simple_selector(element, &compound.simple)
        } else {
            false
        }
    }
    
    /// Check if a simple selector matches an element
    fn matches_simple_selector(&self, element: &Element, selector: &SimpleSelector) -> bool {
        // Check element type
        if let Some(ref selector_type) = selector.element_type {
            if selector_type != &element.tag_name {
                return false;
            }
        }
        
        // Check ID
        if let Some(ref selector_id) = selector.id {
            if let Some(element_id) = self.get_element_id(element) {
                if selector_id != element_id {
                    return false;
                }
            } else {
                return false;
            }
        }
        
        // Check classes
        for selector_class in &selector.classes {
            let element_classes = self.get_element_classes(element);
            if !element_classes.contains(&selector_class.as_str()) {
                return false;
            }
        }
        
        // Check attributes
        for attr_selector in &selector.attributes {
            if !self.matches_attribute_selector(element, attr_selector) {
                return false;
            }
        }
        
        // Check pseudo-classes
        for pseudo_class in &selector.pseudo_classes {
            if !self.matches_pseudo_class(element, pseudo_class) {
                return false;
            }
        }
        
        // Check pseudo-elements
        for pseudo_element in &selector.pseudo_elements {
            if !self.matches_pseudo_element(element, pseudo_element) {
                return false;
            }
        }
        
        true
    }
    
    /// Check if an attribute selector matches an element
    fn matches_attribute_selector(&self, element: &Element, attr_selector: &AttributeSelector) -> bool {
        let element_value = element.attributes.get(&attr_selector.name);
        
        match attr_selector.operator {
            AttributeOperator::Exists => element_value.is_some(),
            AttributeOperator::Equals => {
                if let Some(ref selector_value) = attr_selector.value {
                    if let Some(element_val) = element_value {
                        element_val == selector_value
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            AttributeOperator::ContainsWord => {
                if let Some(ref selector_value) = attr_selector.value {
                    if let Some(element_val) = element_value {
                        element_val.split_whitespace().any(|word| word == selector_value)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            AttributeOperator::StartsWith => {
                if let Some(ref selector_value) = attr_selector.value {
                    if let Some(element_val) = element_value {
                        element_val.starts_with(selector_value)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            AttributeOperator::StartsWithPrefix => {
                if let Some(ref selector_value) = attr_selector.value {
                    if let Some(element_val) = element_value {
                        element_val.starts_with(selector_value) && 
                        (element_val.len() == selector_value.len() || 
                         element_val.chars().nth(selector_value.len()) == Some('-'))
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            AttributeOperator::EndsWith => {
                if let Some(ref selector_value) = attr_selector.value {
                    if let Some(element_val) = element_value {
                        element_val.ends_with(selector_value)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            AttributeOperator::Contains => {
                if let Some(ref selector_value) = attr_selector.value {
                    if let Some(element_val) = element_value {
                        element_val.contains(selector_value)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        }
    }
    
    /// Check if a pseudo-class matches an element
    fn matches_pseudo_class(&self, element: &Element, pseudo_class: &PseudoClass) -> bool {
        self.pseudo_class_evaluator.evaluate_pseudo_class(element, pseudo_class)
    }
    
    /// Check if a pseudo-element matches an element
    fn matches_pseudo_element(&self, _element: &Element, _pseudo_element: &PseudoElement) -> bool {
        // This is a placeholder implementation
        // In a real implementation, this would check pseudo-element conditions
        true
    }
    
    /// Extract tag name from a selector
    fn extract_tag_name(&self, selector: &ComplexSelector) -> Option<String> {
        if let Some(ComplexSelectorPart::Compound(compound)) = selector.parts.first() {
            compound.simple.element_type.clone()
        } else {
            None
        }
    }
    
    /// Extract class names from a selector
    fn extract_class_names(&self, selector: &ComplexSelector) -> Vec<String> {
        if let Some(ComplexSelectorPart::Compound(compound)) = selector.parts.first() {
            compound.simple.classes.clone()
        } else {
            Vec::new()
        }
    }
    
    /// Extract ID from a selector
    fn extract_id(&self, selector: &ComplexSelector) -> Option<String> {
        if let Some(ComplexSelectorPart::Compound(compound)) = selector.parts.first() {
            compound.simple.id.clone()
        } else {
            None
        }
    }
    
    /// Get element classes
    fn get_element_classes<'a>(&self, element: &'a Element) -> Vec<&'a str> {
        if let Some(class_attr) = element.attributes.get("class") {
            class_attr.split_whitespace().collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get element ID
    fn get_element_id<'a>(&self, element: &'a Element) -> Option<&'a str> {
        element.attributes.get("id").map(|s| s.as_str())
    }
    
    /// Get the pseudo-class evaluator
    pub fn pseudo_class_evaluator(&self) -> &PseudoClassEvaluator {
        &self.pseudo_class_evaluator
    }
    
    /// Get the pseudo-class evaluator mutably
    pub fn pseudo_class_evaluator_mut(&mut self) -> &mut PseudoClassEvaluator {
        &mut self.pseudo_class_evaluator
    }
}

/// Fast path selector matcher for common selectors
pub struct FastPathMatcher {
    /// Universal selector matcher
    universal_matcher: Box<dyn Fn(&Element) -> bool + Send + Sync>,
    /// Tag name matcher
    tag_matcher: Box<dyn Fn(&Element) -> bool + Send + Sync>,
    /// Class matcher
    class_matcher: Box<dyn Fn(&Element) -> bool + Send + Sync>,
    /// ID matcher
    id_matcher: Box<dyn Fn(&Element) -> bool + Send + Sync>,
}

impl FastPathMatcher {
    /// Create a new fast path matcher
    pub fn new() -> Self {
        Self {
            universal_matcher: Box::new(|_| false),
            tag_matcher: Box::new(|_| false),
            class_matcher: Box::new(|_| false),
            id_matcher: Box::new(|_| false),
        }
    }
    
    /// Set universal matcher
    pub fn set_universal_matcher<F>(&mut self, matcher: F)
    where
        F: Fn(&Element) -> bool + Send + Sync + 'static,
    {
        self.universal_matcher = Box::new(matcher);
    }
    
    /// Set tag matcher
    pub fn set_tag_matcher<F>(&mut self, matcher: F)
    where
        F: Fn(&Element) -> bool + Send + Sync + 'static,
    {
        self.tag_matcher = Box::new(matcher);
    }
    
    /// Set class matcher
    pub fn set_class_matcher<F>(&mut self, matcher: F)
    where
        F: Fn(&Element) -> bool + Send + Sync + 'static,
    {
        self.class_matcher = Box::new(matcher);
    }
    
    /// Set ID matcher
    pub fn set_id_matcher<F>(&mut self, matcher: F)
    where
        F: Fn(&Element) -> bool + Send + Sync + 'static,
    {
        self.id_matcher = Box::new(matcher);
    }
    
    /// Match an element using fast paths
    pub fn match_element(&self, element: &Element) -> bool {
        (self.universal_matcher)(element) ||
        (self.tag_matcher)(element) ||
        (self.class_matcher)(element) ||
        (self.id_matcher)(element)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css_selector::CssSelectorParser;

    #[test]
    fn test_selector_matcher_creation() {
        let matcher = SelectorMatcher::new();
        assert!(matcher.tag_index.is_empty());
        assert!(matcher.class_index.is_empty());
        assert!(matcher.id_index.is_empty());
    }

    #[test]
    fn test_bloom_filter() {
        let mut filter = AncestorBloomFilter::new(100, 3);
        
        filter.add("div");
        filter.add("span");
        
        assert!(filter.might_contain("div"));
        assert!(filter.might_contain("span"));
        assert!(!filter.might_contain("p"));
    }

    #[test]
    fn test_fast_path_matcher() {
        let mut matcher = FastPathMatcher::new();
        
        matcher.set_tag_matcher(|element| element.tag_name == "div");
        
        let div_element = Element::new("div".to_string());
        let span_element = Element::new("span".to_string());
        
        assert!(matcher.match_element(&div_element));
        assert!(!matcher.match_element(&span_element));
    }

    #[test]
    fn test_selector_matching() {
        let mut matcher = SelectorMatcher::new();
        
        // Add a selector
        let mut parser = CssSelectorParser::new("div.container").unwrap();
        let selector_list = parser.parse_selector_list().unwrap();
        
        for selector in selector_list.selectors {
            matcher.add_selector(selector);
        }
        
        // Create a matching element
        let mut element = Element::new("div".to_string());
        element.attributes.insert("class".to_string(), "container".to_string());
        
        let results = matcher.match_element(&element);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_id_selector_matching() {
        let mut matcher = SelectorMatcher::new();
        
        // Add an ID selector
        let mut parser = CssSelectorParser::new("#main").unwrap();
        let selector_list = parser.parse_selector_list().unwrap();
        
        for selector in selector_list.selectors {
            matcher.add_selector(selector);
        }
        
        // Create a matching element
        let mut element = Element::new("div".to_string());
        element.attributes.insert("id".to_string(), "main".to_string());
        
        let results = matcher.match_element(&element);
        assert!(!results.is_empty());
    }
}
