use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [proconio::marker::Chars; h],
    }

    let mut up = vec![vec![0; w]; h];
    let mut right = vec![vec![0; w]; h];
    let mut down = vec![vec![0; w]; h];
    let mut left = vec![vec![0; w]; h];
    for j in 0..w {
        for i in 0..h {
            if s[i][j] == '.' {
                if i == 0 {
                    up[i][j] = 1;
                } else {
                    up[i][j] = up[i-1][j] + 1 ;
                }
            }
        }
    }
    for i in 0..h {
        for j in (0..w).rev() {
            if s[i][j] == '.' {
                if j + 1 == w {
                    right[i][j] = 1;
                } else {
                    right[i][j] = right[i][j+1] + 1;
                }
            }
        }
    }
    for j in 0..w {
        for i in (0..h).rev() {
            if s[i][j] == '.' {
                if i + 1 == h {
                    down[i][j] = 1
                } else {
                    down[i][j] = down[i+1][j] + 1;
                }
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                if j == 0 {
                    left[i][j] = 1;
                } else {
                    left[i][j] = left[i][j-1] + 1;
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            let count = up[i][j] + right[i][j] + down[i][j] + left[i][j] - 3;
            if ans < count {
                ans = count;
            }
        }
    }

    println!("{}", ans);
}
