extern crate geomprim2d;

use geomprim2d::*;
use std::f64::consts::PI;

fn main() {
    let gamma0 = (1.0f64 + 5f64.sqrt())/2f64;
    let mut gammaN = gamma0;
    let mut gammaNn = 0f64;
    let (a,b): (f64,f64) = (1.0,23.0);
    println!("{}->{}",PI,4f64*(((a/b).sin()*(a/b).sin())+((a/b).cos()*(a/b).cos())).atan());
    for i in 0..100 {
        gammaNn = 1f64/(gammaN-1f64);
        println!("{}->{}",gamma0,gammaNn);
        gammaN = gammaNn;
    }
}