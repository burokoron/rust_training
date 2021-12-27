use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut ans = 0;
    let mut max_gcd = 0;
    for i in 2..1001 {
        let mut gcd = 0;
        for &j in &a {
            if j % i == 0 {gcd += 1};
        }
        if max_gcd < gcd {
            max_gcd = gcd;
            ans = i;
        }
    }

    println!("{}", ans);
}
