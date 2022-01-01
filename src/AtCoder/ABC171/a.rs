use proconio::input;

fn main() {
    input! {
        alpha: char,
    }

    let ans = match alpha {
        'a'..='z' => 'a',
        'A'..='Z' => 'A',
        _ => panic!(),
    };

    println!("{}", ans);
}
