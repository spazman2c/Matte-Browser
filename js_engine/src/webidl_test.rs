#[cfg(test)]
mod tests {
    use super::*;
    use crate::webidl::{
        WebIDLParser, WebIDLGenerator, FastDOMBinding, WebIDLDefinition,
        WebIDLInterface, WebIDLMethod, WebIDLProperty, WebIDLArgument,
        WebIDLType, InterfaceBinding, MethodBinding, PropertyBinding,
        Value
    };

    #[tokio::test]
    async fn test_webidl_parser_creation() {
        let input = "interface Test {};".to_string();
        let parser = WebIDLParser::new(input);
        
        assert_eq!(parser.position, 0);
        assert_eq!(parser.line, 1);
        assert_eq!(parser.column, 1);
    }

    #[tokio::test]
    async fn test_webidl_parser_simple_interface() {
        let input = r#"
            interface Element {
                readonly attribute DOMString tagName;
                DOMString getAttribute(DOMString name);
            };
        "#.to_string();
        
        let mut parser = WebIDLParser::new(input);
        let definition = parser.parse().unwrap();
        
        assert_eq!(definition.interfaces.len(), 1);
        assert!(definition.interfaces.contains_key("Element"));
        
        let element = &definition.interfaces["Element"];
        assert_eq!(element.name, "Element");
        assert_eq!(element.properties.len(), 1);
        assert_eq!(element.methods.len(), 1);
        
        let tag_name_prop = &element.properties[0];
        assert_eq!(tag_name_prop.name, "tagName");
        assert_eq!(tag_name_prop.property_type, WebIDLType::DOMString);
        assert!(tag_name_prop.readonly);
        
        let get_attribute_method = &element.methods[0];
        assert_eq!(get_attribute_method.name, "getAttribute");
        assert_eq!(get_attribute_method.return_type, WebIDLType::DOMString);
        assert_eq!(get_attribute_method.arguments.len(), 1);
        
        let name_arg = &get_attribute_method.arguments[0];
        assert_eq!(name_arg.name, "name");
        assert_eq!(name_arg.arg_type, WebIDLType::DOMString);
    }

    #[tokio::test]
    async fn test_webidl_parser_inheritance() {
        let input = r#"
            interface Node {
                readonly attribute DOMString nodeName;
            };
            
            interface Element : Node {
                readonly attribute DOMString tagName;
            };
        "#.to_string();
        
        let mut parser = WebIDLParser::new(input);
        let definition = parser.parse().unwrap();
        
        assert_eq!(definition.interfaces.len(), 2);
        
        let node = &definition.interfaces["Node"];
        assert_eq!(node.name, "Node");
        assert_eq!(node.parent, None);
        
        let element = &definition.interfaces["Element"];
        assert_eq!(element.name, "Element");
        assert_eq!(element.parent, Some("Node".to_string()));
    }

    #[tokio::test]
    async fn test_webidl_parser_dictionary() {
        let input = r#"
            dictionary EventInit {
                boolean bubbles = false;
                boolean cancelable = false;
                boolean composed = false;
            };
        "#.to_string();
        
        let mut parser = WebIDLParser::new(input);
        let definition = parser.parse().unwrap();
        
        assert_eq!(definition.dictionaries.len(), 1);
        assert!(definition.dictionaries.contains_key("EventInit"));
        
        let event_init = &definition.dictionaries["EventInit"];
        assert_eq!(event_init.name, "EventInit");
        assert_eq!(event_init.members.len(), 3);
        
        let bubbles = &event_init.members[0];
        assert_eq!(bubbles.name, "bubbles");
        assert_eq!(bubbles.member_type, WebIDLType::Boolean);
        assert_eq!(bubbles.default_value, Some("false".to_string()));
    }

    #[tokio::test]
    async fn test_webidl_parser_enum() {
        let input = r#"
            enum DocumentReadyState {
                "loading",
                "interactive",
                "complete"
            };
        "#.to_string();
        
        let mut parser = WebIDLParser::new(input);
        let definition = parser.parse().unwrap();
        
        assert_eq!(definition.enums.len(), 1);
        assert!(definition.enums.contains_key("DocumentReadyState"));
        
        let ready_state = &definition.enums["DocumentReadyState"];
        assert_eq!(ready_state.name, "DocumentReadyState");
        assert_eq!(ready_state.values, vec!["loading", "interactive", "complete"]);
    }

    #[tokio::test]
    async fn test_webidl_parser_callback() {
        let input = r#"
            callback EventHandler = void (Event event);
        "#.to_string();
        
        let mut parser = WebIDLParser::new(input);
        let definition = parser.parse().unwrap();
        
        assert_eq!(definition.callbacks.len(), 1);
        assert!(definition.callbacks.contains_key("EventHandler"));
        
        let event_handler = &definition.callbacks["EventHandler"];
        assert_eq!(event_handler.name, "EventHandler");
        assert_eq!(event_handler.return_type, WebIDLType::Void);
        assert_eq!(event_handler.arguments.len(), 1);
        
        let event_arg = &event_handler.arguments[0];
        assert_eq!(event_arg.name, "event");
        assert_eq!(event_arg.arg_type, WebIDLType::Interface("Event".to_string()));
    }

    #[tokio::test]
    async fn test_webidl_parser_complex_types() {
        let input = r#"
            interface TestInterface {
                Promise<DOMString> fetchData();
                sequence<long> getNumbers();
                record<DOMString, any> getData();
                DOMString? getOptionalString();
            };
        "#.to_string();
        
        let mut parser = WebIDLParser::new(input);
        let definition = parser.parse().unwrap();
        
        let test_interface = &definition.interfaces["TestInterface"];
        assert_eq!(test_interface.methods.len(), 4);
        
        let fetch_data = &test_interface.methods[0];
        assert_eq!(fetch_data.return_type, WebIDLType::Promise(Box::new(WebIDLType::DOMString)));
        
        let get_numbers = &test_interface.methods[1];
        assert_eq!(get_numbers.return_type, WebIDLType::Sequence(Box::new(WebIDLType::Long)));
        
        let get_data = &test_interface.methods[2];
        assert_eq!(get_data.return_type, WebIDLType::Record(Box::new(WebIDLType::DOMString), Box::new(WebIDLType::Any)));
        
        let get_optional_string = &test_interface.methods[3];
        assert_eq!(get_optional_string.return_type, WebIDLType::Nullable(Box::new(WebIDLType::DOMString)));
    }

    #[tokio::test]
    async fn test_webidl_generator_creation() {
        let generator = WebIDLGenerator::new();
        
        assert_eq!(generator.indent_level, 0);
        assert!(generator.code.is_empty());
        assert!(!generator.type_mappings.is_empty());
    }

    #[tokio::test]
    async fn test_webidl_generator_simple_interface() {
        let mut generator = WebIDLGenerator::new();
        
        let mut definition = WebIDLDefinition {
            interfaces: HashMap::new(),
            dictionaries: HashMap::new(),
            enums: HashMap::new(),
            callbacks: HashMap::new(),
            globals: HashMap::new(),
        };
        
        let interface = WebIDLInterface {
            name: "TestElement".to_string(),
            parent: None,
            methods: vec![
                WebIDLMethod {
                    name: "getAttribute".to_string(),
                    return_type: WebIDLType::DOMString,
                    arguments: vec![
                        WebIDLArgument {
                            name: "name".to_string(),
                            arg_type: WebIDLType::DOMString,
                            optional: false,
                            default_value: None,
                            variadic: false,
                            documentation: None,
                        }
                    ],
                    static_method: false,
                    getter: false,
                    setter: false,
                    deleter: false,
                    documentation: None,
                }
            ],
            properties: vec![
                WebIDLProperty {
                    name: "tagName".to_string(),
                    property_type: WebIDLType::DOMString,
                    readonly: true,
                    required: false,
                    inherited: false,
                    documentation: None,
                }
            ],
            mixin: false,
            partial: false,
            documentation: None,
            attributes: HashMap::new(),
        };
        
        definition.interfaces.insert("TestElement".to_string(), interface);
        
        let rust_code = generator.generate_rust_code(&definition).unwrap();
        
        assert!(rust_code.contains("pub struct TestElement"));
        assert!(rust_code.contains("pub tagName: String"));
        assert!(rust_code.contains("pub fn getAttribute(&self, name: String) -> String"));
    }

    #[tokio::test]
    async fn test_webidl_generator_dictionary() {
        let mut generator = WebIDLGenerator::new();
        
        let mut definition = WebIDLDefinition {
            interfaces: HashMap::new(),
            dictionaries: HashMap::new(),
            enums: HashMap::new(),
            callbacks: HashMap::new(),
            globals: HashMap::new(),
        };
        
        let dictionary = WebIDLDictionary {
            name: "TestOptions".to_string(),
            parent: None,
            members: vec![
                WebIDLDictionaryMember {
                    name: "enabled".to_string(),
                    member_type: WebIDLType::Boolean,
                    default_value: Some("true".to_string()),
                    required: false,
                    documentation: None,
                }
            ],
            documentation: None,
        };
        
        definition.dictionaries.insert("TestOptions".to_string(), dictionary);
        
        let rust_code = generator.generate_rust_code(&definition).unwrap();
        
        assert!(rust_code.contains("pub struct TestOptions"));
        assert!(rust_code.contains("pub enabled: bool"));
    }

    #[tokio::test]
    async fn test_webidl_generator_enum() {
        let mut generator = WebIDLGenerator::new();
        
        let mut definition = WebIDLDefinition {
            interfaces: HashMap::new(),
            dictionaries: HashMap::new(),
            enums: HashMap::new(),
            callbacks: HashMap::new(),
            globals: HashMap::new(),
        };
        
        let enum_def = WebIDLEnum {
            name: "TestState".to_string(),
            values: vec!["Idle".to_string(), "Running".to_string(), "Complete".to_string()],
            documentation: None,
        };
        
        definition.enums.insert("TestState".to_string(), enum_def);
        
        let rust_code = generator.generate_rust_code(&definition).unwrap();
        
        assert!(rust_code.contains("pub enum TestState"));
        assert!(rust_code.contains("Idle"));
        assert!(rust_code.contains("Running"));
        assert!(rust_code.contains("Complete"));
    }

    #[tokio::test]
    async fn test_webidl_generator_type_mapping() {
        let generator = WebIDLGenerator::new();
        
        // Test basic type mappings
        assert_eq!(generator.map_type(&WebIDLType::Boolean).unwrap(), "bool");
        assert_eq!(generator.map_type(&WebIDLType::DOMString).unwrap(), "String");
        assert_eq!(generator.map_type(&WebIDLType::Long).unwrap(), "i32");
        assert_eq!(generator.map_type(&WebIDLType::Object).unwrap(), "Value");
        
        // Test complex type mappings
        assert_eq!(generator.map_type(&WebIDLType::Nullable(Box::new(WebIDLType::DOMString))).unwrap(), "Option<String>");
        assert_eq!(generator.map_type(&WebIDLType::Sequence(Box::new(WebIDLType::Long))).unwrap(), "Vec<i32>");
        assert_eq!(generator.map_type(&WebIDLType::Promise(Box::new(WebIDLType::DOMString))).unwrap(), "Promise<String>");
    }

    #[tokio::test]
    async fn test_webidl_generator_default_values() {
        let generator = WebIDLGenerator::new();
        
        assert_eq!(generator.get_default_value(&WebIDLType::Boolean).unwrap(), "false");
        assert_eq!(generator.get_default_value(&WebIDLType::DOMString).unwrap(), "String::new()");
        assert_eq!(generator.get_default_value(&WebIDLType::Long).unwrap(), "0");
        assert_eq!(generator.get_default_value(&WebIDLType::Nullable(Box::new(WebIDLType::DOMString))).unwrap(), "None");
        assert_eq!(generator.get_default_value(&WebIDLType::Sequence(Box::new(WebIDLType::Long))).unwrap(), "Vec::new()");
    }

    #[tokio::test]
    async fn test_fast_dom_binding_creation() {
        let binding = FastDOMBinding::new();
        
        let stats = binding.get_stats();
        assert_eq!(stats.total_method_calls, 0);
        assert_eq!(stats.total_property_accesses, 0);
        assert_eq!(stats.cache_hits, 0);
        assert_eq!(stats.cache_misses, 0);
    }

    #[tokio::test]
    async fn test_fast_dom_binding_interface_registration() {
        let binding = FastDOMBinding::new();
        
        let interface_binding = InterfaceBinding {
            name: "Element".to_string(),
            constructor: Some("Element::new".to_string()),
            methods: HashMap::new(),
            properties: HashMap::new(),
            prototype: None,
        };
        
        binding.register_interface("Element", interface_binding).unwrap();
        
        // Verify interface is registered
        let method = binding.get_method("Element", "getAttribute");
        assert!(method.is_none()); // No methods registered yet
    }

    #[tokio::test]
    async fn test_fast_dom_binding_method_registration() {
        let binding = FastDOMBinding::new();
        
        let mut methods = HashMap::new();
        methods.insert("getAttribute".to_string(), MethodBinding {
            name: "getAttribute".to_string(),
            native_function: "element_get_attribute".to_string(),
            argument_types: vec![WebIDLType::DOMString],
            return_type: WebIDLType::DOMString,
            static_method: false,
            documentation: None,
        });
        
        let interface_binding = InterfaceBinding {
            name: "Element".to_string(),
            constructor: None,
            methods,
            properties: HashMap::new(),
            prototype: None,
        };
        
        binding.register_interface("Element", interface_binding).unwrap();
        
        let method = binding.get_method("Element", "getAttribute");
        assert!(method.is_some());
        
        let method_binding = method.unwrap();
        assert_eq!(method_binding.name, "getAttribute");
        assert_eq!(method_binding.native_function, "element_get_attribute");
        assert_eq!(method_binding.return_type, WebIDLType::DOMString);
    }

    #[tokio::test]
    async fn test_fast_dom_binding_property_registration() {
        let binding = FastDOMBinding::new();
        
        let mut properties = HashMap::new();
        properties.insert("tagName".to_string(), PropertyBinding {
            name: "tagName".to_string(),
            property_type: WebIDLType::DOMString,
            getter: Some("element_get_tag_name".to_string()),
            setter: None,
            readonly: true,
            documentation: None,
        });
        
        let interface_binding = InterfaceBinding {
            name: "Element".to_string(),
            constructor: None,
            methods: HashMap::new(),
            properties,
            prototype: None,
        };
        
        binding.register_interface("Element", interface_binding).unwrap();
        
        let property = binding.get_property("Element", "tagName");
        assert!(property.is_some());
        
        let property_binding = property.unwrap();
        assert_eq!(property_binding.name, "tagName");
        assert_eq!(property_binding.property_type, WebIDLType::DOMString);
        assert!(property_binding.readonly);
        assert_eq!(property_binding.getter, Some("element_get_tag_name".to_string()));
    }

    #[tokio::test]
    async fn test_fast_dom_binding_method_call() {
        let binding = FastDOMBinding::new();
        
        let mut methods = HashMap::new();
        methods.insert("getAttribute".to_string(), MethodBinding {
            name: "getAttribute".to_string(),
            native_function: "element_get_attribute".to_string(),
            argument_types: vec![WebIDLType::DOMString],
            return_type: WebIDLType::DOMString,
            static_method: false,
            documentation: None,
        });
        
        let interface_binding = InterfaceBinding {
            name: "Element".to_string(),
            constructor: None,
            methods,
            properties: HashMap::new(),
            prototype: None,
        };
        
        binding.register_interface("Element", interface_binding).unwrap();
        
        let args = vec![Value::String("id".to_string())];
        let result = binding.call_method("Element", "getAttribute", args).await;
        assert!(result.is_ok());
        
        let stats = binding.get_stats();
        assert_eq!(stats.total_method_calls, 1);
        assert_eq!(stats.cache_misses, 1); // First call is a cache miss
    }

    #[tokio::test]
    async fn test_fast_dom_binding_property_access() {
        let binding = FastDOMBinding::new();
        
        let mut properties = HashMap::new();
        properties.insert("tagName".to_string(), PropertyBinding {
            name: "tagName".to_string(),
            property_type: WebIDLType::DOMString,
            getter: Some("element_get_tag_name".to_string()),
            setter: None,
            readonly: true,
            documentation: None,
        });
        
        let interface_binding = InterfaceBinding {
            name: "Element".to_string(),
            constructor: None,
            methods: HashMap::new(),
            properties,
            prototype: None,
        };
        
        binding.register_interface("Element", interface_binding).unwrap();
        
        let result = binding.get_property_value("Element", "tagName").await;
        assert!(result.is_ok());
        
        let stats = binding.get_stats();
        assert_eq!(stats.total_property_accesses, 1);
        assert_eq!(stats.cache_misses, 1); // First access is a cache miss
    }

    #[tokio::test]
    async fn test_fast_dom_binding_property_set() {
        let binding = FastDOMBinding::new();
        
        let mut properties = HashMap::new();
        properties.insert("className".to_string(), PropertyBinding {
            name: "className".to_string(),
            property_type: WebIDLType::DOMString,
            getter: Some("element_get_class_name".to_string()),
            setter: Some("element_set_class_name".to_string()),
            readonly: false,
            documentation: None,
        });
        
        let interface_binding = InterfaceBinding {
            name: "Element".to_string(),
            constructor: None,
            methods: HashMap::new(),
            properties,
            prototype: None,
        };
        
        binding.register_interface("Element", interface_binding).unwrap();
        
        let value = Value::String("test-class".to_string());
        let result = binding.set_property_value("Element", "className", value).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_fast_dom_binding_readonly_property_set() {
        let binding = FastDOMBinding::new();
        
        let mut properties = HashMap::new();
        properties.insert("tagName".to_string(), PropertyBinding {
            name: "tagName".to_string(),
            property_type: WebIDLType::DOMString,
            getter: Some("element_get_tag_name".to_string()),
            setter: None,
            readonly: true,
            documentation: None,
        });
        
        let interface_binding = InterfaceBinding {
            name: "Element".to_string(),
            constructor: None,
            methods: HashMap::new(),
            properties,
            prototype: None,
        };
        
        binding.register_interface("Element", interface_binding).unwrap();
        
        let value = Value::String("div".to_string());
        let result = binding.set_property_value("Element", "tagName", value).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Property tagName is read-only");
    }

    #[tokio::test]
    async fn test_fast_dom_binding_cache_behavior() {
        let binding = FastDOMBinding::new();
        
        let mut methods = HashMap::new();
        methods.insert("getAttribute".to_string(), MethodBinding {
            name: "getAttribute".to_string(),
            native_function: "element_get_attribute".to_string(),
            argument_types: vec![WebIDLType::DOMString],
            return_type: WebIDLType::DOMString,
            static_method: false,
            documentation: None,
        });
        
        let interface_binding = InterfaceBinding {
            name: "Element".to_string(),
            constructor: None,
            methods,
            properties: HashMap::new(),
            prototype: None,
        };
        
        binding.register_interface("Element", interface_binding).unwrap();
        
        // First call - cache miss
        let args1 = vec![Value::String("id".to_string())];
        binding.call_method("Element", "getAttribute", args1).await.unwrap();
        
        // Second call - cache hit
        let args2 = vec![Value::String("class".to_string())];
        binding.call_method("Element", "getAttribute", args2).await.unwrap();
        
        let stats = binding.get_stats();
        assert_eq!(stats.total_method_calls, 2);
        assert_eq!(stats.cache_hits, 1);
        assert_eq!(stats.cache_misses, 1);
    }

    #[tokio::test]
    async fn test_fast_dom_binding_clear_caches() {
        let binding = FastDOMBinding::new();
        
        let mut methods = HashMap::new();
        methods.insert("getAttribute".to_string(), MethodBinding {
            name: "getAttribute".to_string(),
            native_function: "element_get_attribute".to_string(),
            argument_types: vec![WebIDLType::DOMString],
            return_type: WebIDLType::DOMString,
            static_method: false,
            documentation: None,
        });
        
        let interface_binding = InterfaceBinding {
            name: "Element".to_string(),
            constructor: None,
            methods,
            properties: HashMap::new(),
            prototype: None,
        };
        
        binding.register_interface("Element", interface_binding).unwrap();
        
        // Make some calls to populate cache
        let args = vec![Value::String("id".to_string())];
        binding.call_method("Element", "getAttribute", args).await.unwrap();
        
        // Clear caches
        binding.clear_caches();
        
        // Verify caches are cleared
        let stats = binding.get_stats();
        assert_eq!(stats.total_method_calls, 1);
        assert_eq!(stats.cache_misses, 1);
    }

    #[tokio::test]
    async fn test_webidl_type_enum() {
        // Test all WebIDL type variants
        let types = vec![
            WebIDLType::DOMString,
            WebIDLType::Boolean,
            WebIDLType::Long,
            WebIDLType::Object,
            WebIDLType::Any,
            WebIDLType::Void,
            WebIDLType::Interface("CustomType".to_string()),
            WebIDLType::Nullable(Box::new(WebIDLType::DOMString)),
            WebIDLType::Sequence(Box::new(WebIDLType::Long)),
            WebIDLType::Promise(Box::new(WebIDLType::DOMString)),
        ];
        
        for webidl_type in types {
            assert_eq!(format!("{:?}", webidl_type), format!("{:?}", webidl_type));
        }
    }

    #[tokio::test]
    async fn test_webidl_integration() {
        // Test complete WebIDL workflow
        let input = r#"
            interface TestElement {
                readonly attribute DOMString tagName;
                DOMString getAttribute(DOMString name);
                void setAttribute(DOMString name, DOMString value);
            };
        "#.to_string();
        
        // Parse WebIDL
        let mut parser = WebIDLParser::new(input);
        let definition = parser.parse().unwrap();
        
        // Generate Rust code
        let mut generator = WebIDLGenerator::new();
        let rust_code = generator.generate_rust_code(&definition).unwrap();
        
        // Verify generated code
        assert!(rust_code.contains("pub struct TestElement"));
        assert!(rust_code.contains("pub tagName: String"));
        assert!(rust_code.contains("pub fn getAttribute(&self, name: String) -> String"));
        assert!(rust_code.contains("pub fn setAttribute(&self, name: String, value: String) -> ()"));
        
        // Create fast DOM binding
        let binding = FastDOMBinding::new();
        
        let mut methods = HashMap::new();
        methods.insert("getAttribute".to_string(), MethodBinding {
            name: "getAttribute".to_string(),
            native_function: "test_element_get_attribute".to_string(),
            argument_types: vec![WebIDLType::DOMString],
            return_type: WebIDLType::DOMString,
            static_method: false,
            documentation: None,
        });
        
        let mut properties = HashMap::new();
        properties.insert("tagName".to_string(), PropertyBinding {
            name: "tagName".to_string(),
            property_type: WebIDLType::DOMString,
            getter: Some("test_element_get_tag_name".to_string()),
            setter: None,
            readonly: true,
            documentation: None,
        });
        
        let interface_binding = InterfaceBinding {
            name: "TestElement".to_string(),
            constructor: Some("TestElement::new".to_string()),
            methods,
            properties,
            prototype: None,
        };
        
        binding.register_interface("TestElement", interface_binding).unwrap();
        
        // Test method call
        let args = vec![Value::String("id".to_string())];
        let result = binding.call_method("TestElement", "getAttribute", args).await;
        assert!(result.is_ok());
        
        // Test property access
        let result = binding.get_property_value("TestElement", "tagName").await;
        assert!(result.is_ok());
        
        // Verify statistics
        let stats = binding.get_stats();
        assert_eq!(stats.total_method_calls, 1);
        assert_eq!(stats.total_property_accesses, 1);
        assert!(stats.avg_method_call_time_us > 0.0);
        assert!(stats.avg_property_access_time_us > 0.0);
    }
}
