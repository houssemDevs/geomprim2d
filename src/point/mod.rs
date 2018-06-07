use scalar::{One, Scalar, Zero};
use std::fmt::{Debug, Display};
use std::marker::Send;

#[derive(Debug, Clone, Copy)]
pub struct Point<S> {
    pub x: S,
    pub y: S,
}

unsafe impl<S: Scalar> Send for Point<S> {}

impl<S: Scalar> Point<S> {
    pub fn new() -> Self {
        Point {
            x: S::zero(),
            y: S::zero(),
        }
    }
    pub fn origin() -> Self {
        Point {
            x: S::zero(),
            y: S::zero(),
        }
    }
    pub fn from_coords(x: S, y: S) -> Self {
        Point { x: x, y: y }
    }
    pub fn from_point(p: &Point<S>) -> Self {
        Point { x: p.x, y: p.y }
    }
    pub fn translate(&mut self, dx: S, dy: S) {
        self.x = self.x + dx;
        self.y = self.y + dy;
    }
    pub fn translate_x(&mut self, dx: S) {
        self.x = self.x + dx;
    }
    pub fn translate_y(&mut self, dy: S) {
        self.y = self.y + dy;
    }
}
