use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut count = 0;
    let mut ans = 0;
    for c in s.chars() {
        match c {
            'R' => count += 1,
            'S' => count = 0,
            _ => panic!(),
        }
        ans = ans.max(count);
    }

    println!("{}", ans);
}
