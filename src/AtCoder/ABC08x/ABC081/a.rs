use proconio::input;

fn main() {
    input! {
        s: proconio::marker::Chars,
    }

    let mut ans = 0;
    for &i in &s {
        if i == '1' {
            ans += 1;
        }
    }

    println!("{}", ans);
}
