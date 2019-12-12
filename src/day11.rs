use crate::vm::{run, Memory};
use itertools::Itertools;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[repr(i64)]
#[derive(Copy, Clone, Debug)]
enum Color {
    Black = 0,
    White = 1,
}

impl Color {
    fn render(self) -> String {
        match self {
            Color::Black => ".".to_string(),
            Color::White => "#".to_string(),
        }
    }
}

impl TryFrom<i64> for Color {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Color::Black),
            1 => Ok(Color::White),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn turn_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn advance(self, position: Coordinate) -> Coordinate {
        match self {
            Direction::Up => Coordinate {
                x: position.x,
                y: position.y - 1,
            },
            Direction::Right => Coordinate {
                x: position.x + 1,
                y: position.y,
            },
            Direction::Down => Coordinate {
                x: position.x,
                y: position.y + 1,
            },
            Direction::Left => Coordinate {
                x: position.x - 1,
                y: position.y,
            },
        }
    }
}

type Hull = HashMap<Coordinate, Color>;

#[derive(Debug)]
struct Robot {
    hull: Hull,
    position: Coordinate,
    direction: Direction,
    camera: Sender<i64>,
    driver: Receiver<i64>,
}

impl Robot {
    fn new(starting_color: Color, camera: Sender<i64>, driver: Receiver<i64>) -> Self {
        let position = Coordinate { x: 0, y: 0 };
        let mut hull = HashMap::new();
        hull.insert(position, starting_color);
        Self {
            hull,
            position,
            direction: Direction::Up,
            camera,
            driver,
        }
    }

    fn advance(&mut self) {
        self.position = self.direction.advance(self.position);
    }

    fn run(&mut self) {
        loop {
            self.camera
                .send(*self.hull.get(&self.position).unwrap_or(&Color::Black) as i64)
                .unwrap();
            match self.driver.recv() {
                Ok(color) => {
                    self.hull.insert(self.position, color.try_into().unwrap());
                    self.direction = match self.driver.recv().unwrap() {
                        0 => self.direction.turn_left(),
                        1 => self.direction.turn_right(),
                        n => panic!("unexpected turn {}", n),
                    };
                    self.advance();
                }
                Err(_) => break,
            };
        }
    }
}

fn paint(color: Color) -> Hull {
    let memory = Memory::from(INPUT);
    let (camera_send, camera_recv) = channel();
    let (driver_send, driver_recv) = channel();
    let robot = thread::spawn(move || {
        let mut robot = Robot::new(color, camera_send, driver_recv);
        robot.run();
        robot.hull
    });
    thread::spawn(|| {
        run(memory, camera_recv, driver_send);
    })
    .join()
    .unwrap();
    robot.join().unwrap()
}

fn rasterize(hull: Hull) -> String {
    let (min_x, max_x) = hull
        .keys()
        .map(|&coord| coord.x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = hull
        .keys()
        .map(|&coord| coord.y)
        .minmax()
        .into_option()
        .unwrap();
    (min_y..=max_y)
        .map(|y| {
            (min_x..=max_x)
                .map(|x| {
                    hull.get(&Coordinate { x, y })
                        .unwrap_or(&Color::Black)
                        .render()
                })
                .join("")
        })
        .join("\n")
}

pub fn main() {
    let hull = paint(Color::Black);
    println!("{}", hull.len());
    let hull = paint(Color::White);
    println!("{}", rasterize(hull));
}

static INPUT :&str = "3,8,1005,8,330,1106,0,11,0,0,0,104,1,104,0,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,102,1,8,29,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,101,0,8,51,1,1103,2,10,1006,0,94,1006,0,11,1,1106,13,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,1001,8,0,87,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,1001,8,0,109,2,1105,5,10,2,103,16,10,1,1103,12,10,2,105,2,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,1001,8,0,146,1006,0,49,2,1,12,10,2,1006,6,10,1,1101,4,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,0,8,10,4,10,1001,8,0,183,1,6,9,10,1006,0,32,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,1,10,4,10,101,0,8,213,2,1101,9,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,1,10,4,10,101,0,8,239,1006,0,47,1006,0,4,2,6,0,10,1006,0,58,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,102,1,8,274,2,1005,14,10,1006,0,17,1,104,20,10,1006,0,28,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,1002,8,1,309,101,1,9,9,1007,9,928,10,1005,10,15,99,109,652,104,0,104,1,21101,0,937263411860,1,21102,347,1,0,1105,1,451,21101,932440724376,0,1,21102,1,358,0,1105,1,451,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21101,0,29015167015,1,21101,0,405,0,1106,0,451,21102,1,3422723163,1,21101,0,416,0,1106,0,451,3,10,104,0,104,0,3,10,104,0,104,0,21101,0,868389376360,1,21101,0,439,0,1105,1,451,21102,825544712960,1,1,21102,1,450,0,1106,0,451,99,109,2,21201,-1,0,1,21101,0,40,2,21102,482,1,3,21102,1,472,0,1106,0,515,109,-2,2106,0,0,0,1,0,0,1,109,2,3,10,204,-1,1001,477,478,493,4,0,1001,477,1,477,108,4,477,10,1006,10,509,1101,0,0,477,109,-2,2106,0,0,0,109,4,2101,0,-1,514,1207,-3,0,10,1006,10,532,21102,1,0,-3,22101,0,-3,1,22102,1,-2,2,21102,1,1,3,21101,551,0,0,1106,0,556,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,579,2207,-4,-2,10,1006,10,579,22102,1,-4,-4,1106,0,647,21201,-4,0,1,21201,-3,-1,2,21202,-2,2,3,21102,1,598,0,1106,0,556,22101,0,1,-4,21101,1,0,-1,2207,-4,-2,10,1006,10,617,21102,0,1,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,639,21201,-1,0,1,21102,639,1,0,105,1,514,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2105,1,0";
