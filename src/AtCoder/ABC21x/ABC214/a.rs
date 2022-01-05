use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    match n {
        i if i <= 125 => println!("4"),
        i if i <= 211 => println!("6"),
        i if i <= 214 => println!("8"),
        _ => unreachable!(),
    }
}
