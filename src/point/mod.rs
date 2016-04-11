
use scalar::Scalar;

#[derive(Debug,Clone,Copy)]
pub struct Point<S> {
    pub x: S,
    pub y: S,
}


impl<S> Point<S> 
where S: Scalar {
    pub fn new(x: S, y: S) -> Self {
        Point {
            x: x,
            y: y
        }
    }
    pub fn origin() -> Self {
        Point {
            x: S::zero(),
            y: S::zero()
        }
    }    
}