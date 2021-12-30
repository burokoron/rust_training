use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut p: [i32; n],
    }

    let mut ans = 0;
    p.sort();
    for &i in &p[..k] {
        ans += i;
    }

    println!("{}", ans);
}
