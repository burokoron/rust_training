use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n],
    }

    let mut ans = 0;
    for i in 0..n {
        for j in i+1..n {
            let a = (xy[j].1 - xy[i].1) / (xy[j].0 - xy[i].0);
            if -1. <= a && a <= 1. {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
