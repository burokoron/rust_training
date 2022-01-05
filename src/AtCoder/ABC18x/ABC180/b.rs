use proconio::input;

fn main() {
    input! {
        n: i64,
        x: [f64; n],
    }

    let mut manhattan = 0.;
    let mut euclid = 0.;
    let mut chebyshev: f64 = 0.;
    for &i in &x {
        manhattan += i.abs();
        euclid += i * i;
        chebyshev = if chebyshev > i.abs() {chebyshev} else {i.abs()};
    }

    euclid = euclid.sqrt();

    println!("{}", manhattan);
    println!("{}", euclid);
    println!("{}", chebyshev);
}
