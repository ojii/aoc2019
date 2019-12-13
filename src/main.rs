#![feature(drain_filter)]
#![feature(tau_constant)]

#[macro_use]
extern crate itertools;
extern crate rayon;
extern crate regex;
extern crate threadpool;

mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
pub mod render;
pub mod vm;

const DAYS: [fn(); 12] = [
    day1::main,
    day2::main,
    day3::main,
    day4::main,
    day5::main,
    day6::main,
    day7::main,
    day8::main,
    day9::main,
    day10::main,
    day11::main,
    day12::main,
];

fn main() {
    let day = std::env::args()
        .nth(1)
        .map(|arg| arg.parse::<usize>().ok())
        .flatten()
        .map(|index| index.checked_sub(1))
        .flatten()
        .map(|index| DAYS.get(index))
        .unwrap_or_else(|| DAYS.last())
        .unwrap();
    day();
}
