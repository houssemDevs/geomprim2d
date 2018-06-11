use super::scalar::Scalar;
use super::Point;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub struct Rectangle<S: Scalar> {
  pub tlc: Point<S>,
  pub w: S,
  pub h: S,
}

unsafe impl<S: Scalar> Send for Rectangle<S> {}

impl<S: Scalar> Rectangle<S> {
  pub fn new(tlc: Point<S>, w: S, h: S) -> Self {
    Rectangle { tlc, w, h }
  }
  pub fn square(tlc: Point<S>, w: S) -> Self {
    Rectangle { tlc, w, h: w }
  }
}
