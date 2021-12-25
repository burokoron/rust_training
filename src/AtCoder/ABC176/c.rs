use proconio::input;

fn main() {
    input! {
        n: i64,
        a: [i64; n],
    }

    let mut ans = 0;
    let mut previous = a[0];
    for &i in &a{
        if previous > i{
            ans += previous - i;
        }
        previous = previous.max(i);
    }

    println!("{}", ans);
}
