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

pub fn main() {
    let asteroids = parse(INPUT);
    println!(
        "{}",
        asteroids
            .iter()
            .map(|asteroid| asteroid.calculate_visible(&asteroids))
            .max()
            .unwrap_or(0)
    );
    let WINNER = Asteroid { x: 11, y: 13 };
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
