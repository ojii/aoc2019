use crate::render::render;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const NEIGHBORS: [fn(i64, i64) -> (i64, i64); 4] = [
    |x, y| (x - 1, y),
    |x, y| (x + 1, y),
    |x, y| (x, y - 1),
    |x, y| (x, y + 1),
];

fn k2i(key: &(i64, i64), width: i64) -> u32 {
    (key.0 + (key.1 * width)) as u32
}

struct Grid {
    bugs: HashMap<(i64, i64), bool>,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let bugs = value
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| ((x as i64, y as i64), c == '#'))
            })
            .collect();
        Grid { bugs }
    }
}

impl Grid {
    fn neighbor_bug_count(&self, position: &(i64, i64)) -> usize {
        NEIGHBORS
            .iter()
            .map(|f| f(position.0, position.1))
            .flat_map(|key| self.bugs.get(&key))
            .filter(|&b| *b)
            .count()
    }

    fn evolve(&self) -> Grid {
        Grid {
            bugs: self
                .bugs
                .iter()
                .map(|(key, bug)| {
                    (
                        *key,
                        match (self.neighbor_bug_count(key), bug) {
                            (n, true) if n != 1 => false,
                            (n, false) if n == 1 || n == 2 => true,
                            (_, b) => *b,
                        },
                    )
                })
                .collect(),
        }
    }

    fn biodiversity(&self) -> u64 {
        self.bugs
            .iter()
            .filter_map(|(key, &bug)| if bug { Some(k2i(key, 5)) } else { None })
            .map(|index| 2u64.pow(index))
            .sum()
    }

    fn renderable(&self) -> impl Iterator<Item = ((i64, i64), char)> + '_ {
        self.bugs
            .iter()
            .map(move |(&key, &bug)| (key, if bug { '#' } else { '.' }))
    }
}

pub fn main() {
    let mut grid = Grid::from(INPUT);
    let mut diversities = HashSet::new();
    loop {
        if !diversities.insert(grid.biodiversity()) {
            println!("{}", grid.biodiversity());
            break;
        };
        grid = grid.evolve();
    }
}

const INPUT: &str = "#..#.
..#..
...##
...#.
#.###";
