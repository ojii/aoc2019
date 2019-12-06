use crate::vm::*;

fn math<I: Input, O: Output>(
    vm: &mut VM<I, O>,
    params: Vec<Value>,
    op: fn(i64, i64) -> i64,
) -> InstructionResult {
    let a = params[0].resolve(vm);
    let b = params[1].resolve(vm);
    match params[2] {
        Value::Positional(index) => {
            vm.store(index, op(a, b));
            InstructionResult::Continue
        }
        Value::Immediate(_) => {
            InstructionResult::Error("Cannot write to immediate value".to_string())
        }
    }
}

fn add<I: Input, O: Output>(vm: &mut VM<I, O>, params: Vec<Value>) -> InstructionResult {
    math(vm, params, |a, b| a + b)
}

fn mul<I: Input, O: Output>(vm: &mut VM<I, O>, params: Vec<Value>) -> InstructionResult {
    math(vm, params, |a, b| a * b)
}

fn read<I: Input, O: Output>(vm: &mut VM<I, O>, params: Vec<Value>) -> InstructionResult {
    match vm.read() {
        IOResult::Ok(value) => match params[0] {
            Value::Positional(index) => {
                vm.store(index, value);
                InstructionResult::Continue
            }
            Value::Immediate(_) => {
                InstructionResult::Error("Cannot write to immediate value".to_string())
            }
        },
        IOResult::Error(reason) => InstructionResult::Error(reason),
    }
}

fn write<I: Input, O: Output>(vm: &mut VM<I, O>, params: Vec<Value>) -> InstructionResult {
    let value = params[0].resolve(vm);
    match vm.write(value) {
        IOResult::Ok(_) => InstructionResult::Continue,
        IOResult::Error(reason) => InstructionResult::Error(reason),
    }
}

fn jnz<I: Input, O: Output>(vm: &mut VM<I, O>, params: Vec<Value>) -> InstructionResult {
    if params[0].resolve(vm) != 0 {
        vm.pc = params[1].resolve(vm) as usize;
    }
    InstructionResult::Continue
}

fn jz<I: Input, O: Output>(vm: &mut VM<I, O>, params: Vec<Value>) -> InstructionResult {
    if params[0].resolve(vm) == 0 {
        vm.pc = params[1].resolve(vm) as usize;
    }
    InstructionResult::Continue
}

fn lt<I: Input, O: Output>(vm: &mut VM<I, O>, params: Vec<Value>) -> InstructionResult {
    let result = if params[0].resolve(vm) < params[1].resolve(vm) {
        1
    } else {
        0
    };
    match params[2] {
        Value::Positional(index) => {
            vm.store(index, result);
            InstructionResult::Continue
        }
        Value::Immediate(_) => {
            InstructionResult::Error("Cannot write to immediate value".to_string())
        }
    }
}

fn eq<I: Input, O: Output>(vm: &mut VM<I, O>, params: Vec<Value>) -> InstructionResult {
    let result = if params[0].resolve(vm) == params[1].resolve(vm) {
        1
    } else {
        0
    };
    match params[2] {
        Value::Positional(index) => {
            vm.store(index, result);
            InstructionResult::Continue
        }
        Value::Immediate(_) => {
            InstructionResult::Error("Cannot write to immediate value".to_string())
        }
    }
}

fn halt<I: Input, O: Output>(_vm: &mut VM<I, O>, _params: Vec<Value>) -> InstructionResult {
    InstructionResult::Halt
}

pub fn get_default<I: Input, O: Output>() -> Vec<Instruction<I, O>> {
    vec![
        Instruction::new(1, 3, add),
        Instruction::new(2, 3, mul),
        Instruction::new(3, 1, read),
        Instruction::new(4, 1, write),
        Instruction::new(5, 2, jnz),
        Instruction::new(6, 2, jz),
        Instruction::new(7, 3, lt),
        Instruction::new(8, 3, eq),
        Instruction::new(99, 0, halt),
    ]
}
