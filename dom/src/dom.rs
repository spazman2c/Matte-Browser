//! DOM (Document Object Model) implementation for the Matte browser.
//! 
//! This module provides the core DOM structures and functionality
//! for representing HTML documents as a tree of nodes.

use crate::error::{Error, Result};
use crate::events::{EventManager, EventTarget, EventType, EventListener, Event};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// DOM node types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Node {
    /// Element node (HTML tags)
    Element(Element),
    /// Text node (text content)
    Text(TextNode),
    /// Comment node
    Comment(CommentNode),
    /// Document type node
    DocumentType(DocumentTypeNode),
}

/// HTML element
#[derive(Debug, Clone)]
pub struct Element {
    /// Tag name (e.g., "div", "p", "span")
    pub tag_name: String,
    /// Element attributes
    pub attributes: HashMap<String, String>,
    /// Child nodes
    pub children: Vec<Node>,
    /// Element ID
    pub id: String,
    /// Parent element reference
    pub parent: Option<Arc<RwLock<Element>>>,
    /// Event manager for this element
    pub event_manager: Option<Arc<RwLock<EventManager>>>,
}

impl Element {
    /// Create a new element
    pub fn new(tag_name: String) -> Self {
        let id = format!("element_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
        Self {
            tag_name,
            attributes: HashMap::new(),
            children: Vec::new(),
            id,
            parent: None,
            event_manager: Some(Arc::new(RwLock::new(EventManager::new(id.clone())))),
        }
    }

    /// Get an attribute value
    pub fn get_attribute(&self, name: &str) -> Option<&String> {
        self.attributes.get(name)
    }

    /// Set an attribute
    pub fn set_attribute(&mut self, name: String, value: String) {
        self.attributes.insert(name, value);
    }

    /// Remove an attribute
    pub fn remove_attribute(&mut self, name: &str) -> Option<String> {
        self.attributes.remove(name)
    }

    /// Check if element has an attribute
    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes.contains_key(name)
    }

    /// Get element by ID
    pub fn get_element_by_id(&self, id: &str) -> Option<&Element> {
        if self.get_attribute("id") == Some(&id.to_string()) {
            return Some(self);
        }
        
        for child in &self.children {
            if let Node::Element(element) = child {
                if let Some(found) = element.get_element_by_id(id) {
                    return Some(found);
                }
            }
        }
        
        None
    }

    /// Get elements by tag name
    pub fn get_elements_by_tag_name(&self, tag_name: &str) -> Vec<&Element> {
        let mut elements = Vec::new();
        
        if self.tag_name == tag_name {
            elements.push(self);
        }
        
        for child in &self.children {
            if let Node::Element(element) = child {
                elements.extend(element.get_elements_by_tag_name(tag_name));
            }
        }
        
        elements
    }

    /// Get elements by class name
    pub fn get_elements_by_class_name(&self, class_name: &str) -> Vec<&Element> {
        let mut elements = Vec::new();
        
        if let Some(classes) = self.get_attribute("class") {
            if classes.split_whitespace().any(|c| c == class_name) {
                elements.push(self);
            }
        }
        
        for child in &self.children {
            if let Node::Element(element) = child {
                elements.extend(element.get_elements_by_class_name(class_name));
            }
        }
        
        elements
    }

    /// Append a child node
    pub fn append_child(&mut self, child: Node) {
        self.children.push(child);
    }

    /// Remove a child node
    pub fn remove_child(&mut self, index: usize) -> Option<Node> {
        if index < self.children.len() {
            Some(self.children.remove(index))
        } else {
            None
        }
    }

    /// Insert a child node at a specific position
    pub fn insert_child(&mut self, index: usize, child: Node) -> Result<()> {
        if index <= self.children.len() {
            self.children.insert(index, child);
            Ok(())
        } else {
            Err(Error::InvalidState("Index out of bounds".to_string()))
        }
    }

    /// Get text content of this element and all its descendants
    pub fn text_content(&self) -> String {
        let mut content = String::new();
        
        for child in &self.children {
            match child {
                Node::Text(text_node) => content.push_str(&text_node.content),
                Node::Element(element) => content.push_str(&element.text_content()),
                _ => {}
            }
        }
        
        content
    }

    /// Get inner HTML
    pub fn inner_html(&self) -> String {
        let mut html = String::new();
        
        for child in &self.children {
            match child {
                Node::Text(text_node) => html.push_str(&text_node.content),
                Node::Element(element) => html.push_str(&element.outer_html()),
                Node::Comment(comment) => html.push_str(&format!("<!--{}-->", comment.content)),
                Node::DocumentType(doctype) => html.push_str(&format!("<!DOCTYPE {}>", doctype.name)),
            }
        }
        
        html
    }

    /// Get outer HTML
    pub fn outer_html(&self) -> String {
        let mut html = format!("<{}", self.tag_name);
        
        for (name, value) in &self.attributes {
            html.push_str(&format!(" {}=\"{}\"", name, value));
        }
        
        if self.children.is_empty() && Self::is_self_closing_tag(&self.tag_name) {
            html.push_str(" />");
        } else {
            html.push('>');
            html.push_str(&self.inner_html());
            html.push_str(&format!("</{}>", self.tag_name));
        }
        
        html
    }
}

impl Element {
    /// Check if tag is self-closing
    fn is_self_closing_tag(tag_name: &str) -> bool {
        matches!(
            tag_name,
            "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link" | "meta" | "param" | "source" | "track" | "wbr"
        )
    }
}

/// Text node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextNode {
    /// Text content
    pub content: String,
}

impl TextNode {
    /// Create a new text node
    pub fn new(content: String) -> Self {
        Self { content }
    }

    /// Get text content
    pub fn text_content(&self) -> &str {
        &self.content
    }

    /// Set text content
    pub fn set_text_content(&mut self, content: String) {
        self.content = content;
    }
}

/// Comment node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentNode {
    /// Comment content
    pub content: String,
}

impl CommentNode {
    /// Create a new comment node
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

/// Document type node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentTypeNode {
    /// Document type name
    pub name: String,
    /// Public identifier
    pub public_id: Option<String>,
    /// System identifier
    pub system_id: Option<String>,
}

impl DocumentTypeNode {
    /// Create a new document type node
    pub fn new(name: String) -> Self {
        Self {
            name,
            public_id: None,
            system_id: None,
        }
    }
}

/// HTML document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Document root element
    pub root: Element,
    /// Document title
    pub title: Option<String>,
    /// Document URL
    pub url: Option<String>,
    /// Document character encoding
    pub character_set: String,
}

impl Document {
    /// Create a new document
    pub fn new() -> Self {
        Self {
            root: Element::new("html".to_string()),
            title: None,
            url: None,
            character_set: "UTF-8".to_string(),
        }
    }

    /// Get document title
    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    /// Set document title
    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }

    /// Get document URL
    pub fn url(&self) -> Option<&String> {
        self.url.as_ref()
    }

    /// Set document URL
    pub fn set_url(&mut self, url: String) {
        self.url = Some(url);
    }

    /// Get element by ID
    pub fn get_element_by_id(&self, id: &str) -> Option<&Element> {
        self.root.get_element_by_id(id)
    }

    /// Get elements by tag name
    pub fn get_elements_by_tag_name(&self, tag_name: &str) -> Vec<&Element> {
        self.root.get_elements_by_tag_name(tag_name)
    }

    /// Get elements by class name
    pub fn get_elements_by_class_name(&self, class_name: &str) -> Vec<&Element> {
        self.root.get_elements_by_class_name(class_name)
    }

    /// Get document body element
    pub fn body(&self) -> Option<&Element> {
        for child in &self.root.children {
            if let Node::Element(element) = child {
                if element.tag_name == "body" {
                    return Some(element);
                }
            }
        }
        None
    }

    /// Get document head element
    pub fn head(&self) -> Option<&Element> {
        for child in &self.root.children {
            if let Node::Element(element) = child {
                if element.tag_name == "head" {
                    return Some(element);
                }
            }
        }
        None
    }

    /// Get document HTML as string
    pub fn to_html(&self) -> String {
        format!("<!DOCTYPE html>\n{}", self.root.outer_html())
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

/// DOM traversal utilities
pub struct DomTraversal;

impl DomTraversal {
    /// Traverse DOM tree in depth-first order
    pub fn traverse_dfs<F>(node: &Node, visitor: &mut F)
    where
        F: FnMut(&Node),
    {
        visitor(node);
        
        if let Node::Element(element) = node {
            for child in &element.children {
                Self::traverse_dfs(child, visitor);
            }
        }
    }

    /// Traverse DOM tree in breadth-first order
    pub fn traverse_bfs<F>(root: &Node, visitor: &mut F)
    where
        F: FnMut(&Node),
    {
        use std::collections::VecDeque;
        
        let mut queue = VecDeque::new();
        queue.push_back(root);
        
        while let Some(node) = queue.pop_front() {
            visitor(node);
            
            if let Node::Element(element) = node {
                for child in &element.children {
                    queue.push_back(child);
                }
            }
        }
    }

    /// Find all elements matching a predicate
    pub fn find_elements<F>(root: &Node, predicate: F) -> Vec<&Element>
    where
        F: Fn(&Element) -> bool,
    {
        let mut elements = Vec::new();
        
        if let Node::Element(element) = root {
            if predicate(element) {
                elements.push(element);
            }
            
            for child in &element.children {
                elements.extend(Self::find_elements(child, &predicate));
            }
        }
        
        elements
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_creation() {
        let mut element = Element::new("div".to_string());
        element.set_attribute("class".to_string(), "container".to_string());
        element.set_attribute("id".to_string(), "main".to_string());
        
        assert_eq!(element.tag_name, "div");
        assert_eq!(element.get_attribute("class"), Some(&"container".to_string()));
        assert_eq!(element.get_attribute("id"), Some(&"main".to_string()));
        assert!(element.has_attribute("class"));
    }

    #[test]
    fn test_element_removal() {
        let mut element = Element::new("div".to_string());
        element.set_attribute("class".to_string(), "container".to_string());
        
        assert_eq!(element.remove_attribute("class"), Some("container".to_string()));
        assert_eq!(element.get_attribute("class"), None);
        assert!(!element.has_attribute("class"));
    }

    #[test]
    fn test_text_node() {
        let mut text_node = TextNode::new("Hello, World!".to_string());
        assert_eq!(text_node.text_content(), "Hello, World!");
        
        text_node.set_text_content("Updated text".to_string());
        assert_eq!(text_node.text_content(), "Updated text");
    }

    #[test]
    fn test_document_creation() {
        let mut document = Document::new();
        document.set_title("Test Document".to_string());
        document.set_url("https://example.com".to_string());
        
        assert_eq!(document.title(), Some(&"Test Document".to_string()));
        assert_eq!(document.url(), Some(&"https://example.com".to_string()));
        assert_eq!(document.character_set, "UTF-8");
    }

    #[test]
    fn test_element_by_id() {
        let mut document = Document::new();
        let mut body = Element::new("body".to_string());
        let mut div = Element::new("div".to_string());
        div.set_attribute("id".to_string(), "main".to_string());
        
        body.append_child(Node::Element(div));
        document.root.append_child(Node::Element(body));
        
        let found = document.get_element_by_id("main");
        assert!(found.is_some());
        assert_eq!(found.unwrap().tag_name, "div");
    }

    #[test]
    fn test_elements_by_tag_name() {
        let mut document = Document::new();
        let mut body = Element::new("body".to_string());
        
        let div1 = Element::new("div".to_string());
        let div2 = Element::new("div".to_string());
        let p = Element::new("p".to_string());
        
        body.append_child(Node::Element(div1));
        body.append_child(Node::Element(div2));
        body.append_child(Node::Element(p));
        document.root.append_child(Node::Element(body));
        
        let divs = document.get_elements_by_tag_name("div");
        assert_eq!(divs.len(), 2);
        
        let paragraphs = document.get_elements_by_tag_name("p");
        assert_eq!(paragraphs.len(), 1);
    }

    #[test]
    fn test_elements_by_class_name() {
        let mut document = Document::new();
        let mut body = Element::new("body".to_string());
        
        let mut div1 = Element::new("div".to_string());
        div1.set_attribute("class".to_string(), "container highlight".to_string());
        
        let mut div2 = Element::new("div".to_string());
        div2.set_attribute("class".to_string(), "container".to_string());
        
        let p = Element::new("p".to_string());
        
        body.append_child(Node::Element(div1));
        body.append_child(Node::Element(div2));
        body.append_child(Node::Element(p));
        document.root.append_child(Node::Element(body));
        
        let containers = document.get_elements_by_class_name("container");
        assert_eq!(containers.len(), 2);
        
        let highlights = document.get_elements_by_class_name("highlight");
        assert_eq!(highlights.len(), 1);
    }

    #[test]
    fn test_text_content() {
        let mut element = Element::new("div".to_string());
        element.append_child(Node::Text(TextNode::new("Hello ".to_string())));
        
        let mut strong = Element::new("strong".to_string());
        strong.append_child(Node::Text(TextNode::new("World".to_string())));
        element.append_child(Node::Element(strong));
        
        element.append_child(Node::Text(TextNode::new("!".to_string())));
        
        assert_eq!(element.text_content(), "Hello World!");
    }

    #[test]
    fn test_outer_html() {
        let mut element = Element::new("div".to_string());
        element.set_attribute("class".to_string(), "container".to_string());
        element.append_child(Node::Text(TextNode::new("Content".to_string())));
        
        let html = element.outer_html();
        assert!(html.contains("<div"));
        assert!(html.contains("class=\"container\""));
        assert!(html.contains("Content"));
        assert!(html.contains("</div>"));
    }

    #[test]
    fn test_self_closing_tags() {
        let mut img = Element::new("img".to_string());
        img.set_attribute("src".to_string(), "test.jpg".to_string());
        
        let html = img.outer_html();
        assert!(html.contains("<img"));
        assert!(html.contains("src=\"test.jpg\""));
        assert!(html.contains("/>"));
        assert!(!html.contains("</img>"));
    }
}

impl EventTarget for Element {
    /// Add an event listener
    fn add_event_listener(&mut self, event_type: EventType, listener: EventListener, use_capture: bool) -> Result<()> {
        if let Some(event_manager) = &self.event_manager {
            let mut manager = event_manager.blocking_write();
            manager.add_event_listener(event_type, listener)
        } else {
            Err(Error::ConfigError("Event manager not available".to_string()))
        }
    }
    
    /// Remove an event listener
    fn remove_event_listener(&mut self, event_type: EventType, listener: EventListener, use_capture: bool) -> Result<()> {
        if let Some(event_manager) = &self.event_manager {
            let mut manager = event_manager.blocking_write();
            manager.remove_event_listener(event_type, &listener.id, use_capture)
        } else {
            Err(Error::ConfigError("Event manager not available".to_string()))
        }
    }
    
    /// Dispatch an event
    fn dispatch_event(&mut self, event: Event) -> Result<bool> {
        if let Some(event_manager) = &self.event_manager {
            let mut manager = event_manager.blocking_write();
            manager.dispatch_event(event)
        } else {
            Err(Error::ConfigError("Event manager not available".to_string()))
        }
    }
    
    /// Get event listeners for a specific event type
    fn get_event_listeners(&self, event_type: &EventType, use_capture: bool) -> Vec<&EventListener> {
        if let Some(event_manager) = &self.event_manager {
            let manager = event_manager.blocking_read();
            manager.get_event_listeners(event_type, use_capture)
        } else {
            Vec::new()
        }
    }
}
