use proconio::input;

fn main() {
    input! {
        _: i32,
        mut x: i32,
        s: proconio::marker::Chars,
    }

    for &i in &s {
        if i == 'o' {
            x += 1;
        } else if x > 0 {
            x -= 1
        }
    }

    println!("{}", x);
}
