#[macro_use]
extern crate itertools;
extern crate rayon;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
pub mod vm;

const DAYS: [fn(); 6] = [
    day1::main,
    day2::main,
    day3::main,
    day4::main,
    day5::main,
    day6::main,
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
