//! JavaScript Engine (MatteJS) implementation for the Matte browser.
//! 
//! This crate provides a complete JavaScript engine including parser, AST,
//! bytecode VM, and runtime environment.

pub mod parser;
pub mod ast;
pub mod lexer;
pub mod error;
pub mod source_map;
pub mod es_modules;

#[cfg(test)]
mod es_modules_test;

// Re-export main types
pub use parser::JsParser;
pub use ast::{AstNode, Program, Statement, Expression, Declaration, Identifier, Literal};
pub use lexer::{Token, TokenType, Lexer};
pub use error::{Error, Result};
pub use source_map::SourceMap;
pub use es_modules::{ESModuleSystem, ModuleLoader, ModuleEvaluator, ModuleRecord, ModuleNamespace, ModuleValue};
