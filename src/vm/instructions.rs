pub type Parameters = Vec<Param>;
type Handler = fn(Parameters) -> InstructionAction;

pub struct Instruction {
    pub num_params: usize,
    pub handler: Handler,
}

impl Instruction {
    fn new(num_params: usize, handler: Handler) -> Self {
        Self {
            num_params,
            handler,
        }
    }
}

pub enum Param {
    Positional(usize, i64),
    Immediate(i64),
}

impl Param {
    fn position(&self) -> usize {
        match self {
            Param::Positional(position, _) => *position,
            Param::Immediate(_) => panic!("immediate values have no position"),
        }
    }

    fn value(&self) -> i64 {
        match self {
            Param::Positional(_, value) => *value,
            Param::Immediate(val) => *val,
        }
    }
}

fn math(params: Parameters, op: fn(i64, i64) -> i64) -> InstructionAction {
    InstructionAction::Store(
        params[2].position(),
        op(params[0].value(), params[1].value()),
    )
}

fn add(params: Parameters) -> InstructionAction {
    math(params, |a, b| a + b)
}

fn mul(params: Parameters) -> InstructionAction {
    math(params, |a, b| a * b)
}

fn read(params: Parameters) -> InstructionAction {
    InstructionAction::Read(params[0].position())
}

fn write(params: Parameters) -> InstructionAction {
    InstructionAction::Write(params[0].value())
}

fn jnz(params: Parameters) -> InstructionAction {
    if params[0].value() != 0 {
        InstructionAction::Jump(params[1].value() as usize)
    } else {
        InstructionAction::Noop
    }
}

fn jz(params: Parameters) -> InstructionAction {
    if params[0].value() == 0 {
        InstructionAction::Jump(params[1].value() as usize)
    } else {
        InstructionAction::Noop
    }
}

fn lt(params: Parameters) -> InstructionAction {
    InstructionAction::Store(
        params[2].position(),
        if params[0].value() < params[1].value() {
            1
        } else {
            0
        },
    )
}

fn eq(params: Parameters) -> InstructionAction {
    InstructionAction::Store(
        params[2].position(),
        if params[0].value() == params[1].value() {
            1
        } else {
            0
        },
    )
}

fn halt(_params: Parameters) -> InstructionAction {
    InstructionAction::Halt
}

pub enum InstructionAction {
    Store(usize, i64),
    Read(usize),
    Write(i64),
    Jump(usize),
    Noop,
    Halt,
}

pub fn get_instruction(opcode: i64) -> Instruction {
    match opcode % 100 {
        1 => Instruction::new(3, add),
        2 => Instruction::new(3, mul),
        3 => Instruction::new(1, read),
        4 => Instruction::new(1, write),
        5 => Instruction::new(2, jnz),
        6 => Instruction::new(2, jz),
        7 => Instruction::new(3, lt),
        8 => Instruction::new(3, eq),
        99 => Instruction::new(0, halt),
        n => panic!("Unsupported opcode {} ({})", n, opcode),
    }
}
