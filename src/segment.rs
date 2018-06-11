use super::Point;
use scalar::Scalar;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub struct Segment<S: Scalar> {
  pub s: Point<S>,
  pub e: Point<S>,
}

impl<S: Scalar> Segment<S> {
  pub fn new(s: Point<S>, e: Point<S>) -> Self {
    Segment { s, e }
  }
  pub fn from_segment(l: Segment<S>) -> Self {
    Segment { s: l.s, e: l.e }
  }
  pub fn translate_x(&mut self, dx: S) {
    self.s.translate_x(dx);
    self.e.translate_x(dx);
  }
  pub fn translate_y(&mut self, dy: S) {
    self.s.translate_y(dy);
    self.e.translate_y(dy);
  }
}
