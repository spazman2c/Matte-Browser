//! DOM Event System implementation for the Matte browser.
//! 
//! This module provides a complete event system including event types,
//! event listeners, event bubbling and capturing, and event management.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use crate::error::{Error, Result};
use crate::dom::Document;

/// Event phase enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventPhase {
    /// Event is in the capturing phase
    Capturing = 1,
    /// Event is at the target
    Target = 2,
    /// Event is in the bubbling phase
    Bubbling = 3,
}

/// Event types supported by the DOM
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    // Mouse events
    Click,
    DblClick,
    MouseDown,
    MouseUp,
    MouseMove,
    MouseOver,
    MouseOut,
    MouseEnter,
    MouseLeave,
    ContextMenu,
    
    // Keyboard events
    KeyDown,
    KeyUp,
    KeyPress,
    
    // Form events
    Submit,
    Reset,
    Change,
    Input,
    Focus,
    Blur,
    
    // Document events
    Load,
    Unload,
    BeforeUnload,
    DOMContentLoaded,
    
    // Window events
    Resize,
    Scroll,
    
    // Custom events
    Custom(String),
}

impl EventType {
    /// Get the string representation of the event type
    pub fn as_str(&self) -> &str {
        match self {
            EventType::Click => "click",
            EventType::DblClick => "dblclick",
            EventType::MouseDown => "mousedown",
            EventType::MouseUp => "mouseup",
            EventType::MouseMove => "mousemove",
            EventType::MouseOver => "mouseover",
            EventType::MouseOut => "mouseout",
            EventType::MouseEnter => "mouseenter",
            EventType::MouseLeave => "mouseleave",
            EventType::ContextMenu => "contextmenu",
            EventType::KeyDown => "keydown",
            EventType::KeyUp => "keyup",
            EventType::KeyPress => "keypress",
            EventType::Submit => "submit",
            EventType::Reset => "reset",
            EventType::Change => "change",
            EventType::Input => "input",
            EventType::Focus => "focus",
            EventType::Blur => "blur",
            EventType::Load => "load",
            EventType::Unload => "unload",
            EventType::BeforeUnload => "beforeunload",
            EventType::DOMContentLoaded => "DOMContentLoaded",
            EventType::Resize => "resize",
            EventType::Scroll => "scroll",
            EventType::Custom(name) => name,
        }
    }
    
    /// Create an event type from a string
    pub fn from_str(s: &str) -> Self {
        match s {
            "click" => EventType::Click,
            "dblclick" => EventType::DblClick,
            "mousedown" => EventType::MouseDown,
            "mouseup" => EventType::MouseUp,
            "mousemove" => EventType::MouseMove,
            "mouseover" => EventType::MouseOver,
            "mouseout" => EventType::MouseOut,
            "mouseenter" => EventType::MouseEnter,
            "mouseleave" => EventType::MouseLeave,
            "contextmenu" => EventType::ContextMenu,
            "keydown" => EventType::KeyDown,
            "keyup" => EventType::KeyUp,
            "keypress" => EventType::KeyPress,
            "submit" => EventType::Submit,
            "reset" => EventType::Reset,
            "change" => EventType::Change,
            "input" => EventType::Input,
            "focus" => EventType::Focus,
            "blur" => EventType::Blur,
            "load" => EventType::Load,
            "unload" => EventType::Unload,
            "beforeunload" => EventType::BeforeUnload,
            "DOMContentLoaded" => EventType::DOMContentLoaded,
            "resize" => EventType::Resize,
            "scroll" => EventType::Scroll,
            _ => EventType::Custom(s.to_string()),
        }
    }
}

/// Event target interface
pub trait EventTarget {
    /// Add an event listener
    fn add_event_listener(&mut self, event_type: EventType, listener: EventListener, use_capture: bool) -> Result<()>;
    
    /// Remove an event listener
    fn remove_event_listener(&mut self, event_type: EventType, listener: EventListener, use_capture: bool) -> Result<()>;
    
    /// Dispatch an event
    async fn dispatch_event(&mut self, event: Event) -> Result<bool>;
    
    /// Get event listeners for a specific event type
    fn get_event_listeners(&self, event_type: &EventType, use_capture: bool) -> Vec<EventListener>;
}

/// Event listener function type
pub type EventListenerFn = Box<dyn Fn(&Event) + Send + Sync>;

/// Event listener structure
#[derive(Clone)]
pub struct EventListener {
    /// Unique ID for the listener
    pub id: String,
    /// The callback function
    pub callback: Arc<EventListenerFn>,
    /// Whether this listener uses capture
    pub use_capture: bool,
    /// Whether this listener should be called only once
    pub once: bool,
    /// Whether this listener is passive
    pub passive: bool,
}

impl EventListener {
    /// Create a new event listener
    pub fn new<F>(callback: F, use_capture: bool, once: bool, passive: bool) -> Self 
    where
        F: Fn(&Event) + Send + Sync + 'static,
    {
        Self {
            id: format!("listener_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            callback: Arc::new(Box::new(callback)),
            use_capture,
            once,
            passive,
        }
    }
    
    /// Execute the event listener
    pub fn execute(&self, event: &Event) {
        (self.callback)(event);
    }
}

/// Mouse event data
#[derive(Debug, Clone)]
pub struct MouseEventData {
    pub client_x: f64,
    pub client_y: f64,
    pub screen_x: f64,
    pub screen_y: f64,
    pub button: i32,
    pub buttons: u16,
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub alt_key: bool,
    pub meta_key: bool,
    pub related_target: Option<String>,
}

/// Keyboard event data
#[derive(Debug, Clone)]
pub struct KeyboardEventData {
    pub key: String,
    pub code: String,
    pub key_code: u32,
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub alt_key: bool,
    pub meta_key: bool,
    pub repeat: bool,
    pub is_composing: bool,
}

/// Form event data
#[derive(Debug, Clone)]
pub struct FormEventData {
    pub value: String,
    pub checked: Option<bool>,
    pub files: Option<Vec<String>>,
}

/// Custom event data
#[derive(Debug, Clone)]
pub struct CustomEventData {
    pub detail: serde_json::Value,
}

/// Event data union
#[derive(Debug, Clone)]
pub enum EventData {
    Mouse(MouseEventData),
    Keyboard(KeyboardEventData),
    Form(FormEventData),
    Custom(CustomEventData),
    None,
}

/// DOM Event structure
#[derive(Debug, Clone)]
pub struct Event {
    /// Event type
    pub event_type: EventType,
    /// Event target
    pub target: String,
    /// Current target (element being processed)
    pub current_target: String,
    /// Event phase
    pub phase: EventPhase,
    /// Whether event bubbling is cancelled
    pub bubbles: bool,
    /// Whether event can be cancelled
    pub cancelable: bool,
    /// Whether default action is prevented
    pub default_prevented: bool,
    /// Whether event propagation is stopped
    pub propagation_stopped: bool,
    /// Whether immediate propagation is stopped
    pub immediate_propagation_stopped: bool,
    /// Event timestamp
    pub timestamp: std::time::Instant,
    /// Event data
    pub data: EventData,
    /// Whether this is a trusted event (from user interaction)
    pub is_trusted: bool,
}

impl Event {
    /// Create a new event
    pub fn new(event_type: EventType, target: String, bubbles: bool, cancelable: bool) -> Self {
        Self {
            event_type,
            target: target.clone(),
            current_target: target,
            phase: EventPhase::Target,
            bubbles,
            cancelable,
            default_prevented: false,
            propagation_stopped: false,
            immediate_propagation_stopped: false,
            timestamp: std::time::Instant::now(),
            data: EventData::None,
            is_trusted: false,
        }
    }
    
    /// Create a new mouse event
    pub fn new_mouse_event(
        event_type: EventType,
        target: String,
        client_x: f64,
        client_y: f64,
        button: i32,
    ) -> Self {
        let mut event = Self::new(event_type, target, true, true);
        event.data = EventData::Mouse(MouseEventData {
            client_x,
            client_y,
            screen_x: client_x,
            screen_y: client_y,
            button,
            buttons: 0,
            ctrl_key: false,
            shift_key: false,
            alt_key: false,
            meta_key: false,
            related_target: None,
        });
        event
    }
    
    /// Create a new keyboard event
    pub fn new_keyboard_event(
        event_type: EventType,
        target: String,
        key: String,
        code: String,
    ) -> Self {
        let mut event = Self::new(event_type, target, true, true);
        event.data = EventData::Keyboard(KeyboardEventData {
            key,
            code,
            key_code: 0,
            ctrl_key: false,
            shift_key: false,
            alt_key: false,
            meta_key: false,
            repeat: false,
            is_composing: false,
        });
        event
    }
    
    /// Create a new custom event
    pub fn new_custom_event(
        event_type: String,
        target: String,
        detail: serde_json::Value,
    ) -> Self {
        let mut event = Self::new(EventType::Custom(event_type), target, true, true);
        event.data = EventData::Custom(CustomEventData { detail });
        event
    }
    
    /// Prevent the default action
    pub fn prevent_default(&mut self) {
        if self.cancelable {
            self.default_prevented = true;
        }
    }
    
    /// Stop event propagation
    pub fn stop_propagation(&mut self) {
        self.propagation_stopped = true;
    }
    
    /// Stop immediate event propagation
    pub fn stop_immediate_propagation(&mut self) {
        self.immediate_propagation_stopped = true;
    }
    
    /// Get mouse event data
    pub fn mouse_data(&self) -> Option<&MouseEventData> {
        if let EventData::Mouse(data) = &self.data {
            Some(data)
        } else {
            None
        }
    }
    
    /// Get keyboard event data
    pub fn keyboard_data(&self) -> Option<&KeyboardEventData> {
        if let EventData::Keyboard(data) = &self.data {
            Some(data)
        } else {
            None
        }
    }
    
    /// Get form event data
    pub fn form_data(&self) -> Option<&FormEventData> {
        if let EventData::Form(data) = &self.data {
            Some(data)
        } else {
            None
        }
    }
    
    /// Get custom event data
    pub fn custom_data(&self) -> Option<&CustomEventData> {
        if let EventData::Custom(data) = &self.data {
            Some(data)
        } else {
            None
        }
    }
}

/// Event manager for handling event listeners and dispatching
pub struct EventManager {
    /// Event listeners organized by event type and capture phase
    listeners: HashMap<EventType, (Vec<EventListener>, Vec<EventListener>)>, // (capture, bubble)
    /// Event target ID
    target_id: String,
}

impl std::fmt::Debug for EventManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventManager")
            .field("target_id", &self.target_id)
            .field("listener_count", &self.get_listener_count())
            .finish()
    }
}

impl EventManager {
    /// Create a new event manager
    pub fn new(target_id: String) -> Self {
        Self {
            listeners: HashMap::new(),
            target_id,
        }
    }
    
    /// Add an event listener
    pub fn add_event_listener(&mut self, event_type: EventType, listener: EventListener) -> Result<()> {
        let (capture_listeners, bubble_listeners) = self.listeners.entry(event_type.clone()).or_insert_with(|| (Vec::new(), Vec::new()));
        
        if listener.use_capture {
            capture_listeners.push(listener);
        } else {
            bubble_listeners.push(listener);
        }
        
        debug!("Added event listener for {} on {}", event_type.as_str(), self.target_id);
        Ok(())
    }
    
    /// Remove an event listener
    pub fn remove_event_listener(&mut self, event_type: EventType, listener_id: &str, use_capture: bool) -> Result<()> {
        if let Some((capture_listeners, bubble_listeners)) = self.listeners.get_mut(&event_type) {
            let listeners = if use_capture { capture_listeners } else { bubble_listeners };
            listeners.retain(|l| l.id != listener_id);
        }
        
        debug!("Removed event listener {} for {} on {}", listener_id, event_type.as_str(), self.target_id);
        Ok(())
    }
    
    /// Get event listeners for a specific event type and phase
    pub fn get_event_listeners(&self, event_type: &EventType, use_capture: bool) -> Vec<EventListener> {
        if let Some((capture_listeners, bubble_listeners)) = self.listeners.get(event_type) {
            let listeners = if use_capture { capture_listeners } else { bubble_listeners };
            listeners.iter().cloned().collect()
        } else {
            Vec::new()
        }
    }
    
    /// Dispatch an event
    pub async fn dispatch_event(&mut self, mut event: Event) -> Result<bool> {
        event.current_target = self.target_id.clone();
        
        // Execute capture phase listeners
        if let Some((capture_listeners, _)) = self.listeners.get(&event.event_type) {
            event.phase = EventPhase::Capturing;
            for listener in capture_listeners {
                if event.immediate_propagation_stopped {
                    break;
                }
                
                // Execute the listener
                listener.execute(&event);
                
                // Handle once listeners by marking them for removal
                if listener.once {
                    // In a real implementation, we'd queue this for removal after the event cycle
                    debug!("Marking once listener {} for removal", listener.id);
                }
            }
        }
        
        // Execute target phase listeners (bubble listeners on target)
        if !event.immediate_propagation_stopped {
            if let Some((_, bubble_listeners)) = self.listeners.get(&event.event_type) {
                event.phase = EventPhase::Target;
                for listener in bubble_listeners {
                    if event.immediate_propagation_stopped {
                        break;
                    }
                    
                    // Execute the listener
                    listener.execute(&event);
                    
                    // Handle once listeners by marking them for removal
                    if listener.once {
                        // In a real implementation, we'd queue this for removal after the event cycle
                        debug!("Marking once listener {} for removal", listener.id);
                    }
                }
            }
        }
        
        // Return whether default was prevented
        Ok(event.default_prevented)
    }
    
    /// Get all event types that have listeners
    pub fn get_event_types(&self) -> Vec<&EventType> {
        self.listeners.keys().collect()
    }
    
    /// Clear all event listeners
    pub fn clear_event_listeners(&mut self) {
        self.listeners.clear();
        debug!("Cleared all event listeners for {}", self.target_id);
    }
    
    /// Remove all listeners for a specific event type
    pub fn remove_event_listeners(&mut self, event_type: &EventType) {
        self.listeners.remove(event_type);
        debug!("Removed all event listeners for {} on {}", event_type.as_str(), self.target_id);
    }
    
    /// Get the total number of event listeners
    pub fn get_listener_count(&self) -> usize {
        self.listeners.values().map(|(capture, bubble)| capture.len() + bubble.len()).sum()
    }
    
    /// Check if there are any listeners for a specific event type
    pub fn has_listeners(&self, event_type: &EventType) -> bool {
        self.listeners.contains_key(event_type)
    }
    
    /// Get listeners for both capture and bubble phases
    pub fn get_all_listeners(&self, event_type: &EventType) -> (Vec<EventListener>, Vec<EventListener>) {
        if let Some((capture_listeners, bubble_listeners)) = self.listeners.get(event_type) {
            (capture_listeners.iter().cloned().collect(), bubble_listeners.iter().cloned().collect())
        } else {
            (Vec::new(), Vec::new())
        }
    }
}

/// Event dispatcher for handling event propagation through the DOM tree
pub struct EventDispatcher {
    /// Document reference
    document: Arc<RwLock<Document>>,
}

impl EventDispatcher {
    /// Create a new event dispatcher
    pub fn new(document: Arc<RwLock<Document>>) -> Self {
        Self { document }
    }
    
    /// Dispatch an event through the DOM tree
    pub async fn dispatch_event(&self, mut event: Event, target_id: &str) -> Result<bool> {
        let document = self.document.read().await;
        
        // Find the target element
        let _target_element = document.get_element_by_id(target_id)
            .ok_or_else(|| Error::ConfigError(format!("Target element {} not found", target_id)))?;
        
        // Build the event path (capture phase: document -> target, bubble phase: target -> document)
        let event_path = self.build_event_path(&document, target_id).await?;
        
        drop(document);
        
        info!("Dispatching event {} to target {} with path: {:?}", 
              event.event_type.as_str(), target_id, event_path);
        
        // Execute capture phase (document -> target)
        event.phase = EventPhase::Capturing;
        for element_id in event_path.iter().rev() {
            if event.propagation_stopped {
                debug!("Event propagation stopped during capture phase at {}", element_id);
                break;
            }
            
            let document = self.document.read().await;
            if let Some(element) = document.get_element_by_id(element_id) {
                if let Some(event_manager) = &element.event_manager {
                    let mut manager = event_manager.write().await;
                    debug!("Executing capture phase on element {}", element_id);
                    manager.dispatch_event(event.clone()).await?;
                }
            }
            drop(document);
        }
        
        // Execute target phase
        if !event.propagation_stopped {
            event.phase = EventPhase::Target;
            let document = self.document.read().await;
            if let Some(element) = document.get_element_by_id(target_id) {
                if let Some(event_manager) = &element.event_manager {
                    let mut manager = event_manager.write().await;
                    debug!("Executing target phase on element {}", target_id);
                    manager.dispatch_event(event.clone()).await?;
                }
            }
            drop(document);
        }
        
        // Execute bubble phase (target -> document)
        if event.bubbles && !event.propagation_stopped {
            event.phase = EventPhase::Bubbling;
            for element_id in event_path.iter() {
                if event.propagation_stopped {
                    debug!("Event propagation stopped during bubble phase at {}", element_id);
                    break;
                }
                
                let document = self.document.read().await;
                if let Some(element) = document.get_element_by_id(element_id) {
                    if let Some(event_manager) = &element.event_manager {
                        let mut manager = event_manager.write().await;
                        debug!("Executing bubble phase on element {}", element_id);
                        manager.dispatch_event(event.clone()).await?;
                    }
                }
                drop(document);
            }
        }
        
        info!("Event {} dispatch completed, default prevented: {}", 
              event.event_type.as_str(), event.default_prevented);
        
        Ok(event.default_prevented)
    }
    
    /// Build the event path from target to document root
    async fn build_event_path(&self, document: &Document, target_id: &str) -> Result<Vec<String>> {
        let mut path = Vec::new();
        let mut current_id = target_id.to_string();
        let mut visited = std::collections::HashSet::new();
        
        // Walk up the DOM tree to build the path
        while let Some(element) = document.get_element_by_id(&current_id) {
            // Prevent infinite loops in case of circular references
            if visited.contains(&current_id) {
                warn!("Circular reference detected in DOM tree at element {}", current_id);
                break;
            }
            visited.insert(current_id.clone());
            
            if let Some(parent) = &element.parent {
                // Access the parent element through the Arc<RwLock>
                let parent_id = {
                    let parent_element = parent.blocking_read();
                    parent_element.id.clone()
                };
                path.push(parent_id.clone());
                current_id = parent_id;
            } else {
                // Reached the document root
                break;
            }
        }
        
        debug!("Built event path for target {}: {:?}", target_id, path);
        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_type_creation() {
        let click_event = EventType::Click;
        assert_eq!(click_event.as_str(), "click");
        
        let custom_event = EventType::Custom("myevent".to_string());
        assert_eq!(custom_event.as_str(), "myevent");
    }

    #[test]
    fn test_event_type_from_str() {
        let event_type = EventType::from_str("click");
        assert_eq!(event_type, EventType::Click);
        
        let custom_event = EventType::from_str("myevent");
        assert_eq!(custom_event, EventType::Custom("myevent".to_string()));
    }

    #[test]
    fn test_event_creation() {
        let event = Event::new(EventType::Click, "button1".to_string(), true, true);
        assert_eq!(event.event_type, EventType::Click);
        assert_eq!(event.target, "button1");
        assert!(event.bubbles);
        assert!(event.cancelable);
        assert!(!event.default_prevented);
    }

    #[test]
    fn test_mouse_event_creation() {
        let event = Event::new_mouse_event(EventType::Click, "button1".to_string(), 100.0, 200.0, 0);
        assert_eq!(event.event_type, EventType::Click);
        
        if let Some(mouse_data) = event.mouse_data() {
            assert_eq!(mouse_data.client_x, 100.0);
            assert_eq!(mouse_data.client_y, 200.0);
            assert_eq!(mouse_data.button, 0);
        } else {
            panic!("Expected mouse event data");
        }
    }

    #[test]
    fn test_keyboard_event_creation() {
        let event = Event::new_keyboard_event(EventType::KeyDown, "input1".to_string(), "a".to_string(), "KeyA".to_string());
        assert_eq!(event.event_type, EventType::KeyDown);
        
        if let Some(keyboard_data) = event.keyboard_data() {
            assert_eq!(keyboard_data.key, "a");
            assert_eq!(keyboard_data.code, "KeyA");
        } else {
            panic!("Expected keyboard event data");
        }
    }

    #[test]
    fn test_event_prevention() {
        let mut event = Event::new(EventType::Click, "button1".to_string(), true, true);
        assert!(!event.default_prevented);
        
        event.prevent_default();
        assert!(event.default_prevented);
    }

    #[test]
    fn test_event_propagation() {
        let mut event = Event::new(EventType::Click, "button1".to_string(), true, true);
        assert!(!event.propagation_stopped);
        
        event.stop_propagation();
        assert!(event.propagation_stopped);
    }

    #[test]
    fn test_event_listener_creation() {
        let listener = EventListener::new(
            |event| println!("Event: {:?}", event.event_type),
            false,
            false,
            false
        );
        
        assert!(!listener.use_capture);
        assert!(!listener.once);
        assert!(!listener.passive);
    }

    #[test]
    fn test_event_manager() {
        let mut manager = EventManager::new("button1".to_string());
        
        let listener = EventListener::new(
            |event| println!("Click event: {:?}", event.event_type),
            false,
            false,
            false
        );
        
        let result = manager.add_event_listener(EventType::Click, listener);
        assert!(result.is_ok());
        
        let listeners = manager.get_event_listeners(&EventType::Click, false);
        assert_eq!(listeners.len(), 1);
    }

    #[test]
    fn test_event_manager_removal() {
        let mut manager = EventManager::new("button1".to_string());
        
        let listener = EventListener::new(
            |event| println!("Click event: {:?}", event.event_type),
            false,
            false,
            false
        );
        
        let listener_id = listener.id.clone();
        manager.add_event_listener(EventType::Click, listener).unwrap();
        
        let result = manager.remove_event_listener(EventType::Click, &listener_id, false);
        assert!(result.is_ok());
        
        let listeners = manager.get_event_listeners(&EventType::Click, false);
        assert_eq!(listeners.len(), 0);
    }

    #[test]
    fn test_event_manager_enhanced_features() {
        let mut manager = EventManager::new("button1".to_string());
        
        // Add capture and bubble listeners
        let capture_listener = EventListener::new(
            |event| println!("Capture: {:?}", event.event_type),
            true,
            false,
            false
        );
        
        let bubble_listener = EventListener::new(
            |event| println!("Bubble: {:?}", event.event_type),
            false,
            false,
            false
        );
        
        manager.add_event_listener(EventType::Click, capture_listener).unwrap();
        manager.add_event_listener(EventType::Click, bubble_listener).unwrap();
        
        // Test enhanced features
        assert_eq!(manager.get_listener_count(), 2);
        assert!(manager.has_listeners(&EventType::Click));
        assert!(!manager.has_listeners(&EventType::KeyDown));
        
        let (capture_listeners, bubble_listeners) = manager.get_all_listeners(&EventType::Click);
        assert_eq!(capture_listeners.len(), 1);
        assert_eq!(bubble_listeners.len(), 1);
    }

    #[test]
    fn test_event_phases() {
        let event = Event::new(EventType::Click, "button1".to_string(), true, true);
        
        assert_eq!(event.phase, EventPhase::Target);
        
        let mut event_with_capture = event.clone();
        event_with_capture.phase = EventPhase::Capturing;
        assert_eq!(event_with_capture.phase, EventPhase::Capturing);
        
        let mut event_with_bubble = event.clone();
        event_with_bubble.phase = EventPhase::Bubbling;
        assert_eq!(event_with_bubble.phase, EventPhase::Bubbling);
    }

    #[test]
    fn test_event_propagation_control() {
        let mut event = Event::new(EventType::Click, "button1".to_string(), true, true);
        
        // Test default prevention
        assert!(!event.default_prevented);
        event.prevent_default();
        assert!(event.default_prevented);
        
        // Test propagation stopping
        assert!(!event.propagation_stopped);
        event.stop_propagation();
        assert!(event.propagation_stopped);
        
        // Test immediate propagation stopping
        assert!(!event.immediate_propagation_stopped);
        event.stop_immediate_propagation();
        assert!(event.immediate_propagation_stopped);
    }
}
