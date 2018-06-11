extern crate geomprim2d;

use geomprim2d::Point;
use geomprim2d::Triangle;

fn main() {
  let t1 = Triangle::new(Point::new(1, 1), Point::new(1, 3), Point::new(3, 4));
  println!("{:?}", t1);
}
