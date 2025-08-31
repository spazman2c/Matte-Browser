//! HTML parser for the Matte browser.
//! 
//! This module provides HTML parsing functionality to convert HTML text
//! into a structured DOM tree.

use crate::error::{Error, Result};
use crate::dom::{Document, Element, Node, TextNode};
use std::collections::HashMap;

/// HTML parser state
#[derive(Debug, Clone, PartialEq)]
enum ParserState {
    /// Initial state
    Initial,
    /// Inside a tag
    Tag,
    /// Inside an opening tag
    OpeningTag,
    /// Inside a closing tag
    ClosingTag,
    /// Inside tag name
    TagName,
    /// Inside attribute name
    AttributeName,
    /// Inside attribute value
    AttributeValue,
    /// Inside text content
    Text,
    /// Inside comment
    Comment,
    /// Inside DOCTYPE declaration
    Doctype,
}

/// HTML parser for converting HTML text to DOM
pub struct HtmlParser {
    state: ParserState,
    current_tag_name: String,
    current_attribute_name: String,
    current_attribute_value: String,
    current_text: String,
    stack: Vec<Element>,
    document: Document,
    in_quotes: bool,
    quote_char: Option<char>,
    pending_attributes: HashMap<String, String>,
    is_self_closing_context: bool,
}

impl HtmlParser {
    /// Create a new HTML parser
    pub fn new() -> Self {
        Self {
            state: ParserState::Initial,
            current_tag_name: String::new(),
            current_attribute_name: String::new(),
            current_attribute_value: String::new(),
            current_text: String::new(),
            stack: Vec::new(),
            document: Document::new(),
            in_quotes: false,
            quote_char: None,
            pending_attributes: HashMap::new(),
            is_self_closing_context: false,
        }
    }

    /// Parse HTML text into a DOM document
    pub fn parse(&mut self, html: &str) -> Result<Document> {
        self.reset();
        
        for (i, ch) in html.chars().enumerate() {
            self.process_char(ch, i)?;
        }
        
        // Process any remaining text
        if !self.current_text.trim().is_empty() {
            self.add_text_node();
        }
        
        // Close any unclosed tags
        while let Some(element) = self.stack.pop() {
            if let Some(parent) = self.stack.last_mut() {
                parent.children.push(Node::Element(element));
            } else {
                self.document.root.children.push(Node::Element(element));
            }
        }
        
        Ok(self.document.clone())
    }

    /// Reset parser state
    fn reset(&mut self) {
        self.state = ParserState::Initial;
        self.current_tag_name.clear();
        self.current_attribute_name.clear();
        self.current_attribute_value.clear();
        self.current_text.clear();
        self.stack.clear();
        self.document = Document::new();
        self.in_quotes = false;
        self.quote_char = None;
        self.pending_attributes.clear();
        self.is_self_closing_context = false;
    }

    /// Process a single character
    fn process_char(&mut self, ch: char, _position: usize) -> Result<()> {
        match self.state {
            ParserState::Initial => self.handle_initial_state(ch)?,
            ParserState::Tag => self.handle_tag_state(ch)?,
            ParserState::OpeningTag => self.handle_opening_tag_state(ch)?,
            ParserState::ClosingTag => self.handle_closing_tag_state(ch)?,
            ParserState::TagName => self.handle_tag_name_state(ch)?,
            ParserState::AttributeName => self.handle_attribute_name_state(ch)?,
            ParserState::AttributeValue => self.handle_attribute_value_state(ch)?,
            ParserState::Text => self.handle_text_state(ch)?,
            ParserState::Comment => self.handle_comment_state(ch)?,
            ParserState::Doctype => self.handle_doctype_state(ch)?,
        }
        Ok(())
    }

    /// Handle initial state
    fn handle_initial_state(&mut self, ch: char) -> Result<()> {
        match ch {
            '<' => {
                if !self.current_text.trim().is_empty() {
                    self.add_text_node();
                }
                self.state = ParserState::Tag;
            }
            _ => {
                self.current_text.push(ch);
                self.state = ParserState::Text;
            }
        }
        Ok(())
    }

    /// Handle tag state
    fn handle_tag_state(&mut self, ch: char) -> Result<()> {
        match ch {
            '/' => {
                self.state = ParserState::ClosingTag;
            }
            '!' => {
                self.state = ParserState::Comment;
            }
            '?' => {
                self.state = ParserState::Doctype;
            }
            c if c.is_ascii_alphabetic() => {
                self.current_tag_name.push(c);
                self.state = ParserState::TagName;
            }
            _ => {
                return Err(Error::ParseError(format!("Unexpected character in tag: {}", ch)));
            }
        }
        Ok(())
    }

    /// Handle opening tag state
    fn handle_opening_tag_state(&mut self, ch: char) -> Result<()> {
        match ch {
            '>' => {
                self.finish_opening_tag()?;
                self.state = ParserState::Initial;
            }
            ' ' | '\t' | '\n' | '\r' => {
                self.state = ParserState::AttributeName;
            }
                            '/' => {
                    // Self-closing tag - wait for the '>'
                    self.state = ParserState::ClosingTag;
                }
            c if c.is_ascii_alphabetic() => {
                self.current_attribute_name.push(c);
                self.state = ParserState::AttributeName;
            }
            _ => {
                // Skip unexpected characters in opening tag
                // This is more lenient than the original implementation
            }
        }
        Ok(())
    }

    /// Handle closing tag state
    fn handle_closing_tag_state(&mut self, ch: char) -> Result<()> {
        match ch {
            '>' => {
                // Check if this is a self-closing tag based on context
                if self.is_self_closing_context {
                    // This is a self-closing tag like <img />
                    self.finish_self_closing_tag()?;
                } else {
                    // This is a regular closing tag like </div>
                    self.finish_closing_tag()?;
                }
                self.state = ParserState::Initial;
                self.is_self_closing_context = false;
            }
            c if c.is_ascii_alphabetic() || c.is_ascii_digit() => {
                self.current_tag_name.push(c);
            }
            _ => {
                // Skip unexpected characters in closing tag
                // This is more lenient than the original implementation
            }
        }
        Ok(())
    }

    /// Handle tag name state
    fn handle_tag_name_state(&mut self, ch: char) -> Result<()> {
        match ch {
            '>' => {
                self.finish_opening_tag()?;
                self.state = ParserState::Initial;
            }
            ' ' | '\t' | '\n' | '\r' => {
                self.state = ParserState::AttributeName;
            }
            '/' => {
                // Self-closing tag - wait for the '>'
                self.is_self_closing_context = true;
                self.state = ParserState::ClosingTag;
            }
            c if c.is_ascii_alphabetic() || c.is_ascii_digit() => {
                self.current_tag_name.push(c);
            }
            _ => {
                // Skip unexpected characters in tag name
                // This is more lenient than the original implementation
            }
        }
        Ok(())
    }

    /// Handle attribute name state
    fn handle_attribute_name_state(&mut self, ch: char) -> Result<()> {
        match ch {
            '=' => {
                self.state = ParserState::AttributeValue;
            }
            '>' => {
                // Attribute without value
                self.finish_attribute()?;
                self.finish_opening_tag()?;
                self.state = ParserState::Initial;
            }
            ' ' | '\t' | '\n' | '\r' => {
                // Attribute without value
                self.finish_attribute()?;
            }
            '/' => {
                // Self-closing tag - finish any pending attribute and wait for the '>'
                self.finish_attribute()?;
                self.is_self_closing_context = true;
                self.state = ParserState::ClosingTag;
            }
            c if c.is_ascii_alphabetic() || c == '-' || c == '_' => {
                self.current_attribute_name.push(c);
            }
            _ => {
                return Err(Error::ParseError(format!("Unexpected character in attribute name: {}", ch)));
            }
        }
        Ok(())
    }

    /// Handle attribute value state
    fn handle_attribute_value_state(&mut self, ch: char) -> Result<()> {
        if self.in_quotes {
            if ch == self.quote_char.unwrap() {
                self.in_quotes = false;
                self.quote_char = None;
                self.finish_attribute()?;
                self.state = ParserState::AttributeName;
            } else {
                self.current_attribute_value.push(ch);
            }
        } else {
            match ch {
                '"' | '\'' => {
                    self.in_quotes = true;
                    self.quote_char = Some(ch);
                }
                '>' => {
                    self.finish_attribute()?;
                    self.finish_opening_tag()?;
                    self.state = ParserState::Initial;
                }
                ' ' | '\t' | '\n' | '\r' => {
                    self.finish_attribute()?;
                    self.state = ParserState::AttributeName;
                }
                '/' => {
                    self.finish_attribute()?;
                    // Don't finish the tag yet, wait for the '>'
                    self.state = ParserState::ClosingTag;
                }
                _ => {
                    self.current_attribute_value.push(ch);
                }
            }
        }
        Ok(())
    }

    /// Handle text state
    fn handle_text_state(&mut self, ch: char) -> Result<()> {
        match ch {
            '<' => {
                if !self.current_text.is_empty() {
                    self.add_text_node();
                }
                self.state = ParserState::Tag;
            }
            _ => {
                self.current_text.push(ch);
            }
        }
        Ok(())
    }

    /// Handle comment state
    fn handle_comment_state(&mut self, ch: char) -> Result<()> {
        // Simplified comment handling - just skip until -->
        if ch == '>' && self.current_text.ends_with("--") {
            self.current_text.clear();
            self.state = ParserState::Initial;
        } else {
            self.current_text.push(ch);
        }
        Ok(())
    }

    /// Handle DOCTYPE state
    fn handle_doctype_state(&mut self, ch: char) -> Result<()> {
        // Simplified DOCTYPE handling - just skip until >
        if ch == '>' {
            self.current_text.clear();
            self.state = ParserState::Initial;
        } else {
            self.current_text.push(ch);
        }
        Ok(())
    }

    /// Finish opening tag
    fn finish_opening_tag(&mut self) -> Result<()> {
        // Finish any pending attribute first
        self.finish_attribute()?;
        
        let tag_name = self.current_tag_name.to_lowercase();
        let mut element = Element::new(tag_name.clone());
        // Set attributes after creation
        for (name, value) in &self.pending_attributes {
            element.set_attribute(name.clone(), value.clone());
        }

        // Handle self-closing tags
        if Self::is_self_closing_tag(&tag_name) {
            self.add_element_to_parent(element);
        } else {
            self.stack.push(element);
        }

        self.current_tag_name.clear();
        self.pending_attributes.clear();
        Ok(())
    }

    /// Finish self-closing tag
    fn finish_self_closing_tag(&mut self) -> Result<()> {
        // Finish any pending attribute first
        self.finish_attribute()?;
        
        let tag_name = self.current_tag_name.to_lowercase();
        let mut element = Element::new(tag_name.clone());
        // Set attributes after creation
        for (name, value) in &self.pending_attributes {
            element.set_attribute(name.clone(), value.clone());
        }

        self.add_element_to_parent(element);
        self.current_tag_name.clear();
        self.pending_attributes.clear();
        Ok(())
    }

    /// Finish closing tag
    fn finish_closing_tag(&mut self) -> Result<()> {
        let tag_name = self.current_tag_name.to_lowercase();
        
        // Find matching opening tag
        while let Some(element) = self.stack.pop() {
            if element.tag_name == tag_name {
                self.add_element_to_parent(element);
                break;
            } else {
                // Mismatched tag - add to parent anyway
                if let Some(parent) = self.stack.last_mut() {
                    parent.children.push(Node::Element(element));
                } else {
                    self.document.root.children.push(Node::Element(element));
                }
            }
        }

        self.current_tag_name.clear();
        Ok(())
    }

    /// Finish attribute
    fn finish_attribute(&mut self) -> Result<()> {
        if !self.current_attribute_name.is_empty() {
            let name = self.current_attribute_name.clone();
            let value = self.current_attribute_value.clone();
            self.pending_attributes.insert(name, value);
        }
        
        self.current_attribute_name.clear();
        self.current_attribute_value.clear();
        Ok(())
    }

    /// Add element to parent
    fn add_element_to_parent(&mut self, element: Element) {
        if let Some(parent) = self.stack.last_mut() {
            parent.children.push(Node::Element(element));
        } else {
            self.document.root.children.push(Node::Element(element));
        }
    }

    /// Add text node
    fn add_text_node(&mut self) {
        if !self.current_text.is_empty() {
            let text_node = TextNode {
                content: self.current_text.clone(),
            };
            
            if let Some(parent) = self.stack.last_mut() {
                parent.children.push(Node::Text(text_node));
            } else {
                self.document.root.children.push(Node::Text(text_node));
            }
        }
        self.current_text.clear();
    }

    /// Check if tag is self-closing
    fn is_self_closing_tag(tag_name: &str) -> bool {
        matches!(
            tag_name,
            "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link" | "meta" | "param" | "source" | "track" | "wbr"
        )
    }
}

impl Default for HtmlParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_html() {
        let mut parser = HtmlParser::new();
        let html = "<html><head><title>Test</title></head><body><h1>Hello</h1></body></html>";
        let document = parser.parse(html).unwrap();
        
        assert_eq!(document.root.children.len(), 1);
        if let Node::Element(html_element) = &document.root.children[0] {
            assert_eq!(html_element.tag_name, "html");
            assert_eq!(html_element.children.len(), 2); // head and body
        }
    }

    #[test]
    fn test_parse_with_attributes() {
        let mut parser = HtmlParser::new();
        let html = r#"<div class="container" id="main">Content</div>"#;
        let document = parser.parse(html).unwrap();
        
        if let Node::Element(div_element) = &document.root.children[0] {
            assert_eq!(div_element.tag_name, "div");
            assert_eq!(div_element.attributes.get("class"), Some(&"container".to_string()));
            assert_eq!(div_element.attributes.get("id"), Some(&"main".to_string()));
            assert_eq!(div_element.children.len(), 1);
            
            if let Node::Text(text_node) = &div_element.children[0] {
                assert_eq!(text_node.content, "Content");
            }
        }
    }

    #[test]
    fn test_parse_self_closing_tags() {
        let mut parser = HtmlParser::new();
        let html = r#"<img src="test.jpg" alt="Test" /><br />"#;
        let document = parser.parse(html).unwrap();
        

        
        assert_eq!(document.root.children.len(), 2);
        
        if let Node::Element(img_element) = &document.root.children[0] {
            assert_eq!(img_element.tag_name, "img");
            assert_eq!(img_element.attributes.get("src"), Some(&"test.jpg".to_string()));
            assert_eq!(img_element.attributes.get("alt"), Some(&"Test".to_string()));
        }
        
        if let Node::Element(br_element) = &document.root.children[1] {
            assert_eq!(br_element.tag_name, "br");
        }
    }

    #[test]
    fn test_parse_nested_elements() {
        let mut parser = HtmlParser::new();
        let html = r#"<div><p>Paragraph <strong>bold</strong> text</p></div>"#;
        let document = parser.parse(html).unwrap();
        
        if let Node::Element(div_element) = &document.root.children[0] {
            assert_eq!(div_element.tag_name, "div");
            assert_eq!(div_element.children.len(), 1);
            
            if let Node::Element(p_element) = &div_element.children[0] {
                assert_eq!(p_element.tag_name, "p");
                assert_eq!(p_element.children.len(), 3); // text, strong, text
                
                if let Node::Text(text1) = &p_element.children[0] {
                    assert_eq!(text1.content, "Paragraph ");
                }
                
                if let Node::Element(strong_element) = &p_element.children[1] {
                    assert_eq!(strong_element.tag_name, "strong");
                    if let Node::Text(text2) = &strong_element.children[0] {
                        assert_eq!(text2.content, "bold");
                    }
                }
                
                if let Node::Text(text3) = &p_element.children[2] {
                    assert_eq!(text3.content, " text");
                }
            }
        }
    }

    #[test]
    fn test_parse_with_comments() {
        let mut parser = HtmlParser::new();
        let html = r#"<div>Content<!-- This is a comment -->more content</div>"#;
        let document = parser.parse(html).unwrap();
        
        if let Node::Element(div_element) = &document.root.children[0] {
            assert_eq!(div_element.tag_name, "div");
            assert_eq!(div_element.children.len(), 2); // text nodes, comment is ignored
            
            if let Node::Text(text1) = &div_element.children[0] {
                assert_eq!(text1.content, "Content");
            }
            
            if let Node::Text(text2) = &div_element.children[1] {
                assert_eq!(text2.content, "more content");
            }
        }
    }
}
