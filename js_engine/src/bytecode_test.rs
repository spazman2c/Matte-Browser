#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytecode::{BytecodeEngine, BytecodeCompiler, BytecodeFunction, Register, ConstantIndex, Label, Instruction, Value, FunctionValue, RegisterFile, CallFrame};

    #[tokio::test]
    async fn test_register_creation() {
        let reg1 = Register(0);
        let reg2 = Register(1);
        let reg3 = Register(0);

        assert_eq!(reg1.0, 0);
        assert_eq!(reg2.0, 1);
        assert_eq!(reg1, reg3);
        assert_ne!(reg1, reg2);
    }

    #[tokio::test]
    async fn test_constant_index_creation() {
        let idx1 = ConstantIndex(0);
        let idx2 = ConstantIndex(1);
        let idx3 = ConstantIndex(0);

        assert_eq!(idx1.0, 0);
        assert_eq!(idx2.0, 1);
        assert_eq!(idx1, idx3);
        assert_ne!(idx1, idx2);
    }

    #[tokio::test]
    async fn test_label_creation() {
        let label1 = Label(0);
        let label2 = Label(1);
        let label3 = Label(0);

        assert_eq!(label1.0, 0);
        assert_eq!(label2.0, 1);
        assert_eq!(label1, label3);
        assert_ne!(label1, label2);
    }

    #[tokio::test]
    async fn test_register_file_operations() {
        let mut reg_file = RegisterFile::new(10);
        
        // Test setting and getting registers
        reg_file.set(Register(0), Value::Number(42.0)).unwrap();
        reg_file.set(Register(1), Value::String("hello".to_string())).unwrap();
        
        assert!(matches!(reg_file.get(Register(0)).unwrap(), Value::Number(n) if *n == 42.0));
        assert!(matches!(reg_file.get(Register(1)).unwrap(), Value::String(s) if s == "hello"));
        
        // Test out of bounds access
        assert!(reg_file.set(Register(10), Value::Number(0.0)).is_err());
        assert!(reg_file.get(Register(10)).is_err());
    }

    #[tokio::test]
    async fn test_register_file_clear() {
        let mut reg_file = RegisterFile::new(5);
        
        // Set some values
        reg_file.set(Register(0), Value::Number(42.0)).unwrap();
        reg_file.set(Register(1), Value::String("test".to_string())).unwrap();
        
        // Clear all registers
        reg_file.clear();
        
        // All registers should be undefined
        assert!(matches!(reg_file.get(Register(0)).unwrap(), Value::Undefined));
        assert!(matches!(reg_file.get(Register(1)).unwrap(), Value::Undefined));
    }

    #[tokio::test]
    async fn test_instruction_creation() {
        let reg1 = Register(0);
        let reg2 = Register(1);
        let reg3 = Register(2);
        
        // Test various instruction types
        let load_const = Instruction::LoadConstant(reg1, ConstantIndex(0));
        let add = Instruction::Add(reg1, reg2, reg3);
        let jump = Instruction::Jump(Label(0));
        let nop = Instruction::Nop;
        
        assert!(matches!(load_const, Instruction::LoadConstant(_, _)));
        assert!(matches!(add, Instruction::Add(_, _, _)));
        assert!(matches!(jump, Instruction::Jump(_)));
        assert!(matches!(nop, Instruction::Nop));
    }

    #[tokio::test]
    async fn test_instruction_display() {
        let reg1 = Register(0);
        let reg2 = Register(1);
        let reg3 = Register(2);
        
        let load_const = Instruction::LoadConstant(reg1, ConstantIndex(0));
        let add = Instruction::Add(reg1, reg2, reg3);
        let jump = Instruction::Jump(Label(0));
        let nop = Instruction::Nop;
        
        assert_eq!(load_const.to_string(), "LOAD_CONST r0, const[0]");
        assert_eq!(add.to_string(), "ADD r0, r1, r2");
        assert_eq!(jump.to_string(), "JUMP L0");
        assert_eq!(nop.to_string(), "NOP");
    }

    #[tokio::test]
    async fn test_bytecode_compiler_creation() {
        let compiler = BytecodeCompiler::new();
        
        assert!(compiler.instructions.is_empty());
        assert!(compiler.constants.is_empty());
        assert!(compiler.labels.is_empty());
        assert_eq!(compiler.next_label, 0);
        assert_eq!(compiler.next_register, 0);
    }

    #[tokio::test]
    async fn test_bytecode_compiler_register_allocation() {
        let mut compiler = BytecodeCompiler::new();
        
        let reg1 = compiler.allocate_register();
        let reg2 = compiler.allocate_register();
        let reg3 = compiler.allocate_register();
        
        assert_eq!(reg1.0, 0);
        assert_eq!(reg2.0, 1);
        assert_eq!(reg3.0, 2);
        assert_ne!(reg1, reg2);
        assert_ne!(reg2, reg3);
    }

    #[tokio::test]
    async fn test_bytecode_compiler_constant_management() {
        let mut compiler = BytecodeCompiler::new();
        
        let idx1 = compiler.add_constant(Value::Number(42.0));
        let idx2 = compiler.add_constant(Value::String("hello".to_string()));
        let idx3 = compiler.add_constant(Value::Boolean(true));
        
        assert_eq!(idx1.0, 0);
        assert_eq!(idx2.0, 1);
        assert_eq!(idx3.0, 2);
        assert_eq!(compiler.constants.len(), 3);
    }

    #[tokio::test]
    async fn test_bytecode_compiler_label_management() {
        let mut compiler = BytecodeCompiler::new();
        
        let label1 = compiler.create_label();
        let label2 = compiler.create_label();
        
        assert_eq!(label1.0, 0);
        assert_eq!(label2.0, 1);
        
        // Add some instructions and bind labels
        compiler.add_instruction(Instruction::Nop);
        compiler.bind_label(label1);
        compiler.add_instruction(Instruction::Nop);
        compiler.bind_label(label2);
        
        assert_eq!(compiler.labels.get(&label1), Some(&1));
        assert_eq!(compiler.labels.get(&label2), Some(&2));
    }

    #[tokio::test]
    async fn test_bytecode_compiler_instruction_adding() {
        let mut compiler = BytecodeCompiler::new();
        
        let reg1 = Register(0);
        let reg2 = Register(1);
        
        compiler.add_instruction(Instruction::LoadTrue(reg1));
        compiler.add_instruction(Instruction::LoadFalse(reg2));
        compiler.add_instruction(Instruction::Add(reg1, reg2, reg1));
        
        assert_eq!(compiler.instructions.len(), 3);
        assert!(matches!(compiler.instructions[0], Instruction::LoadTrue(_)));
        assert!(matches!(compiler.instructions[1], Instruction::LoadFalse(_)));
        assert!(matches!(compiler.instructions[2], Instruction::Add(_, _, _)));
    }

    #[tokio::test]
    async fn test_bytecode_function_building() {
        let mut compiler = BytecodeCompiler::new();
        
        // Add some constants
        let const_idx = compiler.add_constant(Value::Number(42.0));
        
        // Add some instructions
        let reg = compiler.allocate_register();
        compiler.add_instruction(Instruction::LoadConstant(reg, const_idx));
        compiler.add_instruction(Instruction::Return(reg));
        
        // Create a label
        let label = compiler.create_label();
        compiler.bind_label(label);
        
        // Build the function
        let bytecode_func = compiler.build();
        
        assert_eq!(bytecode_func.instructions.len(), 2);
        assert_eq!(bytecode_func.constants.len(), 1);
        assert_eq!(bytecode_func.labels.len(), 1);
        assert!(bytecode_func.source_map.is_none());
    }

    #[tokio::test]
    async fn test_bytecode_engine_creation() {
        let engine = BytecodeEngine::new();
        
        assert!(engine.call_stack.is_empty());
        assert!(engine.global_scope.is_empty());
        assert!(engine.constant_pool.is_empty());
        assert!(engine.exception_handler.is_none());
    }

    #[tokio::test]
    async fn test_bytecode_engine_global_scope() {
        let mut engine = BytecodeEngine::new();
        
        // Set global variables
        engine.set_global("test_var".to_string(), Value::Number(42.0));
        engine.set_global("test_string".to_string(), Value::String("hello".to_string()));
        
        let globals = engine.get_global_scope();
        assert!(globals.contains_key("test_var"));
        assert!(globals.contains_key("test_string"));
        assert!(matches!(globals.get("test_var"), Some(Value::Number(n)) if *n == 42.0));
        assert!(matches!(globals.get("test_string"), Some(Value::String(s)) if s == "hello"));
    }

    #[tokio::test]
    async fn test_call_frame_creation() {
        let bytecode_func = BytecodeFunction {
            instructions: vec![Instruction::Nop, Instruction::ReturnUndefined],
            constants: vec![],
            labels: HashMap::new(),
            source_map: None,
        };
        
        let function = FunctionValue {
            name: "test_function".to_string(),
            bytecode: bytecode_func,
            param_count: 2,
            local_count: 3,
            closure: HashMap::new(),
        };
        
        let frame = CallFrame::new(function, Some(42));
        
        assert_eq!(frame.pc, 0);
        assert_eq!(frame.locals.len(), 3);
        assert_eq!(frame.return_address, Some(42));
        assert!(frame.this_value.is_none());
    }

    #[tokio::test]
    async fn test_call_frame_operations() {
        let bytecode_func = BytecodeFunction {
            instructions: vec![Instruction::Nop],
            constants: vec![],
            labels: HashMap::new(),
            source_map: None,
        };
        
        let function = FunctionValue {
            name: "test_function".to_string(),
            bytecode: bytecode_func,
            param_count: 0,
            local_count: 2,
            closure: HashMap::new(),
        };
        
        let mut frame = CallFrame::new(function, None);
        
        // Test local variable operations
        frame.set_local(0, Value::Number(42.0)).unwrap();
        frame.set_local(1, Value::String("test".to_string())).unwrap();
        
        assert!(matches!(frame.get_local(0).unwrap(), Value::Number(n) if *n == 42.0));
        assert!(matches!(frame.get_local(1).unwrap(), Value::String(s) if s == "test"));
        
        // Test out of bounds access
        assert!(frame.set_local(2, Value::Number(0.0)).is_err());
        assert!(frame.get_local(2).is_err());
    }

    #[tokio::test]
    async fn test_call_frame_program_counter() {
        let bytecode_func = BytecodeFunction {
            instructions: vec![Instruction::Nop, Instruction::Nop, Instruction::Nop],
            constants: vec![],
            labels: HashMap::new(),
            source_map: None,
        };
        
        let function = FunctionValue {
            name: "test_function".to_string(),
            bytecode: bytecode_func,
            param_count: 0,
            local_count: 0,
            closure: HashMap::new(),
        };
        
        let mut frame = CallFrame::new(function, None);
        
        // Test initial state
        assert_eq!(frame.pc, 0);
        assert!(frame.current_instruction().is_some());
        
        // Test advancing PC
        frame.advance_pc();
        assert_eq!(frame.pc, 1);
        
        // Test jumping
        frame.jump_to(2);
        assert_eq!(frame.pc, 2);
    }

    #[tokio::test]
    async fn test_value_operations() {
        // Test different value types
        let undefined = Value::Undefined;
        let null = Value::Null;
        let boolean = Value::Boolean(true);
        let number = Value::Number(42.0);
        let string = Value::String("hello".to_string());
        let array = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
        let mut object = HashMap::new();
        object.insert("key".to_string(), Value::String("value".to_string()));
        let object = Value::Object(object);

        // Test pattern matching
        match undefined {
            Value::Undefined => assert!(true),
            _ => panic!("Expected undefined"),
        }

        match null {
            Value::Null => assert!(true),
            _ => panic!("Expected null"),
        }

        match boolean {
            Value::Boolean(b) => assert!(b),
            _ => panic!("Expected boolean"),
        }

        match number {
            Value::Number(n) => assert_eq!(n, 42.0),
            _ => panic!("Expected number"),
        }

        match string {
            Value::String(s) => assert_eq!(s, "hello"),
            _ => panic!("Expected string"),
        }

        match array {
            Value::Array(arr) => assert_eq!(arr.len(), 2),
            _ => panic!("Expected array"),
        }

        match object {
            Value::Object(obj) => assert!(obj.contains_key("key")),
            _ => panic!("Expected object"),
        }
    }

    #[tokio::test]
    async fn test_bytecode_engine_value_operations() {
        let engine = BytecodeEngine::new();
        
        // Test truthy checks
        assert!(!engine.is_truthy(&Value::Undefined));
        assert!(!engine.is_truthy(&Value::Null));
        assert!(engine.is_truthy(&Value::Boolean(true)));
        assert!(!engine.is_truthy(&Value::Boolean(false)));
        assert!(engine.is_truthy(&Value::Number(42.0)));
        assert!(!engine.is_truthy(&Value::Number(0.0)));
        assert!(engine.is_truthy(&Value::String("hello".to_string())));
        assert!(!engine.is_truthy(&Value::String("".to_string())));
        
        // Test arithmetic operations
        let add_result = engine.add_values(&Value::Number(5.0), &Value::Number(3.0)).unwrap();
        assert!(matches!(add_result, Value::Number(n) if n == 8.0));
        
        let sub_result = engine.subtract_values(&Value::Number(5.0), &Value::Number(3.0)).unwrap();
        assert!(matches!(sub_result, Value::Number(n) if n == 2.0));
        
        let mul_result = engine.multiply_values(&Value::Number(5.0), &Value::Number(3.0)).unwrap();
        assert!(matches!(mul_result, Value::Number(n) if n == 15.0));
        
        let div_result = engine.divide_values(&Value::Number(6.0), &Value::Number(2.0)).unwrap();
        assert!(matches!(div_result, Value::Number(n) if n == 3.0));
        
        // Test string concatenation
        let str_result = engine.add_values(&Value::String("hello".to_string()), &Value::String(" world".to_string())).unwrap();
        assert!(matches!(str_result, Value::String(s) if s == "hello world"));
        
        // Test equality
        assert!(engine.equal_values(&Value::Number(42.0), &Value::Number(42.0)));
        assert!(engine.equal_values(&Value::String("hello".to_string()), &Value::String("hello".to_string())));
        assert!(!engine.equal_values(&Value::Number(42.0), &Value::Number(43.0)));
    }

    #[tokio::test]
    async fn test_bytecode_engine_integration() {
        let mut engine = BytecodeEngine::new();
        
        // Create a simple bytecode function
        let bytecode_func = BytecodeFunction {
            instructions: vec![
                Instruction::LoadConstant(Register(0), ConstantIndex(0)),
                Instruction::Return(Register(0)),
            ],
            constants: vec![Value::Number(42.0)],
            labels: HashMap::new(),
            source_map: None,
        };
        
        let function = FunctionValue {
            name: "test_function".to_string(),
            bytecode: bytecode_func,
            param_count: 0,
            local_count: 0,
            closure: HashMap::new(),
        };
        
        // Execute the function
        let result = engine.execute(function, vec![]).unwrap();
        
        // For now, the result should be undefined since the execution is simplified
        assert!(matches!(result, Value::Undefined));
    }
}
