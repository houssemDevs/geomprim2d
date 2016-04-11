
#![feature(zero_one)]

extern crate scalar;

/// Define a 2 dimensional point (x,y).
mod point;
/// Define a line segment by his two ends point a and point b.
mod linesegment;

mod rectangle;

pub use self::point::*;
pub use self::linesegment::*;
pub use self::rectangle::*;
