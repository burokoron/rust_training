use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }

    let mut dot: i64 = 0;
    for i in 0..n {
        dot += a[i] * b[i];
    }

    if dot == 0 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}
