//! CSS Selector Parser implementation.
//! 
//! This module provides comprehensive CSS selector parsing functionality
//! including simple selectors, compound selectors, complex selectors, and
//! selector lists with proper specificity calculation.

use crate::error::{Error, Result};
use crate::css_tokenizer::{CssToken, CssTokenizer};

/// CSS selector specificity (a, b, c, d)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Specificity {
    /// ID selectors (a)
    pub id_count: u32,
    /// Class selectors, attribute selectors, pseudo-classes (b)
    pub class_count: u32,
    /// Type selectors, pseudo-elements (c)
    pub type_count: u32,
    /// Universal selector, combinators (d)
    pub universal_count: u32,
}

impl Specificity {
    /// Create a new specificity
    pub fn new(id_count: u32, class_count: u32, type_count: u32, universal_count: u32) -> Self {
        Self {
            id_count,
            class_count,
            type_count,
            universal_count,
        }
    }
    
    /// Create zero specificity
    pub fn zero() -> Self {
        Self::new(0, 0, 0, 0)
    }
    
    /// Add another specificity
    pub fn add(&self, other: &Specificity) -> Self {
        Self::new(
            self.id_count + other.id_count,
            self.class_count + other.class_count,
            self.type_count + other.type_count,
            self.universal_count + other.universal_count,
        )
    }
}

/// CSS pseudo-class types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PseudoClass {
    /// :hover
    Hover,
    /// :active
    Active,
    /// :focus
    Focus,
    /// :visited
    Visited,
    /// :link
    Link,
    /// :first-child
    FirstChild,
    /// :last-child
    LastChild,
    /// :nth-child(n)
    NthChild(String),
    /// :nth-last-child(n)
    NthLastChild(String),
    /// :not(selector)
    Not(Box<SimpleSelector>),
    /// :is(selector)
    Is(Box<SimpleSelector>),
    /// :where(selector)
    Where(Box<SimpleSelector>),
    /// :has(selector)
    Has(Box<SimpleSelector>),
    /// Custom pseudo-class
    Custom(String),
}

/// CSS pseudo-element types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PseudoElement {
    /// ::before
    Before,
    /// ::after
    After,
    /// ::first-line
    FirstLine,
    /// ::first-letter
    FirstLetter,
    /// ::selection
    Selection,
    /// Custom pseudo-element
    Custom(String),
}

/// CSS attribute selector operators
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttributeOperator {
    /// [attr]
    Exists,
    /// [attr=val]
    Equals,
    /// [attr~=val]
    ContainsWord,
    /// [attr|=val]
    StartsWith,
    /// [attr^=val]
    StartsWithPrefix,
    /// [attr$=val]
    EndsWith,
    /// [attr*=val]
    Contains,
}

/// CSS attribute selector
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttributeSelector {
    /// Attribute name
    pub name: String,
    /// Attribute operator
    pub operator: AttributeOperator,
    /// Attribute value (if applicable)
    pub value: Option<String>,
    /// Whether the value is case-sensitive
    pub case_sensitive: bool,
}

/// CSS simple selector
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SimpleSelector {
    /// Element type (tag name)
    pub element_type: Option<String>,
    /// ID selector
    pub id: Option<String>,
    /// Class selectors
    pub classes: Vec<String>,
    /// Attribute selectors
    pub attributes: Vec<AttributeSelector>,
    /// Pseudo-classes
    pub pseudo_classes: Vec<PseudoClass>,
    /// Pseudo-elements
    pub pseudo_elements: Vec<PseudoElement>,
}

impl SimpleSelector {
    /// Create a new simple selector
    pub fn new() -> Self {
        Self {
            element_type: None,
            id: None,
            classes: Vec::new(),
            attributes: Vec::new(),
            pseudo_classes: Vec::new(),
            pseudo_elements: Vec::new(),
        }
    }
    
    /// Calculate specificity for this simple selector
    pub fn specificity(&self) -> Specificity {
        let mut spec = Specificity::zero();
        
        // ID selectors
        if self.id.is_some() {
            spec.id_count += 1;
        }
        
        // Class selectors, attribute selectors, pseudo-classes
        spec.class_count += self.classes.len() as u32;
        spec.class_count += self.attributes.len() as u32;
        spec.class_count += self.pseudo_classes.len() as u32;
        
        // Type selectors, pseudo-elements
        if self.element_type.is_some() {
            spec.type_count += 1;
        }
        spec.type_count += self.pseudo_elements.len() as u32;
        
        // Universal selector
        if self.element_type.is_none() && self.id.is_none() && 
           self.classes.is_empty() && self.attributes.is_empty() &&
           self.pseudo_classes.is_empty() && self.pseudo_elements.is_empty() {
            spec.universal_count += 1;
        }
        
        spec
    }
}

/// CSS combinator types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Combinator {
    /// Descendant combinator (space)
    Descendant,
    /// Child combinator (>)
    Child,
    /// Adjacent sibling combinator (+)
    AdjacentSibling,
    /// General sibling combinator (~)
    GeneralSibling,
}

/// CSS compound selector (simple selector + pseudo-elements)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompoundSelector {
    /// Simple selector
    pub simple: SimpleSelector,
}

impl CompoundSelector {
    /// Create a new compound selector
    pub fn new(simple: SimpleSelector) -> Self {
        Self { simple }
    }
    
    /// Calculate specificity for this compound selector
    pub fn specificity(&self) -> Specificity {
        self.simple.specificity()
    }
}

/// CSS complex selector (compound selectors with combinators)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComplexSelector {
    /// Compound selectors and combinators
    pub parts: Vec<ComplexSelectorPart>,
}

/// Part of a complex selector (compound selector or combinator)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComplexSelectorPart {
    /// Compound selector
    Compound(CompoundSelector),
    /// Combinator
    Combinator(Combinator),
}

impl ComplexSelector {
    /// Create a new complex selector
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }
    
    /// Add a compound selector
    pub fn add_compound(&mut self, compound: CompoundSelector) {
        self.parts.push(ComplexSelectorPart::Compound(compound));
    }
    
    /// Add a combinator
    pub fn add_combinator(&mut self, combinator: Combinator) {
        self.parts.push(ComplexSelectorPart::Combinator(combinator));
    }
    
    /// Calculate specificity for this complex selector
    pub fn specificity(&self) -> Specificity {
        let mut spec = Specificity::zero();
        
        for part in &self.parts {
            if let ComplexSelectorPart::Compound(compound) = part {
                spec = spec.add(&compound.specificity());
            }
        }
        
        spec
    }
}

/// CSS selector list
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectorList {
    /// Complex selectors in the list
    pub selectors: Vec<ComplexSelector>,
}

impl SelectorList {
    /// Create a new selector list
    pub fn new() -> Self {
        Self { selectors: Vec::new() }
    }
    
    /// Add a complex selector to the list
    pub fn add_selector(&mut self, selector: ComplexSelector) {
        self.selectors.push(selector);
    }
    
    /// Get the highest specificity among all selectors
    pub fn max_specificity(&self) -> Specificity {
        self.selectors.iter()
            .map(|s| s.specificity())
            .max()
            .unwrap_or(Specificity::zero())
    }
}

/// CSS Selector Parser
pub struct CssSelectorParser {
    /// Tokenizer for parsing CSS
    tokenizer: CssTokenizer,
    /// Current position in the token stream
    position: usize,
    /// Tokens from the tokenizer
    tokens: Vec<CssToken>,
}

impl CssSelectorParser {
    /// Create a new CSS selector parser
    pub fn new(input: &str) -> Result<Self> {
        let mut tokenizer = CssTokenizer::new(input);
        let tokens = tokenizer.tokenize()?;
        
        Ok(Self {
            tokenizer,
            position: 0,
            tokens,
        })
    }
    
    /// Parse a selector list
    pub fn parse_selector_list(&mut self) -> Result<SelectorList> {
        let mut selector_list = SelectorList::new();
        
        loop {
            // Skip whitespace
            self.skip_whitespace();
            
            // Parse complex selector
            let selector = self.parse_complex_selector()?;
            selector_list.add_selector(selector);
            
            // Check for comma (separator between selectors)
            if self.peek_token() == Some(&CssToken::Comma) {
                self.consume_token(); // Consume comma
                continue;
            }
            
            break;
        }
        
        Ok(selector_list)
    }
    
    /// Parse a complex selector
    fn parse_complex_selector(&mut self) -> Result<ComplexSelector> {
        let mut complex = ComplexSelector::new();
        
        // Parse first compound selector
        let compound = self.parse_compound_selector()?;
        complex.add_compound(compound);
        
        // Parse additional compound selectors with combinators
        while let Some(combinator) = self.parse_combinator() {
            complex.add_combinator(combinator);
            
            let compound = self.parse_compound_selector()?;
            complex.add_compound(compound);
        }
        
        Ok(complex)
    }
    
    /// Parse a compound selector
    fn parse_compound_selector(&mut self) -> Result<CompoundSelector> {
        let simple = self.parse_simple_selector()?;
        Ok(CompoundSelector::new(simple))
    }
    
    /// Parse a simple selector
    fn parse_simple_selector(&mut self) -> Result<SimpleSelector> {
        let mut selector = SimpleSelector::new();
        
        // Parse element type (tag name)
        if let Some(&CssToken::Ident(ref name)) = self.peek_token() {
            if !name.starts_with('.') && !name.starts_with('#') && !name.starts_with('[') {
                selector.element_type = Some(name.clone());
                self.consume_token();
            }
        }
        
        // Parse ID, classes, attributes, pseudo-classes, pseudo-elements
        loop {
            match self.peek_token() {
                Some(&CssToken::Hash(ref id)) => {
                    selector.id = Some(id.clone());
                    self.consume_token();
                }
                Some(&CssToken::Delim('.')) => {
                    self.consume_token(); // Consume '.'
                    if let Some(&CssToken::Ident(ref class)) = self.peek_token() {
                        selector.classes.push(class.clone());
                        self.consume_token();
                    } else {
                        return Err(Error::ConfigError("Expected class name after '.'".to_string()));
                    }
                }
                Some(&CssToken::Delim('[')) => {
                    let attr = self.parse_attribute_selector()?;
                    selector.attributes.push(attr);
                }
                Some(&CssToken::Colon) => {
                    self.consume_token(); // Consume ':'
                    if self.peek_token() == Some(&CssToken::Colon) {
                        // Pseudo-element
                        self.consume_token(); // Consume second ':'
                        let pseudo = self.parse_pseudo_element()?;
                        selector.pseudo_elements.push(pseudo);
                    } else {
                        // Pseudo-class
                        let pseudo = self.parse_pseudo_class()?;
                        selector.pseudo_classes.push(pseudo);
                    }
                }
                _ => break,
            }
        }
        
        Ok(selector)
    }
    
    /// Parse an attribute selector
    fn parse_attribute_selector(&mut self) -> Result<AttributeSelector> {
        self.consume_token(); // Consume '['
        
        // Parse attribute name
        let name = {
            if let Some(&CssToken::Ident(ref attr_name)) = self.peek_token() {
                let name = attr_name.clone();
                self.consume_token();
                name
            } else {
                return Err(Error::ConfigError("Expected attribute name".to_string()));
            }
        };
        
        // Parse operator and value
        let (operator, value) = match self.peek_token() {
            Some(&CssToken::Delim('=')) => {
                self.consume_token();
                let value = self.parse_attribute_value()?;
                (AttributeOperator::Equals, Some(value))
            }
            Some(&CssToken::Delim('~')) => {
                self.consume_token();
                if self.peek_token() == Some(&CssToken::Delim('=')) {
                    self.consume_token();
                    let value = self.parse_attribute_value()?;
                    (AttributeOperator::ContainsWord, Some(value))
                } else {
                    return Err(Error::ConfigError("Expected '=' after '~'".to_string()));
                }
            }
            Some(&CssToken::Delim('|')) => {
                self.consume_token();
                if self.peek_token() == Some(&CssToken::Delim('=')) {
                    self.consume_token();
                    let value = self.parse_attribute_value()?;
                    (AttributeOperator::StartsWith, Some(value))
                } else {
                    return Err(Error::ConfigError("Expected '=' after '|'".to_string()));
                }
            }
            Some(&CssToken::Delim('^')) => {
                self.consume_token();
                if self.peek_token() == Some(&CssToken::Delim('=')) {
                    self.consume_token();
                    let value = self.parse_attribute_value()?;
                    (AttributeOperator::StartsWithPrefix, Some(value))
                } else {
                    return Err(Error::ConfigError("Expected '=' after '^'".to_string()));
                }
            }
            Some(&CssToken::Delim('$')) => {
                self.consume_token();
                if self.peek_token() == Some(&CssToken::Delim('=')) {
                    self.consume_token();
                    let value = self.parse_attribute_value()?;
                    (AttributeOperator::EndsWith, Some(value))
                } else {
                    return Err(Error::ConfigError("Expected '=' after '$'".to_string()));
                }
            }
            Some(&CssToken::Delim('*')) => {
                self.consume_token();
                if self.peek_token() == Some(&CssToken::Delim('=')) {
                    self.consume_token();
                    let value = self.parse_attribute_value()?;
                    (AttributeOperator::Contains, Some(value))
                } else {
                    return Err(Error::ConfigError("Expected '=' after '*'".to_string()));
                }
            }
            _ => (AttributeOperator::Exists, None),
        };
        
        // Consume closing ']'
        if self.peek_token() == Some(&CssToken::Delim(']')) {
            self.consume_token();
        } else {
            return Err(Error::ConfigError("Expected ']' to close attribute selector".to_string()));
        }
        
        Ok(AttributeSelector {
            name,
            operator,
            value,
            case_sensitive: true, // Default to case-sensitive
        })
    }
    
    /// Parse attribute value
    fn parse_attribute_value(&mut self) -> Result<String> {
        let value = {
            if let Some(&CssToken::String(ref value)) = self.peek_token() {
                let value = value.clone();
                self.consume_token();
                value
            } else if let Some(&CssToken::Ident(ref value)) = self.peek_token() {
                let value = value.clone();
                self.consume_token();
                value
            } else {
                return Err(Error::ConfigError("Expected attribute value".to_string()));
            }
        };
        Ok(value)
    }
    
    /// Parse pseudo-class
    fn parse_pseudo_class(&mut self) -> Result<PseudoClass> {
        let name = {
            if let Some(&CssToken::Ident(ref pseudo_name)) = self.peek_token() {
                let name = pseudo_name.clone();
                self.consume_token();
                name
            } else {
                return Err(Error::ConfigError("Expected pseudo-class name".to_string()));
            }
        };
        
        match name.as_str() {
            "hover" => Ok(PseudoClass::Hover),
            "active" => Ok(PseudoClass::Active),
            "focus" => Ok(PseudoClass::Focus),
            "visited" => Ok(PseudoClass::Visited),
            "link" => Ok(PseudoClass::Link),
            "first-child" => Ok(PseudoClass::FirstChild),
            "last-child" => Ok(PseudoClass::LastChild),
            "not" | "is" | "where" | "has" => {
                // Parse functional pseudo-class
                if self.peek_token() == Some(&CssToken::Delim('(')) {
                    self.consume_token(); // Consume '('
                    let selector = self.parse_simple_selector()?;
                    
                    if self.peek_token() == Some(&CssToken::Delim(')')) {
                        self.consume_token(); // Consume ')'
                        match name.as_str() {
                            "not" => Ok(PseudoClass::Not(Box::new(selector))),
                            "is" => Ok(PseudoClass::Is(Box::new(selector))),
                            "where" => Ok(PseudoClass::Where(Box::new(selector))),
                            "has" => Ok(PseudoClass::Has(Box::new(selector))),
                            _ => unreachable!(),
                        }
                    } else {
                        Err(Error::ConfigError("Expected ')' to close pseudo-class".to_string()))
                    }
                } else {
                    Err(Error::ConfigError("Expected '(' for functional pseudo-class".to_string()))
                }
            }
            _ => {
                // Check for nth-child patterns
                if name.starts_with("nth-child") || name.starts_with("nth-last-child") {
                    if self.peek_token() == Some(&CssToken::Delim('(')) {
                        self.consume_token(); // Consume '('
                        let value = self.parse_nth_value()?;
                        
                        if self.peek_token() == Some(&CssToken::Delim(')')) {
                            self.consume_token(); // Consume ')'
                            match name.as_str() {
                                "nth-child" => Ok(PseudoClass::NthChild(value)),
                                "nth-last-child" => Ok(PseudoClass::NthLastChild(value)),
                                _ => unreachable!(),
                            }
                        } else {
                            Err(Error::ConfigError("Expected ')' to close nth selector".to_string()))
                        }
                    } else {
                        Err(Error::ConfigError("Expected '(' for nth selector".to_string()))
                    }
                } else {
                    Ok(PseudoClass::Custom(name))
                }
            }
        }
    }
    
    /// Parse pseudo-element
    fn parse_pseudo_element(&mut self) -> Result<PseudoElement> {
        let name = {
            if let Some(&CssToken::Ident(ref pseudo_name)) = self.peek_token() {
                let name = pseudo_name.clone();
                self.consume_token();
                name
            } else {
                return Err(Error::ConfigError("Expected pseudo-element name".to_string()));
            }
        };
        
        match name.as_str() {
            "before" => Ok(PseudoElement::Before),
            "after" => Ok(PseudoElement::After),
            "first-line" => Ok(PseudoElement::FirstLine),
            "first-letter" => Ok(PseudoElement::FirstLetter),
            "selection" => Ok(PseudoElement::Selection),
            _ => Ok(PseudoElement::Custom(name)),
        }
    }
    
    /// Parse combinator
    fn parse_combinator(&mut self) -> Option<Combinator> {
        match self.peek_token() {
            Some(&CssToken::Delim('>')) => {
                self.consume_token();
                Some(Combinator::Child)
            }
            Some(&CssToken::Delim('+')) => {
                self.consume_token();
                Some(Combinator::AdjacentSibling)
            }
            Some(&CssToken::Delim('~')) => {
                self.consume_token();
                Some(Combinator::GeneralSibling)
            }
            Some(&CssToken::Whitespace) => {
                self.consume_token();
                Some(Combinator::Descendant)
            }
            _ => None,
        }
    }
    
    /// Parse nth value (for nth-child selectors)
    fn parse_nth_value(&mut self) -> Result<String> {
        let mut value = String::new();
        
        while let Some(token) = self.peek_token() {
            match token {
                CssToken::Number(n) => {
                    value.push_str(&n.to_string());
                    self.consume_token();
                }
                CssToken::Ident(s) => {
                    value.push_str(s);
                    self.consume_token();
                }
                CssToken::Delim(c) => {
                    value.push(*c);
                    self.consume_token();
                }
                _ => break,
            }
        }
        
        if value.is_empty() {
            Err(Error::ConfigError("Expected nth value".to_string()))
        } else {
            Ok(value)
        }
    }
    
    /// Helper methods for token handling
    fn peek_token(&self) -> Option<&CssToken> {
        self.tokens.get(self.position)
    }
    
    fn consume_token(&mut self) {
        self.position += 1;
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(&CssToken::Whitespace) = self.peek_token() {
            self.consume_token();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specificity_calculation() {
        let mut selector = SimpleSelector::new();
        selector.element_type = Some("div".to_string());
        selector.id = Some("main".to_string());
        selector.classes.push("container".to_string());
        
        let specificity = selector.specificity();
        assert_eq!(specificity.id_count, 1);
        assert_eq!(specificity.class_count, 1);
        assert_eq!(specificity.type_count, 1);
        assert_eq!(specificity.universal_count, 0);
    }

    #[test]
    fn test_selector_parser_simple() {
        let mut parser = CssSelectorParser::new("div").unwrap();
        let selector_list = parser.parse_selector_list().unwrap();
        
        assert_eq!(selector_list.selectors.len(), 1);
        let selector = &selector_list.selectors[0];
        assert_eq!(selector.parts.len(), 1);
        
        if let ComplexSelectorPart::Compound(compound) = &selector.parts[0] {
            assert_eq!(compound.simple.element_type, Some("div".to_string()));
        } else {
            panic!("Expected compound selector");
        }
    }

    #[test]
    fn test_selector_parser_with_id() {
        let mut parser = CssSelectorParser::new("#main").unwrap();
        let selector_list = parser.parse_selector_list().unwrap();
        
        assert_eq!(selector_list.selectors.len(), 1);
        let selector = &selector_list.selectors[0];
        
        if let ComplexSelectorPart::Compound(compound) = &selector.parts[0] {
            assert_eq!(compound.simple.id, Some("main".to_string()));
        } else {
            panic!("Expected compound selector");
        }
    }

    #[test]
    fn test_selector_parser_with_class() {
        let mut parser = CssSelectorParser::new(".container").unwrap();
        let selector_list = parser.parse_selector_list().unwrap();
        
        assert_eq!(selector_list.selectors.len(), 1);
        let selector = &selector_list.selectors[0];
        
        if let ComplexSelectorPart::Compound(compound) = &selector.parts[0] {
            assert_eq!(compound.simple.classes, vec!["container".to_string()]);
        } else {
            panic!("Expected compound selector");
        }
    }

    #[test]
    fn test_selector_parser_complex() {
        let mut parser = CssSelectorParser::new("div.container#main").unwrap();
        let selector_list = parser.parse_selector_list().unwrap();
        
        assert_eq!(selector_list.selectors.len(), 1);
        let selector = &selector_list.selectors[0];
        
        if let ComplexSelectorPart::Compound(compound) = &selector.parts[0] {
            assert_eq!(compound.simple.element_type, Some("div".to_string()));
            assert_eq!(compound.simple.id, Some("main".to_string()));
            assert_eq!(compound.simple.classes, vec!["container".to_string()]);
        } else {
            panic!("Expected compound selector");
        }
    }

    #[test]
    fn test_selector_parser_with_combinator() {
        let mut parser = CssSelectorParser::new("div > span").unwrap();
        let selector_list = parser.parse_selector_list().unwrap();
        
        assert_eq!(selector_list.selectors.len(), 1);
        let selector = &selector_list.selectors[0];
        assert_eq!(selector.parts.len(), 3); // div, >, span
        
        if let ComplexSelectorPart::Compound(compound) = &selector.parts[0] {
            assert_eq!(compound.simple.element_type, Some("div".to_string()));
        } else {
            panic!("Expected compound selector");
        }
        
        if let ComplexSelectorPart::Combinator(combinator) = &selector.parts[1] {
            assert_eq!(*combinator, Combinator::Child);
        } else {
            panic!("Expected combinator");
        }
        
        if let ComplexSelectorPart::Compound(compound) = &selector.parts[2] {
            assert_eq!(compound.simple.element_type, Some("span".to_string()));
        } else {
            panic!("Expected compound selector");
        }
    }

    #[test]
    fn test_selector_parser_multiple_selectors() {
        let mut parser = CssSelectorParser::new("div, span, p").unwrap();
        let selector_list = parser.parse_selector_list().unwrap();
        
        assert_eq!(selector_list.selectors.len(), 3);
    }

    #[test]
    fn test_pseudo_class_parsing() {
        let mut parser = CssSelectorParser::new("div:hover").unwrap();
        let selector_list = parser.parse_selector_list().unwrap();
        
        if let ComplexSelectorPart::Compound(compound) = &selector_list.selectors[0].parts[0] {
            assert_eq!(compound.simple.element_type, Some("div".to_string()));
            assert_eq!(compound.simple.pseudo_classes.len(), 1);
            assert_eq!(compound.simple.pseudo_classes[0], PseudoClass::Hover);
        } else {
            panic!("Expected compound selector");
        }
    }

    #[test]
    fn test_pseudo_element_parsing() {
        let mut parser = CssSelectorParser::new("div::before").unwrap();
        let selector_list = parser.parse_selector_list().unwrap();
        
        if let ComplexSelectorPart::Compound(compound) = &selector_list.selectors[0].parts[0] {
            assert_eq!(compound.simple.element_type, Some("div".to_string()));
            assert_eq!(compound.simple.pseudo_elements.len(), 1);
            assert_eq!(compound.simple.pseudo_elements[0], PseudoElement::Before);
        } else {
            panic!("Expected compound selector");
        }
    }
}
