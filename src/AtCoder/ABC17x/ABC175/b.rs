use proconio::input;

fn main() {
    input! {
        n: usize,
        mut l: [i64; n],
    }

    let mut ans: i64 = 0;
    for i in 0..n {
        for j in i+1..n {
            for k in j+1..n {
                if l[i] > l[j] && l[i] > l[k] && l[j] != l[k] {
                    if (l[j] - l[k]).abs() < l[i] && l[i] < l[j] + l[k] {
                        ans += 1;
                    }
                } else if l[j] > l[i] && l[j] > l[k] && l[i] != l[k] {
                    if (l[i] - l[k]).abs() < l[j] && l[j] < l[i] + l[k] {
                        ans += 1;
                    }
                } else if l[k] > l[i] && l[k] > l[j] && l[i] != l[j] {
                    if (l[i] - l[j]).abs() < l[k] && l[k] < l[i] + l[j] {
                        ans += 1;
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
