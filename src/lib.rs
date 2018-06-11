extern crate scalar;

/// Define a 2 dimensional ellipse (a: Scalar, b: Scalar, c: Point)
mod ellipse;
/// Define a 2 dimensional line (source : Point, target : Point)
mod line;
/// Define a 2 dimensional point (x,y).
mod point;
/// Define a 2 dimensional Rectangle
mod rectangle;
/// Define a 2 dimensional line segment (start : Point, end: Point)
mod segment;
/// Define a 2 dimensional triangle
mod triangle;

pub use line::*;
pub use point::*;
pub use rectangle::*;
pub use segment::*;
pub use triangle::*;
