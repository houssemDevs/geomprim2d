use super::Point;
use scalar::Scalar;
use std::fmt::Debug;
use std::vec::Vec;

#[derive(Debug, Clone)]
struct Polygon<S: Scalar> {
  points: Vec<Point<S>>,
}

impl<S: Scalar> Polygon<S> {
  pub fn new() -> Self {
    Polygon { points: Vec::new() }
  }
  pub fn from_points(pts: &[Point<S>]) -> Self {
    Polygon {
      points: Vec::from(pts),
    }
  }
  pub fn add_point(&mut self, p: Point<S>) {
    self.points.push(p);
  }
  pub fn add_points(&mut self, pts: &[Point<S>]) {
    for p in pts {
      self.points.push(*p);
    }
  }
}
