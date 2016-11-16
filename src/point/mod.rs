use std::fmt::{Debug,Display};
use num_traits::{Float,Zero,One,Signed};


pub struct Point<S> {
    x: S,
    y: S
}


impl<S> Point<S>
where S: Float + Zero + One + Signed {
    pub fn new() -> Self {
        Point {
            x: S::zero(),
            y: S::zero()
        }
    }
    pub fn from_coords(x: S, y: S) -> Self {
        Point {
            x: x,
            y: y
        }
    }
}