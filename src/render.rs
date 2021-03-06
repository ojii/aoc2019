use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Display;

pub fn render<B, I: Iterator<Item = ((i64, i64), B)>>(data: I, background: B) -> String
where
    B: Display,
{
    let coords: HashMap<(i64, i64), B> = data.collect();
    let (min_x, max_x) = coords
        .keys()
        .map(|&(x, _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = coords
        .keys()
        .map(|&(_, y)| y)
        .minmax()
        .into_option()
        .unwrap();
    (min_y..=max_y)
        .map(|y| {
            (min_x..=max_x)
                .map(|x| coords.get(&(x, y)).unwrap_or(&background))
                .join("")
        })
        .join("\n")
}
