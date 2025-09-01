#[cfg(test)]
mod tests {
    use super::*;
    use crate::class_system::{ClassSystem, ClassParser, ClassDefinition, ClassInstance, MethodDefinition, MethodKind, PropertyDefinition, PrivateFieldDefinition, Value};

    #[tokio::test]
    async fn test_class_definition_creation() {
        let class_def = ClassDefinition::new("TestClass".to_string());
        
        assert_eq!(class_def.name, "TestClass");
        assert!(class_def.superclass.is_none());
        assert!(class_def.constructor.is_none());
        assert!(class_def.methods.is_empty());
        assert!(class_def.static_methods.is_empty());
        assert!(class_def.properties.is_empty());
        assert!(class_def.static_properties.is_empty());
        assert!(class_def.private_fields.is_empty());
    }

    #[tokio::test]
    async fn test_method_definition_creation() {
        let method = MethodDefinition::new(
            "testMethod".to_string(),
            MethodKind::Method,
            Vec::new(),
            Vec::new(),
        );
        
        assert_eq!(method.name, "testMethod");
        assert_eq!(method.kind, MethodKind::Method);
        assert!(!method.is_static);
        assert!(!method.is_private);
        assert!(!method.is_constructor());
    }

    #[tokio::test]
    async fn test_constructor_method() {
        let constructor = MethodDefinition::new(
            "constructor".to_string(),
            MethodKind::Constructor,
            Vec::new(),
            Vec::new(),
        );
        
        assert_eq!(constructor.name, "constructor");
        assert_eq!(constructor.kind, MethodKind::Constructor);
        assert!(constructor.is_constructor());
    }

    #[tokio::test]
    async fn test_static_method() {
        let mut method = MethodDefinition::new(
            "staticMethod".to_string(),
            MethodKind::Method,
            Vec::new(),
            Vec::new(),
        );
        
        method.set_static(true);
        assert!(method.is_static);
    }

    #[tokio::test]
    async fn test_private_method() {
        let mut method = MethodDefinition::new(
            "privateMethod".to_string(),
            MethodKind::Method,
            Vec::new(),
            Vec::new(),
        );
        
        method.set_private(true);
        assert!(method.is_private);
    }

    #[tokio::test]
    async fn test_property_definition_creation() {
        let property = PropertyDefinition {
            name: "testProperty".to_string(),
            value: Some(Value::String("test".to_string())),
            writable: true,
            enumerable: true,
            configurable: true,
            getter: None,
            setter: None,
        };
        
        assert_eq!(property.name, "testProperty");
        assert!(property.writable);
        assert!(property.enumerable);
        assert!(property.configurable);
        assert!(matches!(property.value, Some(Value::String(s)) if s == "test"));
    }

    #[tokio::test]
    async fn test_private_field_definition_creation() {
        let field = PrivateFieldDefinition {
            name: "privateField".to_string(),
            value: Some(Value::Number(42.0)),
            writable: true,
        };
        
        assert_eq!(field.name, "privateField");
        assert!(field.writable);
        assert!(matches!(field.value, Some(Value::Number(n)) if n == 42.0));
    }

    #[tokio::test]
    async fn test_class_instance_creation() {
        let class_def = ClassDefinition::new("TestClass".to_string());
        let instance = ClassInstance::new(class_def);
        
        assert!(instance.properties.is_empty());
        assert!(instance.private_fields.is_empty());
        assert_eq!(instance.class.name, "TestClass");
        assert!(instance.prototype_chain.is_empty());
    }

    #[tokio::test]
    async fn test_class_instance_properties() {
        let class_def = ClassDefinition::new("TestClass".to_string());
        let mut instance = ClassInstance::new(class_def);
        
        // Set and get properties
        instance.set_property("name".to_string(), Value::String("test".to_string()));
        instance.set_property("value".to_string(), Value::Number(123.0));
        
        assert!(instance.has_property("name"));
        assert!(instance.has_property("value"));
        assert!(!instance.has_property("nonexistent"));
        
        assert!(matches!(instance.get_property("name"), Some(Value::String(s)) if s == "test"));
        assert!(matches!(instance.get_property("value"), Some(Value::Number(n)) if n == 123.0));
        assert!(instance.get_property("nonexistent").is_none());
    }

    #[tokio::test]
    async fn test_class_instance_private_fields() {
        let class_def = ClassDefinition::new("TestClass".to_string());
        let mut instance = ClassInstance::new(class_def);
        
        // Set and get private fields
        instance.set_private_field("privateField".to_string(), Value::Boolean(true));
        
        assert!(instance.has_private_field("privateField"));
        assert!(!instance.has_private_field("nonexistent"));
        
        assert!(matches!(instance.get_private_field("privateField"), Some(Value::Boolean(true))));
        assert!(instance.get_private_field("nonexistent").is_none());
    }

    #[tokio::test]
    async fn test_class_system_creation() {
        let class_system = ClassSystem::new();
        
        assert_eq!(class_system.class_count().await, 0);
        assert!(class_system.get_class_names().await.is_empty());
    }

    #[tokio::test]
    async fn test_class_registration() {
        let class_system = ClassSystem::new();
        let class_def = ClassDefinition::new("TestClass".to_string());
        
        // Register class
        class_system.register_class("TestClass".to_string(), class_def).await.unwrap();
        
        assert_eq!(class_system.class_count().await, 1);
        assert!(class_system.has_class("TestClass").await);
        assert!(!class_system.has_class("NonexistentClass").await);
        
        let class_names = class_system.get_class_names().await;
        assert_eq!(class_names.len(), 1);
        assert!(class_names.contains(&"TestClass".to_string()));
    }

    #[tokio::test]
    async fn test_class_retrieval() {
        let class_system = ClassSystem::new();
        let class_def = ClassDefinition::new("TestClass".to_string());
        
        // Register class
        class_system.register_class("TestClass".to_string(), class_def.clone()).await.unwrap();
        
        // Retrieve class
        let retrieved_class = class_system.get_class("TestClass").await;
        assert!(retrieved_class.is_some());
        
        let retrieved_class = retrieved_class.unwrap();
        assert_eq!(retrieved_class.name, "TestClass");
    }

    #[tokio::test]
    async fn test_class_instance_creation_with_system() {
        let class_system = ClassSystem::new();
        let class_def = ClassDefinition::new("TestClass".to_string());
        
        // Register class
        class_system.register_class("TestClass".to_string(), class_def).await.unwrap();
        
        // Create instance
        let instance = class_system.create_instance("TestClass", &[]).await.unwrap();
        assert_eq!(instance.class.name, "TestClass");
    }

    #[tokio::test]
    async fn test_class_removal() {
        let class_system = ClassSystem::new();
        let class_def = ClassDefinition::new("TestClass".to_string());
        
        // Register class
        class_system.register_class("TestClass".to_string(), class_def).await.unwrap();
        assert_eq!(class_system.class_count().await, 1);
        
        // Remove class
        class_system.remove_class("TestClass").await.unwrap();
        assert_eq!(class_system.class_count().await, 0);
        assert!(!class_system.has_class("TestClass").await);
    }

    #[tokio::test]
    async fn test_class_clear() {
        let class_system = ClassSystem::new();
        let class_def1 = ClassDefinition::new("TestClass1".to_string());
        let class_def2 = ClassDefinition::new("TestClass2".to_string());
        
        // Register classes
        class_system.register_class("TestClass1".to_string(), class_def1).await.unwrap();
        class_system.register_class("TestClass2".to_string(), class_def2).await.unwrap();
        assert_eq!(class_system.class_count().await, 2);
        
        // Clear all classes
        class_system.clear_classes().await;
        assert_eq!(class_system.class_count().await, 0);
        assert!(class_system.get_class_names().await.is_empty());
    }

    #[tokio::test]
    async fn test_class_parser_creation() {
        let parser = ClassParser::new();
        
        // Test that parser is created successfully
        assert!(true); // If we get here, creation succeeded
    }

    #[tokio::test]
    async fn test_method_kind_operations() {
        let constructor = MethodKind::Constructor;
        let method = MethodKind::Method;
        let getter = MethodKind::Getter;
        let setter = MethodKind::Setter;
        
        // Test method kind comparisons
        assert_eq!(constructor, MethodKind::Constructor);
        assert_eq!(method, MethodKind::Method);
        assert_eq!(getter, MethodKind::Getter);
        assert_eq!(setter, MethodKind::Setter);
        
        // Test that different kinds are not equal
        assert_ne!(constructor, method);
        assert_ne!(getter, setter);
    }

    #[tokio::test]
    async fn test_class_prototype_operations() {
        use crate::class_system::ClassPrototype;
        
        let mut prototype = ClassPrototype::new();
        
        // Test property operations
        prototype.set_property("testProp".to_string(), Value::String("test".to_string()));
        assert!(prototype.has_property("testProp"));
        assert!(!prototype.has_property("nonexistent"));
        
        assert!(matches!(prototype.get_property("testProp"), Some(Value::String(s)) if s == "test"));
        assert!(prototype.get_property("nonexistent").is_none());
        
        // Test property names
        let property_names = prototype.get_property_names();
        assert_eq!(property_names.len(), 1);
        assert!(property_names.contains(&"testProp".to_string()));
    }

    #[tokio::test]
    async fn test_class_inheritance_structure() {
        let mut parent_class = ClassDefinition::new("ParentClass".to_string());
        let child_class = ClassDefinition::new("ChildClass".to_string());
        
        // Set up inheritance
        parent_class.add_method(MethodDefinition::new(
            "parentMethod".to_string(),
            MethodKind::Method,
            Vec::new(),
            Vec::new(),
        ));
        
        let mut child_with_parent = ClassDefinition::new("ChildClass".to_string());
        child_with_parent.set_superclass(parent_class);
        
        assert!(child_with_parent.superclass.is_some());
        assert_eq!(child_with_parent.superclass.as_ref().unwrap().name, "ParentClass");
    }

    #[tokio::test]
    async fn test_method_implementation() {
        let mut method = MethodDefinition::new(
            "testMethod".to_string(),
            MethodKind::Method,
            Vec::new(),
            Vec::new(),
        );
        
        // Add implementation
        method.add_implementation(|args, instance| {
            assert_eq!(args.len(), 0);
            assert_eq!(instance.class.name, "TestClass");
            Ok(Value::String("test result".to_string()))
        });
        
        // Test execution
        let class_def = ClassDefinition::new("TestClass".to_string());
        let mut instance = ClassInstance::new(class_def);
        
        let result = method.execute(&[], &mut instance).unwrap();
        assert!(matches!(result, Value::String(s) if s == "test result"));
    }

    #[tokio::test]
    async fn test_class_system_integration() {
        let class_system = ClassSystem::new();
        
        // Create and register a class with methods and properties
        let mut class_def = ClassDefinition::new("TestClass".to_string());
        
        // Add a method
        let mut method = MethodDefinition::new(
            "testMethod".to_string(),
            MethodKind::Method,
            Vec::new(),
            Vec::new(),
        );
        method.add_implementation(|_, _| Ok(Value::Number(42.0)));
        class_def.add_method(method);
        
        // Add a property
        let property = PropertyDefinition {
            name: "testProperty".to_string(),
            value: Some(Value::String("test".to_string())),
            writable: true,
            enumerable: true,
            configurable: true,
            getter: None,
            setter: None,
        };
        class_def.add_property(property);
        
        // Register class
        class_system.register_class("TestClass".to_string(), class_def).await.unwrap();
        
        // Create instance and test functionality
        let instance = class_system.create_instance("TestClass", &[]).await.unwrap();
        assert_eq!(instance.class.name, "TestClass");
        assert!(instance.class.has_method("testMethod"));
        assert!(instance.class.has_property("testProperty"));
    }
}
