use crate::vm::instructions::*;
use crate::vm::io::{Input, Output};

pub type Memory = Vec<i64>;

enum StepResult {
    Continue(usize),
    Halt,
}

pub fn run<I: Input, O: Output>(mut memory: Memory, mut input: I, mut output: O) -> (Memory, O) {
    let mut pc = 0;
    'main: loop {
        match step(pc, &mut memory, &mut input, &mut output) {
            StepResult::Continue(at) => pc = at,
            StepResult::Halt => break 'main,
        }
    }
    (memory, output)
}

fn step<I: Input, O: Output>(
    pc: usize,
    memory: &mut Memory,
    input: &mut I,
    output: &mut O,
) -> StepResult {
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
            StepResult::Continue(pc)
        }
        InstructionAction::Read(position) => {
            memory[position] = input.read().unwrap();
            StepResult::Continue(pc)
        }
        InstructionAction::Write(value) => {
            output.write(value).unwrap();
            StepResult::Continue(pc)
        }
        InstructionAction::Jump(to) => StepResult::Continue(to),
        InstructionAction::Noop => StepResult::Continue(pc),
        InstructionAction::Halt => StepResult::Halt,
    }
}
