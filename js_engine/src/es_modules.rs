use crate::error::{Error, Result};
use crate::ast::{Program, Statement, ImportDeclaration, ExportDeclaration, Identifier, Literal};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Module record representing a loaded ES module
#[derive(Debug, Clone)]
pub struct ModuleRecord {
    /// Module specifier (URL or file path)
    pub specifier: String,
    /// Parsed AST of the module
    pub ast: Program,
    /// Module's export bindings
    pub export_bindings: HashMap<String, ExportBinding>,
    /// Module's import bindings
    pub import_bindings: HashMap<String, ImportBinding>,
    /// Whether the module has been evaluated
    pub evaluated: bool,
    /// Module's namespace object
    pub namespace: Option<ModuleNamespace>,
}

/// Export binding information
#[derive(Debug, Clone)]
pub struct ExportBinding {
    /// Export name
    pub name: String,
    /// Local binding name
    pub local_name: String,
    /// Whether this is a default export
    pub is_default: bool,
    /// Whether this is a re-export
    pub is_reexport: bool,
    /// Source module for re-exports
    pub source_module: Option<String>,
}

/// Import binding information
#[derive(Debug, Clone)]
pub struct ImportBinding {
    /// Import name
    pub name: String,
    /// Local binding name
    pub local_name: String,
    /// Whether this is a default import
    pub is_default: bool,
    /// Whether this is a namespace import
    pub is_namespace: bool,
    /// Source module
    pub source_module: String,
}

/// Module namespace object
#[derive(Debug, Clone)]
pub struct ModuleNamespace {
    /// Namespace properties
    pub properties: HashMap<String, ModuleValue>,
    /// Whether the namespace is sealed
    pub sealed: bool,
}

/// Module value (placeholder for actual JavaScript values)
#[derive(Debug, Clone)]
pub enum ModuleValue {
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
    /// Function value
    Function(String),
    /// Class value
    Class(String),
    /// Object value
    Object(HashMap<String, ModuleValue>),
}

/// ES Module loader and resolver
pub struct ModuleLoader {
    /// Loaded modules cache
    modules: Arc<RwLock<HashMap<String, ModuleRecord>>>,
    /// Module resolution cache
    resolution_cache: Arc<RwLock<HashMap<String, String>>>,
    /// Base URL for resolving relative imports
    base_url: String,
}

impl ModuleLoader {
    /// Create a new module loader
    pub fn new(base_url: String) -> Self {
        Self {
            modules: Arc::new(RwLock::new(HashMap::new())),
            resolution_cache: Arc::new(RwLock::new(HashMap::new())),
            base_url,
        }
    }

    /// Load a module by specifier
    pub async fn load_module(&self, specifier: &str) -> Result<ModuleRecord> {
        // Check if module is already loaded
        {
            let modules = self.modules.read().await;
            if let Some(module) = modules.get(specifier) {
                return Ok(module.clone());
            }
        }

        // Resolve module specifier
        let resolved_specifier = self.resolve_module_specifier(specifier).await?;

        // Check resolution cache
        {
            let cache = self.resolution_cache.read().await;
            if let Some(cached) = cache.get(&resolved_specifier) {
                let modules = self.modules.read().await;
                if let Some(module) = modules.get(cached) {
                    return Ok(module.clone());
                }
            }
        }

        // Load module source
        let source = self.fetch_module_source(&resolved_specifier).await?;

        // Parse module
        let mut parser = crate::parser::JsParser::new(&source);
        let ast = parser.parse()?;

        // Create module record
        let mut module_record = ModuleRecord {
            specifier: resolved_specifier.clone(),
            ast,
            export_bindings: HashMap::new(),
            import_bindings: HashMap::new(),
            evaluated: false,
            namespace: None,
        };

        // Analyze module for imports and exports
        self.analyze_module(&mut module_record).await?;

        // Cache the module
        {
            let mut modules = self.modules.write().await;
            modules.insert(resolved_specifier.clone(), module_record.clone());
        }

        // Cache resolution
        {
            let mut cache = self.resolution_cache.write().await;
            cache.insert(specifier.to_string(), resolved_specifier);
        }

        Ok(module_record)
    }

    /// Resolve a module specifier to a canonical URL
    async fn resolve_module_specifier(&self, specifier: &str) -> Result<String> {
        // Handle different types of specifiers
        if specifier.starts_with("http://") || specifier.starts_with("https://") {
            // Absolute URL
            Ok(specifier.to_string())
        } else if specifier.starts_with('/') {
            // Absolute path from base URL
            let base_url = url::Url::parse(&self.base_url)
                .map_err(|e| Error::parsing(format!("Invalid base URL: {}", e)))?;
            let resolved = base_url.join(specifier)
                .map_err(|e| Error::parsing(format!("Failed to resolve specifier: {}", e)))?;
            Ok(resolved.to_string())
        } else if specifier.starts_with("./") || specifier.starts_with("../") {
            // Relative path
            let base_url = url::Url::parse(&self.base_url)
                .map_err(|e| Error::parsing(format!("Invalid base URL: {}", e)))?;
            let resolved = base_url.join(specifier)
                .map_err(|e| Error::parsing(format!("Failed to resolve specifier: {}", e)))?;
            Ok(resolved.to_string())
        } else {
            // Bare specifier - try to resolve as package
            self.resolve_bare_specifier(specifier).await
        }
    }

    /// Resolve a bare specifier (package name)
    async fn resolve_bare_specifier(&self, specifier: &str) -> Result<String> {
        // For now, assume it's a relative path
        // In a real implementation, this would check node_modules, package.json, etc.
        let resolved = format!("{}/node_modules/{}/index.js", self.base_url, specifier);
        Ok(resolved)
    }

    /// Fetch module source from URL or file system
    async fn fetch_module_source(&self, specifier: &str) -> Result<String> {
        if specifier.starts_with("http://") || specifier.starts_with("https://") {
            // Fetch from network
            let response = reqwest::get(specifier).await
                .map_err(|e| Error::parsing(format!("Failed to fetch module: {}", e)))?;
            
            if !response.status().is_success() {
                return Err(Error::parsing(format!("HTTP error: {}", response.status())));
            }

            let source = response.text().await
                .map_err(|e| Error::parsing(format!("Failed to read response: {}", e)))?;
            
            Ok(source)
        } else {
            // Read from file system
            let path = Path::new(specifier);
            if !path.exists() {
                return Err(Error::parsing(format!("Module not found: {}", specifier)));
            }

            let source = tokio::fs::read_to_string(path).await
                .map_err(|e| Error::parsing(format!("Failed to read file: {}", e)))?;
            
            Ok(source)
        }
    }

    /// Analyze module for imports and exports
    async fn analyze_module(&self, module: &mut ModuleRecord) -> Result<()> {
        for statement in &module.ast.body {
            match statement {
                Statement::Import(import_decl) => {
                    self.analyze_import_declaration(module, import_decl).await?;
                }
                Statement::Export(export_decl) => {
                    self.analyze_export_declaration(module, export_decl).await?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Analyze import declaration
    async fn analyze_import_declaration(
        &self,
        module: &mut ModuleRecord,
        import_decl: &ImportDeclaration,
    ) -> Result<()> {
        let source = match &import_decl.source {
            Literal::String(s) => s.clone(),
            _ => return Err(Error::parsing("Import source must be a string literal".to_string())),
        };

        for specifier in &import_decl.specifiers {
            match specifier {
                crate::ast::ImportSpecifier::Default(default_spec) => {
                    let binding = ImportBinding {
                        name: "default".to_string(),
                        local_name: default_spec.local.name.clone(),
                        is_default: true,
                        is_namespace: false,
                        source_module: source.clone(),
                    };
                    module.import_bindings.insert(default_spec.local.name.clone(), binding);
                }
                crate::ast::ImportSpecifier::Named(named_spec) => {
                    let import_name = named_spec.imported.as_ref()
                        .map(|id| id.name.clone())
                        .unwrap_or_else(|| named_spec.local.name.clone());
                    
                    let binding = ImportBinding {
                        name: import_name,
                        local_name: named_spec.local.name.clone(),
                        is_default: false,
                        is_namespace: false,
                        source_module: source.clone(),
                    };
                    module.import_bindings.insert(named_spec.local.name.clone(), binding);
                }
                crate::ast::ImportSpecifier::Namespace(namespace_spec) => {
                    let binding = ImportBinding {
                        name: "*".to_string(),
                        local_name: namespace_spec.local.name.clone(),
                        is_default: false,
                        is_namespace: true,
                        source_module: source.clone(),
                    };
                    module.import_bindings.insert(namespace_spec.local.name.clone(), binding);
                }
            }
        }
        Ok(())
    }

    /// Analyze export declaration
    async fn analyze_export_declaration(
        &self,
        module: &mut ModuleRecord,
        export_decl: &ExportDeclaration,
    ) -> Result<()> {
        match export_decl {
            ExportDeclaration::Default(default_export) => {
                let binding = ExportBinding {
                    name: "default".to_string(),
                    local_name: "default".to_string(), // Will be resolved during evaluation
                    is_default: true,
                    is_reexport: false,
                    source_module: None,
                };
                module.export_bindings.insert("default".to_string(), binding);
            }
            ExportDeclaration::Named(named_export) => {
                if let Some(declaration) = &named_export.declaration {
                    // Export declaration (function, class, var, etc.)
                    self.analyze_named_export_declaration(module, declaration).await?;
                } else {
                    // Export specifiers
                    for specifier in &named_export.specifiers {
                        let binding = ExportBinding {
                            name: specifier.exported.name.clone(),
                            local_name: specifier.local.name.clone(),
                            is_default: false,
                            is_reexport: named_export.source.is_some(),
                            source_module: named_export.source.as_ref().and_then(|s| {
                                if let Literal::String(ref str_lit) = s {
                                    Some(str_lit.clone())
                                } else {
                                    None
                                }
                            }),
                        };
                        module.export_bindings.insert(specifier.exported.name.clone(), binding);
                    }
                }
            }
            ExportDeclaration::All(all_export) => {
                let source = match &all_export.source {
                    Literal::String(s) => s.clone(),
                    _ => return Err(Error::parsing("Export source must be a string literal".to_string())),
                };

                // Re-export all from source module
                let binding = ExportBinding {
                    name: "*".to_string(),
                    local_name: "*".to_string(),
                    is_default: false,
                    is_reexport: true,
                    source_module: Some(source),
                };
                module.export_bindings.insert("*".to_string(), binding);
            }
        }
        Ok(())
    }

    /// Analyze named export declaration
    async fn analyze_named_export_declaration(
        &self,
        module: &mut ModuleRecord,
        declaration: &crate::ast::Declaration,
    ) -> Result<()> {
        match declaration {
            crate::ast::Declaration::Function(func_decl) => {
                if let Some(id) = &func_decl.id {
                    let binding = ExportBinding {
                        name: id.name.clone(),
                        local_name: id.name.clone(),
                        is_default: false,
                        is_reexport: false,
                        source_module: None,
                    };
                    module.export_bindings.insert(id.name.clone(), binding);
                }
            }
            crate::ast::Declaration::Class(class_decl) => {
                if let Some(id) = &class_decl.id {
                    let binding = ExportBinding {
                        name: id.name.clone(),
                        local_name: id.name.clone(),
                        is_default: false,
                        is_reexport: false,
                        source_module: None,
                    };
                    module.export_bindings.insert(id.name.clone(), binding);
                }
            }
            crate::ast::Declaration::Variable(var_decl) => {
                for declarator in &var_decl.declarations {
                    if let crate::ast::Pattern::Identifier(id) = &declarator.id {
                        let binding = ExportBinding {
                            name: id.name.clone(),
                            local_name: id.name.clone(),
                            is_default: false,
                            is_reexport: false,
                            source_module: None,
                        };
                        module.export_bindings.insert(id.name.clone(), binding);
                    }
                }
            }
        }
        Ok(())
    }

    /// Get all loaded modules
    pub async fn get_loaded_modules(&self) -> HashMap<String, ModuleRecord> {
        let modules = self.modules.read().await;
        modules.clone()
    }

    /// Clear module cache
    pub async fn clear_cache(&self) {
        let mut modules = self.modules.write().await;
        modules.clear();
        
        let mut cache = self.resolution_cache.write().await;
        cache.clear();
    }
}

/// ES Module evaluator
pub struct ModuleEvaluator {
    /// Module loader
    loader: ModuleLoader,
    /// Global scope
    global_scope: HashMap<String, ModuleValue>,
}

impl ModuleEvaluator {
    /// Create a new module evaluator
    pub fn new(loader: ModuleLoader) -> Self {
        Self {
            loader,
            global_scope: HashMap::new(),
        }
    }

    /// Evaluate a module
    pub async fn evaluate_module(&self, specifier: &str) -> Result<ModuleNamespace> {
        // Load the module
        let mut module = self.loader.load_module(specifier).await?;

        // Check if already evaluated
        if module.evaluated {
            return module.namespace.ok_or_else(|| {
                Error::parsing("Module evaluated but no namespace available".to_string())
            });
        }

        // Create module namespace
        let mut namespace = ModuleNamespace {
            properties: HashMap::new(),
            sealed: false,
        };

        // Evaluate module body
        self.evaluate_module_body(&mut module, &mut namespace).await?;

        // Mark as evaluated
        module.evaluated = true;
        module.namespace = Some(namespace.clone());

        // Update module in cache
        {
            let mut modules = self.loader.modules.write().await;
            modules.insert(specifier.to_string(), module);
        }

        Ok(namespace)
    }

    /// Evaluate module body
    async fn evaluate_module_body(
        &self,
        module: &mut ModuleRecord,
        namespace: &mut ModuleNamespace,
    ) -> Result<()> {
        // For now, this is a placeholder implementation
        // In a real implementation, this would:
        // 1. Set up module scope
        // 2. Evaluate declarations
        // 3. Create export bindings
        // 4. Handle imports and dependencies

        // Add exports to namespace
        for (name, binding) in &module.export_bindings {
            if binding.is_default {
                namespace.properties.insert("default".to_string(), ModuleValue::Undefined);
            } else if binding.is_reexport {
                // Handle re-exports
                if let Some(source) = &binding.source_module {
                    let source_namespace = self.evaluate_module(source).await?;
                    if binding.name == "*" {
                        // Re-export all
                        for (key, value) in &source_namespace.properties {
                            namespace.properties.insert(key.clone(), value.clone());
                        }
                    } else {
                        // Re-export specific binding
                        if let Some(value) = source_namespace.properties.get(&binding.name) {
                            namespace.properties.insert(binding.name.clone(), value.clone());
                        }
                    }
                }
            } else {
                // Regular export
                namespace.properties.insert(name.clone(), ModuleValue::Undefined);
            }
        }

        Ok(())
    }

    /// Get global scope
    pub fn get_global_scope(&self) -> &HashMap<String, ModuleValue> {
        &self.global_scope
    }

    /// Set global scope value
    pub fn set_global_value(&mut self, name: String, value: ModuleValue) {
        self.global_scope.insert(name, value);
    }
}

/// ES Module system
pub struct ESModuleSystem {
    /// Module loader
    loader: ModuleLoader,
    /// Module evaluator
    evaluator: ModuleEvaluator,
}

impl ESModuleSystem {
    /// Create a new ES module system
    pub fn new(base_url: String) -> Self {
        let loader = ModuleLoader::new(base_url);
        let evaluator = ModuleEvaluator::new(loader.clone());
        
        Self {
            loader,
            evaluator,
        }
    }

    /// Load and evaluate a module
    pub async fn load_and_evaluate(&self, specifier: &str) -> Result<ModuleNamespace> {
        self.evaluator.evaluate_module(specifier).await
    }

    /// Get module loader
    pub fn get_loader(&self) -> &ModuleLoader {
        &self.loader
    }

    /// Get module evaluator
    pub fn get_evaluator(&self) -> &ModuleEvaluator {
        &self.evaluator
    }
}

// Implement Clone for ModuleLoader
impl Clone for ModuleLoader {
    fn clone(&self) -> Self {
        Self {
            modules: Arc::clone(&self.modules),
            resolution_cache: Arc::clone(&self.resolution_cache),
            base_url: self.base_url.clone(),
        }
    }
}
