use super::scalar::Scalar;
use super::Point;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub struct Triangle<S: Scalar> {
  pub a: Point<S>,
  pub b: Point<S>,
  pub c: Point<S>,
}

unsafe impl<S: Scalar> Send for Triangle<S> {}

impl<S: Scalar> Triangle<S> {
  pub fn new(a: Point<S>, b: Point<S>, c: Point<S>) -> Self {
    Triangle { a, b, c }
  }
}
