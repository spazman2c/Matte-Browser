#[cfg(test)]
mod tests {
    use super::*;
    use crate::es_modules::{ESModuleSystem, ModuleValue, ModuleNamespace, ImportBinding, ExportBinding};
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_basic_module_loading() {
        let module_system = ESModuleSystem::new("file:///test/".to_string());
        
        // This would test basic module loading functionality
        // For now, just verify the system can be created
        assert_eq!(module_system.get_loader().base_url, "file:///test/");
    }

    #[tokio::test]
    async fn test_module_namespace_creation() {
        let mut namespace = ModuleNamespace {
            properties: HashMap::new(),
            sealed: false,
        };

        // Add some test properties
        namespace.properties.insert("test".to_string(), ModuleValue::String("value".to_string()));
        namespace.properties.insert("number".to_string(), ModuleValue::Number(42.0));
        namespace.properties.insert("boolean".to_string(), ModuleValue::Boolean(true));

        assert_eq!(namespace.properties.len(), 3);
        assert!(matches!(namespace.properties.get("test"), Some(ModuleValue::String(s)) if s == "value"));
        assert!(matches!(namespace.properties.get("number"), Some(ModuleValue::Number(n)) if *n == 42.0));
        assert!(matches!(namespace.properties.get("boolean"), Some(ModuleValue::Boolean(b)) if *b));
    }

    #[tokio::test]
    async fn test_export_binding_creation() {
        use crate::es_modules::ExportBinding;

        let binding = ExportBinding {
            name: "myFunction".to_string(),
            local_name: "myFunction".to_string(),
            is_default: false,
            is_reexport: false,
            source_module: None,
        };

        assert_eq!(binding.name, "myFunction");
        assert_eq!(binding.local_name, "myFunction");
        assert!(!binding.is_default);
        assert!(!binding.is_reexport);
        assert!(binding.source_module.is_none());
    }

    #[tokio::test]
    async fn test_import_binding_creation() {
        use crate::es_modules::ImportBinding;

        let binding = ImportBinding {
            name: "default".to_string(),
            local_name: "MyClass".to_string(),
            is_default: true,
            is_namespace: false,
            source_module: "my-module".to_string(),
        };

        assert_eq!(binding.name, "default");
        assert_eq!(binding.local_name, "MyClass");
        assert!(binding.is_default);
        assert!(!binding.is_namespace);
        assert_eq!(binding.source_module, "my-module");
    }

    #[tokio::test]
    async fn test_module_value_operations() {
        let string_value = ModuleValue::String("hello".to_string());
        let number_value = ModuleValue::Number(123.45);
        let boolean_value = ModuleValue::Boolean(true);
        let object_value = ModuleValue::Object(HashMap::new());

        // Test pattern matching
        match string_value {
            ModuleValue::String(s) => assert_eq!(s, "hello"),
            _ => panic!("Expected string value"),
        }

        match number_value {
            ModuleValue::Number(n) => assert_eq!(n, 123.45),
            _ => panic!("Expected number value"),
        }

        match boolean_value {
            ModuleValue::Boolean(b) => assert!(b),
            _ => panic!("Expected boolean value"),
        }

        match object_value {
            ModuleValue::Object(obj) => assert!(obj.is_empty()),
            _ => panic!("Expected object value"),
        }
    }
}
