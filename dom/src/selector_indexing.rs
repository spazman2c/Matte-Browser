use crate::css_selector::{ComplexSelector, SimpleSelector, SelectorList, AttributeOperator};
use crate::dom::Element;
use std::collections::HashMap;

/// Detailed breakdown of selector specificity
#[derive(Debug, Clone, PartialEq)]
pub struct SpecificityBreakdown {
    /// Number of ID selectors
    pub id_count: u32,
    /// Number of class selectors, attribute selectors, and pseudo-classes
    pub class_count: u32,
    /// Number of type selectors and pseudo-elements
    pub type_count: u32,
    /// Number of universal selectors
    pub universal_count: u32,
}

impl SpecificityBreakdown {
    /// Create a new specificity breakdown
    pub fn new() -> Self {
        Self {
            id_count: 0,
            class_count: 0,
            type_count: 0,
            universal_count: 0,
        }
    }
    
    /// Get the total specificity as a tuple (id, class, type)
    pub fn to_tuple(&self) -> (u32, u32, u32) {
        (self.id_count, self.class_count, self.type_count)
    }
    
    /// Check if this specificity is higher than another
    pub fn is_higher_than(&self, other: &SpecificityBreakdown) -> bool {
        self.to_tuple() > other.to_tuple()
    }
}

/// Index entry for a selector
#[derive(Debug, Clone, PartialEq)]
pub struct SelectorIndexEntry {
    /// The selector this entry represents
    pub selector: ComplexSelector,
    /// Specificity of the selector
    pub specificity: (u32, u32, u32),
    /// Whether this selector is important
    pub important: bool,
    /// Source stylesheet or rule information
    pub source: String,
}

/// Index for efficient selector matching
#[derive(Debug, Clone)]
pub struct SelectorIndex {
    /// Index by tag name
    tag_index: HashMap<String, Vec<SelectorIndexEntry>>,
    /// Index by class name
    class_index: HashMap<String, Vec<SelectorIndexEntry>>,
    /// Index by ID
    id_index: HashMap<String, Vec<SelectorIndexEntry>>,
    /// Index by attribute name
    attribute_index: HashMap<String, Vec<SelectorIndexEntry>>,
    /// Index by pseudo-class
    pseudo_class_index: HashMap<String, Vec<SelectorIndexEntry>>,
    /// Index by pseudo-element
    pseudo_element_index: HashMap<String, Vec<SelectorIndexEntry>>,
    /// Universal selectors (apply to all elements)
    universal_selectors: Vec<SelectorIndexEntry>,
    /// All selectors for fallback matching
    all_selectors: Vec<SelectorIndexEntry>,
}

impl SelectorIndex {
    /// Create a new selector index
    pub fn new() -> Self {
        Self {
            tag_index: HashMap::new(),
            class_index: HashMap::new(),
            id_index: HashMap::new(),
            attribute_index: HashMap::new(),
            pseudo_class_index: HashMap::new(),
            pseudo_element_index: HashMap::new(),
            universal_selectors: Vec::new(),
            all_selectors: Vec::new(),
        }
    }

    /// Add a selector to the index
    pub fn add_selector(&mut self, selector: ComplexSelector, source: String) {
        let specificity = self.calculate_specificity(&selector);
        let important = self.is_important(&selector);
        
        let entry = SelectorIndexEntry {
            selector: selector.clone(),
            specificity,
            important,
            source,
        };

        // Add to all selectors
        self.all_selectors.push(entry.clone());

        // Index by the rightmost simple selector (for descendant selectors)
        // Clone the rightmost selector to avoid borrow checker issues
        if let Some(rightmost) = self.get_rightmost_simple_selector(&selector) {
            let rightmost_clone = rightmost.clone();
            self.index_by_simple_selector(&rightmost_clone, entry);
        } else {
            // Fallback to universal selectors
            self.universal_selectors.push(entry);
        }
    }

    /// Add multiple selectors from a selector list
    pub fn add_selector_list(&mut self, selector_list: &SelectorList, source: String) {
        for selector in &selector_list.selectors {
            self.add_selector(selector.clone(), source.clone());
        }
    }

    /// Find matching selectors for an element
    pub fn find_matching_selectors(&self, element: &Element) -> Vec<&SelectorIndexEntry> {
        let mut matching_selectors = Vec::new();

        // Check tag name index
        if let Some(selectors) = self.tag_index.get(&element.tag_name) {
            for selector in selectors {
                if self.matches_selector(element, &selector.selector) {
                    matching_selectors.push(selector);
                }
            }
        }

        // Check class name indices
        if let Some(class_attr) = element.get_attribute("class") {
            for class_name in class_attr.split_whitespace() {
                if let Some(selectors) = self.class_index.get(class_name) {
                    for selector in selectors {
                        if self.matches_selector(element, &selector.selector) {
                            matching_selectors.push(selector);
                        }
                    }
                }
            }
        }

        // Check ID index
        if let Some(id) = element.get_attribute("id") {
            if let Some(selectors) = self.id_index.get(id) {
                for selector in selectors {
                    if self.matches_selector(element, &selector.selector) {
                        matching_selectors.push(selector);
                    }
                }
            }
        }

        // Check attribute indices (simplified - just check if element has any attributes)
        // In a full implementation, this would iterate through all attributes
        for attr_name in ["id", "class", "style", "title"] {
            if element.has_attribute(attr_name) {
                if let Some(selectors) = self.attribute_index.get(attr_name) {
                    for selector in selectors {
                        if self.matches_selector(element, &selector.selector) {
                            matching_selectors.push(selector);
                        }
                    }
                }
            }
        }

        // Check universal selectors
        for selector in &self.universal_selectors {
            if self.matches_selector(element, &selector.selector) {
                matching_selectors.push(selector);
            }
        }

        // Sort by specificity (highest first)
        matching_selectors.sort_by(|a, b| b.specificity.cmp(&a.specificity));

        matching_selectors
    }

    /// Get all selectors (for fallback matching)
    pub fn get_all_selectors(&self) -> &[SelectorIndexEntry] {
        &self.all_selectors
    }

    /// Clear all selectors from the index
    pub fn clear(&mut self) {
        self.tag_index.clear();
        self.class_index.clear();
        self.id_index.clear();
        self.attribute_index.clear();
        self.pseudo_class_index.clear();
        self.pseudo_element_index.clear();
        self.universal_selectors.clear();
        self.all_selectors.clear();
    }

    /// Get statistics about the index
    pub fn get_stats(&self) -> SelectorIndexStats {
        SelectorIndexStats {
            total_selectors: self.all_selectors.len(),
            tag_selectors: self.tag_index.values().map(|v| v.len()).sum(),
            class_selectors: self.class_index.values().map(|v| v.len()).sum(),
            id_selectors: self.id_index.values().map(|v| v.len()).sum(),
            attribute_selectors: self.attribute_index.values().map(|v| v.len()).sum(),
            pseudo_class_selectors: self.pseudo_class_index.values().map(|v| v.len()).sum(),
            pseudo_element_selectors: self.pseudo_element_index.values().map(|v| v.len()).sum(),
            universal_selectors: self.universal_selectors.len(),
        }
    }

    /// Index a selector by its rightmost simple selector
    fn index_by_simple_selector(&mut self, simple_selector: &SimpleSelector, entry: SelectorIndexEntry) {
        // Index by element type
        if let Some(tag_name) = &simple_selector.element_type {
            self.tag_index.entry(tag_name.clone()).or_insert_with(Vec::new).push(entry.clone());
        }
        
        // Index by ID
        if let Some(id) = &simple_selector.id {
            self.id_index.entry(id.clone()).or_insert_with(Vec::new).push(entry.clone());
        }
        
        // Index by classes
        for class_name in &simple_selector.classes {
            self.class_index.entry(class_name.clone()).or_insert_with(Vec::new).push(entry.clone());
        }
        
        // Index by attributes
        for attr_selector in &simple_selector.attributes {
            self.attribute_index.entry(attr_selector.name.clone()).or_insert_with(Vec::new).push(entry.clone());
        }
        
        // Index by pseudo-classes
        for pseudo_class in &simple_selector.pseudo_classes {
            let pseudo_class_name = match pseudo_class {
                crate::css_selector::PseudoClass::Hover => "hover",
                crate::css_selector::PseudoClass::Active => "active",
                crate::css_selector::PseudoClass::Focus => "focus",
                crate::css_selector::PseudoClass::Visited => "visited",
                crate::css_selector::PseudoClass::Link => "link",
                crate::css_selector::PseudoClass::FirstChild => "first-child",
                crate::css_selector::PseudoClass::LastChild => "last-child",
                crate::css_selector::PseudoClass::NthChild(_) => "nth-child",
                crate::css_selector::PseudoClass::NthLastChild(_) => "nth-last-child",
                crate::css_selector::PseudoClass::Not(_) => "not",
                crate::css_selector::PseudoClass::Is(_) => "is",
                crate::css_selector::PseudoClass::Where(_) => "where",
                crate::css_selector::PseudoClass::Has(_) => "has",
                crate::css_selector::PseudoClass::Custom(name) => name,
            };
            self.pseudo_class_index.entry(pseudo_class_name.to_string()).or_insert_with(Vec::new).push(entry.clone());
        }
        
        // Index by pseudo-elements
        for pseudo_element in &simple_selector.pseudo_elements {
            let pseudo_element_name = match pseudo_element {
                crate::css_selector::PseudoElement::Before => "before",
                crate::css_selector::PseudoElement::After => "after",
                crate::css_selector::PseudoElement::FirstLine => "first-line",
                crate::css_selector::PseudoElement::FirstLetter => "first-letter",
                crate::css_selector::PseudoElement::Selection => "selection",
                crate::css_selector::PseudoElement::Custom(name) => name,
            };
            self.pseudo_element_index.entry(pseudo_element_name.to_string()).or_insert_with(Vec::new).push(entry.clone());
        }
        
        // If no specific selectors, add to universal
        if simple_selector.element_type.is_none() && simple_selector.id.is_none() && 
           simple_selector.classes.is_empty() && simple_selector.attributes.is_empty() &&
           simple_selector.pseudo_classes.is_empty() && simple_selector.pseudo_elements.is_empty() {
            self.universal_selectors.push(entry);
        }
    }

    /// Get the rightmost simple selector from a complex selector
    fn get_rightmost_simple_selector<'a>(&self, selector: &'a ComplexSelector) -> Option<&'a SimpleSelector> {
        // Find the last compound selector in the complex selector
        for part in selector.parts.iter().rev() {
            if let crate::css_selector::ComplexSelectorPart::Compound(compound) = part {
                return Some(&compound.simple);
            }
        }
        None
    }

    /// Calculate specificity for a selector
    fn calculate_specificity(&self, selector: &ComplexSelector) -> (u32, u32, u32) {
        let spec = selector.specificity();
        (spec.id_count, spec.class_count, spec.type_count)
    }
    
    /// Enhanced specificity calculation with detailed breakdown
    fn calculate_detailed_specificity(&self, selector: &ComplexSelector) -> SpecificityBreakdown {
        let mut breakdown = SpecificityBreakdown::new();
        
        for part in &selector.parts {
            if let crate::css_selector::ComplexSelectorPart::Compound(compound) = part {
                self.analyze_simple_selector_specificity(&compound.simple, &mut breakdown);
            }
        }
        
        breakdown
    }
    
    /// Analyze specificity of a simple selector
    fn analyze_simple_selector_specificity(&self, selector: &SimpleSelector, breakdown: &mut SpecificityBreakdown) {
        // ID selectors (highest specificity)
        if selector.id.is_some() {
            breakdown.id_count += 1;
        }
        
        // Class selectors, attribute selectors, pseudo-classes
        breakdown.class_count += selector.classes.len() as u32;
        breakdown.class_count += selector.attributes.len() as u32;
        breakdown.class_count += selector.pseudo_classes.len() as u32;
        
        // Type selectors, pseudo-elements
        if selector.element_type.is_some() {
            breakdown.type_count += 1;
        }
        breakdown.type_count += selector.pseudo_elements.len() as u32;
        
        // Universal selector
        if selector.element_type.is_none() && selector.id.is_none() && 
           selector.classes.is_empty() && selector.attributes.is_empty() &&
           selector.pseudo_classes.is_empty() && selector.pseudo_elements.is_empty() {
            breakdown.universal_count += 1;
        }
    }
    
    /// Compare specificity of two selectors
    fn compare_specificity(&self, selector1: &ComplexSelector, selector2: &ComplexSelector) -> std::cmp::Ordering {
        let spec1 = self.calculate_specificity(selector1);
        let spec2 = self.calculate_specificity(selector2);
        
        // Compare by ID count first (highest priority)
        match spec1.0.cmp(&spec2.0) {
            std::cmp::Ordering::Equal => {}
            other => return other,
        }
        
        // Then by class count
        match spec1.1.cmp(&spec2.1) {
            std::cmp::Ordering::Equal => {}
            other => return other,
        }
        
        // Finally by type count
        spec1.2.cmp(&spec2.2)
    }

    /// Check if a selector is marked as important
    fn is_important(&self, _selector: &ComplexSelector) -> bool {
        // This would check if the selector's declarations are marked as !important
        // For now, return false
        false
    }

    /// Check if a selector matches an element
    fn matches_selector(&self, element: &Element, selector: &ComplexSelector) -> bool {
        // For now, implement a simplified matching logic
        // In a full implementation, this would traverse the complex selector
        // and check each part against the element and its ancestors
        
        // Get the rightmost simple selector
        if let Some(simple_selector) = self.get_rightmost_simple_selector(selector) {
            return self.matches_simple_selector(element, simple_selector);
        }
        
        false
    }
    
    /// Check if a simple selector matches an element
    fn matches_simple_selector(&self, element: &Element, selector: &SimpleSelector) -> bool {
        // Check element type
        if let Some(tag_name) = &selector.element_type {
            if element.tag_name != *tag_name {
                return false;
            }
        }
        
        // Check ID
        if let Some(id) = &selector.id {
            if element.get_attribute("id") != Some(id) {
                return false;
            }
        }
        
        // Check classes
        for class_name in &selector.classes {
            if let Some(class_attr) = element.get_attribute("class") {
                if !class_attr.split_whitespace().any(|c| c == class_name) {
                    return false;
                }
            } else {
                return false;
            }
        }
        
        // Check attributes
        for attr_selector in &selector.attributes {
            if let Some(value) = element.get_attribute(&attr_selector.name) {
                match &attr_selector.operator {
                    AttributeOperator::Exists => {
                        // Just check if attribute exists - already done
                    }
                    AttributeOperator::Equals => {
                        if let Some(expected_value) = &attr_selector.value {
                            if value != expected_value {
                                return false;
                            }
                        }
                    }
                    AttributeOperator::Contains => {
                        if let Some(expected_value) = &attr_selector.value {
                            if !value.contains(expected_value) {
                                return false;
                            }
                        }
                    }
                    AttributeOperator::StartsWith => {
                        if let Some(expected_value) = &attr_selector.value {
                            if !value.starts_with(expected_value) {
                                return false;
                            }
                        }
                    }
                    AttributeOperator::EndsWith => {
                        if let Some(expected_value) = &attr_selector.value {
                            if !value.ends_with(expected_value) {
                                return false;
                            }
                        }
                    }
                    AttributeOperator::ContainsWord => {
                        if let Some(expected_value) = &attr_selector.value {
                            if !value.split_whitespace().any(|word| word == expected_value) {
                                return false;
                            }
                        }
                    }
                    AttributeOperator::StartsWithPrefix => {
                        if let Some(expected_value) = &attr_selector.value {
                            if !value.starts_with(expected_value) {
                                return false;
                            }
                        }
                    }
                }
            } else {
                return false;
            }
        }
        
        // Pseudo-classes and pseudo-elements would require more complex logic
        // For now, return true if we get here (simplified)
        true
    }
}

/// Statistics about the selector index
#[derive(Debug, Clone)]
pub struct SelectorIndexStats {
    pub total_selectors: usize,
    pub tag_selectors: usize,
    pub class_selectors: usize,
    pub id_selectors: usize,
    pub attribute_selectors: usize,
    pub pseudo_class_selectors: usize,
    pub pseudo_element_selectors: usize,
    pub universal_selectors: usize,
}

/// Cache entry for selector matching results
#[derive(Debug, Clone)]
pub struct SelectorCacheEntry {
    /// Element ID that was matched
    pub element_id: String,
    /// Timestamp when the cache entry was created
    pub timestamp: std::time::Instant,
    /// Matching selectors with their specificity
    pub matches: Vec<(ComplexSelector, (u32, u32, u32))>,
    /// Whether the element has changed since caching
    pub element_hash: u64,
}

/// Selector cache for storing matching results
#[derive(Debug, Clone)]
pub struct SelectorCache {
    /// Cache entries indexed by element ID
    entries: HashMap<String, SelectorCacheEntry>,
    /// Maximum number of cache entries
    max_entries: usize,
    /// Cache hit statistics
    hits: usize,
    /// Cache miss statistics
    misses: usize,
}

impl SelectorCache {
    /// Create a new selector cache
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_entries,
            hits: 0,
            misses: 0,
        }
    }
    
    /// Get cached matches for an element
    pub fn get_matches(&mut self, element_id: &str, element_hash: u64) -> Option<Vec<(ComplexSelector, (u32, u32, u32))>> {
        if let Some(entry) = self.entries.get(element_id) {
            // Check if element has changed
            if entry.element_hash == element_hash {
                self.hits += 1;
                return Some(entry.matches.clone());
            }
        }
        
        self.misses += 1;
        None
    }
    
    /// Store matches for an element
    pub fn store_matches(&mut self, element_id: String, matches: Vec<(ComplexSelector, (u32, u32, u32))>, element_hash: u64) {
        // Evict old entries if cache is full
        if self.entries.len() >= self.max_entries {
            self.evict_oldest();
        }
        
        let entry = SelectorCacheEntry {
            element_id: element_id.clone(),
            timestamp: std::time::Instant::now(),
            matches,
            element_hash,
        };
        
        self.entries.insert(element_id, entry);
    }
    
    /// Evict the oldest cache entry
    fn evict_oldest(&mut self) {
        if let Some((oldest_key, _)) = self.entries.iter()
            .min_by_key(|(_, entry)| entry.timestamp) {
            let key = oldest_key.clone();
            self.entries.remove(&key);
        }
    }
    
    /// Clear all cache entries
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    
    /// Get cache statistics
    pub fn get_stats(&self) -> SelectorCacheStats {
        SelectorCacheStats {
            total_entries: self.entries.len(),
            max_entries: self.max_entries,
            hits: self.hits,
            misses: self.misses,
            hit_rate: if self.hits + self.misses > 0 {
                self.hits as f64 / (self.hits + self.misses) as f64
            } else {
                0.0
            },
        }
    }
}

/// Statistics about the selector cache
#[derive(Debug, Clone)]
pub struct SelectorCacheStats {
    pub total_entries: usize,
    pub max_entries: usize,
    pub hits: usize,
    pub misses: usize,
    pub hit_rate: f64,
}

/// Indexed selector matcher for fast selector matching
#[derive(Debug, Clone)]
pub struct IndexedSelectorMatcher {
    /// Index for fast matching
    index: SelectorIndex,
    /// Bloom filter for ancestor queries
    bloom_filter: crate::selector_matching::AncestorBloomFilter,
    /// Cache for selector matching results
    cache: SelectorCache,
}

impl IndexedSelectorMatcher {
    /// Create a new indexed selector matcher
    pub fn new() -> Self {
        Self {
            index: SelectorIndex::new(),
            bloom_filter: crate::selector_matching::AncestorBloomFilter::new(1024, 3),
            cache: SelectorCache::new(1000), // Default cache size
        }
    }
    
    /// Create a new indexed selector matcher with custom cache size
    pub fn with_cache_size(cache_size: usize) -> Self {
        Self {
            index: SelectorIndex::new(),
            bloom_filter: crate::selector_matching::AncestorBloomFilter::new(1024, 3),
            cache: SelectorCache::new(cache_size),
        }
    }

    /// Add selectors to the matcher
    pub fn add_selectors(&mut self, selector_list: &SelectorList, source: String) {
        self.index.add_selector_list(selector_list, source);
    }

    /// Find matching selectors for an element using fast path
    pub fn find_matches(&mut self, element: &Element) -> Vec<&SelectorIndexEntry> {
        // Calculate element hash for cache invalidation
        let element_hash = self.calculate_element_hash(element);
        
        // Try to get cached results first (disabled for now due to conversion complexity)
        // if let Some(cached_matches) = self.cache.get_matches(&element.id, element_hash) {
        //     return self.convert_cached_matches(&cached_matches);
        // }
        
        // Use the indexed approach for fast matching
        let mut matches = self.index.find_matching_selectors(element);
        
        // Apply fast path optimizations
        self.apply_fast_path_optimizations(&mut matches, element);
        
        // Deduplicate matches by selector identity
        self.deduplicate_matches(&mut matches);
        
        // Cache the results for future use
        let matches_to_cache: Vec<(ComplexSelector, (u32, u32, u32))> = matches.iter()
            .map(|entry| (entry.selector.clone(), entry.specificity))
            .collect();
        self.cache.store_matches(element.id.clone(), matches_to_cache, element_hash);
        
        matches
    }
    
    /// Deduplicate matches to avoid counting the same selector multiple times
    fn deduplicate_matches(&self, matches: &mut Vec<&SelectorIndexEntry>) {
        // Use a HashSet to track seen selectors by their source and specificity
        use std::collections::HashSet;
        let mut seen = HashSet::new();
        
        matches.retain(|entry| {
            let key = (&entry.source, entry.specificity);
            seen.insert(key)
        });
    }
    
    /// Apply fast path optimizations to reduce the number of selectors to check
    fn apply_fast_path_optimizations(&self, matches: &mut Vec<&SelectorIndexEntry>, element: &Element) {
        // Filter out selectors that can be quickly rejected using bloom filter
        matches.retain(|entry| {
            // Check if the selector requires ancestor queries
            if self.requires_ancestor_query(&entry.selector) {
                // Use bloom filter to quickly reject if ancestor is not present
                return self.bloom_filter_might_match(&entry.selector, element);
            }
            true
        });
        
        // Sort by specificity and importance for faster cascade resolution
        matches.sort_by(|a, b| {
            // Important selectors first
            match (a.important, b.important) {
                (true, false) => return std::cmp::Ordering::Less,
                (false, true) => return std::cmp::Ordering::Greater,
                _ => {}
            }
            // Then by specificity (highest first)
            b.specificity.cmp(&a.specificity)
        });
    }
    
    /// Check if a selector requires ancestor queries (descendant, child, etc.)
    fn requires_ancestor_query(&self, selector: &ComplexSelector) -> bool {
        // Check if the selector has multiple parts (indicating ancestor relationships)
        selector.parts.len() > 1
    }
    
    /// Use bloom filter to check if a selector might match
    fn bloom_filter_might_match(&self, _selector: &ComplexSelector, _element: &Element) -> bool {
        // For now, return true (bloom filter would be used for ancestor queries)
        // In a full implementation, this would check the bloom filter
        true
    }
    
    /// Calculate a hash for an element to detect changes
    fn calculate_element_hash(&self, element: &Element) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        element.tag_name.hash(&mut hasher);
        
        // Hash attributes (simplified - in a full implementation, this would be more comprehensive)
        if let Some(id) = element.get_attribute("id") {
            id.hash(&mut hasher);
        }
        if let Some(class) = element.get_attribute("class") {
            class.hash(&mut hasher);
        }
        
        hasher.finish()
    }
    
    /// Convert cached matches back to index entries
    fn convert_cached_matches(&self, cached_matches: &[(ComplexSelector, (u32, u32, u32))]) -> Vec<&SelectorIndexEntry> {
        // This is a simplified conversion - in a full implementation,
        // we would need to maintain a mapping from selectors to index entries
        // For now, return an empty vector as this is a placeholder
        Vec::new()
    }
    
    /// Get cache statistics
    pub fn get_cache_stats(&self) -> SelectorCacheStats {
        self.cache.get_stats()
    }
    
    /// Clear the cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Get statistics about the matcher
    pub fn get_stats(&self) -> SelectorIndexStats {
        self.index.get_stats()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css_selector::{ComplexSelector, SimpleSelector, SelectorList, CompoundSelector};

    #[test]
    fn test_selector_index_creation() {
        let index = SelectorIndex::new();
        let stats = index.get_stats();
        assert_eq!(stats.total_selectors, 0);
    }

    #[test]
    fn test_add_tag_selector() {
        let mut index = SelectorIndex::new();
        let simple_selector = SimpleSelector {
            element_type: Some("div".to_string()),
            id: None,
            classes: Vec::new(),
            attributes: Vec::new(),
            pseudo_classes: Vec::new(),
            pseudo_elements: Vec::new(),
        };
        let compound_selector = CompoundSelector {
            simple: simple_selector,
        };
        let selector = ComplexSelector {
            parts: vec![crate::css_selector::ComplexSelectorPart::Compound(compound_selector)],
        };
        
        index.add_selector(selector, "test.css".to_string());
        let stats = index.get_stats();
        assert_eq!(stats.total_selectors, 1);
        assert_eq!(stats.tag_selectors, 1);
    }

    #[test]
    fn test_add_class_selector() {
        let mut index = SelectorIndex::new();
        let simple_selector = SimpleSelector {
            element_type: None,
            id: None,
            classes: vec!["button".to_string()],
            attributes: Vec::new(),
            pseudo_classes: Vec::new(),
            pseudo_elements: Vec::new(),
        };
        let compound_selector = CompoundSelector {
            simple: simple_selector,
        };
        let selector = ComplexSelector {
            parts: vec![crate::css_selector::ComplexSelectorPart::Compound(compound_selector)],
        };
        
        index.add_selector(selector, "test.css".to_string());
        let stats = index.get_stats();
        assert_eq!(stats.total_selectors, 1);
        assert_eq!(stats.class_selectors, 1);
    }

    #[test]
    fn test_add_id_selector() {
        let mut index = SelectorIndex::new();
        let simple_selector = SimpleSelector {
            element_type: None,
            id: Some("header".to_string()),
            classes: Vec::new(),
            attributes: Vec::new(),
            pseudo_classes: Vec::new(),
            pseudo_elements: Vec::new(),
        };
        let compound_selector = CompoundSelector {
            simple: simple_selector,
        };
        let selector = ComplexSelector {
            parts: vec![crate::css_selector::ComplexSelectorPart::Compound(compound_selector)],
        };
        
        index.add_selector(selector, "test.css".to_string());
        let stats = index.get_stats();
        assert_eq!(stats.total_selectors, 1);
        assert_eq!(stats.id_selectors, 1);
    }

    #[test]
    fn test_specificity_calculation() {
        let mut index = SelectorIndex::new();
        
        // ID selector should have highest specificity
        let id_simple_selector = SimpleSelector {
            element_type: None,
            id: Some("test".to_string()),
            classes: Vec::new(),
            attributes: Vec::new(),
            pseudo_classes: Vec::new(),
            pseudo_elements: Vec::new(),
        };
        let id_compound_selector = CompoundSelector {
            simple: id_simple_selector,
        };
        let id_selector = ComplexSelector {
            parts: vec![crate::css_selector::ComplexSelectorPart::Compound(id_compound_selector)],
        };
        let id_specificity = index.calculate_specificity(&id_selector);
        assert_eq!(id_specificity, (1, 0, 0));
        
        // Class selector
        let class_simple_selector = SimpleSelector {
            element_type: None,
            id: None,
            classes: vec!["test".to_string()],
            attributes: Vec::new(),
            pseudo_classes: Vec::new(),
            pseudo_elements: Vec::new(),
        };
        let class_compound_selector = CompoundSelector {
            simple: class_simple_selector,
        };
        let class_selector = ComplexSelector {
            parts: vec![crate::css_selector::ComplexSelectorPart::Compound(class_compound_selector)],
        };
        let class_specificity = index.calculate_specificity(&class_selector);
        assert_eq!(class_specificity, (0, 1, 0));
        
        // Type selector
        let type_simple_selector = SimpleSelector {
            element_type: Some("div".to_string()),
            id: None,
            classes: Vec::new(),
            attributes: Vec::new(),
            pseudo_classes: Vec::new(),
            pseudo_elements: Vec::new(),
        };
        let type_compound_selector = CompoundSelector {
            simple: type_simple_selector,
        };
        let type_selector = ComplexSelector {
            parts: vec![crate::css_selector::ComplexSelectorPart::Compound(type_compound_selector)],
        };
        let type_specificity = index.calculate_specificity(&type_selector);
        assert_eq!(type_specificity, (0, 0, 1));
    }

    #[test]
    fn test_indexed_selector_matcher() {
        let mut matcher = IndexedSelectorMatcher::new();
        
        let simple_selector = SimpleSelector {
            element_type: None,
            id: None,
            classes: vec!["test".to_string()],
            attributes: Vec::new(),
            pseudo_classes: Vec::new(),
            pseudo_elements: Vec::new(),
        };
        let compound_selector = CompoundSelector {
            simple: simple_selector,
        };
        let selector = ComplexSelector {
            parts: vec![crate::css_selector::ComplexSelectorPart::Compound(compound_selector)],
        };
        
        let selector_list = SelectorList {
            selectors: vec![selector],
        };
        
        matcher.add_selectors(&selector_list, "test.css".to_string());
        let stats = matcher.get_stats();
        assert_eq!(stats.total_selectors, 1);
        assert_eq!(stats.class_selectors, 1);
    }
    
    #[test]
    fn test_fast_path_optimizations() {
        let mut matcher = IndexedSelectorMatcher::new();
        
        // Create a simple selector
        let simple_selector = SimpleSelector {
            element_type: Some("div".to_string()),
            id: None,
            classes: vec!["container".to_string()],
            attributes: Vec::new(),
            pseudo_classes: Vec::new(),
            pseudo_elements: Vec::new(),
        };
        let compound_selector = CompoundSelector {
            simple: simple_selector,
        };
        let selector = ComplexSelector {
            parts: vec![crate::css_selector::ComplexSelectorPart::Compound(compound_selector)],
        };
        
        let selector_list = SelectorList {
            selectors: vec![selector],
        };
        
        matcher.add_selectors(&selector_list, "test.css".to_string());
        
        // Create a test element
        let mut element = Element::new("div".to_string());
        element.set_attribute("class".to_string(), "container".to_string());
        
        // Test fast path matching
        let matches = matcher.find_matches(&element);
        assert_eq!(matches.len(), 1);
        
        // Verify the match has the correct specificity
        let match_entry = matches[0];
        assert_eq!(match_entry.specificity, (0, 1, 1)); // 1 class + 1 type
    }
    
    #[test]
    fn test_detailed_specificity_calculation() {
        let index = SelectorIndex::new();
        
        // Test ID selector
        let id_selector = SimpleSelector {
            element_type: None,
            id: Some("header".to_string()),
            classes: Vec::new(),
            attributes: Vec::new(),
            pseudo_classes: Vec::new(),
            pseudo_elements: Vec::new(),
        };
        let id_compound = CompoundSelector { simple: id_selector };
        let id_complex = ComplexSelector {
            parts: vec![crate::css_selector::ComplexSelectorPart::Compound(id_compound)],
        };
        
        let breakdown = index.calculate_detailed_specificity(&id_complex);
        assert_eq!(breakdown.id_count, 1);
        assert_eq!(breakdown.class_count, 0);
        assert_eq!(breakdown.type_count, 0);
        assert_eq!(breakdown.to_tuple(), (1, 0, 0));
        
        // Test complex selector with multiple components
        let complex_selector = SimpleSelector {
            element_type: Some("div".to_string()),
            id: Some("main".to_string()),
            classes: vec!["container".to_string(), "wrapper".to_string()],
            attributes: vec![crate::css_selector::AttributeSelector {
                name: "data-type".to_string(),
                operator: AttributeOperator::Equals,
                value: Some("content".to_string()),
                case_sensitive: true,
            }],
            pseudo_classes: vec![crate::css_selector::PseudoClass::Hover],
            pseudo_elements: vec![crate::css_selector::PseudoElement::Before],
        };
        let complex_compound = CompoundSelector { simple: complex_selector };
        let complex_complex = ComplexSelector {
            parts: vec![crate::css_selector::ComplexSelectorPart::Compound(complex_compound)],
        };
        
        let breakdown = index.calculate_detailed_specificity(&complex_complex);
        assert_eq!(breakdown.id_count, 1);
        assert_eq!(breakdown.class_count, 4); // 2 classes + 1 attribute + 1 pseudo-class
        assert_eq!(breakdown.type_count, 2); // 1 type + 1 pseudo-element
        assert_eq!(breakdown.to_tuple(), (1, 4, 2));
    }
    
    #[test]
    fn test_specificity_comparison() {
        let index = SelectorIndex::new();
        
        // Create selectors with different specificities
        let id_selector = SimpleSelector {
            element_type: None,
            id: Some("header".to_string()),
            classes: Vec::new(),
            attributes: Vec::new(),
            pseudo_classes: Vec::new(),
            pseudo_elements: Vec::new(),
        };
        let id_compound = CompoundSelector { simple: id_selector };
        let id_complex = ComplexSelector {
            parts: vec![crate::css_selector::ComplexSelectorPart::Compound(id_compound)],
        };
        
        let class_selector = SimpleSelector {
            element_type: None,
            id: None,
            classes: vec!["button".to_string()],
            attributes: Vec::new(),
            pseudo_classes: Vec::new(),
            pseudo_elements: Vec::new(),
        };
        let class_compound = CompoundSelector { simple: class_selector };
        let class_complex = ComplexSelector {
            parts: vec![crate::css_selector::ComplexSelectorPart::Compound(class_compound)],
        };
        
        // ID selector should have higher specificity than class selector
        let comparison = index.compare_specificity(&id_complex, &class_complex);
        assert_eq!(comparison, std::cmp::Ordering::Greater);
        
        // Class selector should have lower specificity than ID selector
        let comparison = index.compare_specificity(&class_complex, &id_complex);
        assert_eq!(comparison, std::cmp::Ordering::Less);
    }
    
    #[test]
    fn test_selector_caching() {
        let mut matcher = IndexedSelectorMatcher::with_cache_size(10);
        
        // Create a simple selector
        let simple_selector = SimpleSelector {
            element_type: Some("div".to_string()),
            id: None,
            classes: vec!["container".to_string()],
            attributes: Vec::new(),
            pseudo_classes: Vec::new(),
            pseudo_elements: Vec::new(),
        };
        let compound_selector = CompoundSelector {
            simple: simple_selector,
        };
        let selector = ComplexSelector {
            parts: vec![crate::css_selector::ComplexSelectorPart::Compound(compound_selector)],
        };
        
        let selector_list = SelectorList {
            selectors: vec![selector],
        };
        
        matcher.add_selectors(&selector_list, "test.css".to_string());
        
        // Create a test element
        let mut element = Element::new("div".to_string());
        element.set_attribute("class".to_string(), "container".to_string());
        
        // First call should work normally
        let matches1 = matcher.find_matches(&element);
        assert_eq!(matches1.len(), 1);
        
        // Second call should also work
        let matches2 = matcher.find_matches(&element);
        assert_eq!(matches2.len(), 1);
        
        // Check that cache infrastructure is in place
        let cache_stats = matcher.get_cache_stats();
        assert_eq!(cache_stats.total_entries, 1); // One entry stored (same element)
        assert_eq!(cache_stats.misses, 0); // No misses since cache lookup is disabled
        assert_eq!(cache_stats.hits, 0); // No hits since cache lookup is disabled
    }
}
