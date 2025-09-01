//! Accessibility module for Matte Browser
//! 
//! This module provides comprehensive accessibility support including accessibility tree,
//! ARIA support, focus management, navigation, and input handling for keyboard, mouse,
//! touch, and gesture input.

pub mod error;
pub mod accessibility_tree;
pub mod input_handler;

pub use error::{Error, Result};
pub use accessibility_tree::{
    AccessibilityTree, AccessibilityNode, AccessibilityRole, AccessibilityState,
    BoundingBox, LiveRegion, AutoComplete, HasPopup, Orientation, Sort, Current,
    DropEffect, FocusManager, NavigationManager, NavigationMode, NavigationEvent,
    NavigationEventType, NavigationAction, AriaManager, AriaAttribute,
    AriaAttributeType, AriaState, AriaStateType, AriaProperty, AriaPropertyType,
    AriaLandmark, AccessibilityStats,
};
pub use input_handler::{
    InputHandler, KeyboardHandler, KeyCode, KeyState, ModifierKey, KeyBinding,
    KeyAction, KeyboardShortcut, InputMethod, AutoRepeatSettings, MouseHandler,
    MousePosition, MouseButton, ButtonState, MouseWheel, WheelDeltaMode,
    MouseSensitivity, MouseAcceleration, TouchHandler, TouchPoint, TouchPosition,
    TouchRadius, TouchState, TouchGesture, TouchGestureType, GestureState,
    TouchSensitivity, MultiTouchSupport, GestureHandler, GestureRecognizer,
    GestureRecognizerType, RecognizerState, RecognizerSettings, GestureEvent,
    GestureEventType, GestureEventData, SwipeDirection, GestureSettings,
    InputEventQueue, InputEvent, InputEventType, InputEventData, KeyboardEventData,
    MouseEventData, TouchEventData, InputSource, EventHandler, EventFilter,
    EventFilterType, EventFilterCriteria, QueueSettings, InputState,
};

/// Accessibility Manager that combines accessibility tree and input handling
pub struct AccessibilityManager {
    /// Accessibility tree
    accessibility_tree: Arc<RwLock<AccessibilityTree>>,
    /// Input handler
    input_handler: Arc<RwLock<InputHandler>>,
    /// Accessibility state
    state: AccessibilityManagerState,
}

use std::sync::Arc;
use parking_lot::RwLock;

impl AccessibilityManager {
    /// Create new accessibility manager
    pub fn new() -> Self {
        Self {
            accessibility_tree: Arc::new(RwLock::new(AccessibilityTree::new())),
            input_handler: Arc::new(RwLock::new(InputHandler::new())),
            state: AccessibilityManagerState::Enabled,
        }
    }

    /// Get accessibility tree
    pub fn accessibility_tree(&self) -> Arc<RwLock<AccessibilityTree>> {
        self.accessibility_tree.clone()
    }

    /// Get input handler
    pub fn input_handler(&self) -> Arc<RwLock<InputHandler>> {
        self.input_handler.clone()
    }

    /// Enable accessibility
    pub async fn enable_accessibility(&mut self) -> Result<()> {
        self.state = AccessibilityManagerState::Enabled;
        
        // Initialize accessibility tree
        self.initialize_accessibility_tree().await?;
        
        // Initialize input handler
        self.initialize_input_handler().await?;
        
        Ok(())
    }

    /// Disable accessibility
    pub async fn disable_accessibility(&mut self) -> Result<()> {
        self.state = AccessibilityManagerState::Disabled;
        
        // Clean up resources
        self.cleanup_accessibility().await?;
        
        Ok(())
    }

    /// Get accessibility state
    pub fn get_state(&self) -> AccessibilityManagerState {
        self.state
    }

    /// Set accessibility state
    pub fn set_state(&mut self, state: AccessibilityManagerState) {
        self.state = state;
    }

    /// Handle input event with accessibility support
    pub async fn handle_input_event(&self, event_type: InputEventType, event_data: InputEventData) -> Result<()> {
        // Handle input event
        let input_handler = self.input_handler.read();
        match event_data {
            InputEventData::Keyboard(keyboard_data) => {
                input_handler.handle_keyboard_event(keyboard_data).await?;
            }
            InputEventData::Mouse(mouse_data) => {
                input_handler.handle_mouse_event(mouse_data).await?;
            }
            InputEventData::Touch(touch_data) => {
                input_handler.handle_touch_event(touch_data).await?;
            }
            InputEventData::Gesture(gesture_data) => {
                input_handler.handle_gesture_event(gesture_data).await?;
            }
        }
        
        // Update accessibility tree based on input
        self.update_accessibility_from_input(event_type, event_data).await?;
        
        Ok(())
    }

    /// Navigate accessibility tree
    pub async fn navigate_accessibility(&self, navigation_type: NavigationEventType) -> Result<Option<AccessibilityNode>> {
        let accessibility_tree = self.accessibility_tree.read();
        
        match navigation_type {
            NavigationEventType::Next => {
                accessibility_tree.navigate_next().await
            }
            NavigationEventType::Previous => {
                accessibility_tree.navigate_previous().await
            }
            NavigationEventType::First => {
                // Navigate to first focusable element
                let focusable_nodes = accessibility_tree.get_focusable_nodes().await?;
                if let Some(first_node) = focusable_nodes.first() {
                    accessibility_tree.set_focus(&first_node.id).await?;
                    Ok(Some(first_node.clone()))
                } else {
                    Ok(None)
                }
            }
            NavigationEventType::Last => {
                // Navigate to last focusable element
                let focusable_nodes = accessibility_tree.get_focusable_nodes().await?;
                if let Some(last_node) = focusable_nodes.last() {
                    accessibility_tree.set_focus(&last_node.id).await?;
                    Ok(Some(last_node.clone()))
                } else {
                    Ok(None)
                }
            }
            _ => {
                Ok(None)
            }
        }
    }

    /// Get accessibility statistics
    pub async fn get_accessibility_stats(&self) -> Result<CombinedAccessibilityStats> {
        let accessibility_stats = {
            let accessibility_tree = self.accessibility_tree.read();
            accessibility_tree.get_accessibility_stats().await?
        };
        
        let input_stats = {
            let input_handler = self.input_handler.read();
            input_handler.get_input_stats().await?
        };
        
        Ok(CombinedAccessibilityStats {
            accessibility: accessibility_stats,
            input: input_stats,
        })
    }

    /// Initialize accessibility tree
    async fn initialize_accessibility_tree(&self) -> Result<()> {
        let accessibility_tree = self.accessibility_tree.read();
        
        // Create root accessibility node
        let root_node = AccessibilityNode {
            id: "root".to_string(),
            role: AccessibilityRole::Document,
            name: Some("Document".to_string()),
            description: None,
            value: None,
            state: AccessibilityState::Hidden,
            properties: HashMap::new(),
            children: Vec::new(),
            parent: None,
            bounding_box: None,
            is_visible: true,
            is_focusable: false,
            is_enabled: true,
            is_selected: false,
            is_expanded: true,
            is_checked: false,
            is_required: false,
            is_invalid: false,
            is_busy: false,
            is_pressed: false,
            is_read_only: false,
            is_multi_line: false,
            is_multi_selectable: false,
            is_sorted: false,
            is_sorted_ascending: false,
            is_sorted_descending: false,
            is_atomic: false,
            is_live: false,
            live_region: None,
            current_value: None,
            maximum_value: None,
            minimum_value: None,
            step_value: None,
            level: None,
            pos_in_set: None,
            set_size: None,
            column_index: None,
            column_span: None,
            row_index: None,
            row_span: None,
            column_count: None,
            row_count: None,
            column_header_cells: Vec::new(),
            row_header_cells: Vec::new(),
            controls: Vec::new(),
            described_by: Vec::new(),
            details: Vec::new(),
            error_message: Vec::new(),
            flow_to: Vec::new(),
            labeled_by: Vec::new(),
            owns: Vec::new(),
            active_descendant: None,
            auto_complete: None,
            has_popup: None,
            orientation: None,
            sort: None,
            current: None,
            dropeffect: None,
            grabbed: None,
            keyshortcuts: None,
            modal: None,
            multiline: None,
            multiselectable: None,
            placeholder: None,
            readonly: None,
            required: None,
            selected: None,
            setsize: None,
            posinset: None,
            valuemax: None,
            valuemin: None,
            valuenow: None,
            valuetext: None,
        };
        
        accessibility_tree.add_node(root_node).await?;
        
        Ok(())
    }

    /// Initialize input handler
    async fn initialize_input_handler(&self) -> Result<()> {
        let input_handler = self.input_handler.read();
        
        // Set up default key bindings for accessibility
        let mut keyboard_handler = input_handler.keyboard_handler().write();
        
        // Tab navigation
        let tab_binding = KeyBinding {
            key_code: KeyCode::Tab,
            modifiers: vec![],
            context: "accessibility".to_string(),
        };
        let tab_action = KeyAction {
            name: "Navigate Next".to_string(),
            description: "Navigate to next focusable element".to_string(),
            handler: "navigate_next".to_string(),
            is_enabled: true,
        };
        keyboard_handler.add_key_binding(tab_binding, tab_action);
        
        // Shift+Tab navigation
        let shift_tab_binding = KeyBinding {
            key_code: KeyCode::Tab,
            modifiers: vec![ModifierKey::Shift],
            context: "accessibility".to_string(),
        };
        let shift_tab_action = KeyAction {
            name: "Navigate Previous".to_string(),
            description: "Navigate to previous focusable element".to_string(),
            handler: "navigate_previous".to_string(),
            is_enabled: true,
        };
        keyboard_handler.add_key_binding(shift_tab_binding, shift_tab_action);
        
        Ok(())
    }

    /// Update accessibility from input
    async fn update_accessibility_from_input(&self, event_type: InputEventType, event_data: InputEventData) -> Result<()> {
        // This is a simplified implementation
        // In a real implementation, you would update the accessibility tree based on input events
        
        match event_type {
            InputEventType::KeyDown => {
                // Handle keyboard navigation
                if let InputEventData::Keyboard(keyboard_data) = event_data {
                    match keyboard_data.key_code {
                        KeyCode::Tab => {
                            if keyboard_data.modifiers.contains(&ModifierKey::Shift) {
                                self.navigate_accessibility(NavigationEventType::Previous).await?;
                            } else {
                                self.navigate_accessibility(NavigationEventType::Next).await?;
                            }
                        }
                        KeyCode::ArrowUp => {
                            self.navigate_accessibility(NavigationEventType::Previous).await?;
                        }
                        KeyCode::ArrowDown => {
                            self.navigate_accessibility(NavigationEventType::Next).await?;
                        }
                        _ => {}
                    }
                }
            }
            InputEventType::MouseDown => {
                // Handle mouse navigation
                if let InputEventData::Mouse(mouse_data) = event_data {
                    // Find element at mouse position and set focus
                    let accessibility_tree = self.accessibility_tree.read();
                    // This would require finding the element at the mouse position
                }
            }
            _ => {}
        }
        
        Ok(())
    }

    /// Cleanup accessibility
    async fn cleanup_accessibility(&self) -> Result<()> {
        // Clear accessibility tree
        let accessibility_tree = self.accessibility_tree.read();
        // Clear all nodes
        
        // Clear input handler
        let input_handler = self.input_handler.read();
        // Clear event queue
        
        Ok(())
    }
}

/// Accessibility manager state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccessibilityManagerState {
    /// Accessibility is enabled
    Enabled,
    /// Accessibility is disabled
    Disabled,
    /// Accessibility is initializing
    Initializing,
    /// Accessibility is cleaning up
    CleaningUp,
}

/// Combined accessibility statistics
#[derive(Debug, Clone)]
pub struct CombinedAccessibilityStats {
    /// Accessibility tree statistics
    pub accessibility: AccessibilityStats,
    /// Input handler statistics
    pub input: InputStats,
}

/// Input statistics
#[derive(Debug, Clone, Default)]
pub struct InputStats {
    /// Total input events
    pub total_events: usize,
    /// Keyboard events
    pub keyboard_events: usize,
    /// Mouse events
    pub mouse_events: usize,
    /// Touch events
    pub touch_events: usize,
    /// Gesture events
    pub gesture_events: usize,
    /// Event queue size
    pub event_queue_size: usize,
    /// Active key bindings
    pub active_key_bindings: usize,
    /// Active touch points
    pub active_touch_points: usize,
}

impl InputHandler {
    /// Get input statistics
    pub async fn get_input_stats(&self) -> Result<InputStats> {
        let keyboard_handler = self.keyboard_handler().read();
        let touch_handler = self.touch_handler().read();
        let event_queue = self.event_queue().read();
        
        let mut stats = InputStats::default();
        
        // Count key bindings
        stats.active_key_bindings = keyboard_handler.key_bindings.len();
        
        // Count touch points
        stats.active_touch_points = touch_handler.get_touch_points().len();
        
        // Count events
        let events = event_queue.get_events();
        stats.total_events = events.len();
        stats.event_queue_size = events.len();
        
        for event in events {
            match event.event_type {
                InputEventType::KeyDown | InputEventType::KeyUp | InputEventType::KeyPress => {
                    stats.keyboard_events += 1;
                }
                InputEventType::MouseDown | InputEventType::MouseUp | InputEventType::MouseMove | InputEventType::MouseWheel => {
                    stats.mouse_events += 1;
                }
                InputEventType::TouchStart | InputEventType::TouchMove | InputEventType::TouchEnd | InputEventType::TouchCancel => {
                    stats.touch_events += 1;
                }
                InputEventType::GestureStart | InputEventType::GestureChange | InputEventType::GestureEnd | InputEventType::GestureCancel => {
                    stats.gesture_events += 1;
                }
            }
        }
        
        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_accessibility_manager_creation() {
        let accessibility_manager = AccessibilityManager::new();
        assert_eq!(accessibility_manager.get_state(), AccessibilityManagerState::Enabled);
    }

    #[tokio::test]
    async fn test_accessibility_enable_disable() {
        let mut accessibility_manager = AccessibilityManager::new();
        
        // Enable accessibility
        let result = accessibility_manager.enable_accessibility().await;
        assert!(result.is_ok());
        assert_eq!(accessibility_manager.get_state(), AccessibilityManagerState::Enabled);
        
        // Disable accessibility
        let result = accessibility_manager.disable_accessibility().await;
        assert!(result.is_ok());
        assert_eq!(accessibility_manager.get_state(), AccessibilityManagerState::Disabled);
    }

    #[tokio::test]
    async fn test_input_event_handling() {
        let accessibility_manager = AccessibilityManager::new();
        
        // Test keyboard event
        let keyboard_data = KeyboardEventData {
            key_code: KeyCode::Tab,
            key_char: None,
            modifiers: vec![],
            is_repeat: false,
            is_system_key: false,
        };
        
        let result = accessibility_manager.handle_input_event(
            InputEventType::KeyDown,
            InputEventData::Keyboard(keyboard_data),
        ).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_accessibility_navigation() {
        let accessibility_manager = AccessibilityManager::new();
        
        // Test navigation
        let result = accessibility_manager.navigate_accessibility(NavigationEventType::Next).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_accessibility_stats() {
        let accessibility_manager = AccessibilityManager::new();
        
        let stats = accessibility_manager.get_accessibility_stats().await;
        assert!(stats.is_ok());
        
        let stats = stats.unwrap();
        assert_eq!(stats.accessibility.total_nodes, 0);
        assert_eq!(stats.input.total_events, 0);
    }
}
