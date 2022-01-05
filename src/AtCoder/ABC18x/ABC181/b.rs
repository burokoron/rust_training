use proconio::input;

fn main() {
    input! {
        n: i32,
        ab: [(i64, i64); n],
    }

    let mut ans = 0;

    for &(a, b) in &ab{
        ans -= (a - 1) * a / 2;
        ans += b * (b + 1) / 2;
    }

    println!("{}", ans);
}
