use crate::error::{Error, Result};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Styles Inspector
pub struct StylesInspector {
    /// Computed styles cache
    computed_styles: Arc<RwLock<HashMap<String, ComputedStyles>>>,
    /// Style sheets
    style_sheets: Arc<RwLock<Vec<StyleSheet>>>,
    /// Style editor
    style_editor: Arc<RwLock<StyleEditor>>,
    /// Box model display
    box_model: Arc<RwLock<BoxModelDisplay>>,
    /// Layout overlays
    layout_overlays: Arc<RwLock<LayoutOverlays>>,
    /// Inspector state
    state: StylesInspectorState,
}

/// Computed styles for an element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputedStyles {
    /// Element ID
    pub element_id: String,
    /// Computed styles
    pub styles: HashMap<String, StyleProperty>,
    /// Inherited styles
    pub inherited_styles: HashMap<String, StyleProperty>,
    /// User agent styles
    pub user_agent_styles: HashMap<String, StyleProperty>,
    /// Author styles
    pub author_styles: HashMap<String, StyleProperty>,
    /// Pseudo-element styles
    pub pseudo_styles: HashMap<String, HashMap<String, StyleProperty>>,
}

/// Style property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleProperty {
    /// Property name
    pub name: String,
    /// Property value
    pub value: String,
    /// Property priority
    pub priority: PropertyPriority,
    /// Source rule
    pub source_rule: Option<SourceRule>,
    /// Is inherited
    pub is_inherited: bool,
    /// Is important
    pub is_important: bool,
}

/// Property priority
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PropertyPriority {
    /// User agent styles
    UserAgent = 0,
    /// User styles
    User = 1,
    /// Author styles
    Author = 2,
    /// Author important
    AuthorImportant = 3,
    /// User important
    UserImportant = 4,
}

/// Source rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceRule {
    /// Rule type
    pub rule_type: RuleType,
    /// Selector
    pub selector: String,
    /// Style sheet URL
    pub style_sheet_url: Option<String>,
    /// Line number
    pub line_number: Option<u32>,
    /// Column number
    pub column_number: Option<u32>,
}

/// Rule type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RuleType {
    /// CSS rule
    CssRule,
    /// Inline style
    InlineStyle,
    /// Computed style
    ComputedStyle,
    /// User agent style
    UserAgentStyle,
}

/// Style sheet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleSheet {
    /// Style sheet ID
    pub id: String,
    /// Style sheet URL
    pub url: Option<String>,
    /// Style sheet title
    pub title: Option<String>,
    /// Is disabled
    pub is_disabled: bool,
    /// Rules
    pub rules: Vec<CssRule>,
    /// Source text
    pub source_text: Option<String>,
}

/// CSS rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CssRule {
    /// Rule ID
    pub id: String,
    /// Rule type
    pub rule_type: CssRuleType,
    /// Selector text
    pub selector_text: String,
    /// Style text
    pub style_text: String,
    /// Properties
    pub properties: HashMap<String, CssProperty>,
    /// Line number
    pub line_number: Option<u32>,
    /// Column number
    pub column_number: Option<u32>,
    /// Is enabled
    pub is_enabled: bool,
}

/// CSS rule type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CssRuleType {
    /// Style rule
    StyleRule,
    /// Import rule
    ImportRule,
    /// Media rule
    MediaRule,
    /// Font face rule
    FontFaceRule,
    /// Page rule
    PageRule,
    /// Keyframes rule
    KeyframesRule,
    /// Keyframe rule
    KeyframeRule,
}

/// CSS property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CssProperty {
    /// Property name
    pub name: String,
    /// Property value
    pub value: String,
    /// Is important
    pub is_important: bool,
    /// Is valid
    pub is_valid: bool,
    /// Parsed value
    pub parsed_value: Option<CssValue>,
}

/// CSS value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CssValue {
    /// Keyword value
    Keyword(String),
    /// Length value
    Length { value: f64, unit: LengthUnit },
    /// Color value
    Color { r: u8, g: u8, b: u8, a: f64 },
    /// URL value
    Url(String),
    /// Function value
    Function { name: String, arguments: Vec<CssValue> },
    /// List value
    List(Vec<CssValue>),
}

/// Length unit
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LengthUnit {
    /// Pixels
    Px,
    /// Em
    Em,
    /// Rem
    Rem,
    /// Percentage
    Percent,
    /// Viewport width
    Vw,
    /// Viewport height
    Vh,
    /// Points
    Pt,
    /// Inches
    In,
    /// Centimeters
    Cm,
    /// Millimeters
    Mm,
}

/// Style editor
pub struct StyleEditor {
    /// Editable styles
    editable_styles: HashMap<String, EditableStyle>,
    /// Style history
    style_history: Vec<StyleChange>,
    /// Undo stack
    undo_stack: Vec<StyleChange>,
    /// Redo stack
    redo_stack: Vec<StyleChange>,
    /// Auto-complete suggestions
    auto_complete: AutoComplete,
}

/// Editable style
#[derive(Debug, Clone)]
pub struct EditableStyle {
    /// Element ID
    pub element_id: String,
    /// Property name
    pub property_name: String,
    /// Current value
    pub value: String,
    /// Original value
    pub original_value: String,
    /// Is modified
    pub is_modified: bool,
    /// Validation rules
    pub validation_rules: Vec<StyleValidationRule>,
    /// Suggestions
    pub suggestions: Vec<String>,
}

/// Style change
#[derive(Debug, Clone)]
pub struct StyleChange {
    /// Change ID
    pub id: String,
    /// Element ID
    pub element_id: String,
    /// Property name
    pub property_name: String,
    /// Old value
    pub old_value: String,
    /// New value
    pub new_value: String,
    /// Timestamp
    pub timestamp: u64,
}

/// Style validation rule
#[derive(Debug, Clone)]
pub struct StyleValidationRule {
    /// Rule type
    pub rule_type: StyleValidationRuleType,
    /// Rule value
    pub value: String,
    /// Error message
    pub error_message: String,
}

/// Style validation rule type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StyleValidationRuleType {
    /// Required field
    Required,
    /// Valid CSS value
    ValidCssValue,
    /// Valid color
    ValidColor,
    /// Valid length
    ValidLength,
    /// Valid number
    ValidNumber,
    /// Custom validation
    Custom,
}

/// Auto-complete
pub struct AutoComplete {
    /// CSS properties
    css_properties: Vec<String>,
    /// CSS values
    css_values: HashMap<String, Vec<String>>,
    /// Color names
    color_names: Vec<String>,
    /// Units
    units: Vec<String>,
}

/// Box model display
pub struct BoxModelDisplay {
    /// Box model data
    box_models: HashMap<String, BoxModel>,
    /// Display settings
    display_settings: BoxModelDisplaySettings,
    /// Is enabled
    enabled: bool,
}

/// Box model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxModel {
    /// Element ID
    pub element_id: String,
    /// Content box
    pub content: BoxDimensions,
    /// Padding box
    pub padding: BoxDimensions,
    /// Border box
    pub border: BoxDimensions,
    /// Margin box
    pub margin: BoxDimensions,
    /// Computed styles
    pub computed_styles: HashMap<String, String>,
}

/// Box dimensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxDimensions {
    /// Top
    pub top: f64,
    /// Right
    pub right: f64,
    /// Bottom
    pub bottom: f64,
    /// Left
    pub left: f64,
    /// Width
    pub width: f64,
    /// Height
    pub height: f64,
}

/// Box model display settings
#[derive(Debug, Clone)]
pub struct BoxModelDisplaySettings {
    /// Show content box
    pub show_content: bool,
    /// Show padding box
    pub show_padding: bool,
    /// Show border box
    pub show_border: bool,
    /// Show margin box
    pub show_margin: bool,
    /// Content box color
    pub content_color: String,
    /// Padding box color
    pub padding_color: String,
    /// Border box color
    pub border_color: String,
    /// Margin box color
    pub margin_color: String,
}

/// Layout overlays
pub struct LayoutOverlays {
    /// Active overlays
    active_overlays: HashMap<String, LayoutOverlay>,
    /// Overlay types
    overlay_types: HashMap<String, OverlayType>,
    /// Is enabled
    enabled: bool,
}

/// Layout overlay
#[derive(Debug, Clone)]
pub struct LayoutOverlay {
    /// Overlay ID
    pub id: String,
    /// Element ID
    pub element_id: String,
    /// Overlay type
    pub overlay_type: String,
    /// Overlay data
    pub data: OverlayData,
    /// Is visible
    pub is_visible: bool,
}

/// Overlay data
#[derive(Debug, Clone)]
pub enum OverlayData {
    /// Grid overlay
    Grid { columns: Vec<f64>, rows: Vec<f64>, gaps: GridGaps },
    /// Flexbox overlay
    Flexbox { direction: FlexDirection, alignment: FlexAlignment },
    /// Box model overlay
    BoxModel(BoxModel),
    /// Custom overlay
    Custom(serde_json::Value),
}

/// Grid gaps
#[derive(Debug, Clone)]
pub struct GridGaps {
    /// Column gap
    pub column_gap: f64,
    /// Row gap
    pub row_gap: f64,
}

/// Flex direction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexDirection {
    /// Row
    Row,
    /// Column
    Column,
    /// Row reverse
    RowReverse,
    /// Column reverse
    ColumnReverse,
}

/// Flex alignment
#[derive(Debug, Clone)]
pub struct FlexAlignment {
    /// Justify content
    pub justify_content: String,
    /// Align items
    pub align_items: String,
    /// Align content
    pub align_content: String,
}

/// Overlay type
#[derive(Debug, Clone)]
pub struct OverlayType {
    /// Type name
    pub name: String,
    /// Type description
    pub description: String,
    /// Type icon
    pub icon: String,
    /// Type color
    pub color: String,
    /// Type enabled
    pub enabled: bool,
}

/// Styles inspector state
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum StylesInspectorState {
    /// Inspector is idle
    Idle,
    /// Inspector is editing
    Editing,
    /// Inspector is showing box model
    ShowingBoxModel,
    /// Inspector is showing overlays
    ShowingOverlays,
}

impl StylesInspector {
    /// Create new styles inspector
    pub fn new() -> Self {
        Self {
            computed_styles: Arc::new(RwLock::new(HashMap::new())),
            style_sheets: Arc::new(RwLock::new(Vec::new())),
            style_editor: Arc::new(RwLock::new(StyleEditor::new())),
            box_model: Arc::new(RwLock::new(BoxModelDisplay::new())),
            layout_overlays: Arc::new(RwLock::new(LayoutOverlays::new())),
            state: StylesInspectorState::Idle,
        }
    }

    /// Get computed styles for element
    pub async fn get_computed_styles(&self, element_id: &str) -> Result<ComputedStyles> {
        let computed_styles = self.computed_styles.read();
        
        if let Some(styles) = computed_styles.get(element_id) {
            Ok(styles.clone())
        } else {
            Err(Error::inspector(format!("Computed styles not found for element '{}'", element_id)))
        }
    }

    /// Get all computed styles
    pub async fn get_all_computed_styles(&self) -> Result<HashMap<String, ComputedStyles>> {
        let computed_styles = self.computed_styles.read();
        Ok(computed_styles.clone())
    }

    /// Get style sheets
    pub async fn get_style_sheets(&self) -> Result<Vec<StyleSheet>> {
        let style_sheets = self.style_sheets.read();
        Ok(style_sheets.clone())
    }

    /// Get CSS rules for element
    pub async fn get_css_rules_for_element(&self, element_id: &str) -> Result<Vec<CssRule>> {
        let style_sheets = self.style_sheets.read();
        let mut matching_rules = Vec::new();
        
        for style_sheet in style_sheets.iter() {
            for rule in &style_sheet.rules {
                // This is a simplified implementation
                // In a real implementation, you would check if the rule matches the element
                if rule.selector_text.contains("div") || rule.selector_text.contains("span") {
                    matching_rules.push(rule.clone());
                }
            }
        }
        
        Ok(matching_rules)
    }

    /// Edit style property
    pub async fn edit_style_property(&self, element_id: &str, property_name: &str, new_value: &str) -> Result<()> {
        let mut style_editor = self.style_editor.write();
        style_editor.edit_style_property(element_id, property_name, new_value)?;
        
        // Update computed styles
        let mut computed_styles = self.computed_styles.write();
        if let Some(styles) = computed_styles.get_mut(element_id) {
            let property = StyleProperty {
                name: property_name.to_string(),
                value: new_value.to_string(),
                priority: PropertyPriority::Author,
                source_rule: None,
                is_inherited: false,
                is_important: false,
            };
            styles.styles.insert(property_name.to_string(), property);
        }
        
        Ok(())
    }

    /// Get editable styles
    pub async fn get_editable_styles(&self, element_id: &str) -> Result<Vec<EditableStyle>> {
        let style_editor = self.style_editor.read();
        Ok(style_editor.get_editable_styles(element_id))
    }

    /// Get style suggestions
    pub async fn get_style_suggestions(&self, property_name: &str, partial_value: &str) -> Result<Vec<String>> {
        let style_editor = self.style_editor.read();
        Ok(style_editor.get_suggestions(property_name, partial_value))
    }

    /// Undo last style change
    pub async fn undo_style_change(&self) -> Result<()> {
        let mut style_editor = self.style_editor.write();
        style_editor.undo()?;
        
        Ok(())
    }

    /// Redo last style change
    pub async fn redo_style_change(&self) -> Result<()> {
        let mut style_editor = self.style_editor.write();
        style_editor.redo()?;
        
        Ok(())
    }

    /// Get box model for element
    pub async fn get_box_model(&self, element_id: &str) -> Result<BoxModel> {
        let box_model = self.box_model.read();
        
        if let Some(model) = box_model.get_box_model(element_id) {
            Ok(model.clone())
        } else {
            Err(Error::inspector(format!("Box model not found for element '{}'", element_id)))
        }
    }

    /// Show box model
    pub async fn show_box_model(&self, element_id: &str) -> Result<()> {
        let mut box_model = self.box_model.write();
        box_model.show_box_model(element_id)?;
        
        Ok(())
    }

    /// Hide box model
    pub async fn hide_box_model(&self, element_id: &str) -> Result<()> {
        let mut box_model = self.box_model.write();
        box_model.hide_box_model(element_id)?;
        
        Ok(())
    }

    /// Get box model display settings
    pub async fn get_box_model_display_settings(&self) -> Result<BoxModelDisplaySettings> {
        let box_model = self.box_model.read();
        Ok(box_model.get_display_settings())
    }

    /// Update box model display settings
    pub async fn update_box_model_display_settings(&self, settings: BoxModelDisplaySettings) -> Result<()> {
        let mut box_model = self.box_model.write();
        box_model.update_display_settings(settings)?;
        
        Ok(())
    }

    /// Add layout overlay
    pub async fn add_layout_overlay(&self, element_id: &str, overlay_type: &str, data: OverlayData) -> Result<()> {
        let mut layout_overlays = self.layout_overlays.write();
        layout_overlays.add_overlay(element_id, overlay_type, data)?;
        
        Ok(())
    }

    /// Remove layout overlay
    pub async fn remove_layout_overlay(&self, overlay_id: &str) -> Result<()> {
        let mut layout_overlays = self.layout_overlays.write();
        layout_overlays.remove_overlay(overlay_id)?;
        
        Ok(())
    }

    /// Get active overlays
    pub async fn get_active_overlays(&self) -> Result<Vec<LayoutOverlay>> {
        let layout_overlays = self.layout_overlays.read();
        Ok(layout_overlays.get_active_overlays())
    }

    /// Get overlay types
    pub async fn get_overlay_types(&self) -> Result<Vec<OverlayType>> {
        let layout_overlays = self.layout_overlays.read();
        Ok(layout_overlays.get_overlay_types())
    }

    /// Validate CSS property
    pub async fn validate_css_property(&self, property_name: &str, value: &str) -> Result<bool> {
        let style_editor = self.style_editor.read();
        Ok(style_editor.validate_property(property_name, value))
    }

    /// Get CSS property info
    pub async fn get_css_property_info(&self, property_name: &str) -> Result<CssPropertyInfo> {
        let style_editor = self.style_editor.read();
        Ok(style_editor.get_property_info(property_name))
    }

    /// Get inspector state
    pub fn get_state(&self) -> StylesInspectorState {
        self.state
    }

    /// Set inspector state
    pub fn set_state(&mut self, state: StylesInspectorState) {
        self.state = state;
    }
}

impl StyleEditor {
    /// Create new style editor
    pub fn new() -> Self {
        Self {
            editable_styles: HashMap::new(),
            style_history: Vec::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            auto_complete: AutoComplete::new(),
        }
    }

    /// Edit style property
    pub fn edit_style_property(&mut self, element_id: &str, property_name: &str, new_value: &str) -> Result<()> {
        let change_id = Uuid::new_v4().to_string();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Get old value
        let old_value = self.get_style_property_value(element_id, property_name)
            .unwrap_or_else(|| "".to_string());
        
        // Create change record
        let change = StyleChange {
            id: change_id,
            element_id: element_id.to_string(),
            property_name: property_name.to_string(),
            old_value,
            new_value: new_value.to_string(),
            timestamp,
        };
        
        // Add to history
        self.style_history.push(change.clone());
        
        // Clear redo stack
        self.redo_stack.clear();
        
        // Update editable style
        let editable_style = EditableStyle {
            element_id: element_id.to_string(),
            property_name: property_name.to_string(),
            value: new_value.to_string(),
            original_value: change.old_value.clone(),
            is_modified: true,
            validation_rules: Vec::new(),
            suggestions: self.auto_complete.get_suggestions(property_name, new_value),
        };
        
        let key = format!("{}:{}", element_id, property_name);
        self.editable_styles.insert(key, editable_style);
        
        Ok(())
    }

    /// Get editable styles
    pub fn get_editable_styles(&self, element_id: &str) -> Vec<EditableStyle> {
        self.editable_styles
            .values()
            .filter(|style| style.element_id == element_id)
            .cloned()
            .collect()
    }

    /// Get suggestions
    pub fn get_suggestions(&self, property_name: &str, partial_value: &str) -> Vec<String> {
        self.auto_complete.get_suggestions(property_name, partial_value)
    }

    /// Undo last change
    pub fn undo(&mut self) -> Result<()> {
        if let Some(change) = self.style_history.pop() {
            self.undo_stack.push(change.clone());
            
            // Revert the change
            let key = format!("{}:{}", change.element_id, change.property_name);
            if let Some(editable_style) = self.editable_styles.get_mut(&key) {
                editable_style.value = change.old_value.clone();
                editable_style.is_modified = false;
            }
            
            Ok(())
        } else {
            Err(Error::inspector("No changes to undo".to_string()))
        }
    }

    /// Redo last change
    pub fn redo(&mut self) -> Result<()> {
        if let Some(change) = self.undo_stack.pop() {
            self.style_history.push(change.clone());
            
            // Apply the change
            let key = format!("{}:{}", change.element_id, change.property_name);
            if let Some(editable_style) = self.editable_styles.get_mut(&key) {
                editable_style.value = change.new_value.clone();
                editable_style.is_modified = true;
            }
            
            Ok(())
        } else {
            Err(Error::inspector("No changes to redo".to_string()))
        }
    }

    /// Validate property
    pub fn validate_property(&self, property_name: &str, value: &str) -> bool {
        // This is a simplified implementation
        // In a real implementation, you would validate CSS properties
        !value.is_empty()
    }

    /// Get property info
    pub fn get_property_info(&self, property_name: &str) -> CssPropertyInfo {
        // This is a simplified implementation
        // In a real implementation, you would return actual CSS property information
        CssPropertyInfo {
            name: property_name.to_string(),
            description: format!("CSS property: {}", property_name),
            syntax: "".to_string(),
            initial_value: "".to_string(),
            applies_to: "".to_string(),
            inherited: false,
            animation_type: "".to_string(),
        }
    }

    /// Get style property value
    fn get_style_property_value(&self, element_id: &str, property_name: &str) -> Option<String> {
        let key = format!("{}:{}", element_id, property_name);
        self.editable_styles.get(&key).map(|style| style.value.clone())
    }
}

impl AutoComplete {
    /// Create new auto-complete
    pub fn new() -> Self {
        let mut css_properties = vec![
            "color", "background-color", "font-size", "margin", "padding",
            "border", "width", "height", "display", "position", "top",
            "left", "right", "bottom", "z-index", "opacity", "transform",
        ];
        css_properties.sort();
        
        let mut css_values = HashMap::new();
        css_values.insert("color".to_string(), vec![
            "red", "green", "blue", "black", "white", "transparent",
            "#ff0000", "#00ff00", "#0000ff", "rgb(255, 0, 0)",
        ]);
        css_values.insert("display".to_string(), vec![
            "block", "inline", "inline-block", "flex", "grid", "none",
        ]);
        css_values.insert("position".to_string(), vec![
            "static", "relative", "absolute", "fixed", "sticky",
        ]);
        
        let color_names = vec![
            "red", "green", "blue", "black", "white", "yellow", "orange",
            "purple", "pink", "brown", "gray", "cyan", "magenta",
        ];
        
        let units = vec![
            "px", "em", "rem", "%", "vw", "vh", "pt", "in", "cm", "mm",
        ];
        
        Self {
            css_properties,
            css_values,
            color_names,
            units,
        }
    }

    /// Get suggestions
    pub fn get_suggestions(&self, property_name: &str, partial_value: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // Add property suggestions
        if property_name.is_empty() {
            suggestions.extend(self.css_properties.iter().filter(|p| p.starts_with(partial_value)).cloned());
        } else {
            // Add value suggestions
            if let Some(values) = self.css_values.get(property_name) {
                suggestions.extend(values.iter().filter(|v| v.starts_with(partial_value)).cloned());
            }
            
            // Add color suggestions
            if property_name.contains("color") {
                suggestions.extend(self.color_names.iter().filter(|c| c.starts_with(partial_value)).cloned());
            }
            
            // Add unit suggestions
            if partial_value.chars().any(|c| c.is_numeric()) {
                suggestions.extend(self.units.iter().map(|u| format!("{}{}", partial_value, u)));
            }
        }
        
        suggestions
    }
}

impl BoxModelDisplay {
    /// Create new box model display
    pub fn new() -> Self {
        Self {
            box_models: HashMap::new(),
            display_settings: BoxModelDisplaySettings::default(),
            enabled: true,
        }
    }

    /// Get box model
    pub fn get_box_model(&self, element_id: &str) -> Option<&BoxModel> {
        self.box_models.get(element_id)
    }

    /// Show box model
    pub fn show_box_model(&mut self, element_id: &str) -> Result<()> {
        // This is a simplified implementation
        // In a real implementation, you would create a box model for the element
        
        let box_model = BoxModel {
            element_id: element_id.to_string(),
            content: BoxDimensions {
                top: 0.0,
                right: 100.0,
                bottom: 100.0,
                left: 0.0,
                width: 100.0,
                height: 100.0,
            },
            padding: BoxDimensions {
                top: 10.0,
                right: 110.0,
                bottom: 110.0,
                left: 10.0,
                width: 120.0,
                height: 120.0,
            },
            border: BoxDimensions {
                top: 12.0,
                right: 112.0,
                bottom: 112.0,
                left: 12.0,
                width: 124.0,
                height: 124.0,
            },
            margin: BoxDimensions {
                top: 20.0,
                right: 120.0,
                bottom: 120.0,
                left: 20.0,
                width: 140.0,
                height: 140.0,
            },
            computed_styles: HashMap::new(),
        };
        
        self.box_models.insert(element_id.to_string(), box_model);
        
        Ok(())
    }

    /// Hide box model
    pub fn hide_box_model(&mut self, element_id: &str) -> Result<()> {
        self.box_models.remove(element_id);
        Ok(())
    }

    /// Get display settings
    pub fn get_display_settings(&self) -> BoxModelDisplaySettings {
        self.display_settings.clone()
    }

    /// Update display settings
    pub fn update_display_settings(&mut self, settings: BoxModelDisplaySettings) -> Result<()> {
        self.display_settings = settings;
        Ok(())
    }
}

impl Default for BoxModelDisplaySettings {
    fn default() -> Self {
        Self {
            show_content: true,
            show_padding: true,
            show_border: true,
            show_margin: true,
            content_color: "#ff6b6b".to_string(),
            padding_color: "#4ecdc4".to_string(),
            border_color: "#45b7d1".to_string(),
            margin_color: "#96ceb4".to_string(),
        }
    }
}

impl LayoutOverlays {
    /// Create new layout overlays
    pub fn new() -> Self {
        let mut overlay_types = HashMap::new();
        overlay_types.insert("grid".to_string(), OverlayType {
            name: "Grid".to_string(),
            description: "CSS Grid overlay".to_string(),
            icon: "grid".to_string(),
            color: "#ff6b6b".to_string(),
            enabled: true,
        });
        overlay_types.insert("flexbox".to_string(), OverlayType {
            name: "Flexbox".to_string(),
            description: "CSS Flexbox overlay".to_string(),
            icon: "flexbox".to_string(),
            color: "#4ecdc4".to_string(),
            enabled: true,
        });
        overlay_types.insert("box-model".to_string(), OverlayType {
            name: "Box Model".to_string(),
            description: "CSS Box Model overlay".to_string(),
            icon: "box-model".to_string(),
            color: "#45b7d1".to_string(),
            enabled: true,
        });
        
        Self {
            active_overlays: HashMap::new(),
            overlay_types,
            enabled: true,
        }
    }

    /// Add overlay
    pub fn add_overlay(&mut self, element_id: &str, overlay_type: &str, data: OverlayData) -> Result<()> {
        let overlay_id = Uuid::new_v4().to_string();
        
        let overlay = LayoutOverlay {
            id: overlay_id.clone(),
            element_id: element_id.to_string(),
            overlay_type: overlay_type.to_string(),
            data,
            is_visible: true,
        };
        
        self.active_overlays.insert(overlay_id, overlay);
        
        Ok(())
    }

    /// Remove overlay
    pub fn remove_overlay(&mut self, overlay_id: &str) -> Result<()> {
        self.active_overlays.remove(overlay_id);
        Ok(())
    }

    /// Get active overlays
    pub fn get_active_overlays(&self) -> Vec<LayoutOverlay> {
        self.active_overlays.values().cloned().collect()
    }

    /// Get overlay types
    pub fn get_overlay_types(&self) -> Vec<OverlayType> {
        self.overlay_types.values().cloned().collect()
    }
}

/// CSS property information
#[derive(Debug, Clone)]
pub struct CssPropertyInfo {
    /// Property name
    pub name: String,
    /// Property description
    pub description: String,
    /// Property syntax
    pub syntax: String,
    /// Initial value
    pub initial_value: String,
    /// Applies to
    pub applies_to: String,
    /// Is inherited
    pub inherited: bool,
    /// Animation type
    pub animation_type: String,
}
