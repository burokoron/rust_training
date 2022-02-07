use proconio::input;

fn main() {
    input! {
        n: usize,
        b: usize,
        k: usize,
        c: [usize; k],
    }

    fn pow_mod(a: usize, b: usize, mod_number: usize) -> usize {
        let mut p = 1;
        let mut q = a;

        for i in 0..63 {
            if (b & (1 << i)) != 0 {
                p *= q;
                p %= mod_number;
            }
            q *= q;
            q %= mod_number;
        }

        return p;
    }

    let mut power10: Vec<usize> = Vec::new();
    let mod_number = 1000000007_usize;

    for i in 0..63 {
        power10.push(pow_mod(10, 1 << i, b));
    }

    let mut dp: Vec<Vec<usize>> = vec![vec![0; b]; 64];

    for i in 0..k {
        dp[0][c[i] % b] += 1;
    }

    for i in 0..62 {
        for j in 0..b {
            for l in 0..b {
                let nex = (j * power10[i] + l) % b;
                dp[i+1][nex] += dp[i][j] * dp[i][l];
                dp[i+1][nex] %= mod_number;
            }
        }
    }

    let mut ans: Vec<Vec<usize>> = vec![vec![0; b]; 64];

    ans[0][0] = 1;
    for i in 0..62 {
        if (n & (1 << i)) != 0 {
            for j in 0..b {
                for l in 0..b {
                    let nex = (j * power10[i] + l) % b;
                    ans[i+1][nex] += ans[i][j] * dp[i][l];
                    ans[i+1][nex] %= mod_number;
                }
            }
        } else {
            for j in 0..b {
                ans[i+1][j] = ans[i][j];
            }
        }
    }

    println!("{}", ans[62][0]);
}
