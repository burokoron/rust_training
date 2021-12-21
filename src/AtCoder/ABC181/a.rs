use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    if n % 2 == 1{
        println!("Black");
    } else {
        println!("White");
    }
}
