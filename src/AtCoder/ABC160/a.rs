use proconio::input;

fn main() {
    input! {
        s: proconio::marker::Chars,
    }

    if s[2] == s[3] && s[4] == s[5] {
        println!("Yes");
    } else {
        println!("No");
    }
}
