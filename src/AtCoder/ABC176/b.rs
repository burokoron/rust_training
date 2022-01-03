use proconio::input;

fn main() {
    input! {
        n: proconio::marker::Bytes,
    }

    let mut sum = 0;

    for &i in &n {
        sum += (i - b'0') as i64;
    }

    if sum % 9 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
