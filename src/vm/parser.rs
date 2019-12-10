use crate::vm::machine::Memory;

pub fn parse_program(program: &str) -> Memory {
    program
        .split(',')
        .flat_map(|n| n.parse::<i64>().ok())
        .collect()
}
