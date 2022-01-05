use proconio::input;

fn main() {
    input! {
        s: proconio::marker::Chars,
    }

    for &i in &s {
        print!("{}", i);
    }
    if s[s.len() - 1] == 's' {
        println!("es");
    } else {
        println!("s");
    }
}
