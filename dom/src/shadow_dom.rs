use std::collections::HashMap;
use crate::dom::{Element, Node};
use crate::events::{EventTarget, EventManager, EventType, EventListener, Event};
use crate::error::Result;

/// Shadow DOM mode
#[derive(Debug, Clone, PartialEq)]
pub enum ShadowRootMode {
    Open,
    Closed,
}

/// Shadow DOM root
#[derive(Debug)]
pub struct ShadowRoot {
    /// The host element
    pub host: Element,
    /// Shadow root mode (open or closed)
    pub mode: ShadowRootMode,
    /// Child nodes in the shadow tree
    pub children: Vec<Node>,
    /// Event manager for the shadow root
    pub event_manager: EventManager,
    /// Delegates focus to the host element
    pub delegates_focus: bool,
    /// Slot assignment map
    pub slot_assignment: HashMap<String, Vec<Element>>,
}

impl ShadowRoot {
    /// Create a new shadow root
    pub fn new(host: Element, mode: ShadowRootMode) -> Self {
        let host_id = host.id.clone();
        Self {
            host,
            mode,
            children: Vec::new(),
            event_manager: EventManager::new(host_id),
            delegates_focus: false,
            slot_assignment: HashMap::new(),
        }
    }
    
    /// Add a child node to the shadow root
    pub fn append_child(&mut self, child: Node) {
        self.children.push(child);
    }
    
    /// Remove a child node from the shadow root
    pub fn remove_child(&mut self, child: &Node) -> Option<Node> {
        if let Some(index) = self.children.iter().position(|c| c == child) {
            Some(self.children.remove(index))
        } else {
            None
        }
    }
    
    /// Get all child nodes
    pub fn child_nodes(&self) -> &[Node] {
        &self.children
    }
    
    /// Find elements by tag name in the shadow tree
    pub fn get_elements_by_tag_name(&self, tag_name: &str) -> Vec<&Element> {
        let mut elements = Vec::new();
        self.collect_elements_by_tag_name(&self.children, tag_name, &mut elements);
        elements
    }
    
    /// Find elements by class name in the shadow tree
    pub fn get_elements_by_class_name(&self, class_name: &str) -> Vec<&Element> {
        let mut elements = Vec::new();
        self.collect_elements_by_class_name(&self.children, class_name, &mut elements);
        elements
    }
    
    /// Find element by ID in the shadow tree
    pub fn get_element_by_id(&self, id: &str) -> Option<&Element> {
        self.find_element_by_id(&self.children, id)
    }
    
    /// Query selector in the shadow tree
    pub fn query_selector(&self, selector: &str) -> Option<&Element> {
        // This is a simplified implementation
        // In a real browser, you would use the CSS selector engine
        for child in &self.children {
            if let Some(element) = self.query_selector_in_node(child, selector) {
                return Some(element);
            }
        }
        None
    }
    
    /// Query selector all in the shadow tree
    pub fn query_selector_all(&self, selector: &str) -> Vec<&Element> {
        let mut elements = Vec::new();
        self.collect_elements_by_selector(&self.children, selector, &mut elements);
        elements
    }
    
    /// Assign slots to slot elements
    pub fn assign_slots(&mut self) {
        self.slot_assignment.clear();
        
        // Find all slot elements in the shadow tree
        let slot_elements = self.get_elements_by_tag_name("slot");
        
        // Collect slot assignments to avoid borrow checker issues
        let mut assignments = Vec::new();
        for slot_element in slot_elements {
            let slot_name = slot_element.get_attribute("name").map_or("", |v| v).to_string();
            
            // Find assigned elements in the host
            let assigned_elements = self.find_assigned_elements(&slot_name);
            
            assignments.push((slot_name, assigned_elements));
        }
        
        // Apply all assignments
        for (slot_name, assigned_elements) in assignments {
            self.slot_assignment.insert(slot_name, assigned_elements);
        }
    }
    
    /// Get assigned elements for a slot
    pub fn get_assigned_elements(&self, slot_name: &str) -> Vec<&Element> {
        if let Some(assigned) = self.slot_assignment.get(slot_name) {
            assigned.iter().map(|e| e as &Element).collect()
        } else {
            Vec::new()
        }
    }
    
    /// Check if the shadow root is closed
    pub fn is_closed(&self) -> bool {
        matches!(self.mode, ShadowRootMode::Closed)
    }
    
    /// Check if the shadow root is open
    pub fn is_open(&self) -> bool {
        matches!(self.mode, ShadowRootMode::Open)
    }
    
    /// Set delegates focus
    pub fn set_delegates_focus(&mut self, delegates: bool) {
        self.delegates_focus = delegates;
    }
    
    /// Get delegates focus
    pub fn delegates_focus(&self) -> bool {
        self.delegates_focus
    }
    
    // Helper methods for element collection
    fn collect_elements_by_tag_name<'a>(
        &'a self,
        nodes: &'a [Node],
        tag_name: &str,
        elements: &mut Vec<&'a Element>,
    ) {
        for node in nodes {
            match node {
                Node::Element(element) => {
                    if element.tag_name.to_lowercase() == tag_name.to_lowercase() {
                        elements.push(element);
                    }
                    self.collect_elements_by_tag_name(&element.children, tag_name, elements);
                }
                _ => {}
            }
        }
    }
    
    fn collect_elements_by_class_name<'a>(
        &'a self,
        nodes: &'a [Node],
        class_name: &str,
        elements: &mut Vec<&'a Element>,
    ) {
        for node in nodes {
            match node {
                Node::Element(element) => {
                    if let Some(classes) = element.get_attribute("class") {
                        if classes.split_whitespace().any(|c| c == class_name) {
                            elements.push(element);
                        }
                    }
                    self.collect_elements_by_class_name(&element.children, class_name, elements);
                }
                _ => {}
            }
        }
    }
    
    fn find_element_by_id<'a>(&'a self, nodes: &'a [Node], id: &str) -> Option<&'a Element> {
        for node in nodes {
            match node {
                Node::Element(element) => {
                    if let Some(element_id) = element.get_attribute("id") {
                        if element_id == id {
                            return Some(element);
                        }
                    }
                    if let Some(found) = self.find_element_by_id(&element.children, id) {
                        return Some(found);
                    }
                }
                _ => {}
            }
        }
        None
    }
    
    fn query_selector_in_node<'a>(&'a self, node: &'a Node, selector: &str) -> Option<&'a Element> {
        match node {
            Node::Element(element) => {
                // Simple tag name matching for now
                if selector == element.tag_name {
                    return Some(element);
                }
                
                // Check children
                for child in &element.children {
                    if let Some(found) = self.query_selector_in_node(child, selector) {
                        return Some(found);
                    }
                }
            }
            _ => {}
        }
        None
    }
    
    fn collect_elements_by_selector<'a>(
        &'a self,
        nodes: &'a [Node],
        selector: &str,
        elements: &mut Vec<&'a Element>,
    ) {
        for node in nodes {
            if let Some(element) = self.query_selector_in_node(node, selector) {
                elements.push(element);
            }
        }
    }
    
    fn find_assigned_elements(&self, slot_name: &str) -> Vec<Element> {
        let mut assigned = Vec::new();
        
        // Find elements in the host that have slot attribute matching the slot name
        self.collect_assigned_elements(&self.host.children, slot_name, &mut assigned);
        
        assigned
    }
    
    fn collect_assigned_elements(&self, nodes: &[Node], slot_name: &str, assigned: &mut Vec<Element>) {
        for node in nodes {
            match node {
                Node::Element(element) => {
                    if let Some(element_slot) = element.get_attribute("slot") {
                        if element_slot == slot_name {
                            assigned.push(element.clone());
                        }
                    }
                    self.collect_assigned_elements(&element.children, slot_name, assigned);
                }
                _ => {}
            }
        }
    }
}

impl EventTarget for ShadowRoot {
    fn add_event_listener(&mut self, event_type: EventType, listener: EventListener, use_capture: bool) -> Result<()> {
        // The EventManager doesn't take use_capture as a parameter, it's stored in the listener
        self.event_manager.add_event_listener(event_type, listener)
    }
    
    fn remove_event_listener(&mut self, event_type: EventType, listener: EventListener, use_capture: bool) -> Result<()> {
        self.event_manager.remove_event_listener(event_type, &listener.id, use_capture)
    }
    
    async fn dispatch_event(&mut self, event: Event) -> Result<bool> {
        self.event_manager.dispatch_event(event).await
    }
    
    fn get_event_listeners(&self, event_type: &EventType, use_capture: bool) -> Vec<EventListener> {
        self.event_manager.get_event_listeners(event_type, use_capture)
    }
}

/// Shadow DOM manager for handling shadow roots
#[derive(Debug)]
pub struct ShadowDomManager {
    /// Shadow roots by host element ID
    shadow_roots: HashMap<String, ShadowRoot>,
}

impl ShadowDomManager {
    /// Create a new shadow DOM manager
    pub fn new() -> Self {
        Self {
            shadow_roots: HashMap::new(),
        }
    }
    
    /// Attach a shadow root to an element
    pub fn attach_shadow(&mut self, element: &Element, mode: ShadowRootMode) -> Option<&mut ShadowRoot> {
        let element_id = element.id.clone();
        
        // Check if element already has a shadow root
        if self.shadow_roots.contains_key(&element_id) {
            return None;
        }
        
        let shadow_root = ShadowRoot::new(element.clone(), mode);
        self.shadow_roots.insert(element_id.clone(), shadow_root);
        
        self.shadow_roots.get_mut(&element_id)
    }
    
    /// Get shadow root for an element
    pub fn get_shadow_root(&self, element: &Element) -> Option<&ShadowRoot> {
        self.shadow_roots.get(&element.id)
    }
    
    /// Get mutable shadow root for an element
    pub fn get_shadow_root_mut(&mut self, element: &Element) -> Option<&mut ShadowRoot> {
        self.shadow_roots.get_mut(&element.id)
    }
    
    /// Remove shadow root from an element
    pub fn remove_shadow_root(&mut self, element: &Element) -> Option<ShadowRoot> {
        self.shadow_roots.remove(&element.id)
    }
    
    /// Check if element has a shadow root
    pub fn has_shadow_root(&self, element: &Element) -> bool {
        self.shadow_roots.contains_key(&element.id)
    }
    
    /// Get all shadow roots
    pub fn get_all_shadow_roots(&self) -> &HashMap<String, ShadowRoot> {
        &self.shadow_roots
    }
    
    /// Assign slots for all shadow roots
    pub fn assign_all_slots(&mut self) {
        for shadow_root in self.shadow_roots.values_mut() {
            shadow_root.assign_slots();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom::{Element, Node};

    #[test]
    fn test_shadow_root_creation() {
        let host = Element::new("div".to_string());
        let shadow_root = ShadowRoot::new(host.clone(), ShadowRootMode::Open);
        
        assert_eq!(shadow_root.host, host);
        assert_eq!(shadow_root.mode, ShadowRootMode::Open);
        assert!(shadow_root.children.is_empty());
        assert!(!shadow_root.is_closed());
        assert!(shadow_root.is_open());
    }

    #[test]
    fn test_shadow_root_closed_mode() {
        let host = Element::new("div".to_string());
        let shadow_root = ShadowRoot::new(host, ShadowRootMode::Closed);
        
        assert!(shadow_root.is_closed());
        assert!(!shadow_root.is_open());
    }

    #[test]
    fn test_shadow_root_append_child() {
        let host = Element::new("div".to_string());
        let mut shadow_root = ShadowRoot::new(host, ShadowRootMode::Open);
        
        let child = Element::new("span".to_string());
        shadow_root.append_child(Node::Element(child));
        
        assert_eq!(shadow_root.children.len(), 1);
    }

    #[test]
    fn test_shadow_root_query_selector() {
        let host = Element::new("div".to_string());
        let mut shadow_root = ShadowRoot::new(host, ShadowRootMode::Open);
        
        let span = Element::new("span".to_string());
        shadow_root.append_child(Node::Element(span));
        
        let found = shadow_root.query_selector("span");
        assert!(found.is_some());
        assert_eq!(found.unwrap().tag_name, "span");
    }

    #[test]
    fn test_shadow_dom_manager() {
        let mut manager = ShadowDomManager::new();
        let element = Element::new("div".to_string());
        
        // Attach shadow root
        let shadow_root = manager.attach_shadow(&element, ShadowRootMode::Open);
        assert!(shadow_root.is_some());
        
        // Check if element has shadow root
        assert!(manager.has_shadow_root(&element));
        
        // Get shadow root
        let retrieved = manager.get_shadow_root(&element);
        assert!(retrieved.is_some());
        
        // Try to attach another shadow root (should fail)
        let second_shadow = manager.attach_shadow(&element, ShadowRootMode::Closed);
        assert!(second_shadow.is_none());
    }

    #[test]
    fn test_slot_assignment() {
        let mut host = Element::new("div".to_string());
        
        // Create an element with slot attribute first
        let mut slotted = Element::new("p".to_string());
        slotted.set_attribute("slot".to_string(), "content".to_string());
        host.append_child(Node::Element(slotted));
        
        // Then create shadow root with the host that has the slotted element
        let mut shadow_root = ShadowRoot::new(host.clone(), ShadowRootMode::Open);
        
        // Create a slot element
        let mut slot = Element::new("slot".to_string());
        slot.set_attribute("name".to_string(), "content".to_string());
        shadow_root.append_child(Node::Element(slot));
        
        // Assign slots
        shadow_root.assign_slots();
        
        // Check assigned elements
        let assigned = shadow_root.get_assigned_elements("content");
        assert_eq!(assigned.len(), 1);
        assert_eq!(assigned[0].tag_name, "p");
    }

    #[test]
    fn test_delegates_focus() {
        let host = Element::new("div".to_string());
        let mut shadow_root = ShadowRoot::new(host, ShadowRootMode::Open);
        
        assert!(!shadow_root.delegates_focus());
        
        shadow_root.set_delegates_focus(true);
        assert!(shadow_root.delegates_focus());
    }
}
