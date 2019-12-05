use itertools::Itertools;
use rayon::prelude::*;

fn is_valid(number: &i32) -> bool {
    let s = number.to_string();
    "x".chars()
        .chain(s.chars())
        .chain("x".chars())
        .tuple_windows::<(_, _, _, _)>()
        .any(|(a, b, c, d)| a != b && b == c && c != d)
        && s.chars().tuple_windows().all(|(a, b)| a <= b)
}

pub fn main() {
    let valid = (246540..=787419)
        .collect_vec()
        .par_iter()
        .filter(|&n| is_valid(n))
        .count();
    println!("{}", valid);
}
