use proconio::input;

fn main() {
    input! {
        n: i32,
        d: f64,
        xy: [(f64, f64); n],
    }

    let mut ans = 0;
    for &(x, y) in &xy {
        ans += if x.hypot(y) <= d {1} else {0};
    }

    println!("{}", ans);
}
