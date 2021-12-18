use proconio::input;

fn main() {
    input! {
        sx: f64,
        sy: f64,
        gx: f64,
        gy: f64,
    }

    let a = (sy + gy) / (sx - gx);
    let b = sy - a * sx;

    println!("{}", -b / a)
}
