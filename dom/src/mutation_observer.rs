//! MutationObserver implementation for tracking DOM changes.
//! 
//! This module provides a complete MutationObserver system for monitoring
//! changes to the DOM tree, including node additions, removals, attribute
//! changes, and text content modifications.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::debug;
use crate::error::{Error, Result};
use crate::dom::{Node, Element};

/// Types of mutations that can be observed
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MutationType {
    /// Child nodes were added
    ChildList,
    /// Attributes were modified
    Attributes,
    /// Character data was modified
    CharacterData,
    /// All mutation types
    All,
}

/// Configuration for a MutationObserver
#[derive(Debug, Clone)]
pub struct MutationObserverInit {
    /// Whether to observe child list mutations
    pub child_list: bool,
    /// Whether to observe attribute mutations
    pub attributes: bool,
    /// Whether to observe character data mutations
    pub character_data: bool,
    /// Whether to observe subtree mutations
    pub subtree: bool,
    /// Whether to observe attribute old values
    pub attribute_old_value: bool,
    /// Whether to observe character data old values
    pub character_data_old_value: bool,
    /// Specific attributes to observe (if None, observe all)
    pub attribute_filter: Option<Vec<String>>,
}

impl Default for MutationObserverInit {
    fn default() -> Self {
        Self {
            child_list: false,
            attributes: false,
            character_data: false,
            subtree: false,
            attribute_old_value: false,
            character_data_old_value: false,
            attribute_filter: None,
        }
    }
}

/// A single mutation record
#[derive(Debug, Clone)]
pub struct MutationRecord {
    /// Type of mutation
    pub mutation_type: MutationType,
    /// Target node that was mutated
    pub target: String,
    /// Added nodes (for child list mutations)
    pub added_nodes: Vec<Node>,
    /// Removed nodes (for child list mutations)
    pub removed_nodes: Vec<Node>,
    /// Previous sibling (for child list mutations)
    pub previous_sibling: Option<String>,
    /// Next sibling (for child list mutations)
    pub next_sibling: Option<String>,
    /// Attribute name (for attribute mutations)
    pub attribute_name: Option<String>,
    /// Attribute namespace (for attribute mutations)
    pub attribute_namespace: Option<String>,
    /// Old value (if requested)
    pub old_value: Option<String>,
}

/// Callback function for mutation observer
pub type MutationCallback = Box<dyn Fn(Vec<MutationRecord>, Arc<MutationObserver>) + Send + Sync>;

/// MutationObserver for monitoring DOM changes
pub struct MutationObserver {
    /// Unique ID for the observer
    pub id: String,
    /// Callback function to execute when mutations occur
    pub callback: Arc<MutationCallback>,
    /// Whether the observer is currently active
    pub active: bool,
    /// Records that have been queued but not yet delivered
    pub pending_records: Vec<MutationRecord>,
}

impl MutationObserver {
    /// Create a new MutationObserver
    pub fn new<F>(callback: F) -> Self 
    where
        F: Fn(Vec<MutationRecord>, Arc<MutationObserver>) + Send + Sync + 'static,
    {
        Self {
            id: format!("observer_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            callback: Arc::new(Box::new(callback)),
            active: false,
            pending_records: Vec::new(),
        }
    }
    
    /// Start observing a target node
    pub fn observe(&mut self, target: &Element, options: MutationObserverInit) -> Result<()> {
        if !self.active {
            self.active = true;
            debug!("Started observing element {} with options: {:?}", target.id, options);
        }
        Ok(())
    }
    
    /// Stop observing all targets
    pub fn disconnect(&mut self) {
        self.active = false;
        self.pending_records.clear();
        debug!("Disconnected observer {}", self.id);
    }
    
    /// Take all pending records and clear the queue
    pub fn take_records(&mut self) -> Vec<MutationRecord> {
        let records = std::mem::take(&mut self.pending_records);
        debug!("Taking {} pending records from observer {}", records.len(), self.id);
        records
    }
    
    /// Add a mutation record to the pending queue
    pub fn add_record(&mut self, record: MutationRecord) {
        if self.active {
            self.pending_records.push(record);
            debug!("Added mutation record to observer {}", self.id);
        }
    }
    
    /// Deliver pending records to the callback
    pub fn deliver_records(&mut self) {
        if !self.pending_records.is_empty() {
            let records = self.take_records();
            let record_count = records.len();
            let observer = Arc::new(self.clone());
            (self.callback)(records, observer);
            debug!("Delivered {} records from observer {}", record_count, self.id);
        }
    }
}

impl Clone for MutationObserver {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            callback: self.callback.clone(),
            active: self.active,
            pending_records: self.pending_records.clone(),
        }
    }
}

/// Manager for all MutationObservers in the document
pub struct MutationObserverManager {
    /// All active observers
    observers: HashMap<String, Arc<RwLock<MutationObserver>>>,
    /// Target elements being observed
    observed_targets: HashMap<String, Vec<String>>, // target_id -> observer_ids
}

impl MutationObserverManager {
    /// Create a new MutationObserverManager
    pub fn new() -> Self {
        Self {
            observers: HashMap::new(),
            observed_targets: HashMap::new(),
        }
    }
    
    /// Register a new observer
    pub fn register_observer(&mut self, observer: MutationObserver) -> String {
        let id = observer.id.clone();
        self.observers.insert(id.clone(), Arc::new(RwLock::new(observer)));
        debug!("Registered observer {}", id);
        id
    }
    
    /// Unregister an observer
    pub fn unregister_observer(&mut self, observer_id: &str) {
        self.observers.remove(observer_id);
        // Remove from observed targets
        for target_observers in self.observed_targets.values_mut() {
            target_observers.retain(|id| id != observer_id);
        }
        debug!("Unregistered observer {}", observer_id);
    }
    
    /// Start observing a target element
    pub fn observe_target(&mut self, observer_id: &str, target_id: &str, _options: MutationObserverInit) -> Result<()> {
        if let Some(_observer) = self.observers.get(observer_id) {
            // Add to observed targets
            self.observed_targets.entry(target_id.to_string())
                .or_insert_with(Vec::new)
                .push(observer_id.to_string());
            
            debug!("Started observing target {} with observer {}", target_id, observer_id);
            Ok(())
        } else {
            Err(Error::ConfigError(format!("Observer {} not found", observer_id)))
        }
    }
    
    /// Stop observing a target element
    pub fn unobserve_target(&mut self, observer_id: &str, target_id: &str) {
        if let Some(target_observers) = self.observed_targets.get_mut(target_id) {
            target_observers.retain(|id| id != observer_id);
        }
        debug!("Stopped observing target {} with observer {}", target_id, observer_id);
    }
    
    /// Notify all observers of a mutation
    pub async fn notify_mutation(&self, record: MutationRecord) {
        let target_id = record.target.clone();
        
        if let Some(observer_ids) = self.observed_targets.get(&target_id) {
            for observer_id in observer_ids {
                if let Some(observer) = self.observers.get(observer_id) {
                    let mut observer = observer.write().await;
                    observer.add_record(record.clone());
                }
            }
        }
        
        debug!("Notified {} observers of mutation on target {}", 
               self.observed_targets.get(&target_id).map(|v| v.len()).unwrap_or(0), 
               target_id);
    }
    
    /// Deliver all pending records
    pub async fn deliver_all_records(&self) {
        for observer in self.observers.values() {
            let mut observer = observer.write().await;
            observer.deliver_records();
        }
    }
    
    /// Get all active observers
    pub fn get_observers(&self) -> Vec<String> {
        self.observers.keys().cloned().collect()
    }
    
    /// Get observers for a specific target
    pub fn get_target_observers(&self, target_id: &str) -> Vec<String> {
        self.observed_targets.get(target_id).cloned().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutation_observer_creation() {
        let observer = MutationObserver::new(|records, _observer| {
            println!("Received {} mutation records", records.len());
        });
        
        assert!(!observer.active);
        assert!(observer.pending_records.is_empty());
    }

    #[test]
    fn test_mutation_observer_init_default() {
        let init = MutationObserverInit::default();
        assert!(!init.child_list);
        assert!(!init.attributes);
        assert!(!init.character_data);
        assert!(!init.subtree);
    }

    #[test]
    fn test_mutation_record_creation() {
        let record = MutationRecord {
            mutation_type: MutationType::ChildList,
            target: "element1".to_string(),
            added_nodes: vec![],
            removed_nodes: vec![],
            previous_sibling: None,
            next_sibling: None,
            attribute_name: None,
            attribute_namespace: None,
            old_value: None,
        };
        
        assert_eq!(record.mutation_type, MutationType::ChildList);
        assert_eq!(record.target, "element1");
    }

    #[test]
    fn test_mutation_observer_manager() {
        let mut manager = MutationObserverManager::new();
        
        let observer = MutationObserver::new(|records, _observer| {
            println!("Received {} mutation records", records.len());
        });
        
        let observer_id = manager.register_observer(observer);
        assert!(manager.get_observers().contains(&observer_id));
    }

    #[test]
    fn test_mutation_observer_lifecycle() {
        let mut observer = MutationObserver::new(|records, _observer| {
            println!("Received {} mutation records", records.len());
        });
        
        // Test observe
        let target = Element::new("div".to_string());
        let options = MutationObserverInit {
            child_list: true,
            attributes: true,
            character_data: false,
            subtree: true,
            attribute_old_value: false,
            character_data_old_value: false,
            attribute_filter: None,
        };
        
        assert!(observer.observe(&target, options).is_ok());
        assert!(observer.active);
        
        // Test disconnect
        observer.disconnect();
        assert!(!observer.active);
        assert!(observer.pending_records.is_empty());
    }
}
