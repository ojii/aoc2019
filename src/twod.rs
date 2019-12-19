//use crate::render::render;
//use std::collections::HashMap;
//use std::convert::TryFrom;
//use std::fmt::{Display, Error, Formatter};
//use std::ops::{Add, Mul};
//
//pub struct Grid<T>(HashMap<Coord, T>);
//
//impl<T: Into<char>> Display for Grid<T> {
//    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
//        f.write_str(&render(
//            self.0.iter().map(|(c, v)| (c.into(), v.into())),
//            ' ',
//        ))
//    }
//}
//
//const NEIGHBORS: [(i64, i64); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
//
//#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
//pub struct Coord {
//    x: i64,
//    y: i64,
//}
//
//impl Coord {
//    pub fn new(x: i64, y: i64) -> Coord {
//        Coord { x, y }
//    }
//
//    pub fn vector(&self, to: &Coord) -> Vector {
//        Vector::new(to.x - self.x, to.y - self.y)
//    }
//
//    pub fn neighbors<'a>(&'a self) -> impl Iterator<Item = Coord> + 'a {
//        NEIGHBORS
//            .iter()
//            .map(|&t| Vector::from(t))
//            .map(move |v| self + v)
//    }
//}
//
//impl Into<(i64, i64)> for Coord {
//    fn into(self) -> (i64, i64) {
//        (self.x, self.y as i64)
//    }
//}
//
//impl Into<(i64, i64)> for &Coord {
//    fn into(self) -> (i64, i64) {
//        (self.x, self.y as i64)
//    }
//}
//
//impl Add<Vector> for &Coord {
//    type Output = Coord;
//
//    fn add(self, rhs: Vector) -> Self::Output {
//        rhs + self
//    }
//}
//
//impl Add<Vector> for Coord {
//    type Output = Coord;
//
//    fn add(self, rhs: Vector) -> Self::Output {
//        rhs + self
//    }
//}
//
//impl Add<&Vector> for Coord {
//    type Output = Coord;
//
//    fn add(self, rhs: &Vector) -> Self::Output {
//        rhs + self
//    }
//}
//
//impl Add<&Vector> for &Coord {
//    type Output = Coord;
//
//    fn add(self, rhs: &Vector) -> Self::Output {
//        rhs + self
//    }
//}
//
//#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
//pub struct Vector {
//    x: i64,
//    y: i64,
//}
//
//impl Vector {
//    pub fn new(x: i64, y: i64) -> Vector {
//        Vector { x, y }
//    }
//
//    pub fn manhattan_distance(&self) -> i64 {
//        self.x.abs() + self.y.abs()
//    }
//}
//
//impl From<(i64, i64)> for Vector {
//    fn from(tuple: (i64, i64)) -> Self {
//        Vector::new(tuple.0, tuple.1)
//    }
//}
//
//impl Add<Coord> for Vector {
//    type Output = Coord;
//
//    fn add(self, rhs: Coord) -> Self::Output {
//        Coord::new(rhs.x + self.x, rhs.y + self.y)
//    }
//}
//
//impl Add<&Coord> for Vector {
//    type Output = Coord;
//
//    fn add(self, rhs: &Coord) -> Self::Output {
//        Coord::new(rhs.x + self.x, rhs.y + self.y)
//    }
//}
//
//impl Add<Coord> for &Vector {
//    type Output = Coord;
//
//    fn add(self, rhs: Coord) -> Self::Output {
//        Coord::new(rhs.x + self.x, rhs.y + self.y)
//    }
//}
//
//impl Add<&Coord> for &Vector {
//    type Output = Coord;
//
//    fn add(self, rhs: &Coord) -> Self::Output {
//        Coord::new(rhs.x + self.x, rhs.y + self.y)
//    }
//}
//
//impl Mul<i64> for Vector {
//    type Output = Vector;
//
//    fn mul(self, rhs: i64) -> Self::Output {
//        Vector::new(self.x * rhs, self.y * rhs)
//    }
//}
//
//pub enum Turn {
//    Right,
//    Left,
//}
//
//impl TryFrom<(Vector, Vector)> for Turn {
//    type Error = ();
//
//    fn try_from(path: (Vector, Vector)) -> Result<Self, Self::Error> {
//        unimplemented!()
//    }
//}
