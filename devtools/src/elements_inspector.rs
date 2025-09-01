use crate::error::{Error, Result};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// DOM Elements Inspector
pub struct ElementsInspector {
    /// Selected element
    selected_element: Arc<RwLock<Option<ElementInfo>>>,
    /// DOM tree cache
    dom_tree: Arc<RwLock<DomTree>>,
    /// Element highlighting
    highlighting: Arc<RwLock<ElementHighlighting>>,
    /// Attribute editor
    attribute_editor: Arc<RwLock<AttributeEditor>>,
    /// Inspector state
    state: InspectorState,
}

/// DOM tree representation
pub struct DomTree {
    /// Root element
    root: Option<ElementNode>,
    /// Element cache by ID
    elements: HashMap<String, ElementNode>,
    /// Node counter
    node_counter: u64,
}

/// Element node in DOM tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementNode {
    /// Node ID
    pub id: String,
    /// Node type
    pub node_type: NodeType,
    /// Tag name (for elements)
    pub tag_name: Option<String>,
    /// Node name
    pub node_name: String,
    /// Node value (for text nodes)
    pub node_value: Option<String>,
    /// Attributes
    pub attributes: HashMap<String, String>,
    /// Child nodes
    pub children: Vec<String>, // Node IDs
    /// Parent node
    pub parent: Option<String>, // Node ID
    /// Computed styles
    pub computed_styles: HashMap<String, String>,
    /// Bounding box
    pub bounding_box: Option<BoundingBox>,
    /// Is visible
    pub is_visible: bool,
    /// Is selected
    pub is_selected: bool,
    /// Is expanded in tree view
    pub is_expanded: bool,
}

/// Node type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum NodeType {
    /// Element node
    Element,
    /// Text node
    Text,
    /// Comment node
    Comment,
    /// Document node
    Document,
    /// Document fragment
    DocumentFragment,
}

/// Bounding box
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Width
    pub width: f64,
    /// Height
    pub height: f64,
}

/// Element information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementInfo {
    /// Element ID
    pub id: String,
    /// Tag name
    pub tag_name: String,
    /// Element name
    pub name: String,
    /// Attributes
    pub attributes: Vec<AttributeInfo>,
    /// Computed styles
    pub computed_styles: HashMap<String, String>,
    /// Bounding box
    pub bounding_box: Option<BoundingBox>,
    /// Element path
    pub path: Vec<String>,
    /// Child count
    pub child_count: usize,
    /// Text content
    pub text_content: Option<String>,
}

/// Attribute information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeInfo {
    /// Attribute name
    pub name: String,
    /// Attribute value
    pub value: String,
    /// Is editable
    pub is_editable: bool,
    /// Is required
    pub is_required: bool,
}

/// Element highlighting
pub struct ElementHighlighting {
    /// Highlighted elements
    highlighted_elements: HashMap<String, HighlightInfo>,
    /// Highlight styles
    highlight_styles: HighlightStyles,
    /// Is highlighting enabled
    enabled: bool,
}

/// Highlight information
#[derive(Debug, Clone)]
pub struct HighlightInfo {
    /// Element ID
    pub element_id: String,
    /// Highlight color
    pub color: String,
    /// Highlight border
    pub border: String,
    /// Highlight background
    pub background: String,
    /// Highlight opacity
    pub opacity: f64,
    /// Highlight duration
    pub duration: u64,
}

/// Highlight styles
#[derive(Debug, Clone)]
pub struct HighlightStyles {
    /// Default highlight color
    pub default_color: String,
    /// Selected element color
    pub selected_color: String,
    /// Hover element color
    pub hover_color: String,
    /// Error element color
    pub error_color: String,
    /// Warning element color
    pub warning_color: String,
}

/// Attribute editor
pub struct AttributeEditor {
    /// Editable attributes
    editable_attributes: HashMap<String, EditableAttribute>,
    /// Attribute history
    attribute_history: Vec<AttributeChange>,
    /// Undo stack
    undo_stack: Vec<AttributeChange>,
    /// Redo stack
    redo_stack: Vec<AttributeChange>,
}

/// Editable attribute
#[derive(Debug, Clone)]
pub struct EditableAttribute {
    /// Element ID
    pub element_id: String,
    /// Attribute name
    pub name: String,
    /// Current value
    pub value: String,
    /// Original value
    pub original_value: String,
    /// Is modified
    pub is_modified: bool,
    /// Validation rules
    pub validation_rules: Vec<ValidationRule>,
}

/// Attribute change
#[derive(Debug, Clone)]
pub struct AttributeChange {
    /// Change ID
    pub id: String,
    /// Element ID
    pub element_id: String,
    /// Attribute name
    pub attribute_name: String,
    /// Old value
    pub old_value: String,
    /// New value
    pub new_value: String,
    /// Timestamp
    pub timestamp: u64,
}

/// Validation rule
#[derive(Debug, Clone)]
pub struct ValidationRule {
    /// Rule type
    pub rule_type: ValidationRuleType,
    /// Rule value
    pub value: String,
    /// Error message
    pub error_message: String,
}

/// Validation rule type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValidationRuleType {
    /// Required field
    Required,
    /// Pattern match
    Pattern,
    /// Minimum length
    MinLength,
    /// Maximum length
    MaxLength,
    /// Custom validation
    Custom,
}

/// Inspector state
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum InspectorState {
    /// Inspector is idle
    Idle,
    /// Inspector is selecting
    Selecting,
    /// Inspector is editing
    Editing,
    /// Inspector is highlighting
    Highlighting,
}

/// DOM tree event
#[derive(Debug, Clone)]
pub struct DomTreeEvent {
    /// Event type
    pub event_type: DomTreeEventType,
    /// Element ID
    pub element_id: String,
    /// Event data
    pub data: DomTreeEventData,
    /// Timestamp
    pub timestamp: u64,
}

/// DOM tree event type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DomTreeEventType {
    /// Element selected
    ElementSelected,
    /// Element expanded
    ElementExpanded,
    /// Element collapsed
    ElementCollapsed,
    /// Element modified
    ElementModified,
    /// Element added
    ElementAdded,
    /// Element removed
    ElementRemoved,
}

/// DOM tree event data
#[derive(Debug, Clone)]
pub enum DomTreeEventData {
    /// No data
    None,
    /// Element data
    Element(ElementInfo),
    /// Attribute data
    Attribute(AttributeInfo),
    /// String data
    String(String),
}

impl ElementsInspector {
    /// Create new elements inspector
    pub fn new() -> Self {
        Self {
            selected_element: Arc::new(RwLock::new(None)),
            dom_tree: Arc::new(RwLock::new(DomTree::new())),
            highlighting: Arc::new(RwLock::new(ElementHighlighting::new())),
            attribute_editor: Arc::new(RwLock::new(AttributeEditor::new())),
            state: InspectorState::Idle,
        }
    }

    /// Load DOM tree from page
    pub async fn load_dom_tree(&self, page_dom: &str) -> Result<()> {
        let mut dom_tree = self.dom_tree.write();
        dom_tree.load_from_html(page_dom)?;
        
        Ok(())
    }

    /// Select element by ID
    pub async fn select_element(&self, element_id: &str) -> Result<()> {
        let dom_tree = self.dom_tree.read();
        
        if let Some(element) = dom_tree.get_element(element_id) {
            let mut selected = self.selected_element.write();
            *selected = Some(ElementInfo::from_element_node(element));
            
            // Highlight selected element
            self.highlight_element(element_id, "selected").await?;
            
            Ok(())
        } else {
            Err(Error::inspector(format!("Element '{}' not found", element_id)))
        }
    }

    /// Select element by coordinates
    pub async fn select_element_at_position(&self, x: f64, y: f64) -> Result<()> {
        let dom_tree = self.dom_tree.read();
        
        if let Some(element_id) = dom_tree.find_element_at_position(x, y) {
            self.select_element(&element_id).await?;
            Ok(())
        } else {
            Err(Error::inspector("No element found at position".to_string()))
        }
    }

    /// Get selected element
    pub async fn get_selected_element(&self) -> Result<Option<ElementInfo>> {
        let selected = self.selected_element.read();
        Ok(selected.clone())
    }

    /// Expand element in tree view
    pub async fn expand_element(&self, element_id: &str) -> Result<()> {
        let mut dom_tree = self.dom_tree.write();
        dom_tree.expand_element(element_id)?;
        
        Ok(())
    }

    /// Collapse element in tree view
    pub async fn collapse_element(&self, element_id: &str) -> Result<()> {
        let mut dom_tree = self.dom_tree.write();
        dom_tree.collapse_element(element_id)?;
        
        Ok(())
    }

    /// Get DOM tree as JSON
    pub async fn get_dom_tree_json(&self) -> Result<String> {
        let dom_tree = self.dom_tree.read();
        let tree_data = dom_tree.to_json()?;
        
        Ok(serde_json::to_string_pretty(&tree_data)?)
    }

    /// Get element path
    pub async fn get_element_path(&self, element_id: &str) -> Result<Vec<String>> {
        let dom_tree = self.dom_tree.read();
        
        if let Some(path) = dom_tree.get_element_path(element_id) {
            Ok(path)
        } else {
            Err(Error::inspector(format!("Element '{}' not found", element_id)))
        }
    }

    /// Search elements by selector
    pub async fn search_elements(&self, selector: &str) -> Result<Vec<ElementInfo>> {
        let dom_tree = self.dom_tree.read();
        let elements = dom_tree.search_elements(selector)?;
        
        Ok(elements.into_iter().map(ElementInfo::from_element_node).collect())
    }

    /// Search elements by text
    pub async fn search_elements_by_text(&self, text: &str) -> Result<Vec<ElementInfo>> {
        let dom_tree = self.dom_tree.read();
        let elements = dom_tree.search_elements_by_text(text)?;
        
        Ok(elements.into_iter().map(ElementInfo::from_element_node).collect())
    }

    /// Highlight element
    pub async fn highlight_element(&self, element_id: &str, highlight_type: &str) -> Result<()> {
        let mut highlighting = self.highlighting.write();
        highlighting.highlight_element(element_id, highlight_type)?;
        
        Ok(())
    }

    /// Remove highlight
    pub async fn remove_highlight(&self, element_id: &str) -> Result<()> {
        let mut highlighting = self.highlighting.write();
        highlighting.remove_highlight(element_id)?;
        
        Ok(())
    }

    /// Clear all highlights
    pub async fn clear_highlights(&self) -> Result<()> {
        let mut highlighting = self.highlighting.write();
        highlighting.clear_all()?;
        
        Ok(())
    }

    /// Edit attribute
    pub async fn edit_attribute(&self, element_id: &str, attribute_name: &str, new_value: &str) -> Result<()> {
        let mut attribute_editor = self.attribute_editor.write();
        attribute_editor.edit_attribute(element_id, attribute_name, new_value)?;
        
        // Update DOM tree
        let mut dom_tree = self.dom_tree.write();
        dom_tree.update_attribute(element_id, attribute_name, new_value)?;
        
        Ok(())
    }

    /// Get editable attributes
    pub async fn get_editable_attributes(&self, element_id: &str) -> Result<Vec<EditableAttribute>> {
        let attribute_editor = self.attribute_editor.read();
        Ok(attribute_editor.get_editable_attributes(element_id))
    }

    /// Undo last attribute change
    pub async fn undo_attribute_change(&self) -> Result<()> {
        let mut attribute_editor = self.attribute_editor.write();
        attribute_editor.undo()?;
        
        Ok(())
    }

    /// Redo last attribute change
    pub async fn redo_attribute_change(&self) -> Result<()> {
        let mut attribute_editor = self.attribute_editor.write();
        attribute_editor.redo()?;
        
        Ok(())
    }

    /// Get element statistics
    pub async fn get_element_stats(&self) -> Result<ElementStats> {
        let dom_tree = self.dom_tree.read();
        Ok(dom_tree.get_stats())
    }

    /// Get inspector state
    pub fn get_state(&self) -> InspectorState {
        self.state
    }

    /// Set inspector state
    pub fn set_state(&mut self, state: InspectorState) {
        self.state = state;
    }
}

impl DomTree {
    /// Create new DOM tree
    pub fn new() -> Self {
        Self {
            root: None,
            elements: HashMap::new(),
            node_counter: 0,
        }
    }

    /// Load DOM tree from HTML
    pub fn load_from_html(&mut self, html: &str) -> Result<()> {
        // This is a simplified implementation
        // In a real implementation, you would parse the HTML and build the DOM tree
        
        // Create root document node
        let root_id = self.create_node_id();
        let root_node = ElementNode {
            id: root_id.clone(),
            node_type: NodeType::Document,
            tag_name: None,
            node_name: "#document".to_string(),
            node_value: None,
            attributes: HashMap::new(),
            children: Vec::new(),
            parent: None,
            computed_styles: HashMap::new(),
            bounding_box: None,
            is_visible: true,
            is_selected: false,
            is_expanded: true,
        };
        
        self.elements.insert(root_id.clone(), root_node);
        self.root = Some(root_id);
        
        Ok(())
    }

    /// Get element by ID
    pub fn get_element(&self, element_id: &str) -> Option<&ElementNode> {
        self.elements.get(element_id)
    }

    /// Find element at position
    pub fn find_element_at_position(&self, x: f64, y: f64) -> Option<String> {
        // This is a simplified implementation
        // In a real implementation, you would traverse the DOM tree and check bounding boxes
        
        for (id, element) in &self.elements {
            if let Some(bbox) = &element.bounding_box {
                if x >= bbox.x && x <= bbox.x + bbox.width &&
                   y >= bbox.y && y <= bbox.y + bbox.height {
                    return Some(id.clone());
                }
            }
        }
        
        None
    }

    /// Expand element
    pub fn expand_element(&mut self, element_id: &str) -> Result<()> {
        if let Some(element) = self.elements.get_mut(element_id) {
            element.is_expanded = true;
            Ok(())
        } else {
            Err(Error::inspector(format!("Element '{}' not found", element_id)))
        }
    }

    /// Collapse element
    pub fn collapse_element(&mut self, element_id: &str) -> Result<()> {
        if let Some(element) = self.elements.get_mut(element_id) {
            element.is_expanded = false;
            Ok(())
        } else {
            Err(Error::inspector(format!("Element '{}' not found", element_id)))
        }
    }

    /// Get element path
    pub fn get_element_path(&self, element_id: &str) -> Option<Vec<String>> {
        let mut path = Vec::new();
        let mut current_id = element_id;
        
        while let Some(element) = self.elements.get(current_id) {
            path.push(element.node_name.clone());
            
            if let Some(parent_id) = &element.parent {
                current_id = parent_id;
            } else {
                break;
            }
        }
        
        path.reverse();
        Some(path)
    }

    /// Search elements by selector
    pub fn search_elements(&self, selector: &str) -> Result<Vec<&ElementNode>> {
        // This is a simplified implementation
        // In a real implementation, you would parse the CSS selector and match elements
        
        let mut results = Vec::new();
        
        for element in self.elements.values() {
            if element.node_type == NodeType::Element {
                if let Some(tag_name) = &element.tag_name {
                    if tag_name == selector {
                        results.push(element);
                    }
                }
            }
        }
        
        Ok(results)
    }

    /// Search elements by text
    pub fn search_elements_by_text(&self, text: &str) -> Result<Vec<&ElementNode>> {
        let mut results = Vec::new();
        
        for element in self.elements.values() {
            if let Some(node_value) = &element.node_value {
                if node_value.contains(text) {
                    results.push(element);
                }
            }
        }
        
        Ok(results)
    }

    /// Update attribute
    pub fn update_attribute(&mut self, element_id: &str, attribute_name: &str, new_value: &str) -> Result<()> {
        if let Some(element) = self.elements.get_mut(element_id) {
            element.attributes.insert(attribute_name.to_string(), new_value.to_string());
            Ok(())
        } else {
            Err(Error::inspector(format!("Element '{}' not found", element_id)))
        }
    }

    /// Convert to JSON
    pub fn to_json(&self) -> Result<serde_json::Value> {
        let mut tree_data = serde_json::Map::new();
        
        if let Some(root_id) = &self.root {
            if let Some(root_element) = self.elements.get(root_id) {
                tree_data.insert("root".to_string(), self.element_to_json(root_element)?);
            }
        }
        
        Ok(serde_json::Value::Object(tree_data))
    }

    /// Convert element to JSON
    fn element_to_json(&self, element: &ElementNode) -> Result<serde_json::Value> {
        let mut element_data = serde_json::Map::new();
        
        element_data.insert("id".to_string(), serde_json::Value::String(element.id.clone()));
        element_data.insert("nodeType".to_string(), serde_json::Value::String(format!("{:?}", element.node_type)));
        element_data.insert("nodeName".to_string(), serde_json::Value::String(element.node_name.clone()));
        
        if let Some(tag_name) = &element.tag_name {
            element_data.insert("tagName".to_string(), serde_json::Value::String(tag_name.clone()));
        }
        
        if let Some(node_value) = &element.node_value {
            element_data.insert("nodeValue".to_string(), serde_json::Value::String(node_value.clone()));
        }
        
        // Convert attributes
        let mut attributes = serde_json::Map::new();
        for (key, value) in &element.attributes {
            attributes.insert(key.clone(), serde_json::Value::String(value.clone()));
        }
        element_data.insert("attributes".to_string(), serde_json::Value::Object(attributes));
        
        // Convert children
        let mut children = Vec::new();
        for child_id in &element.children {
            if let Some(child_element) = self.elements.get(child_id) {
                children.push(self.element_to_json(child_element)?);
            }
        }
        element_data.insert("children".to_string(), serde_json::Value::Array(children));
        
        element_data.insert("isExpanded".to_string(), serde_json::Value::Bool(element.is_expanded));
        element_data.insert("isSelected".to_string(), serde_json::Value::Bool(element.is_selected));
        
        Ok(serde_json::Value::Object(element_data))
    }

    /// Get statistics
    pub fn get_stats(&self) -> ElementStats {
        let mut stats = ElementStats::default();
        
        for element in self.elements.values() {
            stats.total_elements += 1;
            
            match element.node_type {
                NodeType::Element => stats.element_nodes += 1,
                NodeType::Text => stats.text_nodes += 1,
                NodeType::Comment => stats.comment_nodes += 1,
                NodeType::Document => stats.document_nodes += 1,
                NodeType::DocumentFragment => stats.fragment_nodes += 1,
            }
            
            if element.is_visible {
                stats.visible_elements += 1;
            }
            
            stats.total_attributes += element.attributes.len();
        }
        
        stats
    }

    /// Create node ID
    fn create_node_id(&mut self) -> String {
        self.node_counter += 1;
        format!("node_{}", self.node_counter)
    }
}

impl ElementHighlighting {
    /// Create new element highlighting
    pub fn new() -> Self {
        Self {
            highlighted_elements: HashMap::new(),
            highlight_styles: HighlightStyles::default(),
            enabled: true,
        }
    }

    /// Highlight element
    pub fn highlight_element(&mut self, element_id: &str, highlight_type: &str) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }
        
        let color = match highlight_type {
            "selected" => self.highlight_styles.selected_color.clone(),
            "hover" => self.highlight_styles.hover_color.clone(),
            "error" => self.highlight_styles.error_color.clone(),
            "warning" => self.highlight_styles.warning_color.clone(),
            _ => self.highlight_styles.default_color.clone(),
        };
        
        let highlight_info = HighlightInfo {
            element_id: element_id.to_string(),
            color: color.clone(),
            border: format!("2px solid {}", color),
            background: format!("{}20", color), // 20% opacity
            opacity: 0.8,
            duration: 5000, // 5 seconds
        };
        
        self.highlighted_elements.insert(element_id.to_string(), highlight_info);
        
        Ok(())
    }

    /// Remove highlight
    pub fn remove_highlight(&mut self, element_id: &str) -> Result<()> {
        self.highlighted_elements.remove(element_id);
        Ok(())
    }

    /// Clear all highlights
    pub fn clear_all(&mut self) -> Result<()> {
        self.highlighted_elements.clear();
        Ok(())
    }

    /// Get highlighted elements
    pub fn get_highlighted_elements(&self) -> Vec<&HighlightInfo> {
        self.highlighted_elements.values().collect()
    }
}

impl Default for HighlightStyles {
    fn default() -> Self {
        Self {
            default_color: "#007acc".to_string(),
            selected_color: "#ff6b6b".to_string(),
            hover_color: "#4ecdc4".to_string(),
            error_color: "#ff4757".to_string(),
            warning_color: "#ffa502".to_string(),
        }
    }
}

impl AttributeEditor {
    /// Create new attribute editor
    pub fn new() -> Self {
        Self {
            editable_attributes: HashMap::new(),
            attribute_history: Vec::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    /// Edit attribute
    pub fn edit_attribute(&mut self, element_id: &str, attribute_name: &str, new_value: &str) -> Result<()> {
        let change_id = Uuid::new_v4().to_string();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Get old value
        let old_value = self.get_attribute_value(element_id, attribute_name)
            .unwrap_or_else(|| "".to_string());
        
        // Create change record
        let change = AttributeChange {
            id: change_id,
            element_id: element_id.to_string(),
            attribute_name: attribute_name.to_string(),
            old_value,
            new_value: new_value.to_string(),
            timestamp,
        };
        
        // Add to history
        self.attribute_history.push(change.clone());
        
        // Clear redo stack
        self.redo_stack.clear();
        
        // Update editable attribute
        let editable_attr = EditableAttribute {
            element_id: element_id.to_string(),
            name: attribute_name.to_string(),
            value: new_value.to_string(),
            original_value: change.old_value.clone(),
            is_modified: true,
            validation_rules: Vec::new(),
        };
        
        let key = format!("{}:{}", element_id, attribute_name);
        self.editable_attributes.insert(key, editable_attr);
        
        Ok(())
    }

    /// Get editable attributes
    pub fn get_editable_attributes(&self, element_id: &str) -> Vec<EditableAttribute> {
        self.editable_attributes
            .values()
            .filter(|attr| attr.element_id == element_id)
            .cloned()
            .collect()
    }

    /// Undo last change
    pub fn undo(&mut self) -> Result<()> {
        if let Some(change) = self.attribute_history.pop() {
            self.undo_stack.push(change.clone());
            
            // Revert the change
            let key = format!("{}:{}", change.element_id, change.attribute_name);
            if let Some(editable_attr) = self.editable_attributes.get_mut(&key) {
                editable_attr.value = change.old_value.clone();
                editable_attr.is_modified = false;
            }
            
            Ok(())
        } else {
            Err(Error::inspector("No changes to undo".to_string()))
        }
    }

    /// Redo last change
    pub fn redo(&mut self) -> Result<()> {
        if let Some(change) = self.undo_stack.pop() {
            self.attribute_history.push(change.clone());
            
            // Apply the change
            let key = format!("{}:{}", change.element_id, change.attribute_name);
            if let Some(editable_attr) = self.editable_attributes.get_mut(&key) {
                editable_attr.value = change.new_value.clone();
                editable_attr.is_modified = true;
            }
            
            Ok(())
        } else {
            Err(Error::inspector("No changes to redo".to_string()))
        }
    }

    /// Get attribute value
    fn get_attribute_value(&self, element_id: &str, attribute_name: &str) -> Option<String> {
        let key = format!("{}:{}", element_id, attribute_name);
        self.editable_attributes.get(&key).map(|attr| attr.value.clone())
    }
}

impl ElementInfo {
    /// Create from element node
    pub fn from_element_node(node: &ElementNode) -> Self {
        let attributes = node.attributes
            .iter()
            .map(|(name, value)| AttributeInfo {
                name: name.clone(),
                value: value.clone(),
                is_editable: true,
                is_required: false,
            })
            .collect();
        
        Self {
            id: node.id.clone(),
            tag_name: node.tag_name.clone().unwrap_or_default(),
            name: node.node_name.clone(),
            attributes,
            computed_styles: node.computed_styles.clone(),
            bounding_box: node.bounding_box.clone(),
            path: Vec::new(), // Will be populated separately
            child_count: node.children.len(),
            text_content: node.node_value.clone(),
        }
    }
}

/// Element statistics
#[derive(Debug, Clone, Default)]
pub struct ElementStats {
    /// Total elements
    pub total_elements: usize,
    /// Element nodes
    pub element_nodes: usize,
    /// Text nodes
    pub text_nodes: usize,
    /// Comment nodes
    pub comment_nodes: usize,
    /// Document nodes
    pub document_nodes: usize,
    /// Fragment nodes
    pub fragment_nodes: usize,
    /// Visible elements
    pub visible_elements: usize,
    /// Total attributes
    pub total_attributes: usize,
}
