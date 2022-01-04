use proconio::input;

fn main() {
    input! {
        h: i32,
        w: i32,
        a: [i32; h*w],
    }

    let mut min = 200;
    for &i in &a {
        min = min.min(i);
    }

    let mut ans = 0;
    for &i in &a {
        ans += i - min;
    }

    println!("{}", ans);
}
