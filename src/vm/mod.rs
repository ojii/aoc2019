use std::collections::HashMap;

mod instructions;
mod io;

pub use io::*;

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

type Handler<I, O> = fn(&mut VM<I, O>, Vec<Value>) -> InstructionResult;

#[derive(Clone)]
pub struct Instruction<I: Input, O: Output> {
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
        let instruction_set = instructions::get_default();
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
        .split(',')
        .flat_map(|n| n.parse::<i64>().ok())
        .collect()
}
