use crate::error::{Error, Result};
use std::collections::HashMap;
use std::fmt;

/// Register identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Register(pub u32);

/// Constant pool index
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantIndex(pub u32);

/// Label for jump instructions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Label(pub u32);

/// Bytecode instruction
#[derive(Debug, Clone)]
pub enum Instruction {
    // Load and store operations
    LoadConstant(Register, ConstantIndex),
    LoadUndefined(Register),
    LoadNull(Register),
    LoadTrue(Register),
    LoadFalse(Register),
    LoadGlobal(Register, String),
    StoreGlobal(String, Register),
    LoadLocal(Register, u32),
    StoreLocal(u32, Register),
    LoadProperty(Register, Register, Register),
    StoreProperty(Register, Register, Register),
    LoadIndex(Register, Register, Register),
    StoreIndex(Register, Register, Register),

    // Arithmetic operations
    Add(Register, Register, Register),
    Subtract(Register, Register, Register),
    Multiply(Register, Register, Register),
    Divide(Register, Register, Register),
    Modulo(Register, Register, Register),
    Exponent(Register, Register, Register),
    Negate(Register, Register),
    Increment(Register),
    Decrement(Register),

    // Bitwise operations
    BitwiseAnd(Register, Register, Register),
    BitwiseOr(Register, Register, Register),
    BitwiseXor(Register, Register, Register),
    BitwiseNot(Register, Register),
    LeftShift(Register, Register, Register),
    RightShift(Register, Register, Register),
    UnsignedRightShift(Register, Register, Register),

    // Comparison operations
    Equal(Register, Register, Register),
    NotEqual(Register, Register, Register),
    StrictEqual(Register, Register, Register),
    StrictNotEqual(Register, Register, Register),
    LessThan(Register, Register, Register),
    LessThanEqual(Register, Register, Register),
    GreaterThan(Register, Register, Register),
    GreaterThanEqual(Register, Register, Register),

    // Logical operations
    LogicalAnd(Register, Register, Register),
    LogicalOr(Register, Register, Register),
    LogicalNot(Register, Register),

    // Control flow
    Jump(Label),
    JumpIfTrue(Register, Label),
    JumpIfFalse(Register, Label),
    JumpIfNull(Register, Label),
    JumpIfUndefined(Register, Label),

    // Function operations
    Call(Register, Register, u32), // target, result, arg_count
    CallMethod(Register, Register, Register, u32), // object, method, result, arg_count
    CallConstructor(Register, Register, u32), // constructor, result, arg_count
    Return(Register),
    ReturnUndefined,

    // Object operations
    CreateObject(Register),
    CreateArray(Register, u32), // result, length
    CreateFunction(Register, ConstantIndex, u32), // result, function_index, param_count
    CreateClass(Register, ConstantIndex, ConstantIndex), // result, class_index, super_index

    // Type operations
    TypeOf(Register, Register),
    InstanceOf(Register, Register, Register),
    In(Register, Register, Register),

    // Exception handling
    Try(Label, Label), // try_block, catch_block
    Throw(Register),
    Finally(Label),

    // Debug operations
    DebugPrint(Register),
    DebugBreak,

    // Special operations
    Nop,
    Halt,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::LoadConstant(reg, idx) => write!(f, "LOAD_CONST r{}, const[{}]", reg.0, idx.0),
            Instruction::LoadUndefined(reg) => write!(f, "LOAD_UNDEFINED r{}", reg.0),
            Instruction::LoadNull(reg) => write!(f, "LOAD_NULL r{}", reg.0),
            Instruction::LoadTrue(reg) => write!(f, "LOAD_TRUE r{}", reg.0),
            Instruction::LoadFalse(reg) => write!(f, "LOAD_FALSE r{}", reg.0),
            Instruction::LoadGlobal(reg, name) => write!(f, "LOAD_GLOBAL r{}, '{}'", reg.0, name),
            Instruction::StoreGlobal(name, reg) => write!(f, "STORE_GLOBAL '{}', r{}", name, reg.0),
            Instruction::LoadLocal(reg, idx) => write!(f, "LOAD_LOCAL r{}, local[{}]", reg.0, idx),
            Instruction::StoreLocal(idx, reg) => write!(f, "STORE_LOCAL local[{}], r{}", idx, reg.0),
            Instruction::LoadProperty(obj, prop, result) => write!(f, "LOAD_PROP r{}, r{}, r{}", obj.0, prop.0, result.0),
            Instruction::StoreProperty(obj, prop, value) => write!(f, "STORE_PROP r{}, r{}, r{}", obj.0, prop.0, value.0),
            Instruction::LoadIndex(array, index, result) => write!(f, "LOAD_INDEX r{}, r{}, r{}", array.0, index.0, result.0),
            Instruction::StoreIndex(array, index, value) => write!(f, "STORE_INDEX r{}, r{}, r{}", array.0, index.0, value.0),
            Instruction::Add(a, b, result) => write!(f, "ADD r{}, r{}, r{}", a.0, b.0, result.0),
            Instruction::Subtract(a, b, result) => write!(f, "SUB r{}, r{}, r{}", a.0, b.0, result.0),
            Instruction::Multiply(a, b, result) => write!(f, "MUL r{}, r{}, r{}", a.0, b.0, result.0),
            Instruction::Divide(a, b, result) => write!(f, "DIV r{}, r{}, r{}", a.0, b.0, result.0),
            Instruction::Modulo(a, b, result) => write!(f, "MOD r{}, r{}, r{}", a.0, b.0, result.0),
            Instruction::Exponent(a, b, result) => write!(f, "EXP r{}, r{}, r{}", a.0, b.0, result.0),
            Instruction::Negate(reg, result) => write!(f, "NEG r{}, r{}", reg.0, result.0),
            Instruction::Increment(reg) => write!(f, "INC r{}", reg.0),
            Instruction::Decrement(reg) => write!(f, "DEC r{}", reg.0),
            Instruction::BitwiseAnd(a, b, result) => write!(f, "BIT_AND r{}, r{}, r{}", a.0, b.0, result.0),
            Instruction::BitwiseOr(a, b, result) => write!(f, "BIT_OR r{}, r{}, r{}", a.0, b.0, result.0),
            Instruction::BitwiseXor(a, b, result) => write!(f, "BIT_XOR r{}, r{}, r{}", a.0, b.0, result.0),
            Instruction::BitwiseNot(reg, result) => write!(f, "BIT_NOT r{}, r{}", reg.0, result.0),
            Instruction::LeftShift(a, b, result) => write!(f, "LSHIFT r{}, r{}, r{}", a.0, b.0, result.0),
            Instruction::RightShift(a, b, result) => write!(f, "RSHIFT r{}, r{}, r{}", a.0, b.0, result.0),
            Instruction::UnsignedRightShift(a, b, result) => write!(f, "URSHIFT r{}, r{}, r{}", a.0, b.0, result.0),
            Instruction::Equal(a, b, result) => write!(f, "EQ r{}, r{}, r{}", a.0, b.0, result.0),
            Instruction::NotEqual(a, b, result) => write!(f, "NEQ r{}, r{}, r{}", a.0, b.0, result.0),
            Instruction::StrictEqual(a, b, result) => write!(f, "SEQ r{}, r{}, r{}", a.0, b.0, result.0),
            Instruction::StrictNotEqual(a, b, result) => write!(f, "SNEQ r{}, r{}, r{}", a.0, b.0, result.0),
            Instruction::LessThan(a, b, result) => write!(f, "LT r{}, r{}, r{}", a.0, b.0, result.0),
            Instruction::LessThanEqual(a, b, result) => write!(f, "LTE r{}, r{}, r{}", a.0, b.0, result.0),
            Instruction::GreaterThan(a, b, result) => write!(f, "GT r{}, r{}, r{}", a.0, b.0, result.0),
            Instruction::GreaterThanEqual(a, b, result) => write!(f, "GTE r{}, r{}, r{}", a.0, b.0, result.0),
            Instruction::LogicalAnd(a, b, result) => write!(f, "AND r{}, r{}, r{}", a.0, b.0, result.0),
            Instruction::LogicalOr(a, b, result) => write!(f, "OR r{}, r{}, r{}", a.0, b.0, result.0),
            Instruction::LogicalNot(reg, result) => write!(f, "NOT r{}, r{}", reg.0, result.0),
            Instruction::Jump(label) => write!(f, "JUMP L{}", label.0),
            Instruction::JumpIfTrue(reg, label) => write!(f, "JUMP_IF_TRUE r{}, L{}", reg.0, label.0),
            Instruction::JumpIfFalse(reg, label) => write!(f, "JUMP_IF_FALSE r{}, L{}", reg.0, label.0),
            Instruction::JumpIfNull(reg, label) => write!(f, "JUMP_IF_NULL r{}, L{}", reg.0, label.0),
            Instruction::JumpIfUndefined(reg, label) => write!(f, "JUMP_IF_UNDEFINED r{}, L{}", reg.0, label.0),
            Instruction::Call(target, result, count) => write!(f, "CALL r{}, r{}, {}", target.0, result.0, count),
            Instruction::CallMethod(obj, method, result, count) => write!(f, "CALL_METHOD r{}, r{}, r{}, {}", obj.0, method.0, result.0, count),
            Instruction::CallConstructor(constructor, result, count) => write!(f, "CALL_CONSTRUCTOR r{}, r{}, {}", constructor.0, result.0, count),
            Instruction::Return(reg) => write!(f, "RETURN r{}", reg.0),
            Instruction::ReturnUndefined => write!(f, "RETURN_UNDEFINED"),
            Instruction::CreateObject(reg) => write!(f, "CREATE_OBJECT r{}", reg.0),
            Instruction::CreateArray(reg, len) => write!(f, "CREATE_ARRAY r{}, {}", reg.0, len),
            Instruction::CreateFunction(reg, idx, params) => write!(f, "CREATE_FUNCTION r{}, const[{}], {}", reg.0, idx.0, params),
            Instruction::CreateClass(reg, class_idx, super_idx) => write!(f, "CREATE_CLASS r{}, const[{}], const[{}]", reg.0, class_idx.0, super_idx.0),
            Instruction::TypeOf(reg, result) => write!(f, "TYPEOF r{}, r{}", reg.0, result.0),
            Instruction::InstanceOf(obj, constructor, result) => write!(f, "INSTANCEOF r{}, r{}, r{}", obj.0, constructor.0, result.0),
            Instruction::In(prop, obj, result) => write!(f, "IN r{}, r{}, r{}", prop.0, obj.0, result.0),
            Instruction::Try(try_label, catch_label) => write!(f, "TRY L{}, L{}", try_label.0, catch_label.0),
            Instruction::Throw(reg) => write!(f, "THROW r{}", reg.0),
            Instruction::Finally(label) => write!(f, "FINALLY L{}", label.0),
            Instruction::DebugPrint(reg) => write!(f, "DEBUG_PRINT r{}", reg.0),
            Instruction::DebugBreak => write!(f, "DEBUG_BREAK"),
            Instruction::Nop => write!(f, "NOP"),
            Instruction::Halt => write!(f, "HALT"),
        }
    }
}

/// JavaScript value for bytecode execution
#[derive(Debug, Clone)]
pub enum Value {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    Function(FunctionValue),
    Class(ClassValue),
}

/// Function value for bytecode
#[derive(Debug, Clone)]
pub struct FunctionValue {
    pub name: String,
    pub bytecode: BytecodeFunction,
    pub param_count: u32,
    pub local_count: u32,
    pub closure: HashMap<String, Value>,
}

/// Class value for bytecode
#[derive(Debug, Clone)]
pub struct ClassValue {
    pub name: String,
    pub constructor: Option<FunctionValue>,
    pub methods: HashMap<String, FunctionValue>,
    pub static_methods: HashMap<String, FunctionValue>,
    pub properties: HashMap<String, Value>,
}

/// Bytecode function
#[derive(Debug, Clone)]
pub struct BytecodeFunction {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub labels: HashMap<Label, usize>,
    pub source_map: Option<SourceMap>,
}

/// Source map for debugging
#[derive(Debug, Clone)]
pub struct SourceMap {
    pub mappings: Vec<(usize, usize)>, // (bytecode_offset, source_offset)
    pub source: String,
}

/// Register file for bytecode execution
#[derive(Debug)]
pub struct RegisterFile {
    registers: Vec<Value>,
    max_registers: usize,
}

impl RegisterFile {
    /// Create a new register file
    pub fn new(max_registers: usize) -> Self {
        Self {
            registers: vec![Value::Undefined; max_registers],
            max_registers,
        }
    }

    /// Get a register value
    pub fn get(&self, register: Register) -> Result<&Value> {
        if register.0 as usize >= self.max_registers {
            return Err(Error::parsing(format!("Register {} out of bounds", register.0)));
        }
        Ok(&self.registers[register.0 as usize])
    }

    /// Set a register value
    pub fn set(&mut self, register: Register, value: Value) -> Result<()> {
        if register.0 as usize >= self.max_registers {
            return Err(Error::parsing(format!("Register {} out of bounds", register.0)));
        }
        self.registers[register.0 as usize] = value;
        Ok(())
    }

    /// Get the number of registers
    pub fn register_count(&self) -> usize {
        self.max_registers
    }

    /// Clear all registers
    pub fn clear(&mut self) {
        for reg in &mut self.registers {
            *reg = Value::Undefined;
        }
    }
}

/// Call stack frame
#[derive(Debug)]
pub struct CallFrame {
    pub function: FunctionValue,
    pub pc: usize, // Program counter
    pub registers: RegisterFile,
    pub locals: Vec<Value>,
    pub return_address: Option<usize>,
    pub this_value: Option<Value>,
}

impl CallFrame {
    /// Create a new call frame
    pub fn new(function: FunctionValue, return_address: Option<usize>) -> Self {
        let max_registers = 256; // Default register count
        Self {
            registers: RegisterFile::new(max_registers),
            locals: vec![Value::Undefined; function.local_count as usize],
            pc: 0,
            function,
            return_address,
            this_value: None,
        }
    }

    /// Get the current instruction
    pub fn current_instruction(&self) -> Option<&Instruction> {
        self.function.bytecode.instructions.get(self.pc)
    }

    /// Advance the program counter
    pub fn advance_pc(&mut self) {
        self.pc += 1;
    }

    /// Jump to a specific address
    pub fn jump_to(&mut self, address: usize) {
        self.pc = address;
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
}

/// Bytecode execution engine
pub struct BytecodeEngine {
    call_stack: Vec<CallFrame>,
    global_scope: HashMap<String, Value>,
    constant_pool: Vec<Value>,
    exception_handler: Option<ExceptionHandler>,
}

/// Exception handler
#[derive(Debug)]
pub struct ExceptionHandler {
    pub exception: Value,
    pub catch_address: usize,
    pub finally_address: Option<usize>,
}

impl BytecodeEngine {
    /// Create a new bytecode engine
    pub fn new() -> Self {
        Self {
            call_stack: Vec::new(),
            global_scope: HashMap::new(),
            constant_pool: Vec::new(),
            exception_handler: None,
        }
    }

    /// Execute a bytecode function
    pub fn execute(&mut self, function: FunctionValue, args: Vec<Value>) -> Result<Value> {
        // Create call frame
        let mut frame = CallFrame::new(function, None);
        
        // Set up arguments
        for (i, arg) in args.into_iter().enumerate() {
            if i < frame.function.param_count as usize {
                frame.set_local(i as u32, arg)?;
            }
        }

        // Push frame onto call stack
        self.call_stack.push(frame);

        // Execute until return or error
        self.run()?;

        // Get return value from top frame
        if let Some(frame) = self.call_stack.last() {
            // For now, return undefined
            Ok(Value::Undefined)
        } else {
            Ok(Value::Undefined)
        }
    }

    /// Run the execution engine
    fn run(&mut self) -> Result<()> {
        while let Some(frame) = self.call_stack.last_mut() {
            if frame.pc >= frame.function.bytecode.instructions.len() {
                // Function completed
                self.call_stack.pop();
                continue;
            }

            let instruction = frame.current_instruction()
                .ok_or_else(|| Error::parsing("Invalid program counter".to_string()))?;

            self.execute_instruction(instruction, frame)?;
            frame.advance_pc();
        }

        Ok(())
    }

    /// Execute a single instruction
    fn execute_instruction(&mut self, instruction: &Instruction, frame: &mut CallFrame) -> Result<()> {
        match instruction {
            Instruction::LoadConstant(reg, idx) => {
                let constant = self.get_constant(*idx)?;
                frame.registers.set(*reg, constant.clone())?;
            }
            Instruction::LoadUndefined(reg) => {
                frame.registers.set(*reg, Value::Undefined)?;
            }
            Instruction::LoadNull(reg) => {
                frame.registers.set(*reg, Value::Null)?;
            }
            Instruction::LoadTrue(reg) => {
                frame.registers.set(*reg, Value::Boolean(true))?;
            }
            Instruction::LoadFalse(reg) => {
                frame.registers.set(*reg, Value::Boolean(false))?;
            }
            Instruction::LoadGlobal(reg, name) => {
                let value = self.global_scope.get(name)
                    .cloned()
                    .unwrap_or(Value::Undefined);
                frame.registers.set(*reg, value)?;
            }
            Instruction::StoreGlobal(name, reg) => {
                let value = frame.registers.get(*reg)?.clone();
                self.global_scope.insert(name.clone(), value);
            }
            Instruction::LoadLocal(reg, idx) => {
                let value = frame.get_local(*idx)?.clone();
                frame.registers.set(*reg, value)?;
            }
            Instruction::StoreLocal(idx, reg) => {
                let value = frame.registers.get(*reg)?.clone();
                frame.set_local(*idx, value)?;
            }
            Instruction::Add(a, b, result) => {
                let a_val = frame.registers.get(*a)?;
                let b_val = frame.registers.get(*b)?;
                let result_val = self.add_values(a_val, b_val)?;
                frame.registers.set(*result, result_val)?;
            }
            Instruction::Subtract(a, b, result) => {
                let a_val = frame.registers.get(*a)?;
                let b_val = frame.registers.get(*b)?;
                let result_val = self.subtract_values(a_val, b_val)?;
                frame.registers.set(*result, result_val)?;
            }
            Instruction::Multiply(a, b, result) => {
                let a_val = frame.registers.get(*a)?;
                let b_val = frame.registers.get(*b)?;
                let result_val = self.multiply_values(a_val, b_val)?;
                frame.registers.set(*result, result_val)?;
            }
            Instruction::Divide(a, b, result) => {
                let a_val = frame.registers.get(*a)?;
                let b_val = frame.registers.get(*b)?;
                let result_val = self.divide_values(a_val, b_val)?;
                frame.registers.set(*result, result_val)?;
            }
            Instruction::Equal(a, b, result) => {
                let a_val = frame.registers.get(*a)?;
                let b_val = frame.registers.get(*b)?;
                let result_val = Value::Boolean(self.equal_values(a_val, b_val));
                frame.registers.set(*result, result_val)?;
            }
            Instruction::NotEqual(a, b, result) => {
                let a_val = frame.registers.get(*a)?;
                let b_val = frame.registers.get(*b)?;
                let result_val = Value::Boolean(!self.equal_values(a_val, b_val));
                frame.registers.set(*result, result_val)?;
            }
            Instruction::Jump(label) => {
                let address = self.get_label_address(*label, frame)?;
                frame.jump_to(address);
            }
            Instruction::JumpIfTrue(reg, label) => {
                let value = frame.registers.get(*reg)?;
                if self.is_truthy(value) {
                    let address = self.get_label_address(*label, frame)?;
                    frame.jump_to(address);
                }
            }
            Instruction::JumpIfFalse(reg, label) => {
                let value = frame.registers.get(*reg)?;
                if !self.is_truthy(value) {
                    let address = self.get_label_address(*label, frame)?;
                    frame.jump_to(address);
                }
            }
            Instruction::Call(target, result, count) => {
                let target_val = frame.registers.get(*target)?;
                let result_val = self.call_function(target_val, Vec::new())?;
                frame.registers.set(*result, result_val)?;
            }
            Instruction::Return(reg) => {
                let return_value = frame.registers.get(*reg)?.clone();
                // Store return value and pop frame
                self.call_stack.pop();
                if let Some(frame) = self.call_stack.last_mut() {
                    // Set return value in calling frame
                    frame.registers.set(Register(0), return_value)?;
                }
            }
            Instruction::ReturnUndefined => {
                self.call_stack.pop();
            }
            Instruction::CreateObject(reg) => {
                let object = Value::Object(HashMap::new());
                frame.registers.set(*reg, object)?;
            }
            Instruction::CreateArray(reg, len) => {
                let array = Value::Array(vec![Value::Undefined; *len as usize]);
                frame.registers.set(*reg, array)?;
            }
            Instruction::DebugPrint(reg) => {
                let value = frame.registers.get(*reg)?;
                println!("DEBUG: {:?}", value);
            }
            Instruction::Nop => {
                // Do nothing
            }
            Instruction::Halt => {
                return Err(Error::parsing("Execution halted".to_string()));
            }
            _ => {
                // Placeholder for other instructions
                return Err(Error::parsing(format!("Instruction not implemented: {:?}", instruction)));
            }
        }

        Ok(())
    }

    /// Get a constant from the constant pool
    fn get_constant(&self, index: ConstantIndex) -> Result<&Value> {
        self.constant_pool.get(index.0 as usize)
            .ok_or_else(|| Error::parsing(format!("Constant {} not found", index.0)))
    }

    /// Get label address
    fn get_label_address(&self, label: Label, frame: &CallFrame) -> Result<usize> {
        frame.function.bytecode.labels.get(&label)
            .copied()
            .ok_or_else(|| Error::parsing(format!("Label {} not found", label.0)))
    }

    /// Check if a value is truthy
    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Undefined | Value::Null => false,
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0 && !n.is_nan(),
            Value::String(s) => !s.is_empty(),
            Value::Object(_) | Value::Array(_) | Value::Function(_) | Value::Class(_) => true,
        }
    }

    /// Add two values
    fn add_values(&self, a: &Value, b: &Value) -> Result<Value> {
        match (a, b) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 + n2)),
            (Value::String(s1), Value::String(s2)) => Ok(Value::String(s1.clone() + s2)),
            (Value::String(s1), Value::Number(n2)) => Ok(Value::String(s1.clone() + &n2.to_string())),
            (Value::Number(n1), Value::String(s2)) => Ok(Value::String(n1.to_string() + s2)),
            _ => Ok(Value::Number(f64::NAN)),
        }
    }

    /// Subtract two values
    fn subtract_values(&self, a: &Value, b: &Value) -> Result<Value> {
        match (a, b) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 - n2)),
            _ => Ok(Value::Number(f64::NAN)),
        }
    }

    /// Multiply two values
    fn multiply_values(&self, a: &Value, b: &Value) -> Result<Value> {
        match (a, b) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 * n2)),
            _ => Ok(Value::Number(f64::NAN)),
        }
    }

    /// Divide two values
    fn divide_values(&self, a: &Value, b: &Value) -> Result<Value> {
        match (a, b) {
            (Value::Number(n1), Value::Number(n2)) => {
                if *n2 == 0.0 {
                    Ok(Value::Number(f64::INFINITY))
                } else {
                    Ok(Value::Number(n1 / n2))
                }
            }
            _ => Ok(Value::Number(f64::NAN)),
        }
    }

    /// Check if two values are equal
    fn equal_values(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Undefined, Value::Undefined) => true,
            (Value::Null, Value::Null) => true,
            (Value::Boolean(b1), Value::Boolean(b2)) => b1 == b2,
            (Value::Number(n1), Value::Number(n2)) => n1 == n2,
            (Value::String(s1), Value::String(s2)) => s1 == s2,
            _ => false,
        }
    }

    /// Call a function
    fn call_function(&self, target: &Value, args: Vec<Value>) -> Result<Value> {
        match target {
            Value::Function(func) => {
                // For now, return undefined
                Ok(Value::Undefined)
            }
            _ => Err(Error::parsing("Not a function".to_string())),
        }
    }

    /// Get the current call stack depth
    pub fn call_stack_depth(&self) -> usize {
        self.call_stack.len()
    }

    /// Get the global scope
    pub fn get_global_scope(&self) -> &HashMap<String, Value> {
        &self.global_scope
    }

    /// Set a global variable
    pub fn set_global(&mut self, name: String, value: Value) {
        self.global_scope.insert(name, value);
    }
}

/// Bytecode compiler
pub struct BytecodeCompiler {
    instructions: Vec<Instruction>,
    constants: Vec<Value>,
    labels: HashMap<Label, usize>,
    next_label: u32,
    next_register: u32,
}

impl BytecodeCompiler {
    /// Create a new bytecode compiler
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            labels: HashMap::new(),
            next_label: 0,
            next_register: 0,
        }
    }

    /// Add a constant to the constant pool
    pub fn add_constant(&mut self, value: Value) -> ConstantIndex {
        let index = self.constants.len() as u32;
        self.constants.push(value);
        ConstantIndex(index)
    }

    /// Create a new label
    pub fn create_label(&mut self) -> Label {
        let label = Label(self.next_label);
        self.next_label += 1;
        label
    }

    /// Bind a label to the current instruction
    pub fn bind_label(&mut self, label: Label) {
        self.labels.insert(label, self.instructions.len());
    }

    /// Allocate a new register
    pub fn allocate_register(&mut self) -> Register {
        let reg = Register(self.next_register);
        self.next_register += 1;
        reg
    }

    /// Add an instruction
    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    /// Compile a simple expression
    pub fn compile_expression(&mut self, expr: &crate::ast::Expression) -> Result<Register> {
        match expr {
            crate::ast::Expression::Literal(literal) => {
                let reg = self.allocate_register();
                match literal {
                    crate::ast::Literal::String(s) => {
                        let const_idx = self.add_constant(Value::String(s.clone()));
                        self.add_instruction(Instruction::LoadConstant(reg, const_idx));
                    }
                    crate::ast::Literal::Number(n) => {
                        let const_idx = self.add_constant(Value::Number(*n));
                        self.add_instruction(Instruction::LoadConstant(reg, const_idx));
                    }
                    crate::ast::Literal::Boolean(true) => {
                        self.add_instruction(Instruction::LoadTrue(reg));
                    }
                    crate::ast::Literal::Boolean(false) => {
                        self.add_instruction(Instruction::LoadFalse(reg));
                    }
                    crate::ast::Literal::Null => {
                        self.add_instruction(Instruction::LoadNull(reg));
                    }
                    _ => {
                        self.add_instruction(Instruction::LoadUndefined(reg));
                    }
                }
                Ok(reg)
            }
            crate::ast::Expression::Identifier(ident) => {
                let reg = self.allocate_register();
                self.add_instruction(Instruction::LoadGlobal(reg, ident.name.clone()));
                Ok(reg)
            }
            crate::ast::Expression::Binary(binary) => {
                let left_reg = self.compile_expression(&binary.left)?;
                let right_reg = self.compile_expression(&binary.right)?;
                let result_reg = self.allocate_register();

                match binary.operator {
                    crate::ast::BinaryOperator::Add => {
                        self.add_instruction(Instruction::Add(left_reg, right_reg, result_reg));
                    }
                    crate::ast::BinaryOperator::Subtract => {
                        self.add_instruction(Instruction::Subtract(left_reg, right_reg, result_reg));
                    }
                    crate::ast::BinaryOperator::Multiply => {
                        self.add_instruction(Instruction::Multiply(left_reg, right_reg, result_reg));
                    }
                    crate::ast::BinaryOperator::Divide => {
                        self.add_instruction(Instruction::Divide(left_reg, right_reg, result_reg));
                    }
                    crate::ast::BinaryOperator::Equal => {
                        self.add_instruction(Instruction::Equal(left_reg, right_reg, result_reg));
                    }
                    crate::ast::BinaryOperator::NotEqual => {
                        self.add_instruction(Instruction::NotEqual(left_reg, right_reg, result_reg));
                    }
                    _ => {
                        return Err(Error::parsing("Unsupported binary operator".to_string()));
                    }
                }

                Ok(result_reg)
            }
            _ => {
                let reg = self.allocate_register();
                self.add_instruction(Instruction::LoadUndefined(reg));
                Ok(reg)
            }
        }
    }

    /// Build the bytecode function
    pub fn build(self) -> BytecodeFunction {
        BytecodeFunction {
            instructions: self.instructions,
            constants: self.constants,
            labels: self.labels,
            source_map: None,
        }
    }
}
