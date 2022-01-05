use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    }

    let theta = (m * 6. - (h * 60. + m) * 0.5) * PI / 180.;

    println!("{}", (a.powf(2.) + b.powf(2.) - 2. * a * b * theta.cos()).sqrt());
}
