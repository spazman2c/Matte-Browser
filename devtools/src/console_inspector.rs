use crate::error::{Error, Result};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};

/// Console Inspector
pub struct ConsoleInspector {
    /// Console messages
    messages: Arc<RwLock<Vec<ConsoleMessage>>>,
    /// Console filters
    filters: Arc<RwLock<ConsoleFilters>>,
    /// Runtime evaluator
    evaluator: Arc<RwLock<RuntimeEvaluator>>,
    /// Source maps
    source_maps: Arc<RwLock<SourceMapManager>>,
    /// Stack trace parser
    stack_trace_parser: Arc<RwLock<StackTraceParser>>,
    /// Console state
    state: ConsoleState,
}

/// Console message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleMessage {
    /// Message ID
    pub id: String,
    /// Message type
    pub message_type: ConsoleMessageType,
    /// Message level
    pub level: ConsoleLevel,
    /// Message text
    pub text: String,
    /// Message arguments
    pub arguments: Vec<ConsoleArgument>,
    /// Source location
    pub source: Option<SourceLocation>,
    /// Stack trace
    pub stack_trace: Option<StackTrace>,
    /// Timestamp
    pub timestamp: u64,
    /// Is expanded
    pub is_expanded: bool,
    /// Is selected
    pub is_selected: bool,
    /// Group depth
    pub group_depth: u32,
    /// Group collapsed
    pub group_collapsed: bool,
}

/// Console message type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ConsoleMessageType {
    /// Log message
    Log,
    /// Info message
    Info,
    /// Warning message
    Warning,
    /// Error message
    Error,
    /// Debug message
    Debug,
    /// Table message
    Table,
    /// Group message
    Group,
    /// Group collapsed message
    GroupCollapsed,
    /// Group end message
    GroupEnd,
    /// Time message
    Time,
    /// Time end message
    TimeEnd,
    /// Count message
    Count,
    /// Clear message
    Clear,
    /// Trace message
    Trace,
    /// Assert message
    Assert,
}

/// Console level
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ConsoleLevel {
    /// Verbose level
    Verbose,
    /// Info level
    Info,
    /// Warning level
    Warning,
    /// Error level
    Error,
}

/// Console argument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleArgument {
    /// Argument type
    pub argument_type: ArgumentType,
    /// Argument value
    pub value: serde_json::Value,
    /// Argument preview
    pub preview: Option<String>,
    /// Argument description
    pub description: Option<String>,
}

/// Argument type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ArgumentType {
    /// String argument
    String,
    /// Number argument
    Number,
    /// Boolean argument
    Boolean,
    /// Object argument
    Object,
    /// Array argument
    Array,
    /// Function argument
    Function,
    /// Undefined argument
    Undefined,
    /// Null argument
    Null,
    /// Symbol argument
    Symbol,
    /// BigInt argument
    BigInt,
}

/// Source location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    /// File URL
    pub file_url: String,
    /// Line number
    pub line_number: u32,
    /// Column number
    pub column_number: u32,
    /// Function name
    pub function_name: Option<String>,
    /// Script ID
    pub script_id: Option<String>,
}

/// Stack trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackTrace {
    /// Stack frames
    pub frames: Vec<StackFrame>,
    /// Is truncated
    pub is_truncated: bool,
}

/// Stack frame
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrame {
    /// Function name
    pub function_name: String,
    /// File URL
    pub file_url: String,
    /// Line number
    pub line_number: u32,
    /// Column number
    pub column_number: u32,
    /// Script ID
    pub script_id: Option<String>,
    /// Is native
    pub is_native: bool,
    /// Is eval
    pub is_eval: bool,
    /// Is constructor
    pub is_constructor: bool,
}

/// Console filters
pub struct ConsoleFilters {
    /// Level filters
    level_filters: HashMap<ConsoleLevel, bool>,
    /// Type filters
    type_filters: HashMap<ConsoleMessageType, bool>,
    /// Text filter
    text_filter: Option<String>,
    /// Source filter
    source_filter: Option<String>,
    /// Is regex filter
    is_regex_filter: bool,
    /// Is case sensitive
    is_case_sensitive: bool,
}

/// Runtime evaluator
pub struct RuntimeEvaluator {
    /// Evaluation context
    context: EvaluationContext,
    /// Expression history
    expression_history: Vec<String>,
    /// Evaluation results
    evaluation_results: HashMap<String, EvaluationResult>,
    /// Auto-complete
    auto_complete: ExpressionAutoComplete,
}

/// Evaluation context
#[derive(Debug, Clone)]
pub struct EvaluationContext {
    /// Context ID
    pub id: String,
    /// Context name
    pub name: String,
    /// Context variables
    pub variables: HashMap<String, serde_json::Value>,
    /// Context functions
    pub functions: HashMap<String, String>,
    /// Context scope
    pub scope: EvaluationScope,
}

/// Evaluation scope
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EvaluationScope {
    /// Top scope
    Top,
    /// Function scope
    Function,
    /// Block scope
    Block,
    /// Module scope
    Module,
}

/// Evaluation result
#[derive(Debug, Clone)]
pub struct EvaluationResult {
    /// Result ID
    pub id: String,
    /// Expression
    pub expression: String,
    /// Result value
    pub result: serde_json::Value,
    /// Result type
    pub result_type: String,
    /// Execution time
    pub execution_time: u64,
    /// Error message
    pub error: Option<String>,
    /// Timestamp
    pub timestamp: u64,
}

/// Expression auto-complete
pub struct ExpressionAutoComplete {
    /// JavaScript keywords
    keywords: Vec<String>,
    /// Global objects
    global_objects: Vec<String>,
    /// DOM elements
    dom_elements: Vec<String>,
    /// CSS selectors
    css_selectors: Vec<String>,
}

/// Source map manager
pub struct SourceMapManager {
    /// Source maps
    source_maps: HashMap<String, SourceMap>,
    /// Mapped locations
    mapped_locations: HashMap<String, MappedLocation>,
}

/// Source map
#[derive(Debug, Clone)]
pub struct SourceMap {
    /// Source map ID
    pub id: String,
    /// Source map URL
    pub url: String,
    /// Source map content
    pub content: String,
    /// Source files
    pub sources: Vec<String>,
    /// Source content
    pub source_content: HashMap<String, String>,
    /// Mappings
    pub mappings: Vec<SourceMapping>,
    /// Names
    pub names: Vec<String>,
}

/// Source mapping
#[derive(Debug, Clone)]
pub struct SourceMapping {
    /// Generated line
    pub generated_line: u32,
    /// Generated column
    pub generated_column: u32,
    /// Source file index
    pub source_file_index: Option<u32>,
    /// Source line
    pub source_line: Option<u32>,
    /// Source column
    pub source_column: Option<u32>,
    /// Name index
    pub name_index: Option<u32>,
}

/// Mapped location
#[derive(Debug, Clone)]
pub struct MappedLocation {
    /// Original location
    pub original: SourceLocation,
    /// Mapped location
    pub mapped: SourceLocation,
    /// Source map ID
    pub source_map_id: String,
}

/// Stack trace parser
pub struct StackTraceParser {
    /// Parsed stack traces
    parsed_stack_traces: HashMap<String, ParsedStackTrace>,
    /// Source map manager
    source_map_manager: Arc<RwLock<SourceMapManager>>,
}

/// Parsed stack trace
#[derive(Debug, Clone)]
pub struct ParsedStackTrace {
    /// Original stack trace
    pub original: StackTrace,
    /// Parsed frames
    pub parsed_frames: Vec<ParsedStackFrame>,
    /// Source locations
    pub source_locations: Vec<SourceLocation>,
}

/// Parsed stack frame
#[derive(Debug, Clone)]
pub struct ParsedStackFrame {
    /// Original frame
    pub original: StackFrame,
    /// Parsed function name
    pub parsed_function_name: String,
    /// Parsed file URL
    pub parsed_file_url: String,
    /// Parsed line number
    pub parsed_line_number: u32,
    /// Parsed column number
    pub parsed_column_number: u32,
    /// Source code line
    pub source_code_line: Option<String>,
    /// Is mapped
    pub is_mapped: bool,
}

/// Console state
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ConsoleState {
    /// Console is idle
    Idle,
    /// Console is evaluating
    Evaluating,
    /// Console is filtering
    Filtering,
    /// Console is clearing
    Clearing,
}

impl ConsoleInspector {
    /// Create new console inspector
    pub fn new() -> Self {
        Self {
            messages: Arc::new(RwLock::new(Vec::new())),
            filters: Arc::new(RwLock::new(ConsoleFilters::new())),
            evaluator: Arc::new(RwLock::new(RuntimeEvaluator::new())),
            source_maps: Arc::new(RwLock::new(SourceMapManager::new())),
            stack_trace_parser: Arc::new(RwLock::new(StackTraceParser::new())),
            state: ConsoleState::Idle,
        }
    }

    /// Add console message
    pub async fn add_message(&self, message_type: ConsoleMessageType, text: &str, arguments: Vec<ConsoleArgument>) -> Result<()> {
        let message_id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let level = match message_type {
            ConsoleMessageType::Error => ConsoleLevel::Error,
            ConsoleMessageType::Warning => ConsoleLevel::Warning,
            ConsoleMessageType::Info => ConsoleLevel::Info,
            _ => ConsoleLevel::Verbose,
        };
        
        let message = ConsoleMessage {
            id: message_id,
            message_type,
            level,
            text: text.to_string(),
            arguments,
            source: None,
            stack_trace: None,
            timestamp,
            is_expanded: false,
            is_selected: false,
            group_depth: 0,
            group_collapsed: false,
        };
        
        let mut messages = self.messages.write();
        messages.push(message);
        
        Ok(())
    }

    /// Get console messages
    pub async fn get_messages(&self) -> Result<Vec<ConsoleMessage>> {
        let messages = self.messages.read();
        let filters = self.filters.read();
        
        let filtered_messages = messages
            .iter()
            .filter(|msg| filters.matches_message(msg))
            .cloned()
            .collect();
        
        Ok(filtered_messages)
    }

    /// Clear console
    pub async fn clear_console(&self) -> Result<()> {
        let mut messages = self.messages.write();
        messages.clear();
        
        Ok(())
    }

    /// Evaluate expression
    pub async fn evaluate_expression(&self, expression: &str) -> Result<EvaluationResult> {
        let mut evaluator = self.evaluator.write();
        let result = evaluator.evaluate(expression)?;
        
        Ok(result)
    }

    /// Get evaluation history
    pub async fn get_evaluation_history(&self) -> Result<Vec<String>> {
        let evaluator = self.evaluator.read();
        Ok(evaluator.get_expression_history())
    }

    /// Get evaluation results
    pub async fn get_evaluation_results(&self) -> Result<Vec<EvaluationResult>> {
        let evaluator = self.evaluator.read();
        Ok(evaluator.get_evaluation_results())
    }

    /// Add source map
    pub async fn add_source_map(&self, url: &str, content: &str) -> Result<()> {
        let mut source_maps = self.source_maps.write();
        source_maps.add_source_map(url, content)?;
        
        Ok(())
    }

    /// Get mapped location
    pub async fn get_mapped_location(&self, file_url: &str, line: u32, column: u32) -> Result<Option<MappedLocation>> {
        let source_maps = self.source_maps.read();
        Ok(source_maps.get_mapped_location(file_url, line, column))
    }

    /// Parse stack trace
    pub async fn parse_stack_trace(&self, stack_trace: &str) -> Result<ParsedStackTrace> {
        let mut parser = self.stack_trace_parser.write();
        let parsed = parser.parse_stack_trace(stack_trace)?;
        
        Ok(parsed)
    }

    /// Get source code line
    pub async fn get_source_code_line(&self, file_url: &str, line_number: u32) -> Result<Option<String>> {
        let source_maps = self.source_maps.read();
        Ok(source_maps.get_source_code_line(file_url, line_number))
    }

    /// Set console filter
    pub async fn set_filter(&self, filter_type: FilterType, value: String) -> Result<()> {
        let mut filters = self.filters.write();
        filters.set_filter(filter_type, value)?;
        
        Ok(())
    }

    /// Get console filters
    pub async fn get_filters(&self) -> Result<ConsoleFilters> {
        let filters = self.filters.read();
        Ok(filters.clone())
    }

    /// Get auto-complete suggestions
    pub async fn get_auto_complete_suggestions(&self, partial_expression: &str) -> Result<Vec<String>> {
        let evaluator = self.evaluator.read();
        Ok(evaluator.get_suggestions(partial_expression))
    }

    /// Get console statistics
    pub async fn get_console_stats(&self) -> Result<ConsoleStats> {
        let messages = self.messages.read();
        let mut stats = ConsoleStats::default();
        
        for message in messages.iter() {
            stats.total_messages += 1;
            
            match message.level {
                ConsoleLevel::Verbose => stats.verbose_messages += 1,
                ConsoleLevel::Info => stats.info_messages += 1,
                ConsoleLevel::Warning => stats.warning_messages += 1,
                ConsoleLevel::Error => stats.error_messages += 1,
            }
            
            match message.message_type {
                ConsoleMessageType::Log => stats.log_messages += 1,
                ConsoleMessageType::Error => stats.error_messages += 1,
                ConsoleMessageType::Warning => stats.warning_messages += 1,
                ConsoleMessageType::Info => stats.info_messages += 1,
                ConsoleMessageType::Debug => stats.debug_messages += 1,
                ConsoleMessageType::Table => stats.table_messages += 1,
                ConsoleMessageType::Time => stats.time_messages += 1,
                ConsoleMessageType::Count => stats.count_messages += 1,
                ConsoleMessageType::Trace => stats.trace_messages += 1,
                ConsoleMessageType::Assert => stats.assert_messages += 1,
                _ => {}
            }
        }
        
        Ok(stats)
    }

    /// Get console state
    pub fn get_state(&self) -> ConsoleState {
        self.state
    }

    /// Set console state
    pub fn set_state(&mut self, state: ConsoleState) {
        self.state = state;
    }
}

impl ConsoleFilters {
    /// Create new console filters
    pub fn new() -> Self {
        let mut level_filters = HashMap::new();
        level_filters.insert(ConsoleLevel::Verbose, true);
        level_filters.insert(ConsoleLevel::Info, true);
        level_filters.insert(ConsoleLevel::Warning, true);
        level_filters.insert(ConsoleLevel::Error, true);
        
        let mut type_filters = HashMap::new();
        type_filters.insert(ConsoleMessageType::Log, true);
        type_filters.insert(ConsoleMessageType::Info, true);
        type_filters.insert(ConsoleMessageType::Warning, true);
        type_filters.insert(ConsoleMessageType::Error, true);
        type_filters.insert(ConsoleMessageType::Debug, true);
        type_filters.insert(ConsoleMessageType::Table, true);
        type_filters.insert(ConsoleMessageType::Group, true);
        type_filters.insert(ConsoleMessageType::Time, true);
        type_filters.insert(ConsoleMessageType::Count, true);
        type_filters.insert(ConsoleMessageType::Trace, true);
        type_filters.insert(ConsoleMessageType::Assert, true);
        
        Self {
            level_filters,
            type_filters,
            text_filter: None,
            source_filter: None,
            is_regex_filter: false,
            is_case_sensitive: false,
        }
    }

    /// Set filter
    pub fn set_filter(&mut self, filter_type: FilterType, value: String) -> Result<()> {
        match filter_type {
            FilterType::Level(level) => {
                self.level_filters.insert(level, value == "true");
            }
            FilterType::MessageType(message_type) => {
                self.type_filters.insert(message_type, value == "true");
            }
            FilterType::Text => {
                self.text_filter = if value.is_empty() { None } else { Some(value) };
            }
            FilterType::Source => {
                self.source_filter = if value.is_empty() { None } else { Some(value) };
            }
            FilterType::Regex => {
                self.is_regex_filter = value == "true";
            }
            FilterType::CaseSensitive => {
                self.is_case_sensitive = value == "true";
            }
        }
        
        Ok(())
    }

    /// Check if message matches filters
    pub fn matches_message(&self, message: &ConsoleMessage) -> bool {
        // Check level filter
        if !self.level_filters.get(&message.level).unwrap_or(&true) {
            return false;
        }
        
        // Check type filter
        if !self.type_filters.get(&message.message_type).unwrap_or(&true) {
            return false;
        }
        
        // Check text filter
        if let Some(ref text_filter) = self.text_filter {
            let text_to_check = if self.is_case_sensitive {
                &message.text
            } else {
                &message.text.to_lowercase()
            };
            
            let filter_text = if self.is_case_sensitive {
                text_filter
            } else {
                &text_filter.to_lowercase()
            };
            
            if self.is_regex_filter {
                // This is a simplified implementation
                // In a real implementation, you would use a regex library
                if !text_to_check.contains(filter_text) {
                    return false;
                }
            } else {
                if !text_to_check.contains(filter_text) {
                    return false;
                }
            }
        }
        
        // Check source filter
        if let Some(ref source_filter) = self.source_filter {
            if let Some(ref source) = message.source {
                if !source.file_url.contains(source_filter) {
                    return false;
                }
            } else {
                return false;
            }
        }
        
        true
    }
}

/// Filter type
#[derive(Debug, Clone)]
pub enum FilterType {
    /// Level filter
    Level(ConsoleLevel),
    /// Message type filter
    MessageType(ConsoleMessageType),
    /// Text filter
    Text,
    /// Source filter
    Source,
    /// Regex filter
    Regex,
    /// Case sensitive filter
    CaseSensitive,
}

impl RuntimeEvaluator {
    /// Create new runtime evaluator
    pub fn new() -> Self {
        Self {
            context: EvaluationContext::new(),
            expression_history: Vec::new(),
            evaluation_results: HashMap::new(),
            auto_complete: ExpressionAutoComplete::new(),
        }
    }

    /// Evaluate expression
    pub fn evaluate(&mut self, expression: &str) -> Result<EvaluationResult> {
        let result_id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Add to history
        self.expression_history.push(expression.to_string());
        
        // This is a simplified implementation
        // In a real implementation, you would evaluate the JavaScript expression
        let start_time = SystemTime::now();
        
        // Simulate evaluation
        let result_value = serde_json::json!("evaluated result");
        let result_type = "string".to_string();
        let error = None;
        
        let execution_time = SystemTime::now()
            .duration_since(start_time)
            .unwrap()
            .as_micros() as u64;
        
        let result = EvaluationResult {
            id: result_id.clone(),
            expression: expression.to_string(),
            result: result_value,
            result_type,
            execution_time,
            error,
            timestamp,
        };
        
        self.evaluation_results.insert(result_id, result.clone());
        
        Ok(result)
    }

    /// Get expression history
    pub fn get_expression_history(&self) -> Vec<String> {
        self.expression_history.clone()
    }

    /// Get evaluation results
    pub fn get_evaluation_results(&self) -> Vec<EvaluationResult> {
        self.evaluation_results.values().cloned().collect()
    }

    /// Get suggestions
    pub fn get_suggestions(&self, partial_expression: &str) -> Vec<String> {
        self.auto_complete.get_suggestions(partial_expression)
    }
}

impl EvaluationContext {
    /// Create new evaluation context
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "Console".to_string(),
            variables: HashMap::new(),
            functions: HashMap::new(),
            scope: EvaluationScope::Top,
        }
    }
}

impl ExpressionAutoComplete {
    /// Create new expression auto-complete
    pub fn new() -> Self {
        let keywords = vec![
            "var", "let", "const", "function", "if", "else", "for", "while",
            "return", "break", "continue", "try", "catch", "finally", "throw",
            "new", "delete", "typeof", "instanceof", "in", "of", "with",
        ];
        
        let global_objects = vec![
            "window", "document", "console", "Math", "Date", "Array", "Object",
            "String", "Number", "Boolean", "Function", "RegExp", "Error",
            "Promise", "Map", "Set", "WeakMap", "WeakSet", "Symbol",
        ];
        
        let dom_elements = vec![
            "document.getElementById", "document.querySelector", "document.querySelectorAll",
            "document.createElement", "document.createTextNode", "document.body",
            "document.head", "document.title", "document.URL", "document.domain",
        ];
        
        let css_selectors = vec![
            "querySelector", "querySelectorAll", "getElementById", "getElementsByClassName",
            "getElementsByTagName", "getElementsByName", "closest", "matches",
        ];
        
        Self {
            keywords,
            global_objects,
            dom_elements,
            css_selectors,
        }
    }

    /// Get suggestions
    pub fn get_suggestions(&self, partial_expression: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // Add keyword suggestions
        suggestions.extend(self.keywords.iter().filter(|k| k.starts_with(partial_expression)).cloned());
        
        // Add global object suggestions
        suggestions.extend(self.global_objects.iter().filter(|g| g.starts_with(partial_expression)).cloned());
        
        // Add DOM element suggestions
        suggestions.extend(self.dom_elements.iter().filter(|d| d.starts_with(partial_expression)).cloned());
        
        // Add CSS selector suggestions
        suggestions.extend(self.css_selectors.iter().filter(|c| c.starts_with(partial_expression)).cloned());
        
        suggestions
    }
}

impl SourceMapManager {
    /// Create new source map manager
    pub fn new() -> Self {
        Self {
            source_maps: HashMap::new(),
            mapped_locations: HashMap::new(),
        }
    }

    /// Add source map
    pub fn add_source_map(&mut self, url: &str, content: &str) -> Result<()> {
        let source_map_id = Uuid::new_v4().to_string();
        
        // This is a simplified implementation
        // In a real implementation, you would parse the source map content
        
        let source_map = SourceMap {
            id: source_map_id.clone(),
            url: url.to_string(),
            content: content.to_string(),
            sources: Vec::new(),
            source_content: HashMap::new(),
            mappings: Vec::new(),
            names: Vec::new(),
        };
        
        self.source_maps.insert(source_map_id, source_map);
        
        Ok(())
    }

    /// Get mapped location
    pub fn get_mapped_location(&self, file_url: &str, line: u32, column: u32) -> Option<MappedLocation> {
        let key = format!("{}:{}:{}", file_url, line, column);
        self.mapped_locations.get(&key).cloned()
    }

    /// Get source code line
    pub fn get_source_code_line(&self, file_url: &str, line_number: u32) -> Option<String> {
        // This is a simplified implementation
        // In a real implementation, you would read the source file and return the specific line
        Some(format!("// Source code for {}:{}", file_url, line_number))
    }
}

impl StackTraceParser {
    /// Create new stack trace parser
    pub fn new() -> Self {
        Self {
            parsed_stack_traces: HashMap::new(),
            source_map_manager: Arc::new(RwLock::new(SourceMapManager::new())),
        }
    }

    /// Parse stack trace
    pub fn parse_stack_trace(&mut self, stack_trace: &str) -> Result<ParsedStackTrace> {
        let trace_id = Uuid::new_v4().to_string();
        
        // This is a simplified implementation
        // In a real implementation, you would parse the stack trace string
        
        let frames = vec![
            StackFrame {
                function_name: "anonymous".to_string(),
                file_url: "script.js".to_string(),
                line_number: 1,
                column_number: 1,
                script_id: None,
                is_native: false,
                is_eval: false,
                is_constructor: false,
            }
        ];
        
        let original = StackTrace {
            frames,
            is_truncated: false,
        };
        
        let parsed_frames = vec![
            ParsedStackFrame {
                original: original.frames[0].clone(),
                parsed_function_name: "anonymous".to_string(),
                parsed_file_url: "script.js".to_string(),
                parsed_line_number: 1,
                parsed_column_number: 1,
                source_code_line: Some("console.log('Hello, World!');".to_string()),
                is_mapped: false,
            }
        ];
        
        let source_locations = vec![
            SourceLocation {
                file_url: "script.js".to_string(),
                line_number: 1,
                column_number: 1,
                function_name: Some("anonymous".to_string()),
                script_id: None,
            }
        ];
        
        let parsed = ParsedStackTrace {
            original,
            parsed_frames,
            source_locations,
        };
        
        self.parsed_stack_traces.insert(trace_id, parsed.clone());
        
        Ok(parsed)
    }
}

/// Console statistics
#[derive(Debug, Clone, Default)]
pub struct ConsoleStats {
    /// Total messages
    pub total_messages: usize,
    /// Verbose messages
    pub verbose_messages: usize,
    /// Info messages
    pub info_messages: usize,
    /// Warning messages
    pub warning_messages: usize,
    /// Error messages
    pub error_messages: usize,
    /// Log messages
    pub log_messages: usize,
    /// Debug messages
    pub debug_messages: usize,
    /// Table messages
    pub table_messages: usize,
    /// Time messages
    pub time_messages: usize,
    /// Count messages
    pub count_messages: usize,
    /// Trace messages
    pub trace_messages: usize,
    /// Assert messages
    pub assert_messages: usize,
}
