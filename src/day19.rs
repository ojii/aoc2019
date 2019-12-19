use crate::vm::{run, InputOutput};
use itertools::Itertools;
use std::collections::VecDeque;

fn scan(x: i64, y: i64) -> bool {
    run(
        INPUT.into(),
        InputOutput::new(VecDeque::from(vec![x, y]), 0i64),
    )
    .1 == 1
}

enum CheckResult {
    MoveRight,
    MissY,
    MissX,
    Hit,
}

fn check(x: i64, y: i64) -> CheckResult {
    if !scan(x, y) {
        CheckResult::MoveRight
    } else if !scan(x + 99, y) {
        CheckResult::MissX
    } else if !scan(x, y + 99) {
        CheckResult::MissY
    } else {
        CheckResult::Hit
    }
}

fn search() -> (i64, i64) {
    let mut x = 0;
    let mut y = 50;
    loop {
        match check(x, y) {
            CheckResult::MoveRight => {
                x += 1;
            }
            CheckResult::MissX => {
                y += 1;
            }
            CheckResult::MissY => {
                x += 1;
            }
            CheckResult::Hit => {
                return (x, y);
            }
        };
    }
}

pub fn main() {
    let affected = iproduct!(0i64..50, 0i64..50)
        .filter(|&(x, y)| scan(x, y))
        .count();
    println!("{}", affected);
    let (x, y) = search();
    println!("{}", (x * 10_000) + y);
}

const INPUT: &str  = "109,424,203,1,21101,0,11,0,1105,1,282,21101,18,0,0,1105,1,259,2101,0,1,221,203,1,21102,1,31,0,1105,1,282,21101,0,38,0,1106,0,259,21002,23,1,2,22101,0,1,3,21101,0,1,1,21102,1,57,0,1106,0,303,2102,1,1,222,21001,221,0,3,20102,1,221,2,21101,0,259,1,21101,80,0,0,1106,0,225,21101,0,23,2,21102,91,1,0,1106,0,303,1201,1,0,223,20101,0,222,4,21101,0,259,3,21102,1,225,2,21102,1,225,1,21102,1,118,0,1105,1,225,20102,1,222,3,21101,0,87,2,21101,133,0,0,1106,0,303,21202,1,-1,1,22001,223,1,1,21101,0,148,0,1105,1,259,2101,0,1,223,20102,1,221,4,21002,222,1,3,21101,0,9,2,1001,132,-2,224,1002,224,2,224,1001,224,3,224,1002,132,-1,132,1,224,132,224,21001,224,1,1,21102,1,195,0,106,0,109,20207,1,223,2,21001,23,0,1,21102,1,-1,3,21101,0,214,0,1106,0,303,22101,1,1,1,204,1,99,0,0,0,0,109,5,2102,1,-4,249,21201,-3,0,1,22101,0,-2,2,21202,-1,1,3,21102,250,1,0,1106,0,225,21202,1,1,-4,109,-5,2106,0,0,109,3,22107,0,-2,-1,21202,-1,2,-1,21201,-1,-1,-1,22202,-1,-2,-2,109,-3,2105,1,0,109,3,21207,-2,0,-1,1206,-1,294,104,0,99,21202,-2,1,-2,109,-3,2105,1,0,109,5,22207,-3,-4,-1,1206,-1,346,22201,-4,-3,-4,21202,-3,-1,-1,22201,-4,-1,2,21202,2,-1,-1,22201,-4,-1,1,22102,1,-2,3,21102,1,343,0,1106,0,303,1106,0,415,22207,-2,-3,-1,1206,-1,387,22201,-3,-2,-3,21202,-2,-1,-1,22201,-3,-1,3,21202,3,-1,-1,22201,-3,-1,2,21201,-4,0,1,21102,384,1,0,1105,1,303,1106,0,415,21202,-4,-1,-4,22201,-4,-3,-4,22202,-3,-2,-2,22202,-2,-4,-4,22202,-3,-2,-3,21202,-4,-1,-2,22201,-3,-2,1,21202,1,1,-4,109,-5,2106,0,0";
