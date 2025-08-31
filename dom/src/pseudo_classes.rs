//! Pseudo-class support for CSS selectors.
//! 
//! This module provides functionality for handling CSS pseudo-classes
//! such as :hover, :active, :focus, :first-child, etc.

use std::collections::HashMap;
use tracing::debug;
use crate::dom::Element;
use crate::css_selector::PseudoClass;

/// State of an element for pseudo-class evaluation
#[derive(Debug, Clone, PartialEq)]
pub struct ElementState {
    /// Whether the element is being hovered
    pub hover: bool,
    /// Whether the element is being pressed/active
    pub active: bool,
    /// Whether the element has focus
    pub focus: bool,
    /// Whether the element is enabled
    pub enabled: bool,
    /// Whether the element is disabled
    pub disabled: bool,
    /// Whether the element is checked (for form elements)
    pub checked: bool,
    /// Whether the element is required
    pub required: bool,
    /// Whether the element is valid
    pub valid: bool,
    /// Whether the element is invalid
    pub invalid: bool,
    /// Whether the element is the first child
    pub first_child: bool,
    /// Whether the element is the last child
    pub last_child: bool,
    /// Whether the element is the only child
    pub only_child: bool,
    /// Whether the element is the first of its type
    pub first_of_type: bool,
    /// Whether the element is the last of its type
    pub last_of_type: bool,
    /// Whether the element is the only one of its type
    pub only_of_type: bool,
    /// Whether the element is empty
    pub empty: bool,
    /// Whether the element is the root
    pub root: bool,
    /// Whether the element is the target of the URL fragment
    pub target: bool,
    /// Whether the element is in viewport
    pub in_viewport: bool,
    /// Whether the element is visible
    pub visible: bool,
    /// Whether the element is hidden
    pub hidden: bool,
    /// Whether the element is selected
    pub selected: bool,
    /// Whether the element is indeterminate
    pub indeterminate: bool,
    /// Whether the element is read-only
    pub read_only: bool,
    /// Whether the element is read-write
    pub read_write: bool,
    /// Whether the element is optional
    pub optional: bool,
    /// Whether the element is the default
    pub default: bool,
    /// Whether the element is in range
    pub in_range: bool,
    /// Whether the element is out of range
    pub out_of_range: bool,
    /// Whether the element is placeholder-shown
    pub placeholder_shown: bool,
    /// Whether the element is fullscreen
    pub fullscreen: bool,
    /// Whether the element is modal
    pub modal: bool,
    /// Whether the element is picture-in-picture
    pub picture_in_picture: bool,
    /// Whether the element is playing
    pub playing: bool,
    /// Whether the element is paused
    pub paused: bool,
    /// Whether the element is muted
    pub muted: bool,
    /// Whether the element is volume-locked
    pub volume_locked: bool,
    /// Whether the element is buffering
    pub buffering: bool,
    /// Whether the element is seeking
    pub seeking: bool,
    /// Whether the element is stalled
    pub stalled: bool,
    /// Whether the element is loading
    pub loading: bool,
    /// Whether the element is autoplay
    pub autoplay: bool,
    /// Whether the element is user-invalid
    pub user_invalid: bool,
    /// Whether the element is user-valid
    pub user_valid: bool,
    /// Whether the element is defined
    pub defined: bool,
    /// Whether the element is open
    pub open: bool,
    /// Whether the element is closed
    pub closed: bool,
    /// Whether the element is current
    pub current: bool,
    /// Whether the element is past
    pub past: bool,
    /// Whether the element is future
    pub future: bool,
    /// Whether the element is host
    pub host: bool,
    /// Whether the element is host-context
    pub host_context: bool,
    /// Whether the element is scope
    pub scope: bool,
    /// Whether the element is any-link
    pub any_link: bool,
    /// Whether the element is link
    pub link: bool,
    /// Whether the element is visited
    pub visited: bool,
    /// Whether the element is local-link
    pub local_link: bool,
    /// Whether the element is target-within
    pub target_within: bool,
    /// Whether the element is focus-within
    pub focus_within: bool,
    /// Whether the element is focus-visible
    pub focus_visible: bool,
}

impl Default for ElementState {
    fn default() -> Self {
        Self {
            hover: false,
            active: false,
            focus: false,
            enabled: true,
            disabled: false,
            checked: false,
            required: false,
            valid: true,
            invalid: false,
            first_child: false,
            last_child: false,
            only_child: false,
            first_of_type: false,
            last_of_type: false,
            only_of_type: false,
            empty: true,
            root: false,
            target: false,
            in_viewport: false,
            visible: true,
            hidden: false,
            selected: false,
            indeterminate: false,
            read_only: false,
            read_write: true,
            optional: true,
            default: false,
            in_range: true,
            out_of_range: false,
            placeholder_shown: false,
            fullscreen: false,
            modal: false,
            picture_in_picture: false,
            playing: false,
            paused: false,
            muted: false,
            volume_locked: false,
            buffering: false,
            seeking: false,
            stalled: false,
            loading: false,
            autoplay: false,
            user_invalid: false,
            user_valid: true,
            defined: true,
            open: false,
            closed: true,
            current: false,
            past: false,
            future: false,
            host: false,
            host_context: false,
            scope: false,
            any_link: false,
            link: false,
            visited: false,
            local_link: false,
            target_within: false,
            focus_within: false,
            focus_visible: false,
        }
    }
}

/// Pseudo-class evaluator
pub struct PseudoClassEvaluator {
    /// Element states by element ID
    element_states: HashMap<String, ElementState>,
}

impl PseudoClassEvaluator {
    /// Create a new pseudo-class evaluator
    pub fn new() -> Self {
        Self {
            element_states: HashMap::new(),
        }
    }
    
    /// Get the state of an element
    pub fn get_element_state(&self, element_id: &str) -> ElementState {
        self.element_states.get(element_id)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Set the state of an element
    pub fn set_element_state(&mut self, element_id: String, state: ElementState) {
        self.element_states.insert(element_id, state);
    }
    
    /// Update a specific state property for an element
    pub fn update_element_state<F>(&mut self, element_id: &str, updater: F)
    where
        F: FnOnce(&mut ElementState),
    {
        let mut state = self.get_element_state(element_id);
        updater(&mut state);
        self.element_states.insert(element_id.to_string(), state);
    }
    
    /// Evaluate a pseudo-class against an element
    pub fn evaluate_pseudo_class(&self, element: &Element, pseudo_class: &PseudoClass) -> bool {
        let state = self.get_element_state(&element.id);
        
        match pseudo_class {
            PseudoClass::Hover => state.hover,
            PseudoClass::Active => state.active,
            PseudoClass::Focus => state.focus,
            PseudoClass::Visited => state.visited,
            PseudoClass::Link => state.link,
            PseudoClass::FirstChild => self.is_first_child(element),
            PseudoClass::LastChild => self.is_last_child(element),
            PseudoClass::NthChild(_) => {
                // This is a placeholder implementation
                // In a real implementation, this would parse the nth-child expression
                true
            }
            PseudoClass::NthLastChild(_) => {
                // This is a placeholder implementation
                // In a real implementation, this would parse the nth-last-child expression
                true
            }
            PseudoClass::Not(_) => {
                // This is a placeholder implementation
                // In a real implementation, this would evaluate the negated selector
                true
            }
            PseudoClass::Is(_) => {
                // This is a placeholder implementation
                // In a real implementation, this would evaluate the selector list
                true
            }
            PseudoClass::Where(_) => {
                // This is a placeholder implementation
                // In a real implementation, this would evaluate the selector list
                true
            }
            PseudoClass::Has(_) => {
                // This is a placeholder implementation
                // In a real implementation, this would evaluate the :has() selector
                true
            }
            PseudoClass::Custom(_) => {
                // This is a placeholder implementation
                // In a real implementation, this would handle custom pseudo-classes
                true
            }
        }
    }
    
    /// Check if an element is the first child
    fn is_first_child(&self, _element: &Element) -> bool {
        // This is a placeholder implementation
        // In a real implementation, this would check the DOM structure
        true
    }
    
    /// Check if an element is the last child
    fn is_last_child(&self, _element: &Element) -> bool {
        // This is a placeholder implementation
        // In a real implementation, this would check the DOM structure
        true
    }
    
    /// Check if an element is the only child
    fn is_only_child(&self, _element: &Element) -> bool {
        // This is a placeholder implementation
        // In a real implementation, this would check the DOM structure
        true
    }
    
    /// Check if an element is the first of its type
    fn is_first_of_type(&self, _element: &Element) -> bool {
        // This is a placeholder implementation
        // In a real implementation, this would check the DOM structure
        true
    }
    
    /// Check if an element is the last of its type
    fn is_last_of_type(&self, _element: &Element) -> bool {
        // This is a placeholder implementation
        // In a real implementation, this would check the DOM structure
        true
    }
    
    /// Check if an element is the only one of its type
    fn is_only_of_type(&self, _element: &Element) -> bool {
        // This is a placeholder implementation
        // In a real implementation, this would check the DOM structure
        true
    }
    
    /// Check if an element is empty
    fn is_empty(&self, _element: &Element) -> bool {
        // This is a placeholder implementation
        // In a real implementation, this would check if the element has no children
        true
    }
    
    /// Check if an element is the root
    fn is_root(&self, _element: &Element) -> bool {
        // This is a placeholder implementation
        // In a real implementation, this would check if the element is the document root
        false
    }
}

/// Event handler for pseudo-class state changes
pub struct PseudoClassEventHandler {
    /// The evaluator to update
    evaluator: PseudoClassEvaluator,
}

impl PseudoClassEventHandler {
    /// Create a new pseudo-class event handler
    pub fn new(evaluator: PseudoClassEvaluator) -> Self {
        Self { evaluator }
    }
    
    /// Handle mouse enter event
    pub fn handle_mouse_enter(&mut self, element_id: String) {
        self.evaluator.update_element_state(&element_id, |state| {
            state.hover = true;
        });
        debug!("Element {} is now hovered", element_id);
    }
    
    /// Handle mouse leave event
    pub fn handle_mouse_leave(&mut self, element_id: String) {
        self.evaluator.update_element_state(&element_id, |state| {
            state.hover = false;
        });
        debug!("Element {} is no longer hovered", element_id);
    }
    
    /// Handle mouse down event
    pub fn handle_mouse_down(&mut self, element_id: String) {
        self.evaluator.update_element_state(&element_id, |state| {
            state.active = true;
        });
        debug!("Element {} is now active", element_id);
    }
    
    /// Handle mouse up event
    pub fn handle_mouse_up(&mut self, element_id: String) {
        self.evaluator.update_element_state(&element_id, |state| {
            state.active = false;
        });
        debug!("Element {} is no longer active", element_id);
    }
    
    /// Handle focus event
    pub fn handle_focus(&mut self, element_id: String) {
        self.evaluator.update_element_state(&element_id, |state| {
            state.focus = true;
        });
        debug!("Element {} now has focus", element_id);
    }
    
    /// Handle blur event
    pub fn handle_blur(&mut self, element_id: String) {
        self.evaluator.update_element_state(&element_id, |state| {
            state.focus = false;
        });
        debug!("Element {} no longer has focus", element_id);
    }
    
    /// Handle form validation events
    pub fn handle_validation_change(&mut self, element_id: String, is_valid: bool) {
        self.evaluator.update_element_state(&element_id, |state| {
            state.valid = is_valid;
            state.invalid = !is_valid;
        });
        debug!("Element {} validation changed: valid={}", element_id, is_valid);
    }
    
    /// Handle checkbox/radio change events
    pub fn handle_checked_change(&mut self, element_id: String, is_checked: bool) {
        self.evaluator.update_element_state(&element_id, |state| {
            state.checked = is_checked;
        });
        debug!("Element {} checked state changed: checked={}", element_id, is_checked);
    }
    
    /// Handle disabled state changes
    pub fn handle_disabled_change(&mut self, element_id: String, is_disabled: bool) {
        self.evaluator.update_element_state(&element_id, |state| {
            state.disabled = is_disabled;
            state.enabled = !is_disabled;
        });
        debug!("Element {} disabled state changed: disabled={}", element_id, is_disabled);
    }
    
    /// Get the evaluator
    pub fn evaluator(&self) -> &PseudoClassEvaluator {
        &self.evaluator
    }
    
    /// Get the evaluator mutably
    pub fn evaluator_mut(&mut self) -> &mut PseudoClassEvaluator {
        &mut self.evaluator
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css_selector::PseudoClass;

    #[test]
    fn test_element_state_default() {
        let state = ElementState::default();
        assert!(!state.hover);
        assert!(!state.active);
        assert!(!state.focus);
        assert!(state.enabled);
        assert!(!state.disabled);
    }

    #[test]
    fn test_pseudo_class_evaluator_creation() {
        let evaluator = PseudoClassEvaluator::new();
        let element = Element::new("div".to_string());
        let state = evaluator.get_element_state(&element.id);
        assert_eq!(state, ElementState::default());
    }

    #[test]
    fn test_pseudo_class_evaluation() {
        let mut evaluator = PseudoClassEvaluator::new();
        let element = Element::new("div".to_string());
        
        // Set hover state
        evaluator.update_element_state(&element.id, |state| {
            state.hover = true;
        });
        
        // Test hover pseudo-class
        assert!(evaluator.evaluate_pseudo_class(&element, &PseudoClass::Hover));
        assert!(!evaluator.evaluate_pseudo_class(&element, &PseudoClass::Active));
    }

    #[test]
    fn test_event_handler() {
        let evaluator = PseudoClassEvaluator::new();
        let mut handler = PseudoClassEventHandler::new(evaluator);
        let element_id = "test-element".to_string();
        
        // Test mouse enter
        handler.handle_mouse_enter(element_id.clone());
        let state = handler.evaluator().get_element_state(&element_id);
        assert!(state.hover);
        
        // Test mouse leave
        handler.handle_mouse_leave(element_id.clone());
        let state = handler.evaluator().get_element_state(&element_id);
        assert!(!state.hover);
    }

    #[test]
    fn test_focus_events() {
        let evaluator = PseudoClassEvaluator::new();
        let mut handler = PseudoClassEventHandler::new(evaluator);
        let element_id = "test-element".to_string();
        
        // Test focus
        handler.handle_focus(element_id.clone());
        let state = handler.evaluator().get_element_state(&element_id);
        assert!(state.focus);
        
        // Test blur
        handler.handle_blur(element_id.clone());
        let state = handler.evaluator().get_element_state(&element_id);
        assert!(!state.focus);
    }

    #[test]
    fn test_form_validation_events() {
        let evaluator = PseudoClassEvaluator::new();
        let mut handler = PseudoClassEventHandler::new(evaluator);
        let element_id = "test-element".to_string();
        
        // Test validation change
        handler.handle_validation_change(element_id.clone(), false);
        let state = handler.evaluator().get_element_state(&element_id);
        assert!(!state.valid);
        assert!(state.invalid);
        
        handler.handle_validation_change(element_id.clone(), true);
        let state = handler.evaluator().get_element_state(&element_id);
        assert!(state.valid);
        assert!(!state.invalid);
    }
}
