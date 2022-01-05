use proconio::input;

fn main() {
    input! {
        d: f64,
        t: f64,
        s: f64,
    }

    if d / s <= t{
        println!("Yes");
    } else {
        println!("No");
    }
}
