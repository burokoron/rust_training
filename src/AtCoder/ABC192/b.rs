use proconio::input;

fn main() {
    input! {
        s: proconio::marker::Chars,
    }

    let mut ans = true;
    for i in 0..s.len() {
        if i % 2 == 0 && s[i].is_ascii_uppercase() {
            ans = false;
            break;
        } else if i % 2 == 1 && s[i].is_ascii_lowercase() {
            ans = false;
            break;
        }
    }

    println!("{}", if ans { "Yes" } else { "No" });
}
