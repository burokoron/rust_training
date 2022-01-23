use proconio::input;

fn main() {
    input! {
        n: usize,
        b: usize,
        k: usize,
        c: [usize; k],
    }

    let mut dp: Vec<Vec<usize>> = vec![vec![0; b as usize]; n+1];
    let mod_number = 1000000007_usize;

    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..b {
            for l in 0..k {
                let nex = (10 * j + c[l]) % b;
                dp[i+1][nex] += dp[i][j];
                dp[i+1][nex] %= mod_number;
            }
        }
    }

    println!("{}", dp[n][0]);
}
