use proconio::input;
 
fn main() {
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

    input! {
        n: usize,
        b: usize,
        k: usize,
        c: [usize; k],
    }

    let mut a: Vec<Vec<usize>> = vec![vec![0; b]; b];
    for i in 0..b {
        for j in 0..k {
            let nex = (10 * i + c[j]) % b;
            a[i][nex] += 1;
        }
    }
    let mut ans: Vec<Vec<usize>> = vec![vec![0; 1]; b];
    ans[0][0] = 1;
    let mod_number = 1000000007_usize;

    let mut an = vec![a; 1];
    let mut count = 2;
    while count < n {
        let mut tmp = an[an.len()-1].clone();
        tmp = dot_mod(&tmp, &tmp, mod_number);
        an.push(tmp);
        if count > 10_usize.pow(18) {
            break;
        }
        count *= 2;
    }

    let mut remain = n;
    while remain > 1 {
        let mut idx = 1;
        let mut count = 2;
        loop {
            if count > 10_usize.pow(18) {
                break;
            }
            if count * 2 > remain {
                break;
            } else {
                count *= 2;
                idx += 1;
            }
        }
        ans = dot_mod(&an[idx], &ans, mod_number);
        remain -= count;
    }
    if remain == 1 {
        ans = dot_mod(&an[0], &ans, mod_number);
    }

    println!("{}", ans[0][0]);
}
