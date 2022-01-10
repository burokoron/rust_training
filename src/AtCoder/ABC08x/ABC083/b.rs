use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32,
    }

    let mut ans = 0;
    for i in 1..n+1 {
        let mut num = i;
        let mut sum = 0;
        for j in 0..5 {
            sum += num / 10_i32.pow(4 - j);
            num %= 10_i32.pow(4 - j);
        }
        if a <= sum && sum <= b {
            ans += i;
        }
    }

    print!("{}", ans);
}
