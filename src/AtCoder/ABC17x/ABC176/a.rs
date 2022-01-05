use proconio::input;

fn main() {
    input! {
        n: f32,
        x: f32,
        t: f32,
    }

    println!("{}", ((n / x).ceil() * t) as i32)
}
