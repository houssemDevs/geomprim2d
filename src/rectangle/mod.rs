

use super::Point;
use scalar::Scalar;

pub struct Rectangle<S> {
    pub tl: Point<S>,
    pub w: S,
    pub h: S,
}


impl<S> Rectangle<S>
where S: Scalar {
    pub fn new(tl: Point<S>, w: S, h: S) -> Self {
        Rectangle {
            tl: tl,
            w: w,
            h: h
        }
    }
}