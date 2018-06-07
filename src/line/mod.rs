use super::Point;
use scalar::Scalar;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub struct Line<S: Scalar> {
  pub s: Point<S>,
  pub t: Point<S>,
}

impl<S: Scalar> Line<S> {
  pub fn new() -> Self {
    Line {
      s: Point::new(),
      t: Point::new(),
    }
  }
  pub fn from_points(src: Point<S>, target: Point<S>) -> Self {
    Line { s: src, t: target }
  }
  pub fn from_line(l: Line<S>) -> Self {
    Line { s: l.s, t: l.t }
  }
  pub fn translate_x(&mut self, dx: S) {
    self.s.translate_x(dx);
    self.t.translate_x(dx);
  }
  pub fn translate_y(&mut self, dy: S) {
    self.s.translate_y(dy);
    self.t.translate_y(dy);
  }
}
