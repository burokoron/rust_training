use proconio::input;
use std::char;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: proconio::marker::Chars,
    }

    let mut table: Vec<Vec<usize>> = vec![vec![n; n]; 26];

    for i in (0..n).rev() {
        if i < n - 1 {
            for j in 0..26 {
                table[j][i] = table[j][i+1];
            }
        }
        table[s[i] as usize - 'a' as usize][i] = i;
    }

    let mut ans: Vec<char> = Vec::new();

    let mut i = 0;
    while ans.len() < k {
        for j in 0..26 {
            if n - table[j][i] >= k - ans.len() {
                ans.push(char::from_u32('a' as u32 + j as u32).unwrap());
                i = table[j][i] + 1;
                break;
            }
        }
    }

    let ans: String = ans.iter().collect();
    println!("{}", ans);
}
