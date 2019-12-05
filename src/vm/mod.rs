use std::collections::HashMap;

mod io;

pub use io::*;
use std::fmt::Error;

enum Value {
    Positional(usize),
    Immediate(i64),
}

impl Value {
    fn resolve<I: Input, O: Output>(&self, vm: &VM<I, O>) -> i64 {
        match self {
            Value::Positional(index) => vm.access(*index),
            Value::Immediate(value) => *value,
        }
    }
}

pub enum InstructionResult {
    Continue,
    Halt,
    Error(String),
}

type Handler<I: Input, O: Output> = fn(&mut VM<I, O>, Vec<Value>) -> InstructionResult;

#[derive(Clone)]
struct Instruction<I: Input, O: Output> {
    opcode: i64,
    num_params: usize,
    handler: Handler<I, O>,
}

impl<I: Input, O: Output> Instruction<I, O> {
    fn new(opcode: i64, num_params: usize, handler: Handler<I, O>) -> Instruction<I, O> {
        Instruction {
            opcode,
            num_params,
            handler,
        }
    }
}

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

fn get_default_instructions<I: Input, O: Output>() -> Vec<Instruction<I, O>> {
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

#[derive(Clone)]
pub struct VM<I: Input, O: Output> {
    input: I,
    output: O,
    instructions: HashMap<i64, Instruction<I, O>>,
    memory: Vec<i64>,
    pc: usize,
}

impl<I: Input, O: Output> VM<I, O> {
    pub fn new(memory: Vec<i64>, input: I, output: O) -> VM<I, O> {
        let instruction_set = get_default_instructions();
        let mut instructions = HashMap::with_capacity(instruction_set.len());
        for instruction in instruction_set {
            instructions.insert(instruction.opcode, instruction);
        }
        VM {
            input,
            output,
            instructions,
            memory,
            pc: 0,
        }
    }

    pub fn run(&mut self) -> &O {
        'main: loop {
            match self.step() {
                InstructionResult::Continue => (),
                InstructionResult::Halt => break 'main,
                InstructionResult::Error(e) => panic!("ERROR {}", e),
            };
        }
        &self.output
    }

    pub fn step(&mut self) -> InstructionResult {
        let opcode = self.access(self.pc) % 100;
        let instruction = self.instructions.get(&opcode);
        if instruction.is_none() {
            return InstructionResult::Error(format!(
                "invalid opcode {} ({}) at {}",
                self.access(self.pc),
                opcode,
                self.pc
            ));
        }
        let instruction = instruction.unwrap().clone();
        let opcode = self.consume();
        let mut params = Vec::with_capacity(instruction.num_params);
        for index in 0..instruction.num_params {
            let value = self.consume();
            let mode = (opcode / (10 * (10i64.pow((index + 1) as u32)))) % 10;
            match mode {
                0 => params.push(Value::Positional(value as usize)),
                1 => params.push(Value::Immediate(value)),
                n => {
                    return InstructionResult::Error(format!(
                        "invalid param mode {} for param {} in opcode {}",
                        n, index, opcode
                    ));
                }
            }
        }
        (instruction.handler)(self, params)
    }

    pub fn consume(&mut self) -> i64 {
        let result = self.access(self.pc);
        self.pc += 1;
        result
    }

    pub fn access(&self, index: usize) -> i64 {
        self.memory[index]
    }

    pub fn store(&mut self, index: usize, value: i64) {
        self.memory[index] = value
    }

    pub fn read(&mut self) -> IOResult<i64> {
        self.input.read()
    }

    pub fn write(&mut self, value: i64) -> IOResult<()> {
        self.output.write(value)
    }
}

pub fn parse_program(program: &str) -> Vec<i64> {
    program
        .split(",")
        .flat_map(|n| n.parse::<i64>().ok())
        .collect()
}
