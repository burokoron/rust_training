// modを考慮した行列積の計算
fn dot_mod(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>, mod_number: usize) -> Vec<Vec<usize>> {
    let mut ans: Vec<Vec<usize>> = vec![vec![0; b[0].len()]; b.len()];
    for i in 0..a.len() {
        for j in 0..b.len() {
            for k in 0..b[0].len() {
                ans[i][k] += a[i][j] * b[j][k];
                ans[i][k] %= mod_number;
            }
        }
    }

    return ans;
}
