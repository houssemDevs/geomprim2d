
use scalar::Scalar;
use super::Point;

#[derive(Debug,Clone,Copy)]
pub struct LineSegment<S> {
    pub a: Point<S>,
    pub b: Point<S>,
}


impl<S> LineSegment<S> 
where S: Scalar {
    pub fn new(a: Point<S>, b: Point<S>) -> Self {
        LineSegment{
            a: a,
            b: b
        }
    }
}