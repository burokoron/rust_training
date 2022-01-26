use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: proconio::marker::Chars,
        x: proconio::marker::Chars,
    }

    let mut dp: Vec<HashSet<i32>> = vec![HashSet::new(); n+1];
    dp[n].insert(0);

    for i in (1..n+1).rev() {
        match x[i-1] {
            'T' => {
                for j in 0..7 {
                    let nex = (10 * j) % 7;
                    if dp[i].contains(&nex) {
                        dp[i-1].insert(j);
                    }
                    let nex = (10 * j + s[i-1] as i32 - '0' as i32) % 7;
                    if dp[i].contains(&nex) {
                        dp[i-1].insert(j);
                    }
                }
            },
            'A' => {
                for j in 0..7 {
                    let nex1 = (10 * j) % 7;
                    let nex2 = (10 * j + s[i-1] as i32 - '0' as i32) % 7;
                    if dp[i].contains(&nex1) && dp[i].contains(&nex2) {
                        dp[i-1].insert(j);
                    }
                }
            },
            _ => unreachable!(),
        }
    }

    println!("{}", if dp[0].contains(&0) { "Takahashi" } else { "Aoki" });
}
