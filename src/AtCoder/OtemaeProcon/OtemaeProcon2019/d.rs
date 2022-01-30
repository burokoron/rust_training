use proconio::input;
 
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; m],
    }

    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; n+1]; 15]; n+1];
    let mod_number = 1000000007;
    
    dp[0][0][0] = 1;
    for i in 0..n {
        for j in 0..15_usize {
            for k in 0..m {
                for l in 0..10_usize {
                    if i == 0 && l == 0 {
                        continue;
                    }
                    let nex = (10 * j + l) % 15;
                    match &s[k][..] {
                        "Fizz" => {
                            if nex == 3 || nex == 6 || nex == 9 || nex == 12 {
                                dp[i+1][nex][k+1] += dp[i][j][k];
                                dp[i+1][nex][k+1] %= mod_number;
                            } else if nex != 0 && nex != 5 && nex != 10 {
                                dp[i+1][nex][k] += dp[i][j][k];
                                dp[i+1][nex][k] %= mod_number;
                            }
                        },
                        "Buzz" => {
                            if nex == 5 || nex == 10 {
                                dp[i+1][nex][k+1] += dp[i][j][k];
                                dp[i+1][nex][k+1] %= mod_number;
                            } else if nex != 0 && nex != 3 && nex != 6 && nex != 9 && nex != 12 {
                                dp[i+1][nex][k] += dp[i][j][k];
                                dp[i+1][nex][k] %= mod_number;
                            }
                        },
                        "FizzBuzz" => {
                            if nex == 0 {
                                dp[i+1][nex][k+1] += dp[i][j][k];
                                dp[i+1][nex][k+1] %= mod_number;
                            } else if nex != 3 && nex != 5 &&nex != 6 && nex != 9 && nex != 10 && nex != 12 {
                                dp[i+1][nex][k] += dp[i][j][k];
                                dp[i+1][nex][k] %= mod_number;
                            }
                        },
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
    for i in 0..n {
        for j in 0..15_usize {
            for l in 0..10_usize {
                if i == 0 && l == 0 {
                    continue;
                }
                let nex = (10 * j + l) % 15;
                if nex != 0 && nex != 3 && nex != 5 &&nex != 6 && nex != 9 && nex != 10 && nex != 12 {
                    dp[i+1][nex][m] += dp[i][j][m];
                    dp[i+1][nex][m] %= mod_number;
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..15_usize {
        ans += dp[n][i][m];
        ans %= mod_number;
    }
    println!("{}", ans);
}
