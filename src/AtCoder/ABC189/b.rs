use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        vp: [(i64, i64); n],
    }

    let mut d = 0;
    let mut ans = -1;
    for i in 0..n {
        d += vp[i].0 * vp[i].1;
        if d > x * 100 {
            ans = (i + 1) as i64;
            break;
        }
    }

    println!("{}", ans);
}
