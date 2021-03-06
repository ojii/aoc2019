use crate::vm::instructions::*;
use crate::vm::io::IO;
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
pub struct Memory {
    data: Vec<i64>,
}

impl Memory {
    pub fn new(data: Vec<i64>) -> Self {
        Self { data }
    }
}

impl From<&str> for Memory {
    fn from(program: &str) -> Memory {
        Memory::new(
            program
                .split(',')
                .flat_map(|n| n.parse::<i64>().ok())
                .collect(),
        )
    }
}

impl Index<usize> for Memory {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        self.data.get(index).unwrap_or(&0)
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.data.len() {
            self.data.resize(index + 1, 0);
        }
        self.data.index_mut(index)
    }
}

enum StepResult {
    Continue(usize, usize),
    Halt,
}

pub fn run<I: IO>(mut memory: Memory, mut io: I) -> (Memory, I::Value) {
    let mut pc = 0;
    let mut rb = 0;
    let mut counter = 0;
    'main: loop {
        counter += 1;
        match step(pc, rb, &mut memory, &mut io) {
            StepResult::Continue(at, new_rb) => {
                pc = at;
                rb = new_rb;
            }
            StepResult::Halt => break 'main,
        }
    }
    (memory, io.output())
}

fn step<I: IO>(pc: usize, rb: usize, memory: &mut Memory, io: &mut I) -> StepResult {
    let opcode = memory[pc];
    let instruction = get_instruction(opcode);
    let mut params: Parameters = Vec::with_capacity(instruction.num_params);
    for index in 0..instruction.num_params {
        let value = memory[pc + 1 + index];
        let mode = (opcode / (10 * (10i64.pow((index + 1) as u32)))) % 10;
        match mode {
            0 => params.push({
                let position = value as usize;
                let value = memory[position];
                Param::Positional(position, value)
            }),
            1 => params.push(Param::Immediate(value)),
            2 => params.push({
                let position = (value + (rb as i64)) as usize;
                let value = memory[position];
                Param::Relative(position, value)
            }),
            n => panic!(
                "invalid param mode {} for param {} in opcode {}",
                n, index, opcode
            ),
        }
    }
    let pc = pc + instruction.num_params + 1;
    match (instruction.handler)(params) {
        InstructionAction::Store(position, value) => {
            memory[position] = value;
            StepResult::Continue(pc, rb)
        }
        InstructionAction::Read(position) => match io.read() {
            Some(value) => {
                memory[position] = value;
                StepResult::Continue(pc, rb)
            }
            None => StepResult::Halt,
        },
        InstructionAction::Write(value) => match io.write(value) {
            Some(_) => StepResult::Continue(pc, rb),
            None => StepResult::Halt,
        },
        InstructionAction::Jump(to) => StepResult::Continue(to, rb),
        InstructionAction::Noop => StepResult::Continue(pc, rb),
        InstructionAction::ChangeRelativeBase(by) => {
            StepResult::Continue(pc, ((rb as i64) + by) as usize)
        }
        InstructionAction::Halt => StepResult::Halt,
    }
}
