use itertools::Itertools;

fn is_valid1(number: &i32) -> bool {
    let s = number.to_string();
    s.chars().tuple_windows().any(|(a, b)| a == b) && s.chars().tuple_windows().all(|(a, b)| a <= b)
}

fn is_valid2(number: &i32) -> bool {
    let s = number.to_string();
    "x".chars()
        .chain(s.chars())
        .chain("x".chars())
        .tuple_windows::<(_, _, _, _)>()
        .any(|(a, b, c, d)| a != b && b == c && c != d)
        && s.chars().tuple_windows().all(|(a, b)| a <= b)
}

pub fn main() {
    let candidates = (246540..=787419).collect_vec();
    let valid1 = candidates.iter().filter(|&n| is_valid1(n)).count();
    let valid2 = candidates.iter().filter(|&n| is_valid2(n)).count();
    println!("{}", valid1);
    println!("{}", valid2);
}
