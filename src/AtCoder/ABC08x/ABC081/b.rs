use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }

    let mut ans = 0;
    'outer: loop {
        for i in 0..n {
            if a[i] % 2 == 0 {
                a[i] /= 2;
            } else {
                break 'outer;
            }
        }
        ans += 1;
    }

    println!("{}", ans);
}
