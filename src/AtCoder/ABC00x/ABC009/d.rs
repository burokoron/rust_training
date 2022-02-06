use proconio::input;
 
fn main() {
    input! {
        k: usize,
        m: usize,
        a: [usize; k],
        c: [usize; k],
    }

    if k >= m {
        println!("{}", a[m-1]);
        return;
    }

    fn dot_logical(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let mut ans: Vec<Vec<usize>> = vec![vec![0; b[0].len()]; b.len()];
        for i in 0..a.len() {
            for j in 0..b.len() {
                for k in 0..b[0].len() {
                    ans[i][k] ^= a[i][j] & b[j][k];
                }
            }
        }
    
        return ans;
    }

    let mut left: Vec<Vec<usize>> = vec![vec![0; k]; k];
    let mut right: Vec<Vec<usize>> = vec![vec![0; 1]; k];
    for i in 0..k {
        for j in 0..k {
            if i == 0 {
                left[i][j] = c[j];
            } else if i == j + 1 {
                left[i][j] = !0;
            }
        }
    }
    for i in 0..k {
        right[i][0] = a[k-1-i];
    }

    let mut left_vec: Vec<Vec<Vec<usize>>> = Vec::new();
    left_vec.push(left);
    let mut count = 1;
    while m-k > count {
        let tmp = dot_logical(&left_vec[left_vec.len()-1], &left_vec[left_vec.len()-1]);
        left_vec.push(tmp);
        count *= 2;
    }

    count = m - k;
    while count > 1 {
        for i in 0..left_vec.len() {
            if count < 2_usize.pow(i as u32 + 1) {
                right = dot_logical(&left_vec[i], &right);
                count -= 2_usize.pow(i as u32);
                break;
            }
        }
    }
    if count == 1 {
        right = dot_logical(&left_vec[0], &right);
    }

    println!("{}", right[0][0]);
}
