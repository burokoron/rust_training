use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    }

    if y % 2 == 0 &&  x * 2 <= y && y <= x * 4{
        println!("Yes");
    } else {
        println!("No");
    }
}
