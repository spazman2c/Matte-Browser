use crate::error::{Error, Result};
use crate::ast::{ClassDeclaration, ClassExpression, MethodDefinition, ClassElement, Expression, Statement, Identifier, Literal};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Class definition
#[derive(Debug, Clone)]
pub struct ClassDefinition {
    /// Class name
    pub name: String,
    /// Superclass (if any)
    pub superclass: Option<Box<ClassDefinition>>,
    /// Constructor method
    pub constructor: Option<MethodDefinition>,
    /// Instance methods
    pub methods: HashMap<String, MethodDefinition>,
    /// Static methods
    pub static_methods: HashMap<String, MethodDefinition>,
    /// Instance properties
    pub properties: HashMap<String, PropertyDefinition>,
    /// Static properties
    pub static_properties: HashMap<String, PropertyDefinition>,
    /// Private fields
    pub private_fields: HashMap<String, PrivateFieldDefinition>,
    /// Class prototype
    pub prototype: Option<ClassPrototype>,
}

/// Method definition
#[derive(Debug, Clone)]
pub struct MethodDefinition {
    /// Method name
    pub name: String,
    /// Method kind (constructor, method, get, set)
    pub kind: MethodKind,
    /// Method parameters
    pub parameters: Vec<Parameter>,
    /// Method body
    pub body: Vec<Statement>,
    /// Whether method is static
    pub is_static: bool,
    /// Whether method is private
    pub is_private: bool,
    /// Method implementation
    pub implementation: Option<MethodImplementation>,
}

/// Method kind
#[derive(Debug, Clone, PartialEq)]
pub enum MethodKind {
    /// Constructor method
    Constructor,
    /// Regular method
    Method,
    /// Getter method
    Getter,
    /// Setter method
    Setter,
}

/// Parameter definition
#[derive(Debug, Clone)]
pub struct Parameter {
    /// Parameter name
    pub name: String,
    /// Default value (if any)
    pub default_value: Option<Value>,
    /// Whether parameter is rest parameter
    pub is_rest: bool,
}

/// Property definition
#[derive(Debug, Clone)]
pub struct PropertyDefinition {
    /// Property name
    pub name: String,
    /// Property value
    pub value: Option<Value>,
    /// Whether property is writable
    pub writable: bool,
    /// Whether property is enumerable
    pub enumerable: bool,
    /// Whether property is configurable
    pub configurable: bool,
    /// Property getter (if any)
    pub getter: Option<MethodDefinition>,
    /// Property setter (if any)
    pub setter: Option<MethodDefinition>,
}

/// Private field definition
#[derive(Debug, Clone)]
pub struct PrivateFieldDefinition {
    /// Field name (without #)
    pub name: String,
    /// Field value
    pub value: Option<Value>,
    /// Whether field is writable
    pub writable: bool,
}

/// Class prototype
#[derive(Debug, Clone)]
pub struct ClassPrototype {
    /// Prototype object
    pub object: HashMap<String, Value>,
    /// Constructor reference
    pub constructor: Option<Value>,
    /// Superclass prototype (if any)
    pub super_prototype: Option<Box<ClassPrototype>>,
}

/// Method implementation
#[derive(Debug, Clone)]
pub struct MethodImplementation {
    /// Function that implements the method
    pub function: Box<dyn Fn(&[Value], &mut ClassInstance) -> Result<Value> + Send + Sync>,
    /// Method signature
    pub signature: MethodSignature,
}

/// Method signature
#[derive(Debug, Clone)]
pub struct MethodSignature {
    /// Method name
    pub name: String,
    /// Parameter names
    pub parameters: Vec<String>,
    /// Return type (optional)
    pub return_type: Option<String>,
}

/// Class instance
#[derive(Debug, Clone)]
pub struct ClassInstance {
    /// Instance properties
    pub properties: HashMap<String, Value>,
    /// Private fields
    pub private_fields: HashMap<String, Value>,
    /// Class definition
    pub class: ClassDefinition,
    /// Prototype chain
    pub prototype_chain: Vec<ClassPrototype>,
}

/// JavaScript value (reused from async_await module)
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
    /// Object value
    Object(HashMap<String, Value>),
    /// Function value
    Function(FunctionValue),
    /// Class value
    Class(ClassValue),
    /// Instance value
    Instance(ClassInstance),
}

/// Function value
#[derive(Debug, Clone)]
pub struct FunctionValue {
    /// Function name
    pub name: String,
    /// Function parameters
    pub parameters: Vec<String>,
    /// Function body
    pub body: Vec<Statement>,
    /// Closure environment
    pub environment: HashMap<String, Value>,
}

/// Class value
#[derive(Debug, Clone)]
pub struct ClassValue {
    /// Class definition
    pub definition: ClassDefinition,
    /// Class constructor
    pub constructor: Option<FunctionValue>,
}

impl ClassDefinition {
    /// Create a new class definition
    pub fn new(name: String) -> Self {
        Self {
            name,
            superclass: None,
            constructor: None,
            methods: HashMap::new(),
            static_methods: HashMap::new(),
            properties: HashMap::new(),
            static_properties: HashMap::new(),
            private_fields: HashMap::new(),
            prototype: None,
        }
    }

    /// Set the superclass
    pub fn set_superclass(&mut self, superclass: ClassDefinition) {
        self.superclass = Some(Box::new(superclass));
    }

    /// Add a constructor method
    pub fn add_constructor(&mut self, constructor: MethodDefinition) {
        self.constructor = Some(constructor);
    }

    /// Add an instance method
    pub fn add_method(&mut self, method: MethodDefinition) {
        self.methods.insert(method.name.clone(), method);
    }

    /// Add a static method
    pub fn add_static_method(&mut self, method: MethodDefinition) {
        self.static_methods.insert(method.name.clone(), method);
    }

    /// Add an instance property
    pub fn add_property(&mut self, property: PropertyDefinition) {
        self.properties.insert(property.name.clone(), property);
    }

    /// Add a static property
    pub fn add_static_property(&mut self, property: PropertyDefinition) {
        self.static_properties.insert(property.name.clone(), property);
    }

    /// Add a private field
    pub fn add_private_field(&mut self, field: PrivateFieldDefinition) {
        self.private_fields.insert(field.name.clone(), field);
    }

    /// Get a method by name
    pub fn get_method(&self, name: &str) -> Option<&MethodDefinition> {
        self.methods.get(name)
    }

    /// Get a static method by name
    pub fn get_static_method(&self, name: &str) -> Option<&MethodDefinition> {
        self.static_methods.get(name)
    }

    /// Get a property by name
    pub fn get_property(&self, name: &str) -> Option<&PropertyDefinition> {
        self.properties.get(name)
    }

    /// Get a static property by name
    pub fn get_static_property(&self, name: &str) -> Option<&PropertyDefinition> {
        self.static_properties.get(name)
    }

    /// Get a private field by name
    pub fn get_private_field(&self, name: &str) -> Option<&PrivateFieldDefinition> {
        self.private_fields.get(name)
    }

    /// Check if class has a method
    pub fn has_method(&self, name: &str) -> bool {
        self.methods.contains_key(name)
    }

    /// Check if class has a static method
    pub fn has_static_method(&self, name: &str) -> bool {
        self.static_methods.contains_key(name)
    }

    /// Check if class has a property
    pub fn has_property(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }

    /// Check if class has a static property
    pub fn has_static_property(&self, name: &str) -> bool {
        self.static_properties.contains_key(name)
    }

    /// Check if class has a private field
    pub fn has_private_field(&self, name: &str) -> bool {
        self.private_fields.contains_key(name)
    }
}

impl MethodDefinition {
    /// Create a new method definition
    pub fn new(name: String, kind: MethodKind, parameters: Vec<Parameter>, body: Vec<Statement>) -> Self {
        Self {
            name,
            kind,
            parameters,
            body,
            is_static: false,
            is_private: false,
            implementation: None,
        }
    }

    /// Set method as static
    pub fn set_static(&mut self, is_static: bool) {
        self.is_static = is_static;
    }

    /// Set method as private
    pub fn set_private(&mut self, is_private: bool) {
        self.is_private = is_private;
    }

    /// Add implementation
    pub fn add_implementation<F>(&mut self, implementation: F)
    where
        F: Fn(&[Value], &mut ClassInstance) -> Result<Value> + Send + Sync + 'static,
    {
        let signature = MethodSignature {
            name: self.name.clone(),
            parameters: self.parameters.iter().map(|p| p.name.clone()).collect(),
            return_type: None,
        };

        self.implementation = Some(MethodImplementation {
            function: Box::new(implementation),
            signature,
        });
    }

    /// Execute the method
    pub fn execute(&self, args: &[Value], instance: &mut ClassInstance) -> Result<Value> {
        if let Some(ref impl_) = self.implementation {
            (impl_.function)(args, instance)
        } else {
            // Default implementation - return undefined
            Ok(Value::Undefined)
        }
    }

    /// Get parameter count
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }

    /// Check if method is a constructor
    pub fn is_constructor(&self) -> bool {
        self.kind == MethodKind::Constructor
    }

    /// Check if method is a getter
    pub fn is_getter(&self) -> bool {
        self.kind == MethodKind::Getter
    }

    /// Check if method is a setter
    pub fn is_setter(&self) -> bool {
        self.kind == MethodKind::Setter
    }
}

impl ClassInstance {
    /// Create a new class instance
    pub fn new(class: ClassDefinition) -> Self {
        Self {
            properties: HashMap::new(),
            private_fields: HashMap::new(),
            class,
            prototype_chain: Vec::new(),
        }
    }

    /// Set a property value
    pub fn set_property(&mut self, name: String, value: Value) {
        self.properties.insert(name, value);
    }

    /// Get a property value
    pub fn get_property(&self, name: &str) -> Option<&Value> {
        self.properties.get(name)
    }

    /// Set a private field value
    pub fn set_private_field(&mut self, name: String, value: Value) {
        self.private_fields.insert(name, value);
    }

    /// Get a private field value
    pub fn get_private_field(&self, name: &str) -> Option<&Value> {
        self.private_fields.get(name)
    }

    /// Call a method
    pub fn call_method(&mut self, name: &str, args: &[Value]) -> Result<Value> {
        if let Some(method) = self.class.get_method(name) {
            method.execute(args, self)
        } else {
            // Check prototype chain
            for prototype in &self.prototype_chain {
                if let Some(value) = prototype.object.get(name) {
                    if let Value::Function(func) = value {
                        // Execute function with instance as 'this'
                        return self.execute_function(func, args);
                    }
                }
            }
            Err(Error::parsing(format!("Method '{}' not found", name)))
        }
    }

    /// Call a static method
    pub fn call_static_method(&self, name: &str, args: &[Value]) -> Result<Value> {
        if let Some(method) = self.class.get_static_method(name) {
            // For static methods, we don't pass the instance
            method.execute(args, &mut ClassInstance::new(self.class.clone()))
        } else {
            Err(Error::parsing(format!("Static method '{}' not found", name)))
        }
    }

    /// Execute a function with this instance as context
    fn execute_function(&self, func: &FunctionValue, args: &[Value]) -> Result<Value> {
        // Create a new environment with 'this' bound to the instance
        let mut env = func.environment.clone();
        env.insert("this".to_string(), Value::Instance(self.clone()));

        // Execute function body (simplified)
        Ok(Value::Undefined)
    }

    /// Check if instance has a property
    pub fn has_property(&self, name: &str) -> bool {
        self.properties.contains_key(name) || self.class.has_property(name)
    }

    /// Check if instance has a private field
    pub fn has_private_field(&self, name: &str) -> bool {
        self.private_fields.contains_key(name) || self.class.has_private_field(name)
    }

    /// Get all property names
    pub fn get_property_names(&self) -> Vec<String> {
        self.properties.keys().cloned().collect()
    }

    /// Get all private field names
    pub fn get_private_field_names(&self) -> Vec<String> {
        self.private_fields.keys().cloned().collect()
    }
}

impl ClassPrototype {
    /// Create a new class prototype
    pub fn new() -> Self {
        Self {
            object: HashMap::new(),
            constructor: None,
            super_prototype: None,
        }
    }

    /// Set a property on the prototype
    pub fn set_property(&mut self, name: String, value: Value) {
        self.object.insert(name, value);
    }

    /// Get a property from the prototype
    pub fn get_property(&self, name: &str) -> Option<&Value> {
        self.object.get(name)
    }

    /// Set the constructor
    pub fn set_constructor(&mut self, constructor: Value) {
        self.constructor = Some(constructor);
    }

    /// Set the superclass prototype
    pub fn set_super_prototype(&mut self, super_prototype: ClassPrototype) {
        self.super_prototype = Some(Box::new(super_prototype));
    }

    /// Check if prototype has a property
    pub fn has_property(&self, name: &str) -> bool {
        self.object.contains_key(name)
    }

    /// Get all property names
    pub fn get_property_names(&self) -> Vec<String> {
        self.object.keys().cloned().collect()
    }
}

/// Class system for managing classes
pub struct ClassSystem {
    /// Registered classes
    classes: Arc<RwLock<HashMap<String, ClassDefinition>>>,
    /// Class instances
    instances: Arc<RwLock<HashMap<String, ClassInstance>>>,
}

impl ClassSystem {
    /// Create a new class system
    pub fn new() -> Self {
        Self {
            classes: Arc::new(RwLock::new(HashMap::new())),
            instances: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a class
    pub async fn register_class(&self, name: String, class_def: ClassDefinition) -> Result<()> {
        let mut classes = self.classes.write().await;
        classes.insert(name, class_def);
        Ok(())
    }

    /// Get a class by name
    pub async fn get_class(&self, name: &str) -> Option<ClassDefinition> {
        let classes = self.classes.read().await;
        classes.get(name).cloned()
    }

    /// Create a class instance
    pub async fn create_instance(&self, class_name: &str, args: &[Value]) -> Result<ClassInstance> {
        let class_def = self.get_class(class_name).await
            .ok_or_else(|| Error::parsing(format!("Class '{}' not found", class_name)))?;

        let mut instance = ClassInstance::new(class_def);

        // Call constructor if it exists
        if let Some(constructor) = &instance.class.constructor {
            constructor.execute(args, &mut instance)?;
        }

        // Store instance
        let instance_id = uuid::Uuid::new_v4().to_string();
        let mut instances = self.instances.write().await;
        instances.insert(instance_id, instance.clone());

        Ok(instance)
    }

    /// Get an instance by ID
    pub async fn get_instance(&self, instance_id: &str) -> Option<ClassInstance> {
        let instances = self.instances.read().await;
        instances.get(instance_id).cloned()
    }

    /// Call a static method on a class
    pub async fn call_static_method(&self, class_name: &str, method_name: &str, args: &[Value]) -> Result<Value> {
        let class_def = self.get_class(class_name).await
            .ok_or_else(|| Error::parsing(format!("Class '{}' not found", class_name)))?;

        if let Some(method) = class_def.get_static_method(method_name) {
            let mut temp_instance = ClassInstance::new(class_def);
            method.execute(args, &mut temp_instance)
        } else {
            Err(Error::parsing(format!("Static method '{}' not found on class '{}'", method_name, class_name)))
        }
    }

    /// Get all registered class names
    pub async fn get_class_names(&self) -> Vec<String> {
        let classes = self.classes.read().await;
        classes.keys().cloned().collect()
    }

    /// Check if a class exists
    pub async fn has_class(&self, name: &str) -> bool {
        let classes = self.classes.read().await;
        classes.contains_key(name)
    }

    /// Remove a class
    pub async fn remove_class(&self, name: &str) -> Result<()> {
        let mut classes = self.classes.write().await;
        classes.remove(name)
            .ok_or_else(|| Error::parsing(format!("Class '{}' not found", name)))?;
        Ok(())
    }

    /// Clear all classes
    pub async fn clear_classes(&self) {
        let mut classes = self.classes.write().await;
        classes.clear();
    }

    /// Get class count
    pub async fn class_count(&self) -> usize {
        let classes = self.classes.read().await;
        classes.len()
    }
}

/// Class parser for parsing class syntax
pub struct ClassParser {
    /// Class system
    class_system: ClassSystem,
}

impl ClassParser {
    /// Create a new class parser
    pub fn new() -> Self {
        Self {
            class_system: ClassSystem::new(),
        }
    }

    /// Parse a class declaration
    pub async fn parse_class_declaration(&self, class_decl: &ClassDeclaration) -> Result<ClassDefinition> {
        let mut class_def = ClassDefinition::new(class_decl.id.name.clone());

        // Parse superclass if present
        if let Some(superclass) = &class_decl.super_class {
            let superclass_def = self.parse_class_expression(superclass).await?;
            class_def.set_superclass(superclass_def);
        }

        // Parse class elements
        for element in &class_decl.body.body {
            self.parse_class_element(&mut class_def, element).await?;
        }

        Ok(class_def)
    }

    /// Parse a class expression
    pub async fn parse_class_expression(&self, class_expr: &ClassExpression) -> Result<ClassDefinition> {
        let name = class_expr.id.as_ref()
            .map(|id| id.name.clone())
            .unwrap_or_else(|| "anonymous".to_string());

        let mut class_def = ClassDefinition::new(name);

        // Parse superclass if present
        if let Some(superclass) = &class_expr.super_class {
            let superclass_def = self.parse_class_expression(superclass).await?;
            class_def.set_superclass(superclass_def);
        }

        // Parse class elements
        for element in &class_expr.body.body {
            self.parse_class_element(&mut class_def, element).await?;
        }

        Ok(class_def)
    }

    /// Parse a class element
    async fn parse_class_element(&self, class_def: &mut ClassDefinition, element: &ClassElement) -> Result<()> {
        match element {
            ClassElement::Method(method_def) => {
                let method = self.parse_method_definition(method_def).await?;
                
                if method.is_constructor() {
                    class_def.add_constructor(method);
                } else if method.is_static {
                    class_def.add_static_method(method);
                } else {
                    class_def.add_method(method);
                }
            }
            ClassElement::Property(property_def) => {
                let property = self.parse_property_definition(property_def).await?;
                
                if property_def.is_static {
                    class_def.add_static_property(property);
                } else {
                    class_def.add_property(property);
                }
            }
            ClassElement::PrivateField(field_def) => {
                let field = self.parse_private_field_definition(field_def).await?;
                class_def.add_private_field(field);
            }
            _ => {
                // Handle other class elements (getters, setters, etc.)
            }
        }
        Ok(())
    }

    /// Parse a method definition
    async fn parse_method_definition(&self, method_def: &MethodDefinition) -> Result<MethodDefinition> {
        let kind = match method_def.kind {
            crate::ast::MethodKind::Constructor => MethodKind::Constructor,
            crate::ast::MethodKind::Method => MethodKind::Method,
            crate::ast::MethodKind::Getter => MethodKind::Getter,
            crate::ast::MethodKind::Setter => MethodKind::Setter,
        };

        let mut method = MethodDefinition::new(
            method_def.key.name.clone(),
            kind,
            Vec::new(), // Parse parameters
            method_def.value.body.body.clone(),
        );

        method.set_static(method_def.is_static);
        method.set_private(method_def.is_private);

        Ok(method)
    }

    /// Parse a property definition
    async fn parse_property_definition(&self, property_def: &crate::ast::PropertyDefinition) -> Result<PropertyDefinition> {
        let property = PropertyDefinition {
            name: property_def.key.name.clone(),
            value: None, // Parse value expression
            writable: true,
            enumerable: true,
            configurable: true,
            getter: None,
            setter: None,
        };

        Ok(property)
    }

    /// Parse a private field definition
    async fn parse_private_field_definition(&self, field_def: &crate::ast::PrivateFieldDefinition) -> Result<PrivateFieldDefinition> {
        let field = PrivateFieldDefinition {
            name: field_def.key.name.clone(),
            value: None, // Parse value expression
            writable: true,
        };

        Ok(field)
    }

    /// Get the class system
    pub fn get_class_system(&self) -> &ClassSystem {
        &self.class_system
    }
}
