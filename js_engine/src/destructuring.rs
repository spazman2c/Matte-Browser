use crate::error::{Error, Result};
use crate::ast::{Pattern, Expression, ObjectPattern, ArrayPattern, RestElement, AssignmentPattern, Identifier, Literal};
use std::collections::HashMap;

/// Destructuring context for managing variable assignments
#[derive(Debug, Clone)]
pub struct DestructuringContext {
    /// Variables to be assigned
    pub variables: HashMap<String, Value>,
    /// Default values for patterns
    pub defaults: HashMap<String, Value>,
    /// Rest parameters
    pub rest_params: HashMap<String, Vec<Value>>,
}

/// JavaScript value (reused from other modules)
#[derive(Debug, Clone)]
pub enum Value {
    /// Undefined value
    Undefined,
    /// Null value
    Null,
    /// Boolean value
    Boolean(bool),
    /// Number value
    Number(f64),
    /// String value
    String(String),
    /// Array value
    Array(Vec<Value>),
    /// Object value
    Object(HashMap<String, Value>),
    /// Function value
    Function(FunctionValue),
}

/// Function value
#[derive(Debug, Clone)]
pub struct FunctionValue {
    /// Function name
    pub name: String,
    /// Function parameters
    pub parameters: Vec<String>,
    /// Function body
    pub body: Vec<crate::ast::Statement>,
    /// Closure environment
    pub environment: HashMap<String, Value>,
}

/// Destructuring engine
pub struct DestructuringEngine {
    /// Current destructuring context
    context: DestructuringContext,
}

impl DestructuringEngine {
    /// Create a new destructuring engine
    pub fn new() -> Self {
        Self {
            context: DestructuringContext {
                variables: HashMap::new(),
                defaults: HashMap::new(),
                rest_params: HashMap::new(),
            },
        }
    }

    /// Destructure an object into variables
    pub fn destructure_object(&mut self, pattern: &ObjectPattern, value: Value) -> Result<()> {
        let object = match value {
            Value::Object(obj) => obj,
            Value::Null | Value::Undefined => {
                return Err(Error::parsing("Cannot destructure null or undefined".to_string()));
            }
            _ => {
                // Try to convert to object (for primitives)
                self.convert_to_object(value)?
            }
        };

        for property in &pattern.properties {
            match property {
                crate::ast::ObjectPatternProperty::Single(single_prop) => {
                    self.destructure_single_property(single_prop, &object)?;
                }
                crate::ast::ObjectPatternProperty::Rest(rest_prop) => {
                    self.destructure_rest_property(rest_prop, &object)?;
                }
            }
        }

        Ok(())
    }

    /// Destructure an array into variables
    pub fn destructure_array(&mut self, pattern: &ArrayPattern, value: Value) -> Result<()> {
        let array = match value {
            Value::Array(arr) => arr,
            Value::Null | Value::Undefined => {
                return Err(Error::parsing("Cannot destructure null or undefined".to_string()));
            }
            _ => {
                // Try to convert to array (for iterables)
                self.convert_to_array(value)?
            }
        };

        let mut array_index = 0;

        for element in &pattern.elements {
            match element {
                Some(Pattern::Identifier(ident)) => {
                    if array_index < array.len() {
                        self.context.variables.insert(ident.name.clone(), array[array_index].clone());
                    } else {
                        // Use default value if available
                        if let Some(default) = self.context.defaults.get(&ident.name) {
                            self.context.variables.insert(ident.name.clone(), default.clone());
                        } else {
                            self.context.variables.insert(ident.name.clone(), Value::Undefined);
                        }
                    }
                    array_index += 1;
                }
                Some(Pattern::Object(obj_pattern)) => {
                    if array_index < array.len() {
                        self.destructure_object(obj_pattern, array[array_index].clone())?;
                    } else {
                        // Use default value if available
                        if let Some(default) = self.context.defaults.get(&obj_pattern.to_string()) {
                            self.destructure_object(obj_pattern, default.clone())?;
                        }
                    }
                    array_index += 1;
                }
                Some(Pattern::Array(arr_pattern)) => {
                    if array_index < array.len() {
                        self.destructure_array(arr_pattern, array[array_index].clone())?;
                    } else {
                        // Use default value if available
                        if let Some(default) = self.context.defaults.get(&arr_pattern.to_string()) {
                            self.destructure_array(arr_pattern, default.clone())?;
                        }
                    }
                    array_index += 1;
                }
                Some(Pattern::Rest(rest_pattern)) => {
                    // Handle rest pattern
                    let rest_values = if array_index < array.len() {
                        array[array_index..].to_vec()
                    } else {
                        Vec::new()
                    };
                    self.context.rest_params.insert(rest_pattern.argument.name.clone(), rest_values);
                }
                Some(Pattern::Assignment(assignment_pattern)) => {
                    // Handle assignment pattern with default
                    if array_index < array.len() {
                        self.destructure_assignment_pattern(assignment_pattern, array[array_index].clone())?;
                    } else {
                        // Use default value
                        if let Some(default) = self.context.defaults.get(&assignment_pattern.to_string()) {
                            self.destructure_assignment_pattern(assignment_pattern, default.clone())?;
                        }
                    }
                    array_index += 1;
                }
                None => {
                    // Skip this position
                    array_index += 1;
                }
                _ => {
                    // Handle other pattern types
                    array_index += 1;
                }
            }
        }

        Ok(())
    }

    /// Destructure a single object property
    fn destructure_single_property(&mut self, property: &crate::ast::SingleProperty, object: &HashMap<String, Value>) -> Result<()> {
        let key = match &property.key {
            crate::ast::PropertyKey::Identifier(ident) => ident.name.clone(),
            crate::ast::PropertyKey::Literal(literal) => {
                match literal {
                    Literal::String(s) => s.clone(),
                    Literal::Number(n) => n.to_string(),
                    _ => return Err(Error::parsing("Unsupported property key type".to_string())),
                }
            }
            _ => return Err(Error::parsing("Unsupported property key type".to_string())),
        };

        let value = object.get(&key).cloned().unwrap_or(Value::Undefined);

        match &property.value {
            Pattern::Identifier(ident) => {
                self.context.variables.insert(ident.name.clone(), value);
            }
            Pattern::Object(obj_pattern) => {
                self.destructure_object(obj_pattern, value)?;
            }
            Pattern::Array(arr_pattern) => {
                self.destructure_array(arr_pattern, value)?;
            }
            Pattern::Assignment(assignment_pattern) => {
                self.destructure_assignment_pattern(assignment_pattern, value)?;
            }
            _ => {
                return Err(Error::parsing("Unsupported pattern type in object destructuring".to_string()));
            }
        }

        Ok(())
    }

    /// Destructure a rest property
    fn destructure_rest_property(&mut self, property: &crate::ast::RestProperty, object: &HashMap<String, Value>) -> Result<()> {
        // Create a new object with remaining properties
        let mut rest_object = HashMap::new();
        
        // Get all property names that have been destructured
        let destructured_keys: std::collections::HashSet<String> = self.context.variables.keys().cloned().collect();
        
        for (key, value) in object {
            if !destructured_keys.contains(key) {
                rest_object.insert(key.clone(), value.clone());
            }
        }

        match &property.argument {
            Pattern::Identifier(ident) => {
                self.context.variables.insert(ident.name.clone(), Value::Object(rest_object));
            }
            _ => {
                return Err(Error::parsing("Rest property must be an identifier".to_string()));
            }
        }

        Ok(())
    }

    /// Destructure an assignment pattern
    fn destructure_assignment_pattern(&mut self, pattern: &AssignmentPattern, value: Value) -> Result<()> {
        // Check if value is undefined, use default if so
        let final_value = if matches!(value, Value::Undefined) {
            match &pattern.right {
                Expression::Literal(literal) => self.parse_literal(literal)?,
                _ => {
                    // For now, just use undefined for complex expressions
                    Value::Undefined
                }
            }
        } else {
            value
        };

        match &pattern.left {
            Pattern::Identifier(ident) => {
                self.context.variables.insert(ident.name.clone(), final_value);
            }
            Pattern::Object(obj_pattern) => {
                self.destructure_object(obj_pattern, final_value)?;
            }
            Pattern::Array(arr_pattern) => {
                self.destructure_array(arr_pattern, final_value)?;
            }
            _ => {
                return Err(Error::parsing("Unsupported pattern type in assignment".to_string()));
            }
        }

        Ok(())
    }

    /// Convert a value to an object
    fn convert_to_object(&self, value: Value) -> Result<HashMap<String, Value>> {
        match value {
            Value::String(s) => {
                let mut obj = HashMap::new();
                obj.insert("0".to_string(), Value::String(s));
                obj.insert("length".to_string(), Value::Number(1.0));
                Ok(obj)
            }
            Value::Number(n) => {
                let mut obj = HashMap::new();
                obj.insert("valueOf".to_string(), Value::Function(FunctionValue {
                    name: "valueOf".to_string(),
                    parameters: Vec::new(),
                    body: Vec::new(),
                    environment: HashMap::new(),
                }));
                Ok(obj)
            }
            Value::Boolean(b) => {
                let mut obj = HashMap::new();
                obj.insert("valueOf".to_string(), Value::Function(FunctionValue {
                    name: "valueOf".to_string(),
                    parameters: Vec::new(),
                    body: Vec::new(),
                    environment: HashMap::new(),
                }));
                Ok(obj)
            }
            _ => Err(Error::parsing("Cannot convert value to object".to_string())),
        }
    }

    /// Convert a value to an array
    fn convert_to_array(&self, value: Value) -> Result<Vec<Value>> {
        match value {
            Value::String(s) => {
                let chars: Vec<Value> = s.chars().map(|c| Value::String(c.to_string())).collect();
                Ok(chars)
            }
            Value::Object(obj) => {
                // Convert object to array-like structure
                let mut array = Vec::new();
                for (key, value) in obj {
                    if let Ok(index) = key.parse::<usize>() {
                        while array.len() <= index {
                            array.push(Value::Undefined);
                        }
                        array[index] = value;
                    }
                }
                Ok(array)
            }
            _ => Err(Error::parsing("Cannot convert value to array".to_string())),
        }
    }

    /// Parse a literal expression
    fn parse_literal(&self, literal: &Literal) -> Result<Value> {
        match literal {
            Literal::String(s) => Ok(Value::String(s.clone())),
            Literal::Number(n) => Ok(Value::Number(*n)),
            Literal::Boolean(b) => Ok(Value::Boolean(*b)),
            Literal::Null => Ok(Value::Null),
            _ => Ok(Value::Undefined),
        }
    }

    /// Get all destructured variables
    pub fn get_variables(&self) -> &HashMap<String, Value> {
        &self.context.variables
    }

    /// Get a specific variable
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.context.variables.get(name)
    }

    /// Get rest parameters
    pub fn get_rest_params(&self) -> &HashMap<String, Vec<Value>> {
        &self.context.rest_params
    }

    /// Clear the destructuring context
    pub fn clear(&mut self) {
        self.context.variables.clear();
        self.context.defaults.clear();
        self.context.rest_params.clear();
    }
}

/// Spread operator implementation
pub struct SpreadOperator {
    /// Spread engine
    engine: DestructuringEngine,
}

impl SpreadOperator {
    /// Create a new spread operator
    pub fn new() -> Self {
        Self {
            engine: DestructuringEngine::new(),
        }
    }

    /// Spread an array into another array
    pub fn spread_array(&self, target: &mut Vec<Value>, source: &[Value]) {
        target.extend_from_slice(source);
    }

    /// Spread an object into another object
    pub fn spread_object(&self, target: &mut HashMap<String, Value>, source: &HashMap<String, Value>) {
        for (key, value) in source {
            target.insert(key.clone(), value.clone());
        }
    }

    /// Spread values into function arguments
    pub fn spread_arguments(&self, args: &[Value]) -> Vec<Value> {
        let mut result = Vec::new();
        
        for arg in args {
            match arg {
                Value::Array(arr) => {
                    self.spread_array(&mut result, arr);
                }
                _ => {
                    result.push(arg.clone());
                }
            }
        }
        
        result
    }

    /// Create a new array with spread elements
    pub fn create_spread_array(&self, elements: &[Value]) -> Vec<Value> {
        let mut result = Vec::new();
        
        for element in elements {
            match element {
                Value::Array(arr) => {
                    self.spread_array(&mut result, arr);
                }
                _ => {
                    result.push(element.clone());
                }
            }
        }
        
        result
    }

    /// Create a new object with spread properties
    pub fn create_spread_object(&self, properties: &[(String, Value)]) -> HashMap<String, Value> {
        let mut result = HashMap::new();
        
        for (key, value) in properties {
            match value {
                Value::Object(obj) => {
                    self.spread_object(&mut result, obj);
                }
                _ => {
                    result.insert(key.clone(), value.clone());
                }
            }
        }
        
        result
    }
}

/// Destructuring and spread system
pub struct DestructuringSystem {
    /// Destructuring engine
    destructuring_engine: DestructuringEngine,
    /// Spread operator
    spread_operator: SpreadOperator,
}

impl DestructuringSystem {
    /// Create a new destructuring system
    pub fn new() -> Self {
        Self {
            destructuring_engine: DestructuringEngine::new(),
            spread_operator: SpreadOperator::new(),
        }
    }

    /// Destructure an object
    pub fn destructure_object(&mut self, pattern: &ObjectPattern, value: Value) -> Result<()> {
        self.destructuring_engine.destructure_object(pattern, value)
    }

    /// Destructure an array
    pub fn destructure_array(&mut self, pattern: &ArrayPattern, value: Value) -> Result<()> {
        self.destructuring_engine.destructure_array(pattern, value)
    }

    /// Spread an array
    pub fn spread_array(&self, target: &mut Vec<Value>, source: &[Value]) {
        self.spread_operator.spread_array(target, source);
    }

    /// Spread an object
    pub fn spread_object(&self, target: &mut HashMap<String, Value>, source: &HashMap<String, Value>) {
        self.spread_operator.spread_object(target, source);
    }

    /// Create a spread array
    pub fn create_spread_array(&self, elements: &[Value]) -> Vec<Value> {
        self.spread_operator.create_spread_array(elements)
    }

    /// Create a spread object
    pub fn create_spread_object(&self, properties: &[(String, Value)]) -> HashMap<String, Value> {
        self.spread_operator.create_spread_object(properties)
    }

    /// Get destructured variables
    pub fn get_variables(&self) -> &HashMap<String, Value> {
        self.destructuring_engine.get_variables()
    }

    /// Get a specific variable
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.destructuring_engine.get_variable(name)
    }

    /// Get rest parameters
    pub fn get_rest_params(&self) -> &HashMap<String, Vec<Value>> {
        self.destructuring_engine.get_rest_params()
    }

    /// Clear the system
    pub fn clear(&mut self) {
        self.destructuring_engine.clear();
    }
}

/// Pattern matcher for complex destructuring patterns
pub struct PatternMatcher {
    /// Current matching context
    context: HashMap<String, Value>,
}

impl PatternMatcher {
    /// Create a new pattern matcher
    pub fn new() -> Self {
        Self {
            context: HashMap::new(),
        }
    }

    /// Match a pattern against a value
    pub fn match_pattern(&mut self, pattern: &Pattern, value: Value) -> Result<bool> {
        match pattern {
            Pattern::Identifier(ident) => {
                self.context.insert(ident.name.clone(), value);
                Ok(true)
            }
            Pattern::Object(obj_pattern) => {
                self.match_object_pattern(obj_pattern, value)
            }
            Pattern::Array(arr_pattern) => {
                self.match_array_pattern(arr_pattern, value)
            }
            Pattern::Rest(rest_pattern) => {
                self.match_rest_pattern(rest_pattern, value)
            }
            Pattern::Assignment(assignment_pattern) => {
                self.match_assignment_pattern(assignment_pattern, value)
            }
            _ => Ok(false),
        }
    }

    /// Match an object pattern
    fn match_object_pattern(&mut self, pattern: &ObjectPattern, value: Value) -> Result<bool> {
        let object = match value {
            Value::Object(obj) => obj,
            _ => return Ok(false),
        };

        for property in &pattern.properties {
            match property {
                crate::ast::ObjectPatternProperty::Single(single_prop) => {
                    let key = match &single_prop.key {
                        crate::ast::PropertyKey::Identifier(ident) => ident.name.clone(),
                        crate::ast::PropertyKey::Literal(literal) => {
                            match literal {
                                Literal::String(s) => s.clone(),
                                Literal::Number(n) => n.to_string(),
                                _ => return Ok(false),
                            }
                        }
                        _ => return Ok(false),
                    };

                    if let Some(prop_value) = object.get(&key) {
                        if !self.match_pattern(&single_prop.value, prop_value.clone())? {
                            return Ok(false);
                        }
                    } else {
                        return Ok(false);
                    }
                }
                _ => {
                    // Handle other property types
                }
            }
        }

        Ok(true)
    }

    /// Match an array pattern
    fn match_array_pattern(&mut self, pattern: &ArrayPattern, value: Value) -> Result<bool> {
        let array = match value {
            Value::Array(arr) => arr,
            _ => return Ok(false),
        };

        let mut array_index = 0;

        for element in &pattern.elements {
            match element {
                Some(pattern) => {
                    if array_index < array.len() {
                        if !self.match_pattern(pattern, array[array_index].clone())? {
                            return Ok(false);
                        }
                    } else {
                        return Ok(false);
                    }
                    array_index += 1;
                }
                None => {
                    array_index += 1;
                }
            }
        }

        Ok(true)
    }

    /// Match a rest pattern
    fn match_rest_pattern(&mut self, pattern: &RestElement, value: Value) -> Result<bool> {
        // For now, just store the value
        self.context.insert(pattern.argument.name.clone(), value);
        Ok(true)
    }

    /// Match an assignment pattern
    fn match_assignment_pattern(&mut self, pattern: &AssignmentPattern, value: Value) -> Result<bool> {
        let final_value = if matches!(value, Value::Undefined) {
            match &pattern.right {
                Expression::Literal(literal) => self.parse_literal(literal)?,
                _ => Value::Undefined,
            }
        } else {
            value
        };

        self.match_pattern(&pattern.left, final_value)
    }

    /// Parse a literal expression
    fn parse_literal(&self, literal: &Literal) -> Result<Value> {
        match literal {
            Literal::String(s) => Ok(Value::String(s.clone())),
            Literal::Number(n) => Ok(Value::Number(*n)),
            Literal::Boolean(b) => Ok(Value::Boolean(*b)),
            Literal::Null => Ok(Value::Null),
            _ => Ok(Value::Undefined),
        }
    }

    /// Get matched values
    pub fn get_matched_values(&self) -> &HashMap<String, Value> {
        &self.context
    }

    /// Clear the matcher
    pub fn clear(&mut self) {
        self.context.clear();
    }
}
