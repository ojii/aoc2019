use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Asteroid {
    x: i32,
    y: i32,
}

impl Asteroid {
    fn calculate_visible(&self, universe: &[Asteroid]) -> usize {
        universe
            .iter()
            .filter(|&asteroid| asteroid != self && self.can_see(asteroid, universe))
            .count()
    }

    fn can_see(&self, other: &Asteroid, universe: &[Asteroid]) -> bool {
        universe
            .iter()
            .filter(|&asteroid| asteroid != self && asteroid != other)
            .all(|asteroid| !asteroid.is_between(self, other))
    }

    fn is_between(&self, start: &Asteroid, end: &Asteroid) -> bool {
        self.within(start, end) && self.collinear(start, end)
    }

    fn collinear(&self, start: &Asteroid, end: &Asteroid) -> bool {
        (end.x - start.x) * (self.y - start.y) == (self.x - start.x) * (end.y - start.y)
    }

    fn within(&self, start: &Asteroid, end: &Asteroid) -> bool {
        let (start, us, end) = if start.x != end.x {
            (start.x, self.x, end.x)
        } else {
            (start.y, self.y, end.y)
        };
        (start <= us && us <= end) || (end <= us && us <= start)
    }

    fn distance(&self, to: &Asteroid) -> f64 {
        (((self.x - to.x).pow(2) + (self.y - to.y).pow(2)) as f64).sqrt()
    }

    fn angle(&self, to: &Asteroid) -> f64 {
        let zero = Asteroid {
            x: self.x,
            y: self.y - 1,
        };
        ((to.y - self.y) as f64).atan2((to.x - self.x) as f64)
            - ((zero.y - self.y) as f64).atan2((zero.x - self.x) as f64)
    }
}

fn parse(from: &str) -> Vec<Asteroid> {
    from.lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .flat_map(|(x, c)| {
                    if c == '#' {
                        Some(Asteroid {
                            x: x as i32,
                            y: y as i32,
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<Asteroid>>()
        })
        .flatten()
        .collect()
}

fn shooting_order(from: &Asteroid, universe: &[Asteroid]) -> Vec<Asteroid> {
    let mut pool = Vec::from(universe)
        .iter()
        .filter(|&p| p != from)
        .cloned()
        .collect::<Vec<Asteroid>>();
    let mut colinears: Vec<VecDeque<Asteroid>> = Vec::new();
    while let Some(asteroid) = pool.pop() {
        let mut current = VecDeque::new();
        current.push_front(asteroid.clone());
        current.extend(pool.drain_filter(|other| from.collinear(&asteroid, &other)));
        colinears.push(
            current
                .iter()
                .sorted_by(|&a, &b| from.distance(a).partial_cmp(&from.distance(b)).unwrap())
                .cloned()
                .collect(),
        );
    }

    colinears.sort_by(|a, b| from.angle(&a[0]).partial_cmp(&from.angle(&b[0])).unwrap());

    let mut order = Vec::with_capacity(universe.len() - 1);

    while colinears.iter().any(|v| !v.is_empty()) {
        for asteroids in &mut colinears {
            if let Some(asteroid) = asteroids.pop_front() {
                order.push(asteroid);
            }
        }
    }
    order.to_vec()
}

pub fn main() {
    let asteroids = parse(INPUT);
    let (winner, visible) = asteroids
        .iter()
        .map(|asteroid| (asteroid, asteroid.calculate_visible(&asteroids)))
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();
    println!("{} (at {:?})", visible, winner);
    let order = shooting_order(&winner, &asteroids);
    // 2203 is too high
    let two_hundredth = &order[199];
    println!("{}", (two_hundredth.x * 100) + two_hundredth.y);
}

const INPUT: &str = "....#...####.#.#...........#........
#####..#.#.#......#####...#.#...#...
##.##..#.#.#.....#.....##.#.#..#....
...#..#...#.##........#..#.......#.#
#...##...###...###..#...#.....#.....
##.......#.....#.........#.#....#.#.
..#...#.##.##.....#....##..#......#.
..###..##..#..#...#......##...#....#
##..##.....#...#.#...#......#.#.#..#
...###....#..#.#......#...#.......#.
#....#...##.......#..#.......#..#...
#...........#.....#.....#.#...#.##.#
###..#....####..#.###...#....#..#...
##....#.#..#.#......##.......#....#.
..#.#....#.#.#..#...#.##.##..#......
...#.....#......#.#.#.##.....#..###.
..#.#.###.......#..#.#....##.....#..
.#.#.#...#..#.#..##.#..........#...#
.....#.#.#...#..#..#...###.#...#.#..
#..#..#.....#.##..##...##.#.....#...
....##....#.##...#..........#.##....
...#....###.#...##........##.##..##.
#..#....#......#......###...........
##...#..#.##.##..##....#..#..##..#.#
.#....#..##.....#.#............##...
.###.........#....#.##.#..#.#..#.#..
#...#..#...#.#.#.....#....#......###
#...........##.#....#.##......#.#..#
....#...#..#...#.####...#.#..#.##...
......####.....#..#....#....#....#.#
.##.#..###..####...#.......#.#....#.
#.###....#....#..........#.....###.#
...#......#....##...##..#..#...###..
..#...###.###.........#.#..#.#..#...
.#.#.............#.#....#...........
..#...#.###...##....##.#.#.#....#.#.";
