use regex::Regex;
use std::cmp::Ordering;
use std::iter::Sum;
use std::ops;

fn calculate_change(a: i32, b: i32) -> i32 {
    match a.cmp(&b) {
        Ordering::Greater => -1,
        Ordering::Less => 1,
        Ordering::Equal => 0,
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    velocity: Velocity,
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x,
            y,
            z,
            velocity: Velocity::default(),
        }
    }

    fn update_velocity(&self, universe: &[Moon]) -> Moon {
        Moon {
            x: self.x,
            y: self.y,
            z: self.z,
            velocity: self.velocity
                + universe
                    .iter()
                    .filter(|&moon| moon != self)
                    .map(|&moon| Velocity {
                        x: calculate_change(self.x, moon.x),
                        y: calculate_change(self.y, moon.y),
                        z: calculate_change(self.z, moon.z),
                    })
                    .sum(),
        }
    }

    fn step(&self) -> Moon {
        Moon {
            x: self.x + self.velocity.x,
            y: self.y + self.velocity.y,
            z: self.z + self.velocity.z,
            velocity: self.velocity,
        }
    }

    fn potential_energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn kinetic_energy(&self) -> i32 {
        self.velocity.energy()
    }

    fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Velocity {
    x: i32,
    y: i32,
    z: i32,
}

impl Velocity {
    fn energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Default for Velocity {
    fn default() -> Velocity {
        Velocity { x: 0, y: 0, z: 0 }
    }
}

impl ops::Add<Velocity> for Velocity {
    type Output = Velocity;

    fn add(self, rhs: Velocity) -> Self::Output {
        Velocity {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign for Velocity {
    fn add_assign(&mut self, other: Velocity) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sum for Velocity {
    fn sum<I: Iterator<Item = Velocity>>(iter: I) -> Velocity {
        iter.fold(Self::default(), |a, b| a + b)
    }
}

fn parse(data: &str) -> Vec<Moon> {
    let pattern = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
    data.lines()
        .flat_map(|line| {
            pattern.captures(line).map(|cap| {
                Moon::new(
                    cap[1].parse::<i32>().unwrap(),
                    cap[2].parse::<i32>().unwrap(),
                    cap[3].parse::<i32>().unwrap(),
                )
            })
        })
        .collect()
}

pub fn main() {
    let mut moons = parse(INPUT);
    for _ in 0..1000 {
        moons = moons
            .iter()
            .map(|&moon| moon.update_velocity(&moons))
            .collect();
        moons = moons.iter().map(|&moon| moon.step()).collect();
    }
    let energy: i32 = moons.iter().map(|&moon| moon.total_energy()).sum();
    println!("{}", energy);
}

const INPUT: &str = "<x=15, y=-2, z=-6>
<x=-5, y=-4, z=-11>
<x=0, y=-6, z=0>
<x=5, y=9, z=6>";
