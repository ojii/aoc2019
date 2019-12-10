use rayon::prelude::*;

use crate::vm::{parse_program, run, NullIO};

pub fn main() {
    let memory = parse_program(INPUT);

    let mut first = memory.clone();
    first[1] = 12;
    first[2] = 2;
    let (first, _) = run(first, NullIO::new(), NullIO::new());
    println!("{}", first[0]);

    let value = iproduct!(0..=99, 0..=99)
        .collect::<Vec<(i64, i64)>>()
        .par_iter()
        .map(|(noun, verb)| {
            let mut program = memory.clone();
            program[1] = *noun;
            program[2] = *verb;
            let (memory, _) = run(program, NullIO::new(), NullIO::new());
            (*noun, *verb, memory[0])
        })
        .find_any(|(_, _, zero)| *zero == 19_690_720)
        .map(|(noun, verb, _)| (noun * 100) + verb)
        .unwrap();
    println!("{}", value);
}

const INPUT: &str = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,10,1,19,1,19,6,23,2,13,23,27,1,27,13,31,1,9,31,35,1,35,9,39,1,39,5,43,2,6,43,47,1,47,6,51,2,51,9,55,2,55,13,59,1,59,6,63,1,10,63,67,2,67,9,71,2,6,71,75,1,75,5,79,2,79,10,83,1,5,83,87,2,9,87,91,1,5,91,95,2,13,95,99,1,99,10,103,1,103,2,107,1,107,6,0,99,2,14,0,0";
