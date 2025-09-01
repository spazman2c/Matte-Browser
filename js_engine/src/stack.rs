use crate::error::{Error, Result};
use std::collections::VecDeque;

/// JavaScript value for stack operations
#[derive(Debug, Clone)]
pub enum Value {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Object(std::collections::HashMap<String, Value>),
    Array(Vec<Value>),
    Function(FunctionValue),
    Class(ClassValue),
}

/// Function value
#[derive(Debug, Clone)]
pub struct FunctionValue {
    pub name: String,
    pub param_count: u32,
    pub local_count: u32,
    pub closure: std::collections::HashMap<String, Value>,
}

/// Class value
#[derive(Debug, Clone)]
pub struct ClassValue {
    pub name: String,
    pub constructor: Option<FunctionValue>,
    pub methods: std::collections::HashMap<String, FunctionValue>,
    pub static_methods: std::collections::HashMap<String, FunctionValue>,
    pub properties: std::collections::HashMap<String, Value>,
}

/// Operand stack for expression evaluation
#[derive(Debug)]
pub struct OperandStack {
    stack: Vec<Value>,
    max_size: usize,
}

impl OperandStack {
    /// Create a new operand stack
    pub fn new(max_size: usize) -> Self {
        Self {
            stack: Vec::new(),
            max_size,
        }
    }

    /// Push a value onto the stack
    pub fn push(&mut self, value: Value) -> Result<()> {
        if self.stack.len() >= self.max_size {
            return Err(Error::parsing("Operand stack overflow".to_string()));
        }
        self.stack.push(value);
        Ok(())
    }

    /// Pop a value from the stack
    pub fn pop(&mut self) -> Result<Value> {
        self.stack.pop().ok_or_else(|| {
            Error::parsing("Operand stack underflow".to_string())
        })
    }

    /// Peek at the top value without removing it
    pub fn peek(&self) -> Result<&Value> {
        self.stack.last().ok_or_else(|| {
            Error::parsing("Operand stack is empty".to_string())
        })
    }

    /// Peek at the nth value from the top
    pub fn peek_n(&self, n: usize) -> Result<&Value> {
        if n >= self.stack.len() {
            return Err(Error::parsing("Index out of bounds".to_string()));
        }
        Ok(&self.stack[self.stack.len() - 1 - n])
    }

    /// Get the current stack size
    pub fn size(&self) -> usize {
        self.stack.len()
    }

    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Clear the stack
    pub fn clear(&mut self) {
        self.stack.clear();
    }

    /// Get the maximum stack size
    pub fn max_size(&self) -> usize {
        self.max_size
    }

    /// Duplicate the top value
    pub fn dup(&mut self) -> Result<()> {
        let value = self.peek()?.clone();
        self.push(value)
    }

    /// Swap the top two values
    pub fn swap(&mut self) -> Result<()> {
        if self.stack.len() < 2 {
            return Err(Error::parsing("Not enough values to swap".to_string()));
        }
        let len = self.stack.len();
        self.stack.swap(len - 1, len - 2);
        Ok(())
    }

    /// Rotate the top three values
    pub fn rot(&mut self) -> Result<()> {
        if self.stack.len() < 3 {
            return Err(Error::parsing("Not enough values to rotate".to_string()));
        }
        let len = self.stack.len();
        self.stack.swap(len - 1, len - 3);
        self.stack.swap(len - 2, len - 3);
        Ok(())
    }

    /// Get a slice of the stack (for debugging)
    pub fn as_slice(&self) -> &[Value] {
        &self.stack
    }
}

/// Stack frame for function calls
#[derive(Debug)]
pub struct StackFrame {
    /// Function being executed
    pub function: FunctionValue,
    /// Program counter
    pub pc: usize,
    /// Local variables
    pub locals: Vec<Value>,
    /// Operand stack for this frame
    pub operand_stack: OperandStack,
    /// Return address
    pub return_address: Option<usize>,
    /// This value
    pub this_value: Option<Value>,
    /// Arguments passed to the function
    pub arguments: Vec<Value>,
}

impl StackFrame {
    /// Create a new stack frame
    pub fn new(function: FunctionValue, return_address: Option<usize>) -> Self {
        let max_stack_size = 1024; // Default stack size
        Self {
            operand_stack: OperandStack::new(max_stack_size),
            locals: vec![Value::Undefined; function.local_count as usize],
            pc: 0,
            function,
            return_address,
            this_value: None,
            arguments: Vec::new(),
        }
    }

    /// Get a local variable
    pub fn get_local(&self, index: u32) -> Result<&Value> {
        if index as usize >= self.locals.len() {
            return Err(Error::parsing(format!("Local variable {} out of bounds", index)));
        }
        Ok(&self.locals[index as usize])
    }

    /// Set a local variable
    pub fn set_local(&mut self, index: u32, value: Value) -> Result<()> {
        if index as usize >= self.locals.len() {
            return Err(Error::parsing(format!("Local variable {} out of bounds", index)));
        }
        self.locals[index as usize] = value;
        Ok(())
    }

    /// Get an argument
    pub fn get_argument(&self, index: u32) -> Result<&Value> {
        if index as usize >= self.arguments.len() {
            return Err(Error::parsing(format!("Argument {} out of bounds", index)));
        }
        Ok(&self.arguments[index as usize])
    }

    /// Set arguments
    pub fn set_arguments(&mut self, args: Vec<Value>) {
        self.arguments = args;
    }

    /// Get the current stack depth
    pub fn stack_depth(&self) -> usize {
        self.operand_stack.size()
    }

    /// Check if the frame is at the end of execution
    pub fn is_completed(&self) -> bool {
        // This would depend on the bytecode implementation
        false
    }
}

/// Call stack for managing function calls
#[derive(Debug)]
pub struct CallStack {
    frames: VecDeque<StackFrame>,
    max_depth: usize,
}

impl CallStack {
    /// Create a new call stack
    pub fn new(max_depth: usize) -> Self {
        Self {
            frames: VecDeque::new(),
            max_depth,
        }
    }

    /// Push a frame onto the call stack
    pub fn push(&mut self, frame: StackFrame) -> Result<()> {
        if self.frames.len() >= self.max_depth {
            return Err(Error::parsing("Call stack overflow".to_string()));
        }
        self.frames.push_back(frame);
        Ok(())
    }

    /// Pop a frame from the call stack
    pub fn pop(&mut self) -> Result<StackFrame> {
        self.frames.pop_back().ok_or_else(|| {
            Error::parsing("Call stack underflow".to_string())
        })
    }

    /// Peek at the top frame without removing it
    pub fn peek(&self) -> Result<&StackFrame> {
        self.frames.back().ok_or_else(|| {
            Error::parsing("Call stack is empty".to_string())
        })
    }

    /// Get a mutable reference to the top frame
    pub fn peek_mut(&mut self) -> Result<&mut StackFrame> {
        self.frames.back_mut().ok_or_else(|| {
            Error::parsing("Call stack is empty".to_string())
        })
    }

    /// Get the current stack depth
    pub fn depth(&self) -> usize {
        self.frames.len()
    }

    /// Check if the call stack is empty
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    /// Clear the call stack
    pub fn clear(&mut self) {
        self.frames.clear();
    }

    /// Get the maximum stack depth
    pub fn max_depth(&self) -> usize {
        self.max_depth
    }

    /// Get a frame by index (0 = bottom, len-1 = top)
    pub fn get_frame(&self, index: usize) -> Result<&StackFrame> {
        self.frames.get(index).ok_or_else(|| {
            Error::parsing(format!("Frame {} not found", index))
        })
    }

    /// Get a mutable frame by index
    pub fn get_frame_mut(&mut self, index: usize) -> Result<&mut StackFrame> {
        self.frames.get_mut(index).ok_or_else(|| {
            Error::parsing(format!("Frame {} not found", index))
        })
    }

    /// Get all frames as a slice
    pub fn as_slice(&self) -> &[StackFrame] {
        self.frames.as_slices().0
    }
}

/// Stack manager for coordinating different stack types
pub struct StackManager {
    call_stack: CallStack,
    global_operand_stack: OperandStack,
    exception_stack: Vec<ExceptionInfo>,
}

/// Exception information
#[derive(Debug, Clone)]
pub struct ExceptionInfo {
    pub exception: Value,
    pub catch_address: usize,
    pub finally_address: Option<usize>,
    pub frame_depth: usize,
}

impl StackManager {
    /// Create a new stack manager
    pub fn new(max_call_depth: usize, max_operand_stack: usize) -> Self {
        Self {
            call_stack: CallStack::new(max_call_depth),
            global_operand_stack: OperandStack::new(max_operand_stack),
            exception_stack: Vec::new(),
        }
    }

    /// Get the call stack
    pub fn call_stack(&self) -> &CallStack {
        &self.call_stack
    }

    /// Get a mutable reference to the call stack
    pub fn call_stack_mut(&mut self) -> &mut CallStack {
        &mut self.call_stack
    }

    /// Get the global operand stack
    pub fn operand_stack(&self) -> &OperandStack {
        &self.global_operand_stack
    }

    /// Get a mutable reference to the global operand stack
    pub fn operand_stack_mut(&mut self) -> &mut OperandStack {
        &mut self.global_operand_stack
    }

    /// Push an exception handler
    pub fn push_exception_handler(&mut self, info: ExceptionInfo) {
        self.exception_stack.push(info);
    }

    /// Pop an exception handler
    pub fn pop_exception_handler(&mut self) -> Option<ExceptionInfo> {
        self.exception_stack.pop()
    }

    /// Get the current exception handler
    pub fn current_exception_handler(&self) -> Option<&ExceptionInfo> {
        self.exception_stack.last()
    }

    /// Clear all stacks
    pub fn clear(&mut self) {
        self.call_stack.clear();
        self.global_operand_stack.clear();
        self.exception_stack.clear();
    }

    /// Get the current stack depth
    pub fn call_stack_depth(&self) -> usize {
        self.call_stack.depth()
    }

    /// Get the current operand stack size
    pub fn operand_stack_size(&self) -> usize {
        self.global_operand_stack.size()
    }

    /// Check if any stack is empty
    pub fn is_empty(&self) -> bool {
        self.call_stack.is_empty() && self.global_operand_stack.is_empty()
    }

    /// Get stack statistics
    pub fn get_stats(&self) -> StackStats {
        StackStats {
            call_stack_depth: self.call_stack.depth(),
            operand_stack_size: self.global_operand_stack.size(),
            exception_handlers: self.exception_stack.len(),
            max_call_depth: self.call_stack.max_depth(),
            max_operand_stack_size: self.global_operand_stack.max_size(),
        }
    }
}

/// Stack statistics
#[derive(Debug, Clone)]
pub struct StackStats {
    pub call_stack_depth: usize,
    pub operand_stack_size: usize,
    pub exception_handlers: usize,
    pub max_call_depth: usize,
    pub max_operand_stack_size: usize,
}

/// Stack allocator for managing stack memory
pub struct StackAllocator {
    /// Available stack frames
    frame_pool: Vec<StackFrame>,
    /// Available operand stacks
    operand_stack_pool: Vec<OperandStack>,
    /// Maximum pool size
    max_pool_size: usize,
}

impl StackAllocator {
    /// Create a new stack allocator
    pub fn new(max_pool_size: usize) -> Self {
        Self {
            frame_pool: Vec::new(),
            operand_stack_pool: Vec::new(),
            max_pool_size,
        }
    }

    /// Allocate a stack frame
    pub fn allocate_frame(&mut self, function: FunctionValue, return_address: Option<usize>) -> StackFrame {
        if let Some(mut frame) = self.frame_pool.pop() {
            // Reuse existing frame
            frame.function = function;
            frame.pc = 0;
            frame.locals = vec![Value::Undefined; function.local_count as usize];
            frame.operand_stack.clear();
            frame.return_address = return_address;
            frame.this_value = None;
            frame.arguments.clear();
            frame
        } else {
            // Create new frame
            StackFrame::new(function, return_address)
        }
    }

    /// Deallocate a stack frame
    pub fn deallocate_frame(&mut self, frame: StackFrame) {
        if self.frame_pool.len() < self.max_pool_size {
            self.frame_pool.push(frame);
        }
    }

    /// Allocate an operand stack
    pub fn allocate_operand_stack(&mut self, max_size: usize) -> OperandStack {
        if let Some(mut stack) = self.operand_stack_pool.pop() {
            // Reuse existing stack
            stack.clear();
            stack
        } else {
            // Create new stack
            OperandStack::new(max_size)
        }
    }

    /// Deallocate an operand stack
    pub fn deallocate_operand_stack(&mut self, stack: OperandStack) {
        if self.operand_stack_pool.len() < self.max_pool_size {
            self.operand_stack_pool.push(stack);
        }
    }

    /// Get pool statistics
    pub fn get_pool_stats(&self) -> PoolStats {
        PoolStats {
            frame_pool_size: self.frame_pool.len(),
            operand_stack_pool_size: self.operand_stack_pool.len(),
            max_pool_size: self.max_pool_size,
        }
    }

    /// Clear all pools
    pub fn clear_pools(&mut self) {
        self.frame_pool.clear();
        self.operand_stack_pool.clear();
    }
}

/// Pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub frame_pool_size: usize,
    pub operand_stack_pool_size: usize,
    pub max_pool_size: usize,
}

/// Stack guard for detecting stack overflow
pub struct StackGuard {
    max_depth: usize,
    current_depth: usize,
}

impl StackGuard {
    /// Create a new stack guard
    pub fn new(max_depth: usize) -> Self {
        Self {
            max_depth,
            current_depth: 0,
        }
    }

    /// Enter a new stack level
    pub fn enter(&mut self) -> Result<()> {
        self.current_depth += 1;
        if self.current_depth > self.max_depth {
            return Err(Error::parsing("Stack overflow detected".to_string()));
        }
        Ok(())
    }

    /// Exit a stack level
    pub fn exit(&mut self) {
        if self.current_depth > 0 {
            self.current_depth -= 1;
        }
    }

    /// Get the current depth
    pub fn current_depth(&self) -> usize {
        self.current_depth
    }

    /// Get the maximum depth
    pub fn max_depth(&self) -> usize {
        self.max_depth
    }

    /// Check if at maximum depth
    pub fn is_at_max_depth(&self) -> bool {
        self.current_depth >= self.max_depth
    }

    /// Reset the guard
    pub fn reset(&mut self) {
        self.current_depth = 0;
    }
}
