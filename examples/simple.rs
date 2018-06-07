extern crate geomprim2d;

use geomprim2d::Point;

fn main() {
  let p1: Point<i32> = Point::origin();
  let mut p2: Point<f64> = Point::from_coords(4f64, 3f64);
  println!("{:?}", p1);
  println!("{:?}", p2);
  p2.translate(0f64, 5f64);
  let p3 = Point::from_point(&p2);
  let _p4 = p3;
  println!("translated by 0,5 : {:?}", p2);
  println!("{:?}", p3);
}
