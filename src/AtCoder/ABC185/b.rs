use proconio::input;

fn main() {
    input! {
        n: i64,
        m: usize,
        t: i64,
        ab: [(i64, i64); m],
    }

    let mut battery = n - ab[0].0;
    for i in 0..m {
        if battery <= 0 {
            break;
        }
        battery += ab[i].1 - ab[i].0;
        if battery > n {
            battery = n;
        }
        if i != m - 1 {
            battery -= ab[i+1].0 - ab[i].1;
        }
    }
    battery -= t - ab[m-1].1;

    if battery > 0 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}
