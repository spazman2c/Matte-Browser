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
pub mod async_await;
pub mod class_system;
pub mod destructuring;
pub mod bytecode;
pub mod stack;
pub mod inline_cache;
pub mod tiering;
pub mod hot_path;
pub mod garbage_collector;
pub mod memory_pool;
pub mod webidl;
pub mod builtins;

#[cfg(test)]
mod es_modules_test;
#[cfg(test)]
mod async_await_test;
#[cfg(test)]
mod class_system_test;
#[cfg(test)]
mod destructuring_test;
#[cfg(test)]
mod bytecode_test;
#[cfg(test)]
mod stack_test;
#[cfg(test)]
mod inline_cache_test;
#[cfg(test)]
mod tiering_test;
#[cfg(test)]
mod hot_path_test;
#[cfg(test)]
mod garbage_collector_test;
#[cfg(test)]
mod memory_pool_test;
#[cfg(test)]
mod webidl_test;
#[cfg(test)]
mod builtins_test;

// Re-export main types
pub use parser::JsParser;
pub use ast::{AstNode, Program, Statement, Expression, Declaration, Identifier, Literal};
pub use lexer::{Token, TokenType, Lexer};
pub use error::{Error, Result};
pub use source_map::SourceMap;
pub use es_modules::{ESModuleSystem, ModuleLoader, ModuleEvaluator, ModuleRecord, ModuleNamespace, ModuleValue};
pub use async_await::{AsyncAwaitSystem, AsyncContext, Promise, PromiseState, Value, AsyncFunctionValue, EventLoop};
pub use class_system::{ClassSystem, ClassParser, ClassDefinition, ClassInstance, MethodDefinition, MethodKind, PropertyDefinition, PrivateFieldDefinition, ClassPrototype};
pub use destructuring::{DestructuringSystem, DestructuringEngine, SpreadOperator, PatternMatcher, DestructuringContext};
pub use bytecode::{BytecodeEngine, BytecodeCompiler, BytecodeFunction, Register, ConstantIndex, Label, Instruction, Value as BytecodeValue, FunctionValue, ClassValue, RegisterFile, CallFrame};
pub use stack::{StackManager, StackAllocator, StackGuard, OperandStack, CallStack, StackFrame, FunctionValue as StackFunctionValue, ClassValue as StackClassValue, Value as StackValue, ExceptionInfo, StackStats, PoolStats};
pub use inline_cache::{InlineCacheManager, PropertyCache, MethodCache, GlobalCache, ShapeRegistry, PropertyCacheEntry, MethodCacheEntry, GlobalCacheEntry, Value as CacheValue, ObjectValue, FunctionValue as CacheFunctionValue, ClassValue as CacheClassValue, CacheStats, InlineCacheStats, ShapeDefinition};
pub use tiering::{TieringManager, TieringConfig, ExecutionTier, FunctionStats, CodeCacheEntry, ExecutionResult, TieringStats, EngineStats};
pub use hot_path::{HotPathOptimizer, HotPathConfig, HotPathId, HotPathStats, PathNode, PathNodeType, OptimizationHint, OptimizationHintType, OptimizedPath, OptimizationStats};
pub use garbage_collector::{GarbageCollector, GCConfig, GCStrategy, MemoryObject, RootReference, RootType, ReferenceState, GCStats, GenerationalConfig, IncrementalConfig};
pub use memory_pool::{MemoryPool, PoolConfig, PoolType, PoolStats, PoolEntry, Nursery, NurseryConfig, NurseryStats, MemoryPoolManager, ManagerConfig, ManagerStats};
pub use webidl::{WebIDLParser, WebIDLGenerator, FastDOMBinding, WebIDLDefinition, WebIDLInterface, WebIDLMethod, WebIDLProperty, WebIDLArgument, WebIDLType, InterfaceBinding, MethodBinding, PropertyBinding, Value};
pub use builtins::{TypedArray, TypedArrayType, Promise, PromiseState, FetchAPI, FetchRequest, FetchResponse, TimerManager, TimerType, EventManager, EventType, Event, BuiltinObjects, Value as BuiltinValue};
