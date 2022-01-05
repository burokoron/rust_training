use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    match n % 10 {
        2 | 4 | 5 | 7 | 9 => println!("hon"),
        0 | 1 | 6 | 8 => println!("pon"),
        3 => println!("bon"),
        _ => unreachable!(),
    }
}
