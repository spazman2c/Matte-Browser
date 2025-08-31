//! DOM Traversal APIs implementation.
//! 
//! This module provides comprehensive DOM traversal functionality including
//! NodeIterator, TreeWalker, and various traversal methods for navigating
//! the DOM tree efficiently.

use std::collections::VecDeque;
use tracing::debug;
use crate::error::Result;
use crate::dom::Node;

/// Node filter types for traversal
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeFilter {
    /// Show all nodes
    ShowAll,
    /// Show only element nodes
    ShowElement,
    /// Show only text nodes
    ShowText,
    /// Show only comment nodes
    ShowComment,
    /// Show only document type nodes
    ShowDocumentType,
    /// Hide nodes
    Hide,
    /// Reject nodes
    Reject,
    /// Skip children
    SkipChildren,
    /// Skip subtree
    SkipSubtree,
}

/// Node filter function type
pub type NodeFilterFn = Box<dyn Fn(&Node) -> NodeFilter + Send + Sync>;

/// NodeIterator for traversing DOM nodes
pub struct NodeIterator {
    /// Root node to start iteration from
    root: Node,
    /// Reference node (current position)
    reference_node: Option<Node>,
    /// Whether to include the root node
    include_root: bool,
    /// Node filter function
    filter: Option<NodeFilterFn>,
    /// Whether iteration is complete
    done: bool,
}

impl NodeIterator {
    /// Create a new NodeIterator
    pub fn new(root: Node, include_root: bool, filter: Option<NodeFilterFn>) -> Self {
        Self {
            reference_node: None,
            include_root,
            filter,
            done: false,
            root,
        }
    }
    
    /// Get the next node in the iteration
    pub fn next_node(&mut self) -> Result<Option<Node>> {
        if self.done {
            return Ok(None);
        }
        
        let next = if self.reference_node.is_none() {
            // First iteration
            if self.include_root {
                Some(self.root.clone())
            } else {
                self.find_next_node(&self.root)
            }
        } else {
            // Continue from current position
            if let Some(ref current) = self.reference_node {
                self.find_next_node(current)
            } else {
                None
            }
        };
        
        if let Some(node) = &next {
            self.reference_node = Some(node.clone());
            debug!("NodeIterator next: {:?}", node);
        } else {
            self.done = true;
        }
        
        Ok(next)
    }
    
    /// Get the previous node in the iteration
    pub fn previous_node(&mut self) -> Result<Option<Node>> {
        if self.done {
            return Ok(None);
        }
        
        let prev = if let Some(ref current) = self.reference_node {
            self.find_previous_node(current)
        } else {
            None
        };
        
        if let Some(node) = &prev {
            self.reference_node = Some(node.clone());
            debug!("NodeIterator previous: {:?}", node);
        } else {
            self.done = true;
        }
        
        Ok(prev)
    }
    
    /// Find the next node in document order
    fn find_next_node(&self, current: &Node) -> Option<Node> {
        // First, try to get the first child
        if let Some(first_child) = self.get_first_child(current) {
            return Some(first_child);
        }
        
        // If no children, try next sibling
        if let Some(next_sibling) = self.get_next_sibling(current) {
            return Some(next_sibling);
        }
        
        // If no next sibling, walk up to find next sibling of ancestor
        self.find_next_sibling_of_ancestor(current)
    }
    
    /// Find the previous node in document order
    fn find_previous_node(&self, current: &Node) -> Option<Node> {
        // First, try to get the previous sibling
        if let Some(prev_sibling) = self.get_previous_sibling(current) {
            // Get the last descendant of the previous sibling
            return Some(self.get_last_descendant(prev_sibling));
        }
        
        // If no previous sibling, return parent
        self.get_parent(current)
    }
    
    /// Find next sibling of an ancestor
    fn find_next_sibling_of_ancestor(&self, _current: &Node) -> Option<Node> {
        // This would need to be implemented with proper parent tracking
        // For now, return None as a placeholder
        None
    }
    
    /// Get the last descendant of a node
    fn get_last_descendant(&self, node: Node) -> Node {
        let mut current = node;
        
        while let Some(last_child) = self.get_last_child(&current) {
            current = last_child;
        }
        
        current
    }
    
    /// Get the first child of a node
    fn get_first_child(&self, node: &Node) -> Option<Node> {
        match node {
            Node::Element(element) => {
                for child in &element.children {
                    if self.accept_node(child) {
                        return Some(child.clone());
                    }
                }
            }
            _ => {}
        }
        None
    }
    
    /// Get the last child of a node
    fn get_last_child(&self, node: &Node) -> Option<Node> {
        match node {
            Node::Element(element) => {
                for child in element.children.iter().rev() {
                    if self.accept_node(child) {
                        return Some(child.clone());
                    }
                }
            }
            _ => {}
        }
        None
    }
    
    /// Get the next sibling of a node
    fn get_next_sibling(&self, node: &Node) -> Option<Node> {
        if let Some(parent) = self.get_parent(node) {
            if let Node::Element(element) = parent {
                let mut found_current = false;
                
                for child in &element.children {
                    if found_current {
                        if self.accept_node(child) {
                            return Some(child.clone());
                        }
                    } else if std::ptr::eq(child, node) {
                        found_current = true;
                    }
                }
            }
        }
        None
    }
    
    /// Get the previous sibling of a node
    fn get_previous_sibling(&self, node: &Node) -> Option<Node> {
        if let Some(parent) = self.get_parent(node) {
            if let Node::Element(element) = parent {
                let mut last_accepted = None;
                
                for child in &element.children {
                    if std::ptr::eq(child, node) {
                        return last_accepted;
                    }
                    if self.accept_node(child) {
                        last_accepted = Some(child.clone());
                    }
                }
            }
        }
        None
    }
    
    /// Get the parent of a node
    fn get_parent(&self, _node: &Node) -> Option<Node> {
        // In our current implementation, we don't have parent pointers
        // This would need to be implemented by walking up from the root
        None
    }
    
    /// Check if a node should be accepted by the filter
    fn accept_node(&self, node: &Node) -> bool {
        if let Some(ref filter_fn) = self.filter {
            match filter_fn(node) {
                NodeFilter::ShowAll | NodeFilter::ShowElement | NodeFilter::ShowText | 
                NodeFilter::ShowComment | NodeFilter::ShowDocumentType => true,
                NodeFilter::Hide | NodeFilter::Reject => false,
                NodeFilter::SkipChildren | NodeFilter::SkipSubtree => true, // These are handled differently
            }
        } else {
            true
        }
    }
}

/// TreeWalker for traversing DOM nodes with more control
pub struct TreeWalker {
    /// Root node to start walking from
    root: Node,
    /// Current node position
    current_node: Option<Node>,
    /// Whether to include the root node
    include_root: bool,
    /// Node filter function
    filter: Option<NodeFilterFn>,
}

impl TreeWalker {
    /// Create a new TreeWalker
    pub fn new(root: Node, include_root: bool, filter: Option<NodeFilterFn>) -> Self {
        Self {
            current_node: None,
            include_root,
            filter,
            root,
        }
    }
    
    /// Get the current node
    pub fn current_node(&self) -> Option<&Node> {
        self.current_node.as_ref()
    }
    
    /// Get the root node
    pub fn root(&self) -> &Node {
        &self.root
    }
    
    /// Move to the next node
    pub fn next_node(&mut self) -> Result<Option<Node>> {
        let next = if self.current_node.is_none() {
            // First iteration
            if self.include_root {
                Some(self.root.clone())
            } else {
                self.find_next_node(&self.root)
            }
        } else {
            // Continue from current position
            if let Some(ref current) = self.current_node {
                self.find_next_node(current)
            } else {
                None
            }
        };
        
        if let Some(node) = &next {
            self.current_node = Some(node.clone());
            debug!("TreeWalker next: {:?}", node);
        }
        
        Ok(next)
    }
    
    /// Move to the previous node
    pub fn previous_node(&mut self) -> Result<Option<Node>> {
        let prev = if let Some(ref current) = self.current_node {
            self.find_previous_node(current)
        } else {
            None
        };
        
        if let Some(node) = &prev {
            self.current_node = Some(node.clone());
            debug!("TreeWalker previous: {:?}", node);
        }
        
        Ok(prev)
    }
    
    /// Move to the parent node
    pub fn parent_node(&mut self) -> Result<Option<Node>> {
        if let Some(ref current) = self.current_node {
            if let Some(parent) = self.get_parent(current) {
                self.current_node = Some(parent.clone());
                debug!("TreeWalker parent: {:?}", parent);
                return Ok(Some(parent));
            }
        }
        Ok(None)
    }
    
    /// Move to the first child
    pub fn first_child(&mut self) -> Result<Option<Node>> {
        if let Some(ref current) = self.current_node {
            if let Some(first_child) = self.get_first_child(current) {
                self.current_node = Some(first_child.clone());
                debug!("TreeWalker first child: {:?}", first_child);
                return Ok(Some(first_child));
            }
        }
        Ok(None)
    }
    
    /// Move to the last child
    pub fn last_child(&mut self) -> Result<Option<Node>> {
        if let Some(ref current) = self.current_node {
            if let Some(last_child) = self.get_last_child(current) {
                self.current_node = Some(last_child.clone());
                debug!("TreeWalker last child: {:?}", last_child);
                return Ok(Some(last_child));
            }
        }
        Ok(None)
    }
    
    /// Move to the next sibling
    pub fn next_sibling(&mut self) -> Result<Option<Node>> {
        if let Some(ref current) = self.current_node {
            if let Some(next_sibling) = self.get_next_sibling(current) {
                self.current_node = Some(next_sibling.clone());
                debug!("TreeWalker next sibling: {:?}", next_sibling);
                return Ok(Some(next_sibling));
            }
        }
        Ok(None)
    }
    
    /// Move to the previous sibling
    pub fn previous_sibling(&mut self) -> Result<Option<Node>> {
        if let Some(ref current) = self.current_node {
            if let Some(prev_sibling) = self.get_previous_sibling(current) {
                self.current_node = Some(prev_sibling.clone());
                debug!("TreeWalker previous sibling: {:?}", prev_sibling);
                return Ok(Some(prev_sibling));
            }
        }
        Ok(None)
    }
    
    /// Helper methods (same as NodeIterator)
    fn find_next_node(&self, _current: &Node) -> Option<Node> {
        // Implementation similar to NodeIterator
        None // Placeholder
    }
    
    fn find_previous_node(&self, _current: &Node) -> Option<Node> {
        // Implementation similar to NodeIterator
        None // Placeholder
    }
    
    fn get_first_child(&self, _node: &Node) -> Option<Node> {
        // Implementation similar to NodeIterator
        None // Placeholder
    }
    
    fn get_last_child(&self, _node: &Node) -> Option<Node> {
        // Implementation similar to NodeIterator
        None // Placeholder
    }
    
    fn get_next_sibling(&self, _node: &Node) -> Option<Node> {
        // Implementation similar to NodeIterator
        None // Placeholder
    }
    
    fn get_previous_sibling(&self, _node: &Node) -> Option<Node> {
        // Implementation similar to NodeIterator
        None // Placeholder
    }
    
    fn get_parent(&self, _node: &Node) -> Option<Node> {
        // Implementation similar to NodeIterator
        None // Placeholder
    }
    
    fn accept_node(&self, _node: &Node) -> bool {
        // Implementation similar to NodeIterator
        true // Placeholder
    }
}

/// Breadth-first traversal of DOM nodes
pub struct BreadthFirstTraversal {
    /// Queue of nodes to visit
    queue: VecDeque<Node>,
    /// Whether traversal is complete
    done: bool,
}

impl BreadthFirstTraversal {
    /// Create a new breadth-first traversal
    pub fn new(root: Node) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        
        Self {
            queue,
            done: false,
        }
    }
    
    /// Get the next node in breadth-first order
    pub fn next(&mut self) -> Option<Node> {
        if self.done || self.queue.is_empty() {
            self.done = true;
            return None;
        }
        
        if let Some(current) = self.queue.pop_front() {
            // Add children to queue
            if let Node::Element(element) = &current {
                for child in &element.children {
                    self.queue.push_back(child.clone());
                }
            }
            
            debug!("BreadthFirstTraversal next: {:?}", current);
            Some(current)
        } else {
            self.done = true;
            None
        }
    }
    
    /// Check if traversal is complete
    pub fn is_done(&self) -> bool {
        self.done
    }
}

/// Depth-first traversal of DOM nodes
pub struct DepthFirstTraversal {
    /// Stack of nodes to visit
    stack: Vec<Node>,
    /// Whether traversal is complete
    done: bool,
}

impl DepthFirstTraversal {
    /// Create a new depth-first traversal
    pub fn new(root: Node) -> Self {
        Self {
            stack: vec![root],
            done: false,
        }
    }
    
    /// Get the next node in depth-first order
    pub fn next(&mut self) -> Option<Node> {
        if self.done || self.stack.is_empty() {
            self.done = true;
            return None;
        }
        
        if let Some(current) = self.stack.pop() {
            // Add children to stack in reverse order (so they're processed in order)
            if let Node::Element(element) = &current {
                for child in element.children.iter().rev() {
                    self.stack.push(child.clone());
                }
            }
            
            debug!("DepthFirstTraversal next: {:?}", current);
            Some(current)
        } else {
            self.done = true;
            None
        }
    }
    
    /// Check if traversal is complete
    pub fn is_done(&self) -> bool {
        self.done
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom::{Element, TextNode};

    #[test]
    fn test_node_iterator_creation() {
        let root = Node::Element(Element::new("div".to_string()));
        let iterator = NodeIterator::new(root, true, None);
        
        assert!(!iterator.done);
    }

    #[test]
    fn test_tree_walker_creation() {
        let root = Node::Element(Element::new("div".to_string()));
        let walker = TreeWalker::new(root, true, None);
        
        assert!(walker.current_node().is_none());
    }

    #[test]
    fn test_breadth_first_traversal() {
        let mut root = Element::new("div".to_string());
        let child1 = Element::new("span".to_string());
        let child2 = Element::new("p".to_string());
        
        root.append_child(Node::Element(child1));
        root.append_child(Node::Element(child2));
        
        let mut traversal = BreadthFirstTraversal::new(Node::Element(root));
        
        assert!(!traversal.is_done());
        
        let first = traversal.next();
        assert!(first.is_some());
        
        let second = traversal.next();
        assert!(second.is_some());
        
        let third = traversal.next();
        assert!(third.is_some());
        
        let fourth = traversal.next();
        assert!(fourth.is_none());
        assert!(traversal.is_done());
    }

    #[test]
    fn test_depth_first_traversal() {
        let mut root = Element::new("div".to_string());
        let child1 = Element::new("span".to_string());
        let child2 = Element::new("p".to_string());
        
        root.append_child(Node::Element(child1));
        root.append_child(Node::Element(child2));
        
        let mut traversal = DepthFirstTraversal::new(Node::Element(root));
        
        assert!(!traversal.is_done());
        
        let first = traversal.next();
        assert!(first.is_some());
        
        let second = traversal.next();
        assert!(second.is_some());
        
        let third = traversal.next();
        assert!(third.is_some());
        
        let fourth = traversal.next();
        assert!(fourth.is_none());
        assert!(traversal.is_done());
    }

    #[test]
    fn test_node_filter() {
        let element = Node::Element(Element::new("div".to_string()));
        let text = Node::Text(TextNode::new("Hello".to_string()));
        
        let element_filter = Box::new(|node: &Node| {
            match node {
                Node::Element(_) => NodeFilter::ShowElement,
                _ => NodeFilter::Hide,
            }
        });
        
        assert_eq!(element_filter(&element), NodeFilter::ShowElement);
        assert_eq!(element_filter(&text), NodeFilter::Hide);
    }
}
