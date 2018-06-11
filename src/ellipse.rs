use super::scalar::Scalar;
use super::Point;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub struct Ellipse<S: Scalar> {
  pub c: Point<S>,
  pub a: S,
  pub b: S,
}

unsafe impl<S: Scalar> Send for Ellipse<S> {}

impl<S: Scalar> Ellipse<S> {
  pub fn new(c: Point<S>, a: S, b: S) -> Self {
    Ellipse { c, a, b }
  }
  pub fn circle(c: Point<S>, r: S) -> Self {
    Ellipse { c: a: r, b: r }
  }
}
