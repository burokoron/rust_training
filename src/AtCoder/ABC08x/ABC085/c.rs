use proconio::input;

fn main() {
    input! {
        n: i64,
        y: i64,
    }

    for i in 0..n+1 {
        for j in 0..n+1-i {
            let k = n - i - j;
            if i * 10000 + j * 5000 + k * 1000 == y {
                println!("{} {} {}", i, j, k);
                return;
            }
        }
    }

    println!("-1 -1 -1");
}
