use proconio::input;
 
fn main() {
    input! {
        m: usize,
        r: usize,
    }
 
    let distance = [
        [0, 1, 2, 3, 2, 3, 4, 3, 4, 5],
        [1, 0, 1, 2, 1, 2, 3, 2, 3, 4],
        [2, 1, 0, 1, 2, 1, 2, 3, 2, 3],
        [3, 2, 1, 0, 3, 2, 1, 4, 3, 1],
        [2, 1, 2, 3, 0, 1, 2, 1, 2, 3],
        [3, 2, 1, 2, 1, 0, 1, 2, 1, 2],
        [4, 3, 2, 1, 2, 1, 0, 3, 2, 1],
        [3, 2, 3, 4, 1, 2, 3, 0, 1, 2],
        [4, 3, 2, 3, 2, 1, 2, 1, 0, 1],
        [5, 4, 3, 2, 3, 2, 1, 2, 1, 0],
    ];
    let mut dp: Vec<Vec<(usize, Vec<usize>)>> = vec![vec![(0, Vec::new()); m]; 16];
 
    dp[0][0] = (1, vec![0]);
    for i in 0..15_usize {
        let mut count = vec![(100000000_usize, Vec::new()); m];
        for j in 0..m {
            if dp[i][j].0 == 0 {
                continue;
            }
 
            for k in 0..10_usize {
                let nex = (10 * j + k) % m;
                let mut d = 1000000000_usize;
                for l in 0..dp[i][j].1.len() {
                    if d > distance[dp[i][j].1[l]][k] as usize + dp[i][j].0 + 1 {
                        d = distance[dp[i][j].1[l]][k] as usize + dp[i][j].0 + 1;
                    }
                }
                if count[nex].0 > d {
                    count[nex] = (d, vec![k]);
                } else if count[nex].0 == d {
                    count[nex].1.push(k);
                }
            }
        }
 
        for j in 0..m {
            if count[j].0 == 100000000 {
                count[j] = (0, vec![0]);
            }
            if dp[i][j].0 == 0 {
                dp[i+1][j] = count[j].clone();
            } else if count[j].0 == 0 {
                dp[i+1][j] = dp[i][j].clone();
            } else {
                if dp[i][j].0 > count[j].0 {
                    dp[i+1][j] = count[j].clone();
                } else if dp[i][j].0 == count[j].0 {
                    dp[i+1][j] = dp[i][j].clone();
                    for k in 0..count[j].1.len() {
                        dp[i+1][j].1.push(count[j].1[k]);
                    }
                } else {
                    dp[i+1][j] = dp[i][j].clone();
                }
            }
        }
    }
 
    println!("{}", dp[15][r].0 - 1);
}
