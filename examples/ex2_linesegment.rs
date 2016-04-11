extern crate geomprim2d;

use geomprim2d::{LineSegment,Point};


fn main() {
    println!("{:?}", LineSegment::new(Point::<i32>::origin(), Point::<i32>::new(1,3)));
}